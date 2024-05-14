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

use async_trait::async_trait;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};

use sh_common::{
    MessageData, ServerMessageTypes, ServerWssMessage, ShDataUser, UserMessageTypes, UserWssMessage,
};
use sh_config::ShConfig;
#[macro_use]
extern crate log;

pub struct ShAPIServer {
    _config: std::sync::Arc<ShConfig>,
}

#[async_trait]
impl ShDataUser for ShAPIServer {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start the web server
        self.run_apiserver().await
    }

    // TODO: Can we dynamically start/stop/restart the web server?
    fn stop(&self) {
        // Stop the web server
    }

    fn restart(&self) {
        // Restart the web server
    }
}

impl ShAPIServer {
    #[must_use]
    pub fn new(config: std::sync::Arc<ShConfig>) -> Self {
        Self { _config: config }
    }

    /// # Errors
    /// - Error binding socket for websocket server: {e}
    pub async fn run_apiserver(&self) -> Result<(), Box<dyn std::error::Error>> {
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

        info!("listening for websocket connections on {}", local_addr);
        if axum::serve(listener, app()).await.is_err() {
            error!("Error starting WebSocket server");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Error starting WebSocket server",
            )));
        }

        Ok(())
    }
}

fn app() -> Router {
    debug!("Starting WebSocket server");

    Router::new().route("/sdre-hub", get(ws_handler))
}

async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    debug!("WebSocket connection initiated");
    ws.on_upgrade(ws_handle_socket)
}

async fn ws_handle_socket(mut socket: WebSocket) {
    while let Some(Ok(msg)) = socket.recv().await {
        // deserialize the message and see if it's a request for config
        match msg {
            Message::Text(text) => {
                let message: UserWssMessage = match serde_json::from_str::<UserWssMessage>(&text) {
                    Ok(message) => message,
                    Err(e) => {
                        error!("Error deserializing message: {e}");
                        continue;
                    }
                };

                match message.message_type {
                    UserMessageTypes::UserRequestConfig => {
                        // check the data
                        if message.data != MessageData::None {
                            error!("Received UserRequestConfig message with data");
                            continue;
                        }

                        let response_type = ServerMessageTypes::ServerResponseConfig;
                        let data = MessageData::Config("test".to_string());
                        let message = ServerWssMessage::new(response_type, data);
                        let config = serde_json::to_string(&message).unwrap();
                        socket.send(Message::Text(config)).await.unwrap();
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
                trace!("Close messages not supported");
            }
        }
    }
}
