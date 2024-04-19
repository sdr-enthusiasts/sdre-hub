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

use libsdrehub::SdreHub;
use libshconfig::ShConfig;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config = ShConfig::new();
    let hub = SdreHub::new(config);

    match hub.run().await {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Error running SDRE Hub: {e}");
            return Err(e);
        }
    }

    Ok(())
}
