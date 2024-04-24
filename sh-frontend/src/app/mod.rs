// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;
use yew_router::prelude::*;

pub mod about;
pub mod live;
pub mod settings;

use crate::components::nav::Nav;
use about::About;
use live::Live;
use settings::Settings;

/// App routes
#[derive(Routable, Debug, Clone, PartialEq, Eq)]
pub enum AppRoute {
    #[at("/about")]
    About,
    #[not_found]
    #[at("/page-not-found")]
    PageNotFound,
    #[at("/")]
    Live,
    #[at("/settings")]
    Settings,
}

/// Switch app routes
pub fn switch(routes: AppRoute) -> Html {
    match routes.clone() {
        AppRoute::Live => html! { <Live /> },
        AppRoute::About => html! { <About /> },
        AppRoute::Settings => html! { <Settings /> },
        AppRoute::PageNotFound => html! { "Page not found" },
    }
}

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <HashRouter>
            <div class="flex min-h-screen flex-col p-1">
                <Nav />
                <Switch<AppRoute> render={switch} />
            </div>
        </HashRouter>
    }
}
