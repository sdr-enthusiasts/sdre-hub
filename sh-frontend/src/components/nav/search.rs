// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

// Search component
#[function_component(Search)]
pub fn search() -> Html {
    log::debug!("Rendering search bar.");

    html! {
        <div class="search-bar">
            <input type="text" placeholder="Search..." />
        </div>
    }
}
