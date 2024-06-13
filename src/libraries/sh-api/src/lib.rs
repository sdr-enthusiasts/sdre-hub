// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![deny(
    clippy::pedantic,
//    clippy::cargo,
    clippy::nursery,
    clippy::style,
    clippy::correctness,
    clippy::all
)]

use sh_common_server::ShDataUser;
use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
    routing::get,
    Router,
};

use sh_common::{
    MessageData, ServerMessageTypes, ServerType, ServerWssMessage, UserMessageTypes, UserWssMessage,
};
use sh_config::ShConfig;
#[macro_use]
extern crate log;

pub struct ShAPIServer {}

struct ShAPIServerState {
    config: Arc<Mutex<ShConfig>>,
}

#[async_trait]
impl ShDataUser for ShAPIServer {
    async fn start(
        &self,
        data: Option<Arc<Mutex<ShConfig>>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Start the web server
        let Some(config) = data else {
            error!("No configuration provided to start the API server");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No configuration provided to start the API server",
            )));
        };
        self.run_apiserver(config).await
    }

    // TODO: Can we dynamically start/stop/restart the web server?
    fn stop(&self) {
        // Stop the web server
    }

    fn restart(&self) {
        // Restart the web server
    }

    fn get_server_type(&self) -> sh_common::ServerType {
        ServerType::WebSocket
    }
}

impl Default for ShAPIServer {
    fn default() -> Self {
        Self::new()
    }
}

impl ShAPIServer {
    #[must_use]
    pub const fn new() -> Self {
        Self {}
    }

    /// # Errors
    /// - Error binding socket for websocket server: {e}
    pub async fn run_apiserver(
        &self,
        config: Arc<Mutex<ShConfig>>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let listener = match tokio::net::TcpListener::bind("0.0.0.0:3000").await {
            Ok(listener) => listener,
            Err(e) => {
                error!("Error binding socket for websocket server: {e}");
                return Err(Box::new(e));
            }
        };

        let local_addr = match listener.local_addr() {
            Ok(addr) => addr,
            Err(e) => {
                error!("Error getting local address for websocket server: {e}");
                return Err(Box::new(e));
            }
        };

        let server = Arc::new(ShAPIServerState { config });

        info!("listening for websocket connections on {}", local_addr);
        if axum::serve(listener, app(server)).await.is_err() {
            error!("Error starting WebSocket server");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error starting WebSocket server",
            )));
        }

        Ok(())
    }
}

fn app(server: Arc<ShAPIServerState>) -> Router {
    debug!("Starting WebSocket server");

    Router::new()
        .route("/sdre-hub", get(ws_handler))
        .with_state(server)
}

async fn ws_handler(ws: WebSocketUpgrade, State(server): State<Arc<ShAPIServerState>>) -> Response {
    debug!("WebSocket connection initiated");
    ws.on_upgrade(|socket| ws_handle_socket(socket, server))
}

