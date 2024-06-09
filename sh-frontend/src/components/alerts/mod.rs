// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://github.com/next-rs/yew-alert

pub mod error;
pub mod notice;

use notice::AlertNotice;
use error::AlertError;
use yew::prelude::*;
use yew_alert::Alert;

// FIXME: Before ridding of tailwind the "position" part of this prop needs us to implement some more CSS classes. See: https://github.com/next-rs/yew-alert/blob/37da6d37d10cb32dc778d4f7a642800eb8188175/src/lib.rs#L233

#[derive(Clone, PartialEq)]
pub enum AlertType {
    Error(AlertError),
    Notice(AlertNotice),
}

impl Default for AlertType {
    fn default() -> Self {
        AlertType::Notice(AlertNotice::new())
    }
}

impl AlertPropsTrait for AlertType {
    fn new() -> Self {
        AlertType::Notice(AlertNotice::new())
    }

    fn get_position(&self) ->  &'static str {
        match self {
            AlertType::Error(details) => details.get_position(),
            AlertType::Notice(details) => details.get_position(),
        }
    }

    fn get_icon_type(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_icon_type(),
            AlertType::Notice(details) => details.get_icon_type(),
        }
    }

    fn get_alert_class(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_alert_class(),
            AlertType::Notice(details) => details.get_alert_class(),
        }
    }

    fn get_title_class(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_title_class(),
            AlertType::Notice(details) => details.get_title_class(),
        }
    }

    fn get_message_class(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_message_class(),
            AlertType::Notice(details) => details.get_message_class(),
        }
    }

    fn get_icon_class(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_icon_class(),
            AlertType::Notice(details) => details.get_icon_class(),
        }
    }

    fn get_confirm_button_text(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_confirm_button_text(),
            AlertType::Notice(details) => details.get_confirm_button_text(),
        }
    }

    fn get_cancel_button_text(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_cancel_button_text(),
            AlertType::Notice(details) => details.get_cancel_button_text(),
        }
    }

    fn get_icon_color(&self) -> &'static str {
        match self {
            AlertType::Error(details) => details.get_icon_color(),
            AlertType::Notice(details) => details.get_icon_color(),
        }
    }
}

pub trait AlertPropsTrait {
    fn new() -> Self;
    fn get_position(&self) -> &'static str;
    fn get_icon_type(&self) -> &'static str;
    fn get_alert_class(&self) -> &'static str;
    fn get_title_class(&self) -> &'static str;
    fn get_message_class(&self) -> &'static str;
    fn get_icon_class(&self) -> &'static str;
    fn get_confirm_button_text(&self) -> &'static str;
    fn get_cancel_button_text(&self) -> &'static str;
    fn get_icon_color(&self) -> &'static str;
}

#[derive(Clone, PartialEq, Properties)]
pub struct ShAlertProps {
    #[prop_or_default]
    pub alert_type: AlertType,
    #[prop_or_default]
    pub message: &'static str,
    #[prop_or_default]
    pub title: &'static str,
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

#[function_component(ShAlert)]
pub fn sh_alert(props: &ShAlertProps) -> Html {
    let show_alert_as_state = use_state_eq(|| props.show_alert);
    let on_confirm = props.on_confirm.clone();
    let on_cancel = props.on_cancel.clone();
    let show_cancel = props.show_cancel_button;
    let show_confirm = props.show_confirm_button;
    let title = props.title;
    let message = props.message;
    let cancel_button_class = props.cancel_button_class;
    let confirm_button_class = props.confirm_button_class;
    let timeout = props.timeout;
    let new_show_alert = show_alert_as_state.clone();

    let alert_type = props.alert_type.clone();
    let position = alert_type.get_position();
    let icon_type = alert_type.get_icon_type();
    let alert_class = alert_type.get_alert_class();
    let title_class = alert_type.get_title_class();
    let message_class = alert_type.get_message_class();
    let icon_class = alert_type.get_icon_class();
    let confirm_button_text = alert_type.get_confirm_button_text();
    let cancel_button_text = alert_type.get_cancel_button_text();
    let icon_color = alert_type.get_icon_color();

    use_effect_with(props.show_alert, move |show_alert| {
        show_alert_as_state.set(show_alert.clone());
    });

    if *new_show_alert != props.show_alert {
        on_confirm.emit(());
    }

    html! {
        <Alert
            message={message}
            timeout={timeout}
            show_alert={new_show_alert}
            title={title}
            container_class={""}
            cancel_button_class={cancel_button_class}
            confirm_button_class={confirm_button_class}
            icon_type={icon_type}
            position={position}
            alert_class={alert_class}
            title_class={title_class}
            message_class={message_class}
            icon_class={icon_class}
            confirm_button_text={confirm_button_text}
            cancel_button_text={cancel_button_text}
            show_confirm_button={show_confirm}
            show_cancel_button={show_cancel}
            show_close_button={false}
            on_confirm={ on_confirm }
            on_cancel={ on_cancel }
            icon_color={icon_color}
            icon_width={"50"}
            />
    }
}