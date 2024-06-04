// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://github.com/next-rs/yew-alert

pub mod alert_error;
pub mod alert_config;

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct AlertProps {
    #[prop_or_default]
    pub message: &'static str,
    #[prop_or_default]
    pub title: &'static str,
    pub show_alert: UseStateHandle<bool>,
    #[prop_or(Callback::noop())]
    pub on_confirm: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_cancel: Callback<()>,
    #[prop_or(5000)]
    pub timeout: u32,
    #[prop_or("button")]
    pub cancel_button_class: &'static str,
    #[prop_or("button")]
    pub confirm_button_class: &'static str,
    #[prop_or_default]
    pub show_cancel_button: bool,
    #[prop_or(true)]
    pub show_confirm_button: bool,
}

#[derive(Clone, PartialEq, Properties)]
pub struct AlertPropsAlternate {
    #[prop_or_default]
    pub counter: u32,
    #[prop_or_default]
    pub message: &'static str,
    #[prop_or_default]
    pub title: &'static str,
    #[prop_or_default]
    pub show_alert: bool,
    #[prop_or(Callback::noop())]
    pub on_confirm: Callback<()>,
    #[prop_or(Callback::noop())]
    pub on_cancel: Callback<()>,
    #[prop_or(5000)]
    pub timeout: u32,
    #[prop_or("button")]
    pub cancel_button_class: &'static str,
    #[prop_or("button")]
    pub confirm_button_class: &'static str,
    #[prop_or_default]
    pub show_cancel_button: bool,
    #[prop_or(true)]
    pub show_confirm_button: bool,
}
