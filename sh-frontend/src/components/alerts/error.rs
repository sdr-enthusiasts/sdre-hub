// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use super::base::{IconType, Position};
use super::AlertPropsTrait;

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ShAlertErrorBox {}

impl AlertPropsTrait for ShAlertErrorBox {
    fn new() -> Self {
        Self {}
    }

    fn get_position(&self) -> Position {
        Position::Middle
    }

    fn get_icon_type(&self) -> IconType {
        IconType::Error
    }

    fn get_alert_class(&self) -> &'static str {
        "alert-error top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
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
