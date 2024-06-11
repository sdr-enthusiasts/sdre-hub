// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

pub mod sh_app_config;
pub mod sh_data_sources;
pub mod sh_enabled_data_sources;
pub mod sh_map;

pub enum ButtonAction {
    Update,
    Reset,
}

impl From<String> for ButtonAction {
    fn from(action: String) -> Self {
        match action.to_lowercase().as_str() {
            "update" => ButtonAction::Update,
            "reset" => ButtonAction::Reset,
            _ => ButtonAction::Update,
        }
    }
}
