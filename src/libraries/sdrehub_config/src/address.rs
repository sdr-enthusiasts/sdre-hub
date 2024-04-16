// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

/// Struct to store the address of an ACARS router
#[derive(Debug, Deserialize, Serialize)]
pub struct AcarsRouterAddress {
    address: String,
    port: u32,
}

impl AcarsRouterAddress {
    /// Create a new AcarsRouterAddress from a string
    /// Input should be in the format "address:port"
    /// Returns an Option containing the AcarsRouterAddress if successful

    pub fn new(input: String) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();

        if parts.len() != 2 {
            return None;
        }

        let port = match parts[1].parse::<u32>() {
            Ok(port) => port,
            Err(_) => {
                return None;
            }
        };

        Some(AcarsRouterAddress {
            address: parts[0].trim().to_string(),
            port,
        })
    }

    pub fn new_from_parts(address: String, port: u32) -> Self {
        AcarsRouterAddress { address, port }
    }
}
