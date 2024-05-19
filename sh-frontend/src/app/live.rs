// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::app::Actions;
use crate::components::help::ShHelp;
use crate::components::map_display::ShMap;
use crate::components::settings::ShSettings;
use crate::components::stats::ShStatistics;
use crate::{app::MessageContext, components::acars_messages::AcarsMessages};
use gloo::storage::LocalStorage;
use gloo_storage::Storage;
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_visible};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Panels {
    Messages,
    Map,
    Stats,
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
            Self::Stats => write!(f, "Stats"),
            Self::Settings => write!(f, "Settings"),
            Self::Help => write!(f, "Help"),
            Self::None => write!(f, "None"),
        }
    }
}

// implement try_from for Panels
impl Panels {
    fn from(s: &str) -> Self {
        match s {
            "Messages" => Self::Messages,
            "Map" => Self::Map,
            "Settings" => Self::Settings,
            "Stats" => Self::Stats,
            "Help" => Self::Help,
            _ => Self::None,
        }
    }
}

impl Panels {
    #[must_use]
    pub fn next(&self, skip: Self) -> Self {
        // go to the next panel, skipping the one we're currently on

        match self {
            Self::Messages => {
                if skip == Self::Map {
                    Self::Stats
                } else {
                    Self::Map
                }
            }
            Self::Map => {
                if skip == Self::Stats {
                    Self::Settings
                } else {
                    Self::Map
                }
            }
            Self::Stats => {
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

    #[must_use]
    pub fn previous(&self, skip: Self) -> Self {
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
            Self::Stats => {
                if skip == Self::Map {
                    Self::Messages
                } else {
                    Self::Map
                }
            }
            Self::Settings => {
                if skip == Self::Stats {
                    Self::Map
                } else {
                    Self::Stats
                }
            }
            Self::Help => {
                if skip == Self::Settings {
                    Self::Stats
                } else {
                    Self::Settings
                }
            }

            Self::None => Self::None,
        }
    }
}

// FIXME: we get a bunch of double rendering on menu switch. The cause is that we have the panel state
// checking the old value vs the new one and setting the panel state if it's changed. This flags a re-render
/// Home page
#[function_component(Live)]
pub fn live() -> Html {
    let msg_ctx = use_context::<MessageContext>().unwrap();
    log::debug!("msg_ctx: {:?}", msg_ctx);
    let node = use_node_ref();
    let visible = use_visible(node.clone(), false);

    // set the visibility of the right panel
    msg_ctx.dispatch(Actions::SetRightPanelVisible(visible));

    // Grab the current panel state from storage:
    let left_panel = use_state_eq(|| {
        let panel: Option<String> = LocalStorage::get("left_panel").unwrap_or_default();

        panel.map_or(Panels::Messages, |panel| {
            // set the context
            msg_ctx.dispatch(Actions::SetPanelLeft(Panels::from(panel.as_str())));
            Panels::from(panel.as_str())
        })
    });

    let right_panel = use_state_eq(|| {
        let panel: Option<String> = LocalStorage::get("right_panel").unwrap_or_default();

        panel.map_or(Panels::Messages, |panel| {
            // set the context
            msg_ctx.dispatch(Actions::SetPanelRight(Panels::from(panel.as_str())));
            Panels::from(panel.as_str())
        })
    });

    // The state object is only every run once, so if the context changed we need to update the actual value

    if msg_ctx.left_panel != *left_panel && msg_ctx.left_panel != Panels::None {
        log::debug!("Setting left panel");
        left_panel.set(msg_ctx.left_panel);
        // set the local storage
        LocalStorage::set("left_panel", msg_ctx.left_panel.to_string().as_str()).unwrap();
    }

    if msg_ctx.right_panel != *right_panel && msg_ctx.right_panel != Panels::None {
        log::debug!("Setting right panel");
        right_panel.set(msg_ctx.right_panel);
        // set the local storage
        LocalStorage::set("right_panel", msg_ctx.right_panel.to_string().as_str()).unwrap();
    }

    // FIXME: we shouldn't have the same panel on both sides

    // FIXME: We probably shouldn't panic here, and instead alert the user that something went wrong
    // and reset the panels to a default state.

    let right_panel_status = {
        let right_panel = right_panel.clone();
        match *right_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <ShSettings />},
            Panels::Help => html! { <ShHelp /> },
            Panels::Stats => html! { <ShStatistics /> },
            Panels::None => panic!("Right Panel is none!!!"),
        }
    };

    let left_panel_show = {
        match *left_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <ShSettings />},
            Panels::Help => html! { <ShHelp /> },
            Panels::Stats => html! { <ShStatistics /> },
            Panels::None => panic!("Left Panel is none!!!"),
        }
    };

    let right_panel_clone = right_panel.clone();
    let left_panel_clone = left_panel.clone();

    use_event_with_window("keydown", move |e: KeyboardEvent| {
        // if control is pressed, with left arrow, go to the previous panel
        if visible && e.key() == "F1" {
            // set the context
            msg_ctx.dispatch(Actions::SetPanelRight(
                right_panel_clone.previous(*left_panel_clone),
            ));
            right_panel_clone.set(right_panel_clone.previous(*left_panel_clone));
            LocalStorage::set(
                "right_panel",
                right_panel_clone
                    .previous(*left_panel_clone)
                    .to_string()
                    .as_str(),
            )
            .unwrap();
        }

        // if control is pressed, with right arrow, go to the next panel
        if visible && e.key() == "F2" {
            // set the context
            msg_ctx.dispatch(Actions::SetPanelRight(
                right_panel_clone.next(*left_panel_clone),
            ));
            right_panel_clone.set(right_panel_clone.next(*left_panel_clone));
            LocalStorage::set(
                "right_panel",
                right_panel_clone
                    .next(*left_panel_clone)
                    .to_string()
                    .as_str(),
            )
            .unwrap();
        }

        // if alt is pressed, with left arrow, go to the previous panel
        if e.key() == "F3" {
            let previous = if visible {
                *right_panel_clone
            } else {
                Panels::None
            };

            // set the context
            msg_ctx.dispatch(Actions::SetPanelLeft(left_panel_clone.previous(previous)));
            left_panel_clone.set(left_panel_clone.previous(previous));
            LocalStorage::set(
                "left_panel",
                left_panel_clone.previous(previous).to_string().as_str(),
            )
            .unwrap();
        }

        // if alt is pressed, with right arrow, go to the next panel
        if e.key() == "F4" {
            let previous = if visible {
                *right_panel_clone
            } else {
                Panels::None
            };

            // set the context
            msg_ctx.dispatch(Actions::SetPanelLeft(left_panel_clone.next(previous)));
            left_panel_clone.set(left_panel_clone.next(previous));
            LocalStorage::set(
                "left_panel",
                left_panel_clone.next(previous).to_string().as_str(),
            )
            .unwrap();
        }
    });

    html! {
        <div class="content flex w-full h-full">
            <div class="content p-2 m-0 mt-1 md:w-96 h-full w-full rounded-2xl border-[#8963ba] border-4" id="live-left">
                { left_panel_show.clone() }
             </div>
            <div class="content m-0 mt-1 ml-2 h-full w-full rounded-2xl border-[#8963ba] border-4 hidden md:block" style="overflow:hidden" id="live-right" ref={node}>
                if visible {
                    { right_panel_status.clone() }
                }
            </div>
        </div>
    }
}
