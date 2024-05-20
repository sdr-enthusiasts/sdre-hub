// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use super::live::Live;
use crate::components::message_provider::MessageProvider;
use crate::components::nav::Nav;
use crate::services::websocket::ShWebSocketComponent;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <MessageProvider>
        <ShWebSocketComponent />
            <div class="flex flex-col h-full w-full max-h-full max-w-full">
                <Nav />
                <section class="container text-left p-0 mt-1 h-full w-full max-h-full max-w-full">
                    <Live />
                </section>
            </div>
        </MessageProvider>
    }
}
