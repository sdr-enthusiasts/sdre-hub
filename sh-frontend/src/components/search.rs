// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

// Search component
#[function_component(Search)]
pub fn search() -> Html {
    html! {
        <div class="flex flex-col items-center justify-center">
            <input type="text" class="border border-gray-300 rounded-lg mt-1 text-black" placeholder="Search..." />
        </div>
    }
}
