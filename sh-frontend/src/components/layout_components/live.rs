// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::map_components::map_display::ShMap;
use crate::components::pages::acars_messages::AcarsMessages;
use crate::components::pages::help::ShHelp;
use crate::components::pages::settings::ShSettings;
use crate::components::pages::stats::ShStatistics;
use crate::services::saved_state::WebAppState;
use crate::{
    common::panels::Panels, common::wssprops::WssCommunicationProps,
    services::temp_state::WebAppStateTemp,
};
use yew::prelude::*;
use yew_hooks::{use_event_with_window, use_visible};
use yewdux::prelude::*;

// FIXME: we get a bunch of double rendering on menu switch. The cause is that we have the panel state
// checking the old value vs the new one and setting the panel state if it's changed. This flags a re-render
/// Home page
#[function_component(Live)]
pub fn live(props: &WssCommunicationProps) -> Html {
    let (_state_local, dispatch_local) = use_store::<WebAppStateTemp>();
    let (_state, dispatch) = use_store::<WebAppState>();
    log::debug!("Re-rendering live page");
    let node = use_node_ref();
    let visible = use_visible(node.clone(), false);

    // set the visibility of the right panel
    dispatch_local.reduce_mut(|state| {
        state.right_panel_visible = visible;
    });

    // Grab the current panel state from storage:
    let left_panel = use_selector(|state: &WebAppState| state.left_panel);

    let right_panel = use_selector(|state: &WebAppState| state.right_panel);

    // FIXME: we shouldn't have the same panel on both sides

    // FIXME: We probably shouldn't panic here, and instead alert the user that something went wrong
    // and reset the panels to a default state.

    let right_panel_status = {
        let right_panel = right_panel.clone();
        match *right_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <ShSettings send_message={props.send_message.clone()}/>},
            Panels::Help => html! { <ShHelp /> },
            Panels::Stats => html! { <ShStatistics /> },
            Panels::None => panic!("Right Panel is none!!!"),
        }
    };

    let left_panel_show = {
        match *left_panel {
            Panels::Messages => html! { <AcarsMessages /> },
            Panels::Map => html! { <ShMap /> },
            Panels::Settings => html! { <ShSettings send_message={props.send_message.clone()} />},
            Panels::Help => html! { <ShHelp /> },
            Panels::Stats => html! { <ShStatistics /> },
            Panels::None => panic!("Left Panel is none!!!"),
        }
    };

    let right_panel_clone = right_panel.clone();
    let right_panel_dispatch = dispatch.clone();
    let left_panel_clone = left_panel.clone();
    let left_panel_dispatch = dispatch.clone();

    use_event_with_window("keydown", move |e: KeyboardEvent| {
        // if control is pressed, with left arrow, go to the previous panel
        if visible && e.key() == "F1" {
            right_panel_dispatch.reduce_mut(|state| {
                state.right_panel = right_panel_clone.previous(*left_panel_clone);
            });
        }

        // if control is pressed, with right arrow, go to the next panel
        if visible && e.key() == "F2" {
            // set the context
            right_panel_dispatch.reduce_mut(|state| {
                state.right_panel = right_panel_clone.next(*left_panel_clone);
            });
        }

        // if alt is pressed, with left arrow, go to the previous panel
        if e.key() == "F3" {
            let previous = if visible {
                *right_panel_clone
            } else {
                Panels::None
            };

            left_panel_dispatch.reduce_mut(|state| {
                state.left_panel = left_panel_clone.previous(previous);
            });
        }

        // if alt is pressed, with right arrow, go to the next panel
        if e.key() == "F4" {
            let previous = if visible {
                *right_panel_clone
            } else {
                Panels::None
            };

            right_panel_dispatch.reduce_mut(|state| {
                state.right_panel = right_panel_clone.next(previous);
            });
        }
    });

    // FIXME: This feels garbage and yuck. Ideally we use tailwind classes and dynamically set them?
    let pad_inside_left = if *left_panel == Panels::Map {
        "padding: 0px;"
    } else {
        "padding: 0.5rem;"
    };

    let pad_inside_right = if *right_panel == Panels::Map {
        "padding: 0px;"
    } else {
        "padding: 0.5rem;"
    };

    html! {
        <>
            <div class="live-panel-left" style={pad_inside_left} id="live-left">
                { left_panel_show.clone() }
             </div>
            <div class="live-panel-right" style={pad_inside_right} id="live-right" ref={node}>
                if visible {
                    { right_panel_status.clone() }
                }
            </div>
        </>
    }
}
