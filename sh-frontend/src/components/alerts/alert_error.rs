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
    #[prop_or_default]
    pub on_confirm: Callback<()>,
    #[prop_or_default]
    pub on_cancel: Callback<()>,
}

#[function_component(AlertError)]
pub fn alert_error(props: &AlertErrorProps) -> Html {
    let show_alert = props.show_alert.clone();
    let on_confirm = props.on_confirm.clone();
    let on_cancel = props.on_cancel.clone();
    let title = props.title;
    let message = props.message;
    html! {
        <Alert
            message={message}
            timeout={5000}
            show_alert={show_alert}
            title={title}
            container_class={""}
            cancel_button_class={"mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200"}
            confirm_button_class={"mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200"}
            icon_type={"error"}
            position={"middle"}
            alert_class={"text-center rounded-md shadow-lg bg-red-500 text-white border border-yellow-700 p-4"}
            title_class={"text-black"}
            message_class={"text-black"}
            icon_class={"flex justify-center"}
            confirm_button_text={"Dismiss"}
            cancel_button_text={""}
            show_confirm_button={true}
            show_cancel_button={false}
            show_close_button={false}
            on_confirm={ on_confirm }
            on_cancel={ on_cancel }
            icon_color={"black"}
            icon_width={"50"}
            />
    }
}
