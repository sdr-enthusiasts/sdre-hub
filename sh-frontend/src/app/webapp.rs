// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::layout_components::footer::Footer;
use crate::components::layout_components::live::Live;
use crate::components::layout_components::nav::Nav;
use crate::services::websocket::ShWebSocketComponent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
        <ShWebSocketComponent />
        <div class="flex flex-col h-full w-full max-h-full max-w-full overflow-hidden">
            <Nav />
            <section class="container flex text-left p-0 pb-1 mt-1 mb-1 h-full w-full max-h-full max-w-full overflow-hidden">
                <Live />
            </section>
            <Footer />
        </div>
        </>
    }
}
