// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::rc::Rc;

use sh_common::{MessageData, ServerMessageTypes, ServerWssMessage};
use sh_config::web::sh_web_config::ShWebConfig;
use yew::prelude::*;
use crate::components::nav::Nav;
use crate::services::websocket::ShWebSocketComponent;
use super::live::Live;

// FIXME: move the actual permanent app state stuff to yewdux. Use this as the messaging pipeline for temp states
#[derive(Debug, PartialEq, Clone)]
pub struct ShAppState {
    pub right_panel_visible: bool,
    pub config: Option<ShWebConfig>,
}

impl Default for ShAppState {
    fn default() -> Self {
        Self {
            right_panel_visible: true,
            config: None,
        }
    }
}

pub enum Actions {
    SetRightPanelVisible(bool),
    WsReceivedMessage(ServerWssMessage),
}

impl Reducible for ShAppState {
    type Action = Actions;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Actions::SetRightPanelVisible(visible) => Rc::new(Self {
                right_panel_visible: visible,
                config: self.config.clone(),
            }),
            Actions::WsReceivedMessage(msg) => match msg.get_message_type() {
                ServerMessageTypes::ServerResponseConfig => {
                    let data = msg.get_data();
                    match data {
                        MessageData::ShConfig(config) => Rc::new(Self {
                            right_panel_visible: self.right_panel_visible,
                            config: Some(config.clone()),
                        }),
                        MessageData::NoData => self,
                    }
                }
            },
        }
    }
}

pub type MessageContext = UseReducerHandle<ShAppState>;

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn MessageProvider(props: &MessageProviderProps) -> Html {
    let msg_tempstate = use_reducer(|| ShAppState::default());

    html! {
        <ContextProvider<MessageContext> context={msg_tempstate}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <MessageProvider>
        <ShWebSocketComponent />
            <div class="flex flex-col h-full w-full max-h-full max-w-full">
                <Nav />
                <section class="container text-left p-0 mt-1 h-full w-full max-h-full max-w-full">
                    <Live />
                </section>
            </div>
        </MessageProvider>
    }
}
