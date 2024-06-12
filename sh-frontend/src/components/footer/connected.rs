// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::services::temp_state::WebAppStateTemp;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Connected)]
pub fn connected() -> Html {
    let connected = use_selector(|state: &WebAppStateTemp| state.websocket_connected);

    html! {
        <div> {
            if *connected {
                html!{<span class="text-sdre-green-lighter">{"Connected"}</span>}
            } else {
                html!{<span class="text-sdre-red-darker">{"Disconnected"}</span>}
            }
         }
         </div>
    }
}
