// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::setting_components::{
    sh_app_config::ShAppConfig, sh_data_sources::ShDataSourcesConfig,
    sh_enabled_data_sources::ShEnabledDataSourcesConfig, sh_map::ShMapConfig,
};
use yew::prelude::*;

/// Home page
#[function_component(ShSettings)]
pub fn settings() -> Html {
    html! {
        <>
            <div class="settings h-full w-full max-h-full max-w-full overflow-y-scroll">
                <ShAppConfig />
                <ShEnabledDataSourcesConfig />
                <ShDataSourcesConfig />
                <ShMapConfig />
            </div>
        </>
    }
}
