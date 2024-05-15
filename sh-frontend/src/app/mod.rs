// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::rc::Rc;

use sh_common::{MessageData, UserMessageTypes, UserWssMessage};
use yew::prelude::*;

pub mod live;

use crate::{components::nav::Nav, services::websocket::WebsocketService};
use live::Live;

use self::live::Panels;

pub struct App {
    _wss: WebsocketService,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Message {
    pub left_panel: Panels,
    pub right_panel: Panels,
    pub right_panel_visible: bool,
}

impl Default for Message {
    fn default() -> Self {
        Self {
            left_panel: Panels::None,
            right_panel: Panels::None,
            right_panel_visible: true,
        }
    }
}

pub enum Actions {
    SetPanelLeft(Panels),
    SetPanelRight(Panels),
    SetRightPanelVisible(bool),
}

impl Reducible for Message {
    type Action = Actions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Actions::SetPanelLeft(panel) => Rc::new(Self {
                left_panel: panel,
                right_panel: self.right_panel,
                right_panel_visible: self.right_panel_visible,
            }),

            Actions::SetPanelRight(panel) => Rc::new(Self {
                left_panel: self.left_panel,
                right_panel: panel,
                right_panel_visible: self.right_panel_visible,
            }),
            Actions::SetRightPanelVisible(visible) => Rc::new(Self {
                left_panel: self.left_panel,
                right_panel: self.right_panel,
                right_panel_visible: visible,
            }),
        }
    }
}

pub type MessageContext = UseReducerHandle<Message>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MessageProvider(props: &MessageProviderProps) -> Html {
    let msg = use_reducer(|| Message::default());

    html! {
        <ContextProvider<MessageContext> context={msg}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let wss = WebsocketService::new();
        let message = UserWssMessage::new(UserMessageTypes::UserRequestConfig, MessageData::None);

        match serde_json::to_string(&message) {
            Ok(message) => {
                if let Ok(_) = wss.tx.clone().try_send(message) {
                    log::info!("Sent message to server");
                } else {
                    log::error!("Failed to send message to server");
                }
            }
            Err(e) => {
                log::error!("Error serializing message: {}", e);
            }
        }

        Self { _wss: wss }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <MessageProvider>
                <div class="flex min-h-screen flex-col h-full w-full">
                    <Nav />
                    <section class="container text-left p-0 mt-1 h-full w-full max-w-full">
                        <Live />
                    </section>
                </div>
            </MessageProvider>
        }
    }
}
