// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::acars_messages::AcarsMessages;
use crate::components::map_display::ShMap;
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::prelude::*;
use yew_hooks::use_event_with_window;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Panels {
    Messages,
    Map,
    Settings,
    Help,
    None,
}

// implement to_string for Panels
impl std::fmt::Display for Panels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Messages => write!(f, "Messages"),
            Self::Map => write!(f, "Map"),
            Self::Settings => write!(f, "Settings"),
            Self::Help => write!(f, "Help"),
            Self::None => write!(f, "None"),
        }
    }
}

// implement try_from for Panels
impl Panels {
    fn from(s: &str) -> Panels {
        match s {
            "Messages" => Self::Messages,
            "Map" => Self::Map,
            "Settings" => Self::Settings,
            "Help" => Self::Help,
            _ => Self::None,
        }
    }
}

impl Panels {
    fn next(&self, skip: Panels) -> Panels {
        // go to the next panel, skipping the one we're currently on

        match self {
            Self::Messages => {
                if skip == Self::Map {
                    Self::Settings
                } else {
                    Self::Map
                }
            }
            Self::Map => {
                if skip == Self::Settings {
                    Self::Help
                } else {
                    Self::Settings
                }
            }
            Self::Settings => {
                if skip == Self::Help {
                    Self::Messages
                } else {
                    Self::Help
                }
            }
            Self::Help => {
                if skip == Self::Messages {
                    Self::Map
                } else {
                    Self::Messages
                }
            }
            Self::None => Self::None,
        }
    }

    fn previous(&self, skip: Panels) -> Panels {
        // go to the previous panel, skipping the one we're currently on

        match self {
            Self::Messages => {
                if skip == Self::Help {
                    Self::Settings
                } else {
                    Self::Help
                }
            }
            Self::Map => {
                if skip == Self::Messages {
                    Self::Help
                } else {
                    Self::Messages
                }
            }
            Self::Settings => {
                if skip == Self::Map {
                    Self::Messages
                } else {
                    Self::Map
                }
            }
            Self::Help => {
                if skip == Self::Settings {
                    Self::Map
                } else {
                    Self::Settings
                }
            }
            Self::None => Self::None,
        }
    }
}

/// Home page
#[function_component(Live)]
pub fn live() -> Html {
    log::debug!("Rendering Live page");
    // Grab the current panel state from storage:
    let left_panel = use_state(|| {
        let panel: Option<String> = LocalStorage::get("left_panel").unwrap_or_default();
        match panel {
            Some(panel) => Panels::from(panel.as_str()),
            None => Panels::Messages,
        }
    });
    let right_panel = use_state(|| {
        let panel: Option<String> = LocalStorage::get("right_panel").unwrap_or_default();
        match panel {
            Some(panel) => Panels::from(panel.as_str()),
            None => Panels::Messages,
        }
    });

    let right_panel_status = {
        let right_panel = right_panel.clone();
        match *right_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <div>{"Settings"}</div> },
            Panels::Help => html! { <div>{"Help"}</div> },
            Panels::None => html! { <div>{"None"}</div> },
        }
    };

    let left_panel_show = {
        match *left_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <div>{"Settings"}</div> },
            Panels::Help => html! { <div>{"Help"}</div> },
            Panels::None => html! { <div>{"None"}</div> },
        }
    };

    use_event_with_window("keydown", move |e: KeyboardEvent| {
        log::debug!("Key pressed: {}", e.key());

        let right_panel = right_panel.clone();
        let left_panel = left_panel.clone();

        // if control is pressed, with left arrow, go to the previous panel
        if e.key() == "F1" {
            right_panel.set(right_panel.previous(*left_panel));
            LocalStorage::set(
                "right_panel",
                right_panel.previous(*left_panel).to_string().as_str(),
            )
            .unwrap();
        }

        // if control is pressed, with right arrow, go to the next panel
        if e.key() == "F2" {
            right_panel.set(right_panel.next(*left_panel));
            LocalStorage::set(
                "right_panel",
                right_panel.next(*left_panel).to_string().as_str(),
            )
            .unwrap();
        }

        // if alt is pressed, with left arrow, go to the previous panel
        if e.key() == "F3" {
            left_panel.set(left_panel.previous(*right_panel));
            LocalStorage::set(
                "left_panel",
                left_panel.previous(*right_panel).to_string().as_str(),
            )
            .unwrap();
        }

        // if alt is pressed, with right arrow, go to the next panel
        if e.key() == "F4" {
            left_panel.set(left_panel.next(*right_panel));
            LocalStorage::set(
                "left_panel",
                left_panel.next(*right_panel).to_string().as_str(),
            )
            .unwrap();
        }
    });

    html! {
        <div class="content flex w-full h-screen">
            <div class="content p-2 m-0 mt-1 md:w-96 h-full w-full rounded-2xl border-[#8963ba] border-4" id="live-left">
                { left_panel_show.clone() }
             </div>
            <div class="content m-0 mt-1 ml-2 h-full w-full rounded-2xl border-[#8963ba] border-4 hidden md:block" style="overflow:hidden" id="live-right">
                { right_panel_status.clone() }
            </div>
        </div>
    }
}
