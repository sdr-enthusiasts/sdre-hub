// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// https://github.com/next-rs/yew-alert

use gloo::timers::callback::Timeout;
use yew::prelude::*;

const TITLE: &'static str = "Info";
const ALERT_CLASS: &'static str = "w-96 h-48 text-white";
const ICON_CLASS: &'static str = "flex justify-center";
const CONFIRM_BUTTON_TEXT: &'static str = "Okay";
const CANCEL_BUTTON_TEXT: &'static str = "Cancel";
const CONFIRM_BUTTON_CLASS: &'static str =
    "mt-2 mx-2 px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600 focus:outline-none focus:border-blue-700 focus:ring focus:ring-blue-200";
const CANCEL_BUTTON_CLASS: &'static str =
    "mt-2 mx-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600 focus:outline-none focus:border-gray-700 focus:ring focus:ring-gray-200";
const CONTAINER_CLASS: &'static str =
    "flex items-center text-center justify-center bg-gray-800 text-white border border-gray-600";
const TITLE_CLASS: &'static str = "dark:text-white";
const MESSAGE_CLASS: &'static str = "dark:text-gray-300";
const ICON_COLOR: &'static str = "";
const ICON_WIDTH: &'static str = "50";

#[derive(Debug, PartialEq, Clone, Default)]
pub enum Position {
    TopLeft,
    TopRight,
    Middle,
    Bottom,
    Top,
    #[default]
    BottomRight,
    BottomLeft,
}

impl Position {
    pub fn get_class(&self) -> &'static str {
        match self {
            Position::TopLeft => "top-0 left-0",
            Position::TopRight => "top-0 right-0",
            Position::Middle => "top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2",
            Position::Bottom => "bottom-0 left-1/2 transform -translate-x-1/2",
            Position::Top => "top-0 left-1/2 transform -translate-x-1/2",
            Position::BottomRight => "bottom-0 right-0",
            Position::BottomLeft => "bottom-0 left-0",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Default)]
pub enum IconType {
    Warning,
    Error,
    Success,
    Info,
    #[default]
    Question,
}

