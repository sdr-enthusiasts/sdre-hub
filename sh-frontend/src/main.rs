// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![deny(
    clippy::pedantic,
    //clippy::cargo,
    clippy::nursery,
    clippy::style,
    clippy::correctness,
    clippy::all
)]

use sh_frontend::app::App;

// This is the entry point for the web app
fn main() {
    // if we are in a debug build we want the level to be debug
    let level = if cfg!(debug_assertions) {
        log::Level::Debug
    } else {
        log::Level::Info
    };

    wasm_logger::init(wasm_logger::Config::new(level));

    yew::Renderer::<App>::new().render();
}
