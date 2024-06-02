// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::thread::Scope;

use crate::components::layout_components::footer::Footer;
use crate::components::layout_components::live::Live;
use crate::components::layout_components::nav::Nav;
use crate::services::websocket::{ShWebSocketComponent, Msg, WsAction};
use sh_common::UserWssMessage;
use yew::prelude::*;

pub struct App {}

impl Component for App {
    type Message = crate::services::websocket::Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("App update: {:?}", msg);
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let send_data_to_wss = ctx.link().callback(move |input: UserWssMessage| {
            log::debug!("Got a message. Sending up the chain to the websocket!");
            Msg::WsAction(WsAction::SendData(input))
        });

        html! {
            <>
            <ShWebSocketComponent />
            <div class="flex flex-col h-full w-full max-h-full max-w-full overflow-hidden">
                <Nav />
                <section class="container flex text-left p-0 pb-1 mt-1 mb-1 h-full w-full max-h-full max-w-full overflow-hidden">
                    <Live send_message={send_data_to_wss}/>
                </section>
                <Footer />
            </div>
            </>
        }
    }

}
