// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;
use yew_alert::Alert;

#[derive(Clone, PartialEq, Properties)]
pub struct AlertErrorProps {
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

// FIXME: Before ridding of tailwind the "position" part of this prop needs us to implement some more CSS classes. See: https://github.com/next-rs/yew-alert/blob/37da6d37d10cb32dc778d4f7a642800eb8188175/src/lib.rs#L233

#[function_component(AlertError)]
pub fn alert_error(props: &AlertErrorProps) -> Html {
    let show_alert = props.show_alert.clone();
    let on_confirm = props.on_confirm.clone();
    let on_cancel = props.on_cancel.clone();
    let show_cancel = props.show_cancel_button;
    let show_confirm = props.show_confirm_button;
    let title = props.title;
    let message = props.message;
    let timeout = props.timeout;
    let cancel_button_class = props.cancel_button_class;
    let confirm_button_class = props.confirm_button_class;

    html! {
        <Alert
            message={message}
            timeout={timeout}
            show_alert={show_alert}
            title={title}
            container_class={""}
            cancel_button_class={cancel_button_class}
            confirm_button_class={confirm_button_class}
            icon_type={"error"}
            position={"middle"}
            alert_class={"alert-error"}
            title_class={"text-background-color"}
            message_class={"text-background-color"}
            icon_class={"alert-icon"}
            confirm_button_text={"Dismiss"}
            cancel_button_text={"Cancel"}
            show_confirm_button={show_confirm}
            show_cancel_button={show_cancel}
            show_close_button={false}
            on_confirm={ on_confirm }
            on_cancel={ on_cancel }
            icon_color={"text-background-color"}
            icon_width={"50"}
            />
    }
}
