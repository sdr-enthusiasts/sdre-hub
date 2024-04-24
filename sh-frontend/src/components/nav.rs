use std::{fmt, ops::Not};

use yew::prelude::*;
use yew_router::prelude::*;

use crate::app::AppRoute;

// This enum seems a bit overkill. Originally, I wrote this to just use a standard bool, but I wanted to
// But the code's readability is improved by using this enum. Also, for state management in Yew, it's better to use
// To control the state of the menu, and more specifically to just know the state which now we do.

#[derive(Debug, Clone, Copy)]
enum Checked {
    True,
    False,
}

impl Not for Checked {
    type Output = Checked;

    fn not(self) -> Checked {
        match self {
            Checked::True => Checked::False,
            Checked::False => Checked::True,
        }
    }
}

impl From<Checked> for bool {
    fn from(checked: Checked) -> bool {
        match checked {
            Checked::True => true,
            Checked::False => false,
        }
    }
}

impl fmt::Display for Checked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Checked::True => write!(f, "True"),
            Checked::False => write!(f, "False"),
        }
    }
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let menu_state = use_state(|| Checked::False);

    let mouse_hide_menu = {
        let menu_state = menu_state.clone();
        Callback::from(move |_: MouseEvent| menu_state.set(Checked::False))
    };

    let mouse_show_menu = {
        let menu_state = menu_state.clone();
        let current_state = *menu_state.clone();
        Callback::from(move |_: MouseEvent| menu_state.set(!current_state))
    };

    let hidden_menu = {
        let menu_state = *menu_state.clone();
        log::info!("Menu state: {menu_state}");
        menu_state
    };

    html! {
      <section class="top-nav">
          <div>
              <Link<AppRoute> to={AppRoute::Home} classes="text-emerald-800 underline" >
              <img class="w-10 h-10" src="logo.svg" alt="SDR Enthusiasts Hub" /></Link<AppRoute>>
          </div>
          <input id="menu-toggle" checked={hidden_menu.into()} onclick={mouse_show_menu} type="checkbox" />
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