impl IconType {
    pub fn get_default_color(&self) -> &'static str {
        match self {
            IconType::Warning => "#CC5500", // Burnt Orange
            IconType::Error => "#EE4B2B",   // Bright Red
            IconType::Success => "lightgreen",
            IconType::Info => "lightblue",
            IconType::Question => "lightgray",
        }
    }

    pub fn get_icon(&self, icon_width: &'static str, icon_color: &'static str) -> Html {
        match self {
            IconType::Warning => {
                html! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 512 512"
                    >
                        <path
                            d="M248.4 84.3c1.6-2.7 4.5-4.3 7.6-4.3s6 1.6 7.6 4.3L461.9 410c1.4 2.3 2.1 4.9 2.1 7.5c0 8-6.5 14.5-14.5 14.5H62.5c-8 0-14.5-6.5-14.5-14.5c0-2.7 .7-5.3 2.1-7.5L248.4 84.3zm-41-25L9.1 385c-6 9.8-9.1 21-9.1 32.5C0 452 28 480 62.5 480h387c34.5 0 62.5-28 62.5-62.5c0-11.5-3.2-22.7-9.1-32.5L304.6 59.3C294.3 42.4 275.9 32 256 32s-38.3 10.4-48.6 27.3zM288 368a32 32 0 1 0 -64 0 32 32 0 1 0 64 0zm-8-184c0-13.3-10.7-24-24-24s-24 10.7-24 24v96c0 13.3 10.7 24 24 24s24-10.7 24-24V184z"
                        />
                    </svg>
                }
            }
            IconType::Error => {
                html! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 20 20"
                    >
                        <path
                            d="M12.71,7.291c-0.15-0.15-0.393-0.15-0.542,0L10,9.458L7.833,7.291c-0.15-0.15-0.392-0.15-0.542,0c-0.149,0.149-0.149,0.392,0,0.541L9.458,10l-2.168,2.167c-0.149,0.15-0.149,0.393,0,0.542c0.15,0.149,0.392,0.149,0.542,0L10,10.542l2.168,2.167c0.149,0.149,0.392,0.149,0.542,0c0.148-0.149,0.148-0.392,0-0.542L10.542,10l2.168-2.168C12.858,7.683,12.858,7.44,12.71,7.291z M10,1.188c-4.867,0-8.812,3.946-8.812,8.812c0,4.867,3.945,8.812,8.812,8.812s8.812-3.945,8.812-8.812C18.812,5.133,14.867,1.188,10,1.188z M10,18.046c-4.444,0-8.046-3.603-8.046-8.046c0-4.444,3.603-8.046,8.046-8.046c4.443,0,8.046,3.602,8.046,8.046C18.046,14.443,14.443,18.046,10,18.046z"
                        />
                    </svg>
                }
            }
            IconType::Success => {
                html! {
                    <svg
                    xmlns="http://www.w3.org/2000/svg"
                    width={icon_width}
                    class="p-2 m-2"
                    fill={icon_color}
                    viewBox="0 0 512 512"
                    >
                        <path
                            d="M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM369 209c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0l-111 111-47-47c-9.4-9.4-24.6-9.4-33.9 0s-9.4 24.6 0 33.9l64 64c9.4 9.4 24.6 9.4 33.9 0L369 209z"
                        />
                    </svg>
                }
            }
            IconType::Info => {
                html! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 16 16"
                    >
                        <path
                            d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"
                        />
                        <path
                            d="M8.93 6.588-2.29.287-.082.38.45.083c.294.07.352.176.288.469l-.738 3.468c-.194.897.105 1.319.808 1.319.545 0 1.178-.252 1.465-.598l.088-.416c-.2.176-.492.246-.686.246-.275 0-.375-.193-.304-.533L8.93 6.588zM9 4.5a1 1 0 1 1-2 0 1 1 0 0 1 2 0z"
                        />
                    </svg>
                }
            }
            IconType::Question => {
                html! {
                    <svg
                        xmlns="http://www.w3.org/2000/svg"
                        width={icon_width}
                        class="p-2 m-2"
                        fill={icon_color}
                        viewBox="0 0 16 16"
                    >
                        <path
                            d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"
                        />
                        <path
                            d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"
                        />
                    </svg>
                }
            }
        }
    }
}

/// Props for the Alert component.
#[derive(Debug, PartialEq, Properties, Clone)]
pub struct AlertProps {
    /// The message to be displayed in the alert.
    #[prop_or_default]
    pub message: &'static str,

    /// State handle to control the visibility of the alert.
    pub show_alert: UseStateHandle<bool>,

    /// Time duration in milliseconds before the alert automatically closes.
    #[prop_or(2500)]
    pub timeout: u32,

    /// The title of the alert.
    #[prop_or(TITLE)]
    pub title: &'static str,

    /// CSS class for styling the alert.
    #[prop_or(ALERT_CLASS)]
    pub alert_class: &'static str,

    /// CSS class for styling the icon in the alert.
    #[prop_or(ICON_CLASS)]
    pub icon_class: &'static str,

    /// Text for the confirm button in the alert.
    #[prop_or(CONFIRM_BUTTON_TEXT)]
    pub confirm_button_text: &'static str,

    /// Text for the cancel button in the alert.
    #[prop_or(CANCEL_BUTTON_TEXT)]
    pub cancel_button_text: &'static str,

    /// CSS class for styling the confirm button.
    #[prop_or(CONFIRM_BUTTON_CLASS)]
    pub confirm_button_class: &'static str,

    /// CSS class for styling the cancel button.
    #[prop_or(CANCEL_BUTTON_CLASS)]
    pub cancel_button_class: &'static str,

    /// Flag to determine if the confirm button should be displayed.
    #[prop_or(true)]
    pub show_confirm_button: bool,

    /// Flag to determine if the cancel button should be displayed.
    #[prop_or(true)]
    pub show_cancel_button: bool,

