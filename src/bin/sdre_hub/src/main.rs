// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![deny(
    clippy::pedantic,
    clippy::cargo,
    clippy::nursery,
    clippy::style,
    clippy::correctness,
    clippy::all
)]

use sdrehub_config::ShConfig;

fn main() {
    let config = ShConfig::new();
    config.enable_logging();
    config.write_config();
    config.show_config();
}
