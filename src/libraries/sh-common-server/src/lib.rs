// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use async_trait::async_trait;
use sh_common::ServerType;
use sh_config::ShConfig;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait ShDataUser {
    async fn start(
        &self,
        data: Option<Arc<Mutex<ShConfig>>>,
    ) -> Result<(), Box<dyn std::error::Error>>;
    fn stop(&self);
    fn restart(&self);
    fn get_server_type(&self) -> ServerType;
}

pub type ShDataUserList = Vec<Box<dyn ShDataUser + Send + Sync>>;
