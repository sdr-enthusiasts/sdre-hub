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

use async_trait::async_trait;

#[async_trait]
pub trait ShDataUser {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self);
    fn restart(&self);
}

pub type ShDataUserList = Vec<Box<dyn ShDataUser + Send + Sync>>;
