// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use yewdux::prelude::*;
use crate::common::panels::Panels;

#[derive(Clone, PartialEq, Eq, Store, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
pub struct WebAppState {
    pub left_panel: Panels,
    pub right_panel: Panels,
}

impl Default for WebAppState {
    fn default() -> Self {
        Self {
            left_panel: Panels::Messages,
            right_panel: Panels::Map,
        }
    }
}