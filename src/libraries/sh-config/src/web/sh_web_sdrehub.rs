// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct ShWebSDREHub {
    pub database_url: String,
    pub log_level: String,
    pub data_path: String,
}

impl ShWebSDREHub {
    #[must_use]
    pub const fn new(database_url: String, log_level: String, data_path: String) -> Self {
        Self {
            database_url,
            log_level,
            data_path,
        }
    }
}
