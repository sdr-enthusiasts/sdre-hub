use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let menu_state = use_state(|| false);

    let mouse_hide_menu = {
        let menu_state = menu_state.clone();
        Callback::from(move |_: MouseEvent| menu_state.set(false))
    };

    let hidden_menu = {
        let menu_state = *menu_state.clone();
        log::info!("Menu state: {:?}", menu_state);
        menu_state
    };

    html! {
      <section class="top-nav">
          <div>
              <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >
              <img class="w-10 h-10" src="logo.svg" alt="SDR Enthusiasts Hub" /></Link<AppRoute>>
          </div>
          <input id="menu-toggle" checked={hidden_menu} type="checkbox" />
          <label class="menu-button-container" for="menu-toggle">
              <div class="menu-button"></div>
          </label>
          <ul class="menu">
              <li onclick={mouse_hide_menu.clone()}><Link<AppRoute> to={AppRoute::Home} >{ "Home" }</Link<AppRoute>></li>
              <li onclick={mouse_hide_menu.clone()}><Link<AppRoute> to={AppRoute::About} >{ "About" }</Link<AppRoute>></li>
          </ul>
      </section>
    }
}
