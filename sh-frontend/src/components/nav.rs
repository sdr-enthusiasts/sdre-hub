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

#[derive(PartialEq)]
enum PanelSide {
    Left,
    Right,
}

struct MenuItemState {
    context: MessageContext,
    menu_state: UseStateHandle<Checked>,
    panel_side: PanelSide,
    panel: Panels,
}

impl MenuItemState {
    fn new(
        context: MessageContext,
        menu_state: UseStateHandle<Checked>,
        panel_side: PanelSide,
        panel: Panels,
    ) -> Self {
        Self {
            context,
            menu_state,
            panel_side,
            panel,
        }
    }

    pub fn callback(self) -> Callback<MouseEvent> {
        Callback::from(move |_: MouseEvent| {
            self.menu_state.set(Checked::False);
            if self.panel_side == PanelSide::Left {
                self.context.dispatch(Actions::SetPanelLeft(self.panel));
            } else {
                self.context.dispatch(Actions::SetPanelRight(self.panel));
            }
        })
    }

    pub fn hidden(&self) -> bool {
        if self.panel_side == PanelSide::Left {
            self.context.left_panel != self.panel || self.context.right_panel == self.panel
        } else {
            self.context.right_panel == self.panel || self.context.left_panel == self.panel
        }
    }
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let msg_ctx = use_context::<MessageContext>().expect("No message context found!");
    let menu_state = use_state(|| Checked::False);

    let right_panel_map = MenuItemState::new(
        msg_ctx.clone(),
        menu_state.clone(),
        PanelSide::Right,
        Panels::Map,
    );
    let right_panel_stats = MenuItemState::new(
        msg_ctx.clone(),
        menu_state.clone(),
        PanelSide::Right,
        Panels::Stats,
    );
    let right_panel_settings = MenuItemState::new(
        msg_ctx.clone(),
        menu_state.clone(),
        PanelSide::Right,
        Panels::Settings,
    );
    let right_panel_help = MenuItemState::new(
        msg_ctx.clone(),
        menu_state.clone(),
        PanelSide::Right,
        Panels::Help,
    );
    let right_panel_messages = MenuItemState::new(
        msg_ctx.clone(),
        menu_state.clone(),
        PanelSide::Right,
        Panels::Messages,
    );

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
                    { if !right_panel_messages.hidden() { html! { <li onclick={right_panel_messages.callback()}>{ "Messages" }</li> } } else { html! {} } }
                    { if !right_panel_map.hidden() { html! { <li onclick={right_panel_map.callback()}>{ "Map" }</li> } } else { html! {} } }
                    { if !right_panel_stats.hidden() { html! { <li onclick={right_panel_stats.callback()}>{ "Statistics" }</li> } } else { html! {} } }
                    { if !right_panel_settings.hidden() { html! { <li onclick={right_panel_settings.callback()}>{ "Settings" }</li> } } else { html! {} } }
                    { if !right_panel_help.hidden() { html! { <li onclick={right_panel_help.callback()}>{ "Help" }</li> } } else { html! {} } }
                </ul>
          </ul>
      </section>
    }
}
