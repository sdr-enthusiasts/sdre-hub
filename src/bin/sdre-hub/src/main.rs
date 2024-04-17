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
// FIXME: This is a temporary fix because there is an upstream error(I think)
// in the tokio::select macro
#![allow(clippy::redundant_pub_crate)]
#[macro_use]
extern crate log;
pub use sdrehub_api::run_webserver;
use sdrehub_config::ShConfig;
use std::error::Error;
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ShConfig::new();
    config.enable_logging();
    config.write_config();
    config.show_config();

    select! {
        () = run_webserver() => {
            error!("Webserver exited");
        }
    }

    Ok(())
}
