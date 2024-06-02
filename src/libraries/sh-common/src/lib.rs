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

use serde::{Deserialize, Serialize};
use sh_config::{
    web::{sh_web_config::ShWebConfig, sh_web_sdrehub::ShWebSDREHub},
    ShConfig,
};

#[derive(Serialize, Deserialize, Debug)]
pub enum UserMessageTypes {
    UserRequestConfig,
    UserUpdateAppConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessageTypes {
    ServerResponseConfig,
    ServerWriteConfigSuccess,
    ServerWriteConfigFailure,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub enum MessageData {
    ShConfig(ShWebConfig),
    ShAppConfig(ShWebSDREHub),
    ShConfigSuccess(String),
    ShConfigFailure(String),
    NoData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServerWssMessage {
    message_type: ServerMessageTypes,
    data: MessageData,
}

impl ServerWssMessage {
    #[must_use]
    pub fn new(message_type: ServerMessageTypes, data: MessageData) -> Self {
        Self { message_type, data }
    }

    pub fn get_message_type(&self) -> &ServerMessageTypes {
        &self.message_type
    }

    pub fn get_data(&self) -> &MessageData {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserWssMessage {
    pub message_type: UserMessageTypes,
    pub data: MessageData,
}

impl UserWssMessage {
    #[must_use]
    pub fn new(message_type: UserMessageTypes, data: MessageData) -> Self {
        Self { message_type, data }
    }

    pub fn get_message_type(&self) -> &UserMessageTypes {
        &self.message_type
    }

    pub fn get_data(&self) -> &MessageData {
        &self.data
    }
}

#[derive(Debug)]
pub enum ServerType {
    WebSocket,
    Other,
}
