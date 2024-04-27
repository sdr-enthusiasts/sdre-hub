// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::map_display::ShMap;
use yew::prelude::*;
use yew_hooks::use_event_with_window;

/// Home page
#[function_component(Live)]
pub fn live() -> Html {
    use_event_with_window("keydown", move |e: KeyboardEvent| {
        log::debug!("Key pressed: {}", e.key());
    });

    html! {
        <div class="content flex pr-1 w-full h-screen">
            <div class="content p-2 m-1 md:w-96 h-full w-full rounded-2xl border-[#8963ba] border-2" id="live-left">
                {"ACARS Box"}
             </div>
            <div class="content p-2 m-1 h-full w-full rounded-2xl border-[#8963ba] border-2 hidden md:block" id="live-right">
                <ShMap />
            </div>
        </div>
    }
}
