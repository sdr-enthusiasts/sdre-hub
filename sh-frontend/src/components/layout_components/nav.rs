// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::common::panels::Panels;
use crate::components::nav_components::search::Search;
use crate::services::saved_state::WebAppState;
use crate::services::temp_state::WebAppStateTemp;
use std::rc::Rc;
use std::{fmt, ops::Not};
use yew::prelude::*;
use yewdux::prelude::*;

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
    right_panel_visible: Rc<bool>,
    menu_state: UseStateHandle<Checked>,
    panel_side: PanelSide,
    panel: Panels,
    state: Rc<WebAppState>,
    dispatch: Dispatch<WebAppState>,
}

impl MenuItemState {
    fn new(
        right_panel_visible: Rc<bool>,
        menu_state: UseStateHandle<Checked>,
        panel_side: PanelSide,
        panel: Panels,
        state: Rc<WebAppState>,
        dispatch: Dispatch<WebAppState>,
    ) -> Self {
        Self {
            right_panel_visible,
            menu_state,
            panel_side,
            panel,
            state: state,
            dispatch,
        }
    }

    pub fn callback(self) -> Callback<MouseEvent> {
        Callback::from(move |_: MouseEvent| {
            self.menu_state.set(Checked::False);
            if self.panel_side == PanelSide::Left {
                self.dispatch.reduce_mut(|state| {
                    state.left_panel = self.panel;
                });
            } else {
                self.dispatch.reduce_mut(|state| {
                    state.right_panel = self.panel;
                });
            }
        })
    }

    pub fn show(&self) -> bool {
        if *self.menu_state == Checked::False {
            return false;
        }

        if PanelSide::Left == self.panel_side && !*self.right_panel_visible {
            return true;
        }

        let left_panel = self.state.left_panel;
        let right_panel = self.state.right_panel;

        if self.panel_side == PanelSide::Left {
            left_panel != self.panel && right_panel != self.panel
        } else {
            right_panel != self.panel && left_panel != self.panel
        }
    }
}

/// Nav component
#[function_component(Nav)]
pub fn nav() -> Html {
    let menu_state_right = use_state_eq(|| Checked::False);
    let menu_state_left = use_state_eq(|| Checked::False);
    let (state_local, dispatch_local) = use_store::<WebAppState>();
    let right_panel_visible = use_selector(|state: &WebAppStateTemp| state.right_panel_visible);

    let right_panel_map = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Map,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let right_panel_stats = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Stats,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let right_panel_settings = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Settings,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let right_panel_help = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Help,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let right_panel_messages = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_right.clone(),
        PanelSide::Right,
        Panels::Messages,
        state_local.clone(),
        dispatch_local.clone(),
    );

    let left_panel_messages = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Messages,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let left_panel_map = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Map,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let left_panel_stats = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Stats,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let left_panel_settings = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Settings,
        state_local.clone(),
        dispatch_local.clone(),
    );
    let left_panel_help = MenuItemState::new(
        Rc::new(*right_panel_visible),
        menu_state_left.clone(),
        PanelSide::Left,
        Panels::Help,
        state_local,
        dispatch_local,
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
      <section class="top-nav">
        <input id="menu-toggle-left" checked={hidden_menu_left.into()} onclick={mouse_show_menu_left.clone()} type="checkbox" />
        <label class="menu-button-container" for="menu-toggle-left">
            <div class="menu-button" title="Left Panel"></div>
        </label>
        {
            if hidden_menu_left == Checked::True {
                html! {
                    <ul class="menu menu-left">
                    {
                        if left_panel_messages.show() {
                            html! {
                                <li onclick={left_panel_messages.callback()}>{ "Left: Messages" }</li>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if left_panel_map.show() {
                            html! {
                                <li onclick={left_panel_map.callback()}>{ "Left: Map" }</li>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if left_panel_stats.show()
                        {
                            html! {
                                <li onclick={left_panel_stats.callback()}>{ "Left: Statistics" }</li>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if left_panel_settings.show() {
                            html! {
                                <li onclick={left_panel_settings.callback()}>{ "Left: Settings" }</li>
                            }
                        } else {
                            html! {}
                        }
                    }
                    {
                        if left_panel_help.show() {
                            html! {
                                <li onclick={left_panel_help.callback()}>{ "Left: Help" }</li>
                            }
                        } else {
                            html! {}
                        }
                    }
            </ul>
        } } else { html! { } } }
        <Search />
        {
            if *right_panel_visible {
                html! {
                    <><input id="menu-toggle-right" checked={hidden_menu_right.into()} onclick={mouse_show_menu_right.clone()} type="checkbox" />
                    <label class="menu-button-container" for="menu-toggle-right">
                        <div class="menu-button" title="Right Panel"></div>
                    </label>
                        {
                        if hidden_menu_right == Checked::True {
                            html! {
                                <ul class="menu menu-right">
                                    {
                                        if right_panel_messages.show() {
                                            html! {
                                                <li onclick={right_panel_messages.callback()}>{ "Right: Messages" }</li>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if right_panel_map.show() {
                                            html! {
                                                <li onclick={right_panel_map.callback()}>{ "Right: Map" }</li>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if right_panel_stats.show() {
                                            html! {
                                                <li onclick={right_panel_stats.callback()}>{ "Right: Statistics" }</li>
                                        }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if right_panel_settings.show() {
                                            html! {
                                                <li onclick={right_panel_settings.callback()}>{ "Right: Settings" }</li>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                    {
                                        if right_panel_help.show() {
                                            html! {
                                                <li onclick={right_panel_help.callback()}>{ "Right: Help" }</li>
                                            }
                                        } else {
                                            html! {}
                                        }
                                    }
                                </ul>
                            }
                        } else {
                            html! { }
                        }
                    }
                    </>
                }
            } else {
                html! { }
            }
        }
        </section>
    }
}
