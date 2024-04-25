// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

// Search component
#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <div class="absolute focus:outline-black" style="left: 0; right: 0; margin: auto; width: 200px; height: 40px;">
            <input type="text" class="border border-gray-300 rounded-lg text-black p-2 focus:ring-gray-50/30 focus:outline-none focus:ring-4" style="width: 200px; height: 40px;" placeholder="Search..." />
        </div>
    }
}
