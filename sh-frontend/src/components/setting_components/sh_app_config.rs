// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.


use std::rc::Rc;

use sh_config::web::sh_web_config::ShWebConfig;
use yew::prelude::*;
use yewdux::prelude::*;
use crate::services::temp_state::WebAppStateTemp;
use crate::components::input::input_field::InputField;

#[function_component(ShAppConfig)]
pub fn sh_app_config() -> Html {
    let (state, dispatch) = use_store::<WebAppStateTemp>();

    let config = use_selector(|state: &WebAppStateTemp | state.config.clone());

    let database_url_node = use_node_ref();

    let onsubmit = {
            Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            log::debug!("Saving config");
        })
    };

    html! {
        <>
        <input id="collapsible_app_config" class="toggle" type="checkbox" />
        <label for="collapsible_app_config" class="lbl-toggle">{"SDR-E Hub Configuration"}</label>
        <div class="collapsible-content">
          <div class="content-inner">
                {
                    if let Some(config) = config.as_ref() {
                        log::debug!("Config: {:?}", config);
                        let app = config.app.clone();
                        log::debug!("App: {:?}", app);
                        let database_url: String = app.database_url.clone();
                        html! {
                            <form onsubmit={onsubmit} class="registration-form">
                                <InputField input_node_ref={database_url_node} label={"Database URL"} name={"databaseurl"} field_type={"text"} input_value={database_url} />
                                <button type="submit" class="button">{"Update Configuration"}</button>
                            </form>
                        }
                    } else {
                        html!{ "Still loading!" }
                    }
                    // let config = Rc::<std::option::Option<ShWebConfig>>::get_mut(&mut config).unwrap().as_ref().unwrap();
                    // let app = config.app.clone();
                    // let database_url: String = app.database_url.clone();
                }
          </div>
        </div>
        </>
    }
}
