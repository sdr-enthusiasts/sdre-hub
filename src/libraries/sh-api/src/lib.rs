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
use sh_common::ShDataUser;
#[macro_use]
extern crate log;

pub struct ShAPIServer {}

#[async_trait]
impl ShDataUser for ShAPIServer {
    async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        // Start the web server
        self.run_apiserver().await
    }

    fn stop(&self) {
        // Stop the web server
    }

    fn restart(&self) {
        // Restart the web server
    }
}

impl ShAPIServer {
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
        if let Message::Text(msg) = msg {
            if socket
                .send(Message::Text(format!("You said: {msg}")))
                .await
                .is_err()
            {
                break;
            }
        }
    }
}
