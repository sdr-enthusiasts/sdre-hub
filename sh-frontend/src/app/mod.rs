// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use sh_common::{MessageData, UserMessageTypes, UserWssMessage};
use yew::prelude::*;
use yew_router::prelude::*;

pub mod live;

use crate::{components::nav::Nav, services::websocket::WebsocketService};
use live::Live;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum ShAppRoute {
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Live,
}

/// Switch app routes
// we need this to stop clippy whinging about something we can't control.
#[allow(clippy::needless_pass_by_value)]
pub fn switch(routes: ShAppRoute) -> Html {
    match routes {
        ShAppRoute::Live => html! { <Live /> },
        ShAppRoute::PageNotFound => html! { "Page not found" },
    }
}

pub struct App {
    _wss: WebsocketService,
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let wss = WebsocketService::new();
        let message = UserWssMessage::new(UserMessageTypes::UserRequestConfig, MessageData::None);

        match serde_json::to_string(&message) {
            Ok(message) => {
                if let Ok(_) = wss.tx.clone().try_send(message) {
                    log::info!("Sent message to server");
                } else {
                    log::error!("Failed to send message to server");
                }
            }
            Err(e) => {
                log::error!("Error serializing message: {}", e);
            }
        }

        Self { _wss: wss }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <HashRouter>
                <div class="flex min-h-screen flex-col h-full w-full">
                    <Nav />
                    <section class="container text-left p-0 mt-1 h-full w-full">
                        <Switch<ShAppRoute> render={switch} />
                    </section>
                </div>
            </HashRouter>
        }
    }
}
