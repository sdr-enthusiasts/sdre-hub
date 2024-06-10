// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::common::alert_boxes::AlertBoxToShow;
use crate::common::wssprops::WssCommunicationProps;
use crate::components::input::input_field::{InputField, InputFieldType};
use crate::components::setting_components::ButtonAction;
use crate::services::temp_state::WebAppStateTemp;
use serde::{Deserialize, Serialize};
use sh_common::{MessageData, UserMessageTypes, UserWssMessage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

// TODO: This entire panel should be hidden behind some kind of "advanced settings" toggle

#[derive(Clone, PartialEq, Store, Default, Serialize, Deserialize)]
#[store(storage = "local", storage_tab_sync)]
struct ConfigAppState {
    pub is_visible: bool,
}

#[function_component(ShAppConfig)]
pub fn sh_app_config(props: &WssCommunicationProps) -> Html {
    let config = use_selector(|state: &WebAppStateTemp| state.config.clone());
    let (state, dispatch) = use_store::<ConfigAppState>();
    let is_visible = use_state(|| state.is_visible);
    let current_visible = *is_visible;

    let database_url_node = use_node_ref();
    let log_level_node = use_node_ref();
    let data_path_node = use_node_ref();
    let log_levels = vec![
        "trace".to_string(),
        "debug".to_string(),
        "info".to_string(),
        "error".to_string(),
        "warn".to_string(),
    ];

    let local_props = props.clone();

    let onsubmit = {
        let config = config.clone();
        let database_url_node = database_url_node.clone();
        let log_level_node = log_level_node.clone();
        let data_path_node = data_path_node.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            match ButtonAction::from(event.submitter().unwrap().id()) {
                ButtonAction::Update => {
                    log::debug!("Saving config");

                    let database_url = database_url_node
                        .cast::<HtmlInputElement>()
                        .unwrap()
                        .value();
                    let log_level = log_level_node.cast::<HtmlInputElement>().unwrap().value();
                    let data_path = data_path_node.cast::<HtmlInputElement>().unwrap().value();

                    if let Some(config) = config.as_ref() {
                        let app = config.app.clone();
                        let database_url_original = app.database_url.clone();
                        let log_level_original = app.log_level.clone();
                        let data_path_original = app.data_path.clone();

                        if database_url != database_url_original
                            || log_level != log_level_original
                            || data_path != data_path_original
                        {
                            log::debug!("Values have changed");
                            // save the new values

                            let mut new_config = config.clone().app;
                            new_config.database_url = database_url;
                            new_config.log_level = log_level;
                            new_config.data_path = data_path;

                            let message = UserWssMessage::new(
                                UserMessageTypes::UserUpdateAppConfig,
                                MessageData::ShAppConfig(new_config),
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
                        let app = config.clone().app;
                        let database_url = app.database_url.clone();
                        let log_level = app.log_level.clone();
                        let data_path = app.data_path.clone();

                        database_url_node
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&database_url);
                        log_level_node
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&log_level);
                        data_path_node
                            .cast::<HtmlInputElement>()
                            .unwrap()
                            .set_value(&data_path);
                    }
                }
            }
        })
    };

    let current_state = is_visible.clone();
    let show_panel = {
        // lets see if the user has changed any of the values
        // if they have, we should prompt them to save the changes

        let config = config.clone();
        let database_url_node = database_url_node.clone();
        let log_level_node = log_level_node.clone();
        let data_path_node = data_path_node.clone();
        let show_alert = props.request_alert_box.clone();

        Callback::from(move |_: MouseEvent| {
            // if we are changing from not showing the panel to showing the panel, just save the current state
            // FIXME: Implement above logic to save literal nanoseconds of CPU time and memory lel
            if let Some(config) = config.as_ref() {
                let app = config.app.clone();
                let database_url = app.database_url.clone();
                let log_level = app.log_level.clone();
                let data_path = app.data_path.clone();

                // verify none of the inputs are empty

                let database_url_node = database_url_node
                    .cast::<HtmlInputElement>()
                    .unwrap()
                    .value();
                let log_level_node = log_level_node.cast::<HtmlInputElement>().unwrap().value();
                let data_path_node = data_path_node.cast::<HtmlInputElement>().unwrap().value();

                if database_url_node.is_empty()
                    || log_level_node.is_empty()
                    || data_path_node.is_empty()
                {
                    log::debug!("One of the fields is empty");
                    return;
                }

                // verify none of the inputs have changed

                if database_url_node == database_url
                    && log_level_node == log_level
                    && data_path_node == data_path
                {
                    log::debug!("None of the fields have changed");
                } else {
                    log::debug!("One of the fields has changed");
                    current_state.set(true);
                    dispatch.reduce_mut(move |state| state.is_visible = true);
                    // prompt the user to save the changes

                    show_alert.emit(AlertBoxToShow::UnsavedChanges);
                    return;
                }
            }
        })
    };

    html! {
        <>
        <input id="collapsible_app_config" class="toggle" type="checkbox" checked={current_visible} onclick={show_panel}/>
        <label for="collapsible_app_config" class="lbl-toggle">{"SDR-E Hub Configuration"}</label>
        <div class="collapsible-content">
          <div class="content-inner">
                {
                    if let Some(config) = config.as_ref() {
                        log::trace!("Config: {:?}", config);
                        let app = config.app.clone();
                        let database_url: String = app.database_url.clone();
                        let log_level: String = app.log_level.clone();
                        let data_path: String = app.data_path.clone();
                        html! {
                            <form onsubmit={onsubmit}>
                                <div class="settings-item"><InputField input_node_ref={database_url_node} label={"Database URL"} name={"databaseurl"} field_type={InputFieldType::Text} input_value={database_url} select_options={None} read_only={true} /></div>
                                <div class="settings-item"><InputField input_node_ref={log_level_node} label={"Log Level"} name={"loglevel"} field_type={InputFieldType::Select} input_value={log_level} select_options={Some(log_levels)} /></div>
                                <div class="settings-item"><InputField input_node_ref={data_path_node} label={"Data Path"} name={"datapath"} field_type={InputFieldType::Text} input_value={data_path} select_options={None} read_only={true}/></div>
                                <div class="settings-item buttons">
                                <div><button type="submit" class="button" id="update">{"Update Configuration"}</button></div>
                                <div><button type="submit" class="button" id="reset">{"Reset Configuration"}</button></div>
                                </div>
                            </form>
                        }
                    } else {
                        html!{ "Still loading!" }
                    }
                }
          </div>
        </div>
        </>
    }
}