    /// Flag to determine if the close button should be displayed.
    #[prop_or(false)]
    pub show_close_button: bool,

    /// Callback function triggered on confirm button click.
    #[prop_or_default]
    pub on_confirm: Callback<()>,

    /// Callback function triggered on cancel button click.
    #[prop_or_default]
    pub on_cancel: Callback<()>,

    /// Position of the alert on the screen (e.g., "top-left", "middle", "bottom-right").
    #[prop_or_default]
    pub position: Position,

    /// CSS class for styling the alert container.
    #[prop_or(CONTAINER_CLASS)]
    pub container_class: &'static str,

    /// CSS class for styling the title in the alert.
    #[prop_or(TITLE_CLASS)]
    pub title_class: &'static str,

    /// CSS class for styling the message in the alert.
    #[prop_or(MESSAGE_CLASS)]
    pub message_class: &'static str,

    /// Type of the icon to be displayed in the alert (e.g., "warning", "error", "success").
    #[prop_or_default]
    pub icon_type: IconType,

    /// Color of the icon in the alert.
    #[prop_or(ICON_COLOR)]
    pub icon_color: &'static str,

    /// Width of the icon in the alert.
    #[prop_or(ICON_WIDTH)]
    pub icon_width: &'static str,
}

/// Alert Component
#[function_component]
pub fn Alert(props: &AlertProps) -> Html {
    let show = *props.show_alert;
    let timeout = props.timeout.clone();
    let show_alert = props.show_alert.clone();

    use_effect_with(show_alert.clone(), move |show_alert| {
        if **show_alert {
            let show_alert = show_alert.clone();
            let handle = Timeout::new(timeout, move || show_alert.set(false)).forget();
            let clear_handle = move || {
                web_sys::Window::clear_timeout_with_handle(
                    &web_sys::window().unwrap(),
                    handle.as_f64().unwrap() as i32,
                );
            };

            Box::new(clear_handle) as Box<dyn FnOnce()>
        } else {
            Box::new(|| {}) as Box<dyn FnOnce()>
        }
    });

    let on_cancel = {
        let on_cancel = props.on_cancel.clone();

        Callback::from(move |_| {
            on_cancel.emit(());
            show_alert.set(false);
        })
    };
    let on_confirm = {
        let on_confirm = props.on_confirm.clone();

        Callback::from(move |_| {
            on_confirm.emit(());
        })
    };

    let position_class = props.position.get_class();

    let icon_color = if props.icon_color.is_empty() {
        props.icon_type.get_default_color()
    } else {
        props.icon_color
    };

    // SVGs taken from: https://fontawesome.com/icons
    let icon_path = props.icon_type.get_icon(props.icon_width, icon_color);

    html! {
        if show {
            <div
                class={format!("top-0 left-0 fixed p-3 z-10 transition duration-300 ease-in bg-gray-900 bg-opacity-75 w-screen h-screen {}", props.container_class)}
            >
                <div
                    class={format!("absolute items-center {} {}", props.alert_class, position_class)}
                >
                    { if props.show_close_button {
                        html! {
                            <button type="button" class="absolute top-0 right-0 p-2 m-4 text-white bg-black border rounded-xl border-2" onclick={on_cancel.clone()}>{"x"}</button>
                        }
                    } else {
                        html! {}
                    } }
                    <div class={props.icon_class}>{ icon_path }</div>
                    <strong class={props.title_class}>{ props.title }</strong>
                    <hr class="my-2 border-gray-600" />
                    <p class={props.message_class}>{ props.message }</p>
                    { if props.show_confirm_button {
                        html! {
                            <button class={props.confirm_button_class} onclick={on_confirm}>
                                {props.confirm_button_text}
                            </button>
                        }
                    } else {
                        html! {}
                    } }
                    { if props.show_cancel_button {
                        html! {
                            <button class={props.cancel_button_class} onclick={on_cancel.clone()}>
                                {props.cancel_button_text}
                            </button>
                        }
                    } else {
                        html! {}
                    } }
                </div>
            </div>
        }
    }
}
