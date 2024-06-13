// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::common::alert_boxes::AlertBoxToShow;
use crate::common::wssprops::WssCommunicationProps;
use crate::components::input::field::{
    make_sure_string_has_five_digits, CoordinateType, NumberProperties,
};
use crate::components::input::field::{InputField, InputFieldType};
use crate::components::setting::ButtonAction;
use crate::services::temp_state::WebAppStateTemp;
use serde::{Deserialize, Serialize};
use sh_common::{MessageData, UserMessageTypes, UserWssMessage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

// TODO: This entire panel should be hidden behind some kind of "advanced settings" toggle

#[derive(Clone, PartialEq, Store, Default, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
struct ConfigMapState {
    pub is_visible: bool,
}

#[function_component(ShMapConfig)]
pub fn sh_map_config(props: &WssCommunicationProps) -> Html {
    let config = use_selector(|state: &WebAppStateTemp| state.config.clone());
    let (state, dispatch) = use_store::<ConfigMapState>();
    let is_visible = use_state(|| state.is_visible);
    let current_visible = *is_visible;

    let latitude_node = use_node_ref();
    let longitude_node = use_node_ref();

    let local_props = props.clone();

    let onsubmit = {
        let config = config.clone();
        let latitude_node = latitude_node.clone();
        let longitude_node = longitude_node.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            match ButtonAction::from(event.submitter().unwrap().id()) {
                ButtonAction::Update => {
                    log::debug!("Saving config");

                    let latitude = latitude_node.cast::<HtmlInputElement>().unwrap().value();

                    let longitude = longitude_node.cast::<HtmlInputElement>().unwrap().value();

                    if let Some(config) = config.as_ref() {
                        let map = config.map.clone();
                        let center_latitude_original = map.center_latitude.to_string();
                        let center_longitude_original = map.center_longitude.to_string();

                        if latitude != center_latitude_original
                            || longitude != center_longitude_original
                        {
                            log::debug!("Values have changed");
                            // save the new values

                            let mut new_config = config.clone().map;
                            new_config.center_latitude = latitude.parse().unwrap();
                            new_config.center_longitude = longitude.parse().unwrap();

                            let message = UserWssMessage::new(
                                UserMessageTypes::UserUpdateMapConfig,
                                MessageData::ShMapConfig(new_config),
                            );

                            // send a message using the props callback
                            local_props.send_message.emit(message);
                        } else {
                            log::debug!("Values have not changed");
                        }
                    }
                }
                ButtonAction::Reset => {
                    log::debug!("Resetting config");
                    // set all the values back to the original values
                    if let Some(config) = config.as_ref() {
                        let map = config.clone().map;
                        let center_latitude_original =
                            make_sure_string_has_five_digits(&map.center_latitude.to_string());
                        let center_longitude_original =
                            make_sure_string_has_five_digits(&map.center_longitude.to_string());

                        latitude_node
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&center_latitude_original);

                        longitude_node
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&center_longitude_original);
                    }
                }
            }
        })
    };

    let current_state = is_visible;
    let show_panel = {
        // lets see if the user has changed any of the values
        // if they have, we should prompt them to save the changes

        let config = config.clone();
        let latitude_node = latitude_node.clone();
        let longitude_node = longitude_node.clone();
        let show_alert = props.request_alert_box.clone();

        Callback::from(move |_: MouseEvent| {
            // if we are changing from not showing the panel to showing the panel, just save the current state
            // FIXME: Implement above logic to save literal nanoseconds of CPU time and memory lel
            if let Some(config) = config.as_ref() {
                let map = config.clone().map;
                let center_latitude_original =
                    make_sure_string_has_five_digits(&map.center_latitude.to_string());
                let center_longitude_original =
                    make_sure_string_has_five_digits(&map.center_longitude.to_string());

                // verify none of the inputs are empty

                let longitude_node = longitude_node.cast::<HtmlInputElement>().unwrap().value();
                let latitude_node = latitude_node.cast::<HtmlInputElement>().unwrap().value();

                if longitude_node.is_empty() || latitude_node.is_empty() {
                    log::debug!("One of the fields is empty");
                    dispatch.reduce_mut(move |state| state.is_visible = true);
                    return;
                }

                // verify none of the inputs have changed

                if longitude_node == center_longitude_original
                    && latitude_node == center_latitude_original
                {
                    if *current_state {
                        current_state.set(false);
                        dispatch.reduce_mut(move |state| state.is_visible = false);
                    } else {
                        current_state.set(true);
                        dispatch.reduce_mut(move |state| state.is_visible = true);
                    }
                } else {
                    log::debug!("One of the fields has changed");
                    current_state.set(true);
                    dispatch.reduce_mut(move |state| state.is_visible = true);
                    // prompt the user to save the changes

                    show_alert.emit(AlertBoxToShow::UnsavedChanges);
                }
            }
        })
    };

    html! {
        <>
        <input id="collapsible_map_config" class="toggle" type="checkbox" checked={current_visible} onclick={show_panel} />
        <label for="collapsible_map_config" class="lbl-toggle">{"Map Configuration"}</label>
        <div class="collapsible-content">
          <div class="content-inner">
            {
              config.as_ref().as_ref().map_or_else(|| html! { "Loading..." }, |config| {
                  let map = config.map.clone();

                  let latitude = make_sure_string_has_five_digits(&map.center_latitude.to_string());
                  let longitude = make_sure_string_has_five_digits(&map.center_longitude.to_string());

                  let longitude_options = NumberProperties::new(&CoordinateType::Longitude, longitude.clone());
                  let latitude_options = NumberProperties::new(&CoordinateType::Latitude, latitude.clone());

                  html! {
                      <form onsubmit={onsubmit}>
                          <div class="settings-item"><InputField input_node_ref={latitude_node} label={"Map Latitude"} name={"maplatitude"} input_value={latitude} field_type={InputFieldType::Number} number_properties={Some(latitude_options)} /></div>
                          <div class="settings-item"><InputField input_node_ref={longitude_node} label={"Map Longitude"} name={"maplongitude"} field_type={InputFieldType::Number} input_value={longitude} number_properties={Some(longitude_options)}/></div>
                          <div class="settings-item buttons">
                          <div><button type="submit" class="button" id="update">{"Update Configuration"}</button></div>
                          <div><button type="submit" class="button" id="reset">{"Reset Configuration"}</button></div>
                          </div>
                      </form>
                  }
              })
          }
          </div>
        </div>
        </>
    }
}
