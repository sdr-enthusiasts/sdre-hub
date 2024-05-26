// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use sh_config::web::sh_web_config::ShWebConfig;
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Store)]
pub struct WebAppStateTemp {
    pub right_panel_visible: bool,
    pub config: Option<ShWebConfig>,
    pub websocket_connected: bool,
}

impl Default for WebAppStateTemp {
    fn default() -> Self {
        Self {
            right_panel_visible: true,
            config: None,
            websocket_connected: false,
        }
    }
}
