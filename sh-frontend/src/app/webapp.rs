// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::layout_components::footer::Footer;
use crate::components::layout_components::live::Live;
use crate::components::layout_components::nav::Nav;
use crate::services::temp_state::WebAppStateTemp;
use anyhow::Error;
use sh_common::{
    MessageData, ServerMessageTypes, ServerWssMessage, UserMessageTypes, UserWssMessage,
};
use yew::{html, Component, Context, Html};
use yew_websocket::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yewdux::Dispatch;

// https://github.com/security-union/yew-websocket/

#[derive(Debug)]
pub enum WsAction {
    Connect,
    SendData(UserWssMessage),
    Disconnect,
    Lost,
}

#[derive(Debug)]
pub enum Msg {
    WsAction(WsAction),
    WsReady(Result<String, Error>),
}

impl From<WsAction> for Msg {
    fn from(action: WsAction) -> Self {
        Self::WsAction(action)
    }
}

pub struct App {
    pub fetching: bool,
    pub ws: Option<WebSocketTask>,
    pub dispatch: Dispatch<WebAppStateTemp>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let dispatch = Dispatch::<WebAppStateTemp>::global();

        Self {
            fetching: false,
            ws: None,
            dispatch,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    let callback = ctx.link().callback(|data| Msg::WsReady(data));
                    let notification = ctx.link().batch_callback(|status| match status {
                        WebSocketStatus::Opened => {
                            let initial_message = UserWssMessage::new(
                                UserMessageTypes::UserRequestConfig,
                                MessageData::NoData,
                            );
                            Some(WsAction::SendData(initial_message).into())
                        }
                        WebSocketStatus::Closed | WebSocketStatus::Error => {
                            Some(WsAction::Lost.into())
                        }
                    });
                    let task = WebSocketService::connect_text(
                        "ws://127.0.0.1:3000/sdre-hub",
                        callback,
                        notification,
                    )
                    .unwrap();
                    self.ws = Some(task);
                    false
                }
                WsAction::SendData(data) => {
                    log::debug!("Sending data: {:?}", data);
                    log::debug!("Sending data: {:?}", serde_json::to_string(&data).unwrap());
                    let serialized_data = serde_json::to_string(&data).unwrap();
                    self.ws
                        .as_mut()
                        .unwrap()
                        .send(serde_json::to_string(&serialized_data).unwrap());

                    if self.fetching == false {
                        self.fetching = true;
                        self.dispatch.reduce_mut(|state| {
                            state.websocket_connected = true;
                        });
                    }
                    false
                }
                WsAction::Disconnect => {
                    self.ws.take();
                    log::info!(
                        "WebSocket connection disconnected. Why? This should be unreachable"
                    );
                    self.fetching = false;
                    self.dispatch.reduce_mut(|state| {
                        state.config = None;
                        state.websocket_connected = false;
                    });

                    false
                }
                WsAction::Lost => {
                    self.ws = None;
                    log::error!("WebSocket connection lost. Reconnecting");
                    self.fetching = false;
                    // reconnect
                    ctx.link().send_message(WsAction::Connect);
                    self.dispatch.reduce_mut(|state| {
                        state.config = None;
                        state.websocket_connected = false;
                    });

                    false
                }
            },
            Msg::WsReady(response) => {
                log::debug!("Received data: {:?}", response);

                if response.is_err() {
                    log::error!("Error: {:?}", response.err().unwrap());
                    return false;
                }

                let data = response.unwrap();
                // remove the first and last characters

                log::debug!(
                    "Received text message after trimming and replacement: {}",
                    data
                );

                let data_deserialized: ServerWssMessage = match serde_json::from_str(&data) {
                    Ok(message) => message,
                    Err(e) => {
                        log::error!("Error deserializing message: {:?}", e);
                        return false;
                    }
                };

                match data_deserialized.get_message_type() {
                    ServerMessageTypes::ServerResponseConfig => {
                        log::debug!("Received config message");
                        self.dispatch
                            .reduce_mut(|state| match data_deserialized.get_data() {
                                MessageData::ShConfig(config) => {
                                    state.config = Some(config.clone());
                                }
                                _ => {
                                    log::error!("Received invalid data type");
                                }
                            });
                    }

                    ServerMessageTypes::ServerWriteConfigFailure => {
                        match data_deserialized.get_data() {
                            MessageData::ShConfigFailure(data) => {
                                log::error!("Failed to write config: {}", data);
                            }
                            _ => {
                                log::error!("Invalid response type");
                            }
                        }
                        log::error!("Failed to write config");
                    }

                    ServerMessageTypes::ServerWriteConfigSuccess => {
                        // see if there is any data
                        match data_deserialized.get_data() {
                            MessageData::ShConfigSuccess(data) => {
                                log::info!("Config written successfully: {}", data);
                            }
                            MessageData::ShConfigFailure(_) => {
                                log::error!("Invalid response type");
                            }
                            _ => ()
                        }
                        log::info!("Config written successfully");
                    }
                }

                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.ws.is_none() {
            ctx.link().send_message(WsAction::Connect);
            log::info!("Connecting to WebSocket");
        } else {
            log::info!("WebSocket is connected");
            ctx.link()
                .send_message(WsAction::SendData(UserWssMessage::new(
                    UserMessageTypes::UserRequestConfig,
                    MessageData::NoData,
                )));
        }

        let send_data_to_wss = ctx.link().callback(move |input: UserWssMessage| {
            log::debug!("Got a message. Sending up the chain to the websocket!");
            Msg::WsAction(WsAction::SendData(input))
        });

        html! {
            <>
            <div class="flex flex-col h-full w-full max-h-full max-w-full overflow-hidden">
                <Nav />
                <section class="container flex text-left p-0 pb-1 mt-1 mb-1 h-full w-full max-h-full max-w-full overflow-hidden">
                    <Live send_message={send_data_to_wss}/>
                </section>
                <Footer />
            </div>
            </>
        }
    }
}