async fn ws_handle_socket(mut socket: WebSocket, state: Arc<ShAPIServerState>) {
    // send the initial message
    // let response_type = ServerMessageTypes::ServerResponseConfig;
    // let config = state.config.lock().unwrap().clone();
    //                     let data = MessageData::Config(config);
    //                     let message = ServerWssMessage::new(response_type, data);
    //                     let config = serde_json::to_string(&message).unwrap();
    //                     socket.send(Message::Text(config)).await.unwrap();

    while let Some(Ok(msg)) = socket.recv().await {
        // deserialize the message and see if it's a request for config
        debug!("Received message: {:?}", msg);

        match msg {
            Message::Text(text) => {
                let text = text.trim_matches('"').replace("\\\"", "\"");

                debug!("Received text message after trimming and replacement: {text}");
                let message: UserWssMessage = match serde_json::from_str(&text) {
                    Ok(message) => message,
                    Err(e) => {
                        error!("Error deserializing message: {e}");
                        continue;
                    }
                };

                match message.message_type {
                    UserMessageTypes::UserRequestConfig => {
                        // check the data
                        if message.data != MessageData::NoData {
                            error!("Received UserRequestConfig message with data");
                            continue;
                        }

                        let response_type = ServerMessageTypes::ServerResponseConfig;
                        // get the server config
                        let data = MessageData::ShConfig(state.config.lock().await.to_web_config());
                        let message = ServerWssMessage::new(response_type, data);
                        let config_serialized = serde_json::to_string(&message).unwrap();
                        debug!("Sending config message: {config_serialized}");
                        socket.send(Message::Text(config_serialized)).await.unwrap();
                    }
                    UserMessageTypes::UserUpdateAppConfig => {
                        // check the data
                        if message.data == MessageData::NoData {
                            error!("Received UserUpdateAppConfig message without data");
                            continue;
                        }

                        debug!("Received UserUpdateAppConfig message with data");
                        // FIXME: This needs to be a match once messagedata has more shit in it
                        let MessageData::ShAppConfig(data) = message.data else {
                            error!("Received UserUpdateAppConfig message with incorrect data type");
                            continue;
                        };

                        // TODO: Maybe. For now the DB path and config path are not going to be user configurable.

                        // check and see if the log level has changed

                        let mut config = state.config.lock().await;

                        if *config.app.log_level != data.log_level {
                            debug!("New log level: {}", data.log_level);

                            let new_log_level = data.log_level.clone();

                            config.app.log_level = new_log_level;
                            match config.write_config() {
                                Ok(()) => {
                                    // tell the user that the config write was successful and they need to restart
                                    // because the log level changed

                                    let response_type =
                                        ServerMessageTypes::ServerWriteConfigSuccess;
                                    let data = MessageData::ShConfigSuccess("Log level has been updated. Please restart the server for the change to take affect".to_string());

                                    let message = ServerWssMessage::new(response_type, data);

                                    let config = serde_json::to_string(&message).unwrap();

                                    socket.send(Message::Text(config)).await.unwrap();
                                }

                                Err(e) => {
                                    // tell the user that the config write failed
                                    let response_type =
                                        ServerMessageTypes::ServerWriteConfigFailure;
                                    let data = MessageData::ShConfigFailure(format!(
                                        "Error writing config file: {e}"
                                    ));

                                    let message = ServerWssMessage::new(response_type, data);

                                    let config = serde_json::to_string(&message).unwrap();

                                    socket.send(Message::Text(config)).await.unwrap();
                                }
                            }
                        }

                        debug!("Received app config: {:?}", data);
                        debug!("Current app state: {:?}", config);
                    }
                    UserMessageTypes::UserUpdateMapConfig => {
                        // check the data
                        if message.data == MessageData::NoData {
                            error!("Received UserUpdateMapConfig message without data");
                            continue;
                        }

                        let MessageData::ShMapConfig(data) = message.data else {
                            error!("Received UserUpdateMapConfig message with incorrect data type");
                            continue;
                        };

                        debug!("Received UserUpdateMapConfig message with data");

                        let mut config = state.config.lock().await;

                        if config.map != data {
                            debug!("New map config: {:?}", data);

                            let new_map_config = data.clone();

                            config.map = new_map_config;
                            match config.write_config() {
                                Ok(()) => {
                                    // tell the user that the config write was successful and they need to restart
                                    // because the map config changed

                                    let response_type =
                                        ServerMessageTypes::ServerWriteConfigSuccess;
                                    let data = MessageData::ShConfigSuccess(
                                        "Map config has been updated.".to_string(),
                                    );

                                    let message = ServerWssMessage::new(response_type, data);

                                    let config = serde_json::to_string(&message).unwrap();

                                    socket.send(Message::Text(config)).await.unwrap();
                                }

                                Err(e) => {
                                    // tell the user that the config write failed
                                    let response_type =
                                        ServerMessageTypes::ServerWriteConfigFailure;
                                    let data = MessageData::ShConfigFailure(format!(
                                        "Error writing config file: {e}"
                                    ));

                                    let message = ServerWssMessage::new(response_type, data);

                                    let config = serde_json::to_string(&message).unwrap();

                                    socket.send(Message::Text(config)).await.unwrap();
                                }
                            }
                        }
                    }
                }
            }
            Message::Binary(_) => {
                error!("Binary messages not supported");
            }
            Message::Ping(_) => {
                // respond with a pong
                log::debug!("Received ping, responding with pong");
                socket.send(Message::Pong(vec![])).await.unwrap();
            }
            Message::Pong(_) => {
                debug!("Received pong");
            }
            Message::Close(_) => {
                trace!("Socket Closed");
            }
        }
    }
}
