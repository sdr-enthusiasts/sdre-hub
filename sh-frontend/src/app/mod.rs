// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;
use yew_router::prelude::*;

pub mod live;

use crate::components::nav::Nav;
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

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
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
