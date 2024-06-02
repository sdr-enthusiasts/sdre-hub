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

use sh_api::ShAPIServer;
use sh_common::ServerType;
use sh_common_server::ShDataUserList;
use sh_config::ShConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;

pub struct SdreHub {
    config: std::sync::Arc<Mutex<ShConfig>>,
    data_users: ShDataUserList,
}

impl SdreHub {
    #[must_use]
    pub fn new(config: ShConfig) -> Self {
        Self {
            config: std::sync::Arc::new(Mutex::new(config)),
            data_users: Vec::new(),
        }
    }

    /// # Errors
    /// - Error starting consumer: {e}
    pub async fn run(mut self) -> Result<(), Box<dyn std::error::Error>> {
        // get the config lock
        let config_lock = Arc::clone(&self.config);
        let config = config_lock.lock().await;

        // init logging and stuff
        config.enable_logging();
        match config.write_config() {
            Ok(_) => {}
            Err(e) => {
                error!("Error writing config: {e}");
                std::process::exit(1);
            }
        }
        config.show_config();

        // release the lock
        drop(config);

        // Start the web server

        let mut consumer_set = JoinSet::new();

        // lets generate the consumers

        self.data_users.push(Box::new(ShAPIServer::new()));

        debug!("Starting consumers");

        for fut in self.data_users {
            // if the kind of ShDataUser is ShAPIServer, then we can spawn it
            let config = match fut.get_server_type() {
                ServerType::WebSocket => {
                    // Start the web server
                    Some(Arc::clone(&self.config))
                }
                ServerType::Other => None,
            };

            consumer_set.spawn(tokio::spawn(async move {
                match fut.start(config).await {
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
