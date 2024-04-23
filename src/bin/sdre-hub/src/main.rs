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

use sdrehub::SdreHub;
use sh_config::ShConfig;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Check the command line args and see if `--version` was passed.
    // If so, print version and exit 0

    if std::env::args().any(|x| x == *"--version") {
        // get the version from the Cargo.toml file
        let version = env!("CARGO_PKG_VERSION");
        println!("SDRE Hub version: {version}");
        return Ok(());
    }

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
