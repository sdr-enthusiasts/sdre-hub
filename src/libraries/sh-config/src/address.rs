// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

/// Struct to store the address of an ACARS router
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
pub struct ShAcarsRouterConfig {
    address: String,
    port: u32,
}

impl ShAcarsRouterConfig {
    /// Create a new `AcarsRouterAddress` from a string
    /// Input should be in the format "address:port"
    /// Returns an Option containing the `AcarsRouterAddress` if successful
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();

        if parts.len() != 2 {
            return None;
        }

        let Ok(port) = parts[1].parse::<u32>() else {
            return None;
        };

        Some(Self {
            address: parts[0].trim().to_string(),
            port,
        })
    }

    #[must_use]
    pub const fn new_from_parts(address: String, port: u32) -> Self {
        Self { address, port }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct SHAdsbConfig {
    address: String,
    port: u32,
    latitude: f64,
    longitude: f64,
}

impl SHAdsbConfig {
    /// Create a new `AcarsRouterAddress` from a string
    /// Input should be in the format "address:port"
    /// Returns an Option containing the `AcarsRouterAddress` if successful
    #[must_use]
    pub fn new(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split(':').collect();

        if parts.len() != 4 {
            return None;
        }

        let port = match parts[1].parse::<u32>() {
            Ok(port) => {
                // verify that the port is in the correct range

                if port == 0 || port > 65535 {
                    return None;
                }

                port
            }
            Err(_) => {
                return None;
            }
        };

        let latitude = match parts[2].parse::<f64>() {
            Ok(latitude) => {
                // verify that the latitude is in the correct range

                if !(-90.0..=90.0).contains(&latitude) {
                    return None;
                }

                latitude
            }
            Err(_) => {
                return None;
            }
        };

        let longitude = match parts[3].parse::<f64>() {
            Ok(longitude) => {
                // verify that the longitude is in the correct range

                if !(-180.0..=180.0).contains(&longitude) {
                    return None;
                }

                longitude
            }
            Err(_) => {
                return None;
            }
        };

        Some(Self::new_from_parts(
            parts[0].trim().to_string(),
            port,
            latitude,
            longitude,
        ))
    }

    #[must_use]
    pub const fn new_from_parts(address: String, port: u32, latitude: f64, longitude: f64) -> Self {
        Self {
            address,
            port,
            latitude,
            longitude,
        }
    }
}
