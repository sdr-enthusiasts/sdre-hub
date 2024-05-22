// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

#[function_component(ShMapConfig)]
pub fn sh_map_config() -> Html {
    html! {
        <>
        <input id="collapsible_map_config" class="toggle" type="checkbox" />
        <label for="collapsible_map_config" class="lbl-toggle">{"Map Configuration"}</label>
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
