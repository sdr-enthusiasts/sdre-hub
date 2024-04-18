// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// This is the brain of the entire operation. This is the state machine that
// Will take in the configuration, start, stop, restart consumers on config change
// (if applicable to the SDRE Hub implementation), and handle the incoming messages
// from the producers, generating the app state, and responding with information to user
// inputs.

// Additionally, this module will send data out to consumers, such as the database and any
// connected users (web interface) or the application window (if applicable).

// This is the main loop of the SDRE Hub.
#![deny(
    clippy::pedantic,
    //clippy::cargo,
    clippy::nursery,
    clippy::style,
    clippy::correctness,
    clippy::all
)]

#[macro_use]
extern crate log;

use libsdrehubcommon::ShDataUserList;
use sdrehub_api::ShWebServer;
use sdrehub_config::ShConfig;
use tokio::task::JoinSet;

pub struct SdreHub {
    config: ShConfig,
    data_users: ShDataUserList,
}

impl SdreHub {
    #[must_use]
    pub fn new(config: ShConfig) -> Self {
        Self {
            config,
            data_users: Vec::new(),
        }
    }

    /// # Errors
    /// - Error starting consumer: {e}
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        // init logging and stuff
        self.config.enable_logging();
        self.config.write_config();
        self.config.show_config();
        // Start the web server

        let mut consumer_set = JoinSet::new();

        // lets generate the consumers

        self.data_users.push(Box::new(ShWebServer {}));

        debug!("Starting consumers");

        for fut in self.data_users {
            consumer_set.spawn(tokio::spawn(async move {
                match fut.start().await {
                    Ok(()) => {}
                    Err(e) => {
                        error!("Error starting consumer: {e}");
                    }
                }
            }));
        }

        while let Some(res) = consumer_set.join_next().await {
            match res {
                Ok(_) => {}
                Err(e) => {
                    error!("Error starting consumer: {e}");
                }
            };
        }

        Ok(())
    }
}
