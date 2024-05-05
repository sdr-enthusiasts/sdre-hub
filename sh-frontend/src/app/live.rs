// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::acars_messages::AcarsMessages;
use crate::components::map_display::ShMap;
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_measure, use_size, use_window_size};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Panels {
    Messages,
    Map,
    Settings,
    Help,
    None,
}

impl Panels {
    fn next(&self, skip: Panels) -> Panels {
        // go to the next panel, skipping the one we're currently on

        match self {
            Panels::Messages => {
                if skip == Panels::Map {
                    Panels::Settings
                } else {
                    Panels::Map
                }
            }
            Panels::Map => {
                if skip == Panels::Settings {
                    Panels::Help
                } else {
                    Panels::Settings
                }
            }
            Panels::Settings => {
                if skip == Panels::Help {
                    Panels::Messages
                } else {
                    Panels::Help
                }
            }
            Panels::Help => {
                if skip == Panels::Messages {
                    Panels::Map
                } else {
                    Panels::Messages
                }
            }
            Panels::None => Panels::None,
        }
    }

    fn previous(&self, skip: Panels) -> Panels {
        // go to the previous panel, skipping the one we're currently on

        match self {
            Panels::Messages => {
                if skip == Panels::Help {
                    Panels::Settings
                } else {
                    Panels::Help
                }
            }
            Panels::Map => {
                if skip == Panels::Messages {
                    Panels::Help
                } else {
                    Panels::Messages
                }
            }
            Panels::Settings => {
                if skip == Panels::Map {
                    Panels::Messages
                } else {
                    Panels::Map
                }
            }
            Panels::Help => {
                if skip == Panels::Settings {
                    Panels::Map
                } else {
                    Panels::Settings
                }
            }
            Panels::None => Panels::None,
        }
    }
}

/// Home page
#[function_component(Live)]
pub fn live() -> Html {
    // TODO: Grab current state from local storage
    let left_panel = use_state(|| Panels::Messages);

    let node = use_node_ref();
    let state = use_size(node.clone());

    let window_state = use_window_size();
    let right_panel = use_state(|| if window_state.0 > 1000.0 {
        Panels::Map
    } else {
        Panels::None
    });

    log::debug!("Window size: {:?}", window_state);

    use_event_with_window("resize", move |_: KeyboardEvent| {
        let state = state.clone();
        let current_right_panel = right_panel.clone();
        if state.0 > 0 && *current_right_panel == Panels::None {
            right_panel.set(Panels::Map);
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
        if e.shift_key() && e.key() == "ArrowLeft" {
            right_panel.set(right_panel.previous(*left_panel));
        }

        // if control is pressed, with right arrow, go to the next panel
        if e.shift_key() && e.key() == "ArrowRight" {
            right_panel.set(right_panel.next(*left_panel));
        }

        // if alt is pressed, with left arrow, go to the previous panel
        if e.alt_key() && e.key() == "ArrowLeft" {
            left_panel.set(left_panel.previous(*right_panel));
        }

        // if alt is pressed, with right arrow, go to the next panel
        if e.alt_key() && e.key() == "ArrowRight" {
            left_panel.set(left_panel.next(*right_panel));
        }
    });

    html! {
        <div class="content flex w-full h-screen">
            <div class="content p-2 m-0 mt-1 md:w-96 h-full w-full rounded-2xl border-[#8963ba] border-4" id="live-left">
                { left_panel_show.clone() }
             </div>
            <div class="content m-0 mt-1 ml-2 h-full w-full rounded-2xl border-[#8963ba] border-4 hidden md:block" style="overflow:hidden" id="live-right" ref={node}>
                { right_panel_status.clone() }
            </div>
        </div>
    }
}
