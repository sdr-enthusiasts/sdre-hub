// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::common::alert_boxes::AlertBoxToShow;
use crate::components::alerts::error::ShAlertErrorBox;
use crate::components::alerts::AlertPropsTrait;
use crate::components::alerts::{AlertType, ShAlert};
use crate::components::layout::footer::Footer;
use crate::components::layout::live::Live;
use crate::components::layout::nav::Nav;
use crate::services::temp_state::WebAppStateTemp;
use anyhow::Error;
use sh_common::{
    MessageData, ServerMessageTypes, ServerWssMessage, UserMessageTypes, UserWssMessage,
};
use yew::prelude::*;
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
    ShowAlert(AlertBoxToShow),
    HideAlert,
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
    pub alert_box_type: AlertBoxToShow,
}

impl App {
    fn handle_wsaction_connect(&mut self, ctx: &Context<Self>) {
        let callback = ctx.link().callback(Msg::WsReady);
        let notification = ctx.link().batch_callback(|status| match status {
            WebSocketStatus::Opened => {
                let initial_message =
                    UserWssMessage::new(UserMessageTypes::UserRequestConfig, MessageData::NoData);
                Some(WsAction::SendData(initial_message).into())
            }
            WebSocketStatus::Closed | WebSocketStatus::Error => Some(WsAction::Lost.into()),
        });
        let task =
            WebSocketService::connect_text("ws://127.0.0.1:3000/sdre-hub", callback, notification)
                .unwrap();
        self.ws = Some(task);
    }

    fn handle_wsaction_send_data(&mut self, data: &UserWssMessage) {
        log::debug!("Sending data: {:?}", data);
        let serialized_data = serde_json::to_string(&data).unwrap();
        self.ws
            .as_mut()
            .unwrap()
            .send(serde_json::to_string(&serialized_data).unwrap());

        if !self.fetching {
            self.fetching = true;
            self.dispatch.reduce_mut(|state| {
                state.websocket_connected = true;
            });
        }
    }

    fn handle_wsaction_disconnect(&mut self) {
        self.ws.take();
        log::info!("WebSocket connection disconnected. Why? This should be unreachable");
        self.fetching = false;
        self.dispatch.reduce_mut(|state| {
            state.config = None;
            state.websocket_connected = false;
        });
    }

    fn handle_wsaction_lost(&mut self, ctx: &Context<Self>) {
        self.ws = None;
        log::error!("WebSocket connection lost. Reconnecting");
        self.fetching = false;
        // reconnect
        ctx.link().send_message(WsAction::Connect);
        self.dispatch.reduce_mut(|state| {
            state.config = None;
            state.websocket_connected = false;
        });
    }

    fn handle_wsaction_ready(&mut self, ctx: &Context<Self>, response: Result<String, Error>) {
        log::trace!("Received data: {:?}", response);

        if response.is_err() {
            log::error!("Error: {:?}", response.err().unwrap());
            return;
        }

        let data = response.unwrap();
        // remove the first and last characters

        let data_deserialized: ServerWssMessage = match serde_json::from_str(&data) {
            Ok(message) => message,
            Err(e) => {
                log::error!("Error deserializing message: {:?}", e);
                return;
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

                // lets regrab the config

                let initial_message =
                    UserWssMessage::new(UserMessageTypes::UserRequestConfig, MessageData::NoData);

                ctx.link().send_message(WsAction::SendData(initial_message));

                // show alert
                ctx.link()
                    .send_message(Msg::ShowAlert(AlertBoxToShow::ConfigWriteFailure));
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
                    _ => (),
                }
                log::info!("Config written successfully");

                // lets regrab the config

                let initial_message =
                    UserWssMessage::new(UserMessageTypes::UserRequestConfig, MessageData::NoData);

                ctx.link().send_message(WsAction::SendData(initial_message));

                // show alert
                ctx.link()
                    .send_message(Msg::ShowAlert(AlertBoxToShow::ConfigWriteSuccess));
            }
        }
    }

    fn handle_msg_show_alert(&mut self, alert_box_type: &AlertBoxToShow) {
        log::debug!("Showing alert box: {:?}", alert_box_type);
        match alert_box_type {
            AlertBoxToShow::ConfigWriteSuccess => {
                self.alert_box_type = AlertBoxToShow::ConfigWriteSuccess;
            }
            AlertBoxToShow::ConfigWriteFailure => {
                self.alert_box_type = AlertBoxToShow::ConfigWriteFailure;
            }
            AlertBoxToShow::UnsavedChanges => {
                self.alert_box_type = AlertBoxToShow::UnsavedChanges;
            }
            AlertBoxToShow::None => {
                self.alert_box_type = AlertBoxToShow::None;
            }
        }
    }

    fn handle_msg_hide_alert(&mut self) {
        self.alert_box_type = AlertBoxToShow::None;
    }
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
            alert_box_type: AlertBoxToShow::None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::WsAction(action) => match action {
                WsAction::Connect => {
                    self.handle_wsaction_connect(ctx);
                    false
                }
                WsAction::SendData(data) => {
                    self.handle_wsaction_send_data(&data);
                    false
                }
                WsAction::Disconnect => {
                    self.handle_wsaction_disconnect();
                    false
                }
                WsAction::Lost => {
                    self.handle_wsaction_lost(ctx);
                    false
                }
            },
            Msg::WsReady(response) => {
                self.handle_wsaction_ready(ctx, response);
                false
            }

            Msg::ShowAlert(alert_box_type) => {
                self.handle_msg_show_alert(&alert_box_type);
                true
            }

            Msg::HideAlert => {
                self.handle_msg_hide_alert();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.ws.is_none() {
            ctx.link().send_message(WsAction::Connect);
            log::info!("Connecting to WebSocket");
        }

        let send_data_to_wss = ctx.link().callback(move |input: UserWssMessage| {
            log::trace!("Got a message. Sending up the chain to the websocket!");
            Msg::WsAction(WsAction::SendData(input))
        });

        let hide_alert_box = ctx.link().callback(|()| Msg::HideAlert);
        let show_alert_box = ctx.link().callback(Msg::ShowAlert);

        html! {
            <>
                <div class="flex flex-col h-full w-full max-h-full max-w-full overflow-hidden">
                {
                    match self.alert_box_type {
                        AlertBoxToShow::ConfigWriteSuccess => {
                            html! {
                                <ShAlert show_alert={true} message={"Configuration successfully saved. If you changed the Log Level please restart the server/app"} title={"Configuration Successfully Written"} on_confirm={hide_alert_box} />
                            }
                        }
                        AlertBoxToShow::ConfigWriteFailure => {
                            html! {
                                <ShAlert show_alert={true} message={"Failed to write configuration. Please try again."} title={"Configuration Write Failed"} />
                            }
                        }
                        AlertBoxToShow::UnsavedChanges => {
                            html! {
                                <ShAlert show_alert={true} message={"You have unsaved changes. Please save changes, or reset, before continuing."} title={"Unsaved Changes"} on_confirm={hide_alert_box} alert_type={AlertType::Error(ShAlertErrorBox::new())}/>
                            }
                        }
                        AlertBoxToShow::None => {
                            html! {}
                        }
                    }
                }
                <Nav />
                <section class="container flex text-left p-0 pb-1 mt-1 mb-1 h-full w-full max-h-full max-w-full overflow-hidden">
                    <Live send_message={send_data_to_wss} request_alert_box={show_alert_box} />
                </section>
                <Footer />
            </div>
            </>
        }
    }
}
