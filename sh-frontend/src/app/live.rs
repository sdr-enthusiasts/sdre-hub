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
        <div class="content flex max-w-none pr-1">
            <div class="content w-96">
                {"ACARS Box"}
             </div>
            <div class="content w-full">
                {"Map Box"}
            </div>
        </div>
    }
}
