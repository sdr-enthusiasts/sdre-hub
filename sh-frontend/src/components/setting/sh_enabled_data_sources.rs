// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

#[function_component(ShEnabledDataSourcesConfig)]
pub fn sh_enabled_data_sources_config() -> Html {
    html! {
        <>
        <input id="collapsible_enabled_data_sources" class="toggle" type="checkbox" />
        <label for="collapsible_enabled_data_sources" class="lbl-toggle">{"Enabled Data Sources"}</label>
        <div class="collapsible-content">
          <div class="content-inner">
            <p>
              {"QUnit is by calling one of the object that are embedded in JavaScript, and faster JavaScript program could also used with
              its elegant, well documented, and functional programming using JS, HTML pages Modernizr is a popular browsers without
              plug-ins. Test-Driven Development."}
            </p>
          </div>
        </div>
        </>
    }
}
