// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::footer_components::connected::Connected;
use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer class="mt-1 p-0 flex flex-row h-12 w-full max-h14 max-w-full rounded-2xl border-[#8963ba] border-4 justify-center">
            <Connected />
            <div class="m-2">{ "Use of this source code is governed by an MIT-style" }</div>
        </footer>
    }
}
