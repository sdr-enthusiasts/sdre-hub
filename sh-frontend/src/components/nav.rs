// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::{fmt, ops::Not};

use yew::prelude::*;

use crate::app::{live::Panels, Actions, MessageContext};

use super::search::Search;

// This enum seems a bit overkill. Originally, I wrote this to just use a standard bool, but I wanted to
// But the code's readability is improved by using this enum. Also, for state management in Yew, it's better to use
// To control the state of the menu, and more specifically to just know the state which now we do.

#[derive(Debug, Clone, Copy)]
enum Checked {
    True,
    False,
}

impl Not for Checked {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Self::True => Self::False,
            Self::False => Self::True,
        }
    }
}

impl From<Checked> for bool {
    fn from(checked: Checked) -> Self {
        match checked {
            Checked::True => true,
            Checked::False => false,
        }
    }
}

impl fmt::Display for Checked {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
        }
    }
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let msg_ctx = use_context::<MessageContext>().expect("No message context found!");
    let menu_state = use_state(|| Checked::False);

    let msg_ctx_map_right = msg_ctx.clone();
    let mouse_hide_menu_map_right = {
        let menu_state = menu_state.clone();
        Callback::from(move |_: MouseEvent| {
            menu_state.set(Checked::False);
            msg_ctx_map_right.dispatch(Actions::SetPanelRight(Panels::Map));
        })
    };

    let mouse_hide_menu_settings_right = {
        let menu_state = menu_state.clone();
        Callback::from(move |_: MouseEvent| {
            menu_state.set(Checked::False);
            msg_ctx.dispatch(Actions::SetPanelRight(Panels::Settings));
        })
    };

    let mouse_show_menu = {
        let menu_state = menu_state.clone();
        let current_state = *menu_state;
        Callback::from(move |_: MouseEvent| menu_state.set(!current_state))
    };

    let hidden_menu = {
        let menu_state = *menu_state;
        menu_state
    };

    html! {
      <section class="top-nav rounded-3xl">
          <div class="hidden lg:block lg:w-2/6">
                <img class="w-10 h-10" src="logo.svg" alt="SDR Enthusiasts Hub" />
          </div>
                <Search />
          <input id="menu-toggle" checked={hidden_menu.into()} onclick={mouse_show_menu} type="checkbox" />
          <label class="menu-button-container" for="menu-toggle">
                <div class="menu-button"></div>
          </label>
          // FIXME: I think we should be fixing the menu link stuff to a width based on container size?
          <ul class="menu text-[#101110]">
                <ul class="menu text-[#101110]">
                    <li onclick={mouse_hide_menu_map_right.clone()}>{ "Map" }</li>
                    // <li onclick={mouse_hide_menu.clone()}>{ "Messages" }</li>
                    <li onclick={mouse_hide_menu_settings_right.clone()}>{ "Settings" }</li>
                    // <li onclick={mouse_hide_menu.clone()}>{ "Help" }</li>
                </ul>
          </ul>
      </section>
    }
}
