// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::rc::Rc;

use serde::{Deserialize, Serialize};
use sh_common::{MessageData, ServerMessageTypes, ServerWssMessage};
use sh_config::web::sh_web_config::ShWebConfig;
use yew::prelude::*;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
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
