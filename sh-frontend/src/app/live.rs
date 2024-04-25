// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;
use yew_hooks::use_event_with_window;

/// Home page
#[function_component(Live)]
pub fn live() -> Html {
    use_event_with_window("keydown", move |e: KeyboardEvent| {
        log::debug!("Key pressed: {}", e.key());
    });

    html! {
        <div class="content flex width-max height-max">
            <div class="content flex h-max max-w-96 ">
                {"ACARS Box"}
             </div>
            <div class="content flex pl-2">
                {"Map Box"}
            </div>
        </div>
    }
}
