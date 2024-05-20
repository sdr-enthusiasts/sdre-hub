// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::rc::Rc;

use sh_common::{MessageData, ServerMessageTypes, ServerWssMessage};
use sh_config::web::sh_web_config::ShWebConfig;
use yew::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct WebAppStateTemp {
    pub right_panel_visible: bool,
    pub config: Option<ShWebConfig>,
}

impl Default for WebAppStateTemp {
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

impl Reducible for WebAppStateTemp {
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

pub type MessageContext = UseReducerHandle<WebAppStateTemp>;