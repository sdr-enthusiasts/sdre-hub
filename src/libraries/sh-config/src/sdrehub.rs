// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::{web::sh_web_sdrehub::ShWebSDREHub, ShConfig};

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct SDREHub {
    #[serde_inline_default("sqlite://sdre-hub.db".to_string())]
    pub database_url: String,
    #[serde_inline_default("info".to_string())]
    pub log_level: String,
    #[serde_inline_default(ShConfig::get_application_data_path())]
    pub data_path: String,
    #[serde_inline_default(ShConfig::get_config_file_path())]
    #[serde(skip_serializing)]
    pub config_file: String,
}

impl SDREHub {
    #[must_use]
    pub const fn new(
        database_url: String,
        log_level: String,
        data_path: String,
        config_file: String,
    ) -> Self {
        Self {
            database_url,
            log_level,
            data_path,
            config_file,
        }
    }

    #[must_use]
    pub fn to_web_sdrehub(&self) -> ShWebSDREHub {
        ShWebSDREHub::new(
            self.database_url.clone(),
            self.log_level.clone(),
            self.data_path.clone(),
        )
    }
}

impl Default for SDREHub {
    fn default() -> Self {
        let path = ShConfig::get_application_data_path();
        Self {
            database_url: format!("sqlite://{path}sdre-hub.db"),
            data_path: path,
            log_level: "info".to_string(),
            config_file: ShConfig::get_config_file_path(),
        }
    }
}
