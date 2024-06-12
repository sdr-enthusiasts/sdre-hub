// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use super::base::{IconType, Position};
use super::AlertPropsTrait;

#[derive(Default, Clone, Eq, PartialEq)]
pub struct AlertNotice {}

impl AlertPropsTrait for AlertNotice {
    fn new() -> Self {
        Self {}
    }

    fn get_position(&self) -> Position {
        Position::BottomRight
    }

    fn get_icon_type(&self) -> IconType {
        IconType::Info
    }

    fn get_alert_class(&self) -> &'static str {
        "alert-notification bottom-0 right-0"
    }

    fn get_title_class(&self) -> &'static str {
        "text-background-color"
    }

    fn get_message_class(&self) -> &'static str {
        "text-background-color"
    }

    fn get_icon_class(&self) -> &'static str {
        "alert-icon"
    }

    fn get_confirm_button_text(&self) -> &'static str {
        "Dismiss"
    }

    fn get_cancel_button_text(&self) -> &'static str {
        "Cancel"
    }

    fn get_icon_color(&self) -> &'static str {
        "text-background-color"
    }
}
