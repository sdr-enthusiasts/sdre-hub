// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

/// Help page
#[function_component(ShHelp)]
pub fn help() -> Html {
    log::debug!("Rendering help page.");

    html! {
        <div class="m-1">
            <h1 class="pb-1">{"SDR-E Hub Help"}</h1>
            // TODO: Write help content
            <p>{"Welcome to SDR-E Hub help."}</p>
        </div>
    }
}
