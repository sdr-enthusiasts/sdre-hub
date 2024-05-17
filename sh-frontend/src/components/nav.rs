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

#[derive(Debug, Clone, Copy, PartialEq)]
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
    const fn new(
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

    pub fn show(&self) -> bool {
        if *self.menu_state == Checked::False {
            return false;
        }

        if self.panel_side == PanelSide::Left {
            self.context.left_panel != self.panel && self.context.right_panel != self.panel
        } else {
            self.context.right_panel != self.panel && self.context.left_panel != self.panel
        }
    }
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let msg_ctx = use_context::<MessageContext>().expect("No message context found!");
    let menu_state_right = use_state_eq(|| Checked::False);
    let menu_state_left = use_state_eq(|| Checked::False);

    let right_panel_map = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Map,
    );
    let right_panel_stats = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Stats,
    );
    let right_panel_settings = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Settings,
    );
    let right_panel_help = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Help,
    );
    let right_panel_messages = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Messages,
    );

    let left_panel_messages = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Messages,
    );
    let left_panel_map = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Map,
    );
    let left_panel_stats = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Stats,
    );
    let left_panel_settings = MenuItemState::new(
        msg_ctx.clone(),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Settings,
    );
    let left_panel_help = MenuItemState::new(
        msg_ctx,
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Help,
    );

    let show_menu_callback_right_left_panel = menu_state_left.clone();
    let show_menu_callback_right_right_panel = menu_state_right.clone();
    let mouse_show_menu_right = {
        let current_state = *show_menu_callback_right_right_panel;
        Callback::from(move |_: MouseEvent| {
            show_menu_callback_right_left_panel.set(Checked::False);
            show_menu_callback_right_right_panel.set(!current_state);
        })
    };

    let show_menu_callback_left_left_panel = menu_state_left.clone();
    let show_menu_callback_left_right_panel = menu_state_right.clone();
    let mouse_show_menu_left = {
        let current_state = *show_menu_callback_left_left_panel;
        Callback::from(move |_: MouseEvent| {
            show_menu_callback_left_right_panel.set(Checked::False);
            show_menu_callback_left_left_panel.set(!current_state);
        })
    };

    let hidden_menu_right = { *menu_state_right };
    let hidden_menu_left = { *menu_state_left };

    html! {
      <section class="top-nav rounded-3xl">
        <input id="menu-toggle-left" checked={hidden_menu_left.into()} onclick={mouse_show_menu_left.clone()} type="checkbox" />
        <label class="menu-button-container" for="menu-toggle-left">
            <div class="menu-button"></div>
        </label>
        { if hidden_menu_left == Checked::True { html! {
            <ul class="menu text-[#101110] menu-left">
                { if left_panel_messages.show() { html! { <li onclick={left_panel_messages.callback()}>{ "Left: Messages" }</li> } } else { html! {} } }
                { if left_panel_map.show() { html! { <li onclick={left_panel_map.callback()}>{ "Left: Map" }</li> } } else { html! {} } }
                { if left_panel_stats.show() { html! { <li onclick={left_panel_stats.callback()}>{ "Left: Statistics" }</li> } } else { html! {} } }
                { if left_panel_settings.show() { html! { <li onclick={left_panel_settings.callback()}>{ "Left: Settings" }</li> } } else { html! {} } }
                { if left_panel_help.show() { html! { <li onclick={left_panel_help.callback()}>{ "Left: Help" }</li> } } else { html! {} } }
            </ul>
        } } else { html! { } } }
        <Search />
        <input id="menu-toggle-right" checked={hidden_menu_right.into()} onclick={mouse_show_menu_right.clone()} type="checkbox" />
        <label class="menu-button-container" for="menu-toggle-right">
            <div class="menu-button"></div>
        </label>
        { if hidden_menu_right == Checked::True { html! {
            <ul class="menu text-[#101110] menu-right">
                { if right_panel_messages.show() { html! { <li onclick={right_panel_messages.callback()}>{ "Right: Messages" }</li> } } else { html! {} } }
                { if right_panel_map.show() { html! { <li onclick={right_panel_map.callback()}>{ "Right: Map" }</li> } } else { html! {} } }
                { if right_panel_stats.show() { html! { <li onclick={right_panel_stats.callback()}>{ "Right: Statistics" }</li> } } else { html! {} } }
                { if right_panel_settings.show() { html! { <li onclick={right_panel_settings.callback()}>{ "Right: Settings" }</li> } } else { html! {} } }
                { if right_panel_help.show() { html! { <li onclick={right_panel_help.callback()}>{ "Right: Help" }</li> } } else { html! {} } }
            </ul>
        } } else { html! { } } }
      </section>
    }
}
