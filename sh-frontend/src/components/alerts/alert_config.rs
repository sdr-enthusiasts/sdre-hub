// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;
use yew_alert::Alert;
use super::AlertPropsAlternate;

// FIXME: Before ridding of tailwind the "position" part of this prop needs us to implement some more CSS classes. See: https://github.com/next-rs/yew-alert/blob/37da6d37d10cb32dc778d4f7a642800eb8188175/src/lib.rs#L233

#[function_component(AlertConfig)]
pub fn alert_config(props: &AlertPropsAlternate) -> Html {
    let show_alert_as_state = use_state_eq(|| props.show_alert);
    let on_confirm = props.on_confirm.clone();
    let on_cancel = props.on_cancel.clone();
    let show_cancel = props.show_cancel_button;
    let show_confirm = props.show_confirm_button;
    let title = props.title;
    let message = props.message;
    let timeout = props.timeout;
    let cancel_button_class = props.cancel_button_class;
    let confirm_button_class = props.confirm_button_class;

    log::debug!("AlertConfig: show_alert: {:?}", props.show_alert);
    log::debug!("AlertConfig: show_alert_as_state: {:?}", show_alert_as_state.clone());
    let new_show_alert = show_alert_as_state.clone();

    use_effect_with(props.counter, move |_show_alert| {
        show_alert_as_state.set(true)
    });

    html! {
        <Alert
            message={message}
            timeout={timeout}
            show_alert={new_show_alert}
            title={title}
            container_class={""}
            cancel_button_class={cancel_button_class}
            confirm_button_class={confirm_button_class}
            icon_type={"info"}
            position={"bottom-right"}
            alert_class={"alert-notification"}
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
