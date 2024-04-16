// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Deserialize)]
pub struct Address {
    address: String,
    port: u16,
}

impl Address {
    pub fn new(input: String) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();

        if parts.len() != 2 {
            return None;
        }

        let port = match parts[1].parse::<u16>() {
            Ok(port) => port,
            Err(_) => {
                return None;
            }
        };

        Some(Address {
            address: parts[0].trim().to_string(),
            port,
        })
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        format!("{}:{}", self.address, self.port).serialize(serializer)
    }
}
