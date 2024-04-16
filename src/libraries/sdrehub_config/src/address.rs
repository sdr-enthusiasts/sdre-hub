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

#[derive(Debug, Deserialize, Serialize)]
pub struct AdsbAddress {
    address: String,
    port: u32,
    lat: f64,
    lon: f64,
}

impl AdsbAddress {
    /// Create a new AcarsRouterAddress from a string
    /// Input should be in the format "address:port"
    /// Returns an Option containing the AcarsRouterAddress if successful

    pub fn new(input: String) -> Option<Self> {
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

        let lat = match parts[2].parse::<f64>() {
            Ok(lat) => {
                // verify that the latitude is in the correct range

                if !(-90.0..=90.0).contains(&lat) {
                    return None;
                }

                lat
            }
            Err(_) => {
                return None;
            }
        };

        let lon = match parts[3].parse::<f64>() {
            Ok(lon) => {
                // verify that the longitude is in the correct range

                if !(-180.0..=180.0).contains(&lon) {
                    return None;
                }

                lon
            }
            Err(_) => {
                return None;
            }
        };

        Some(AdsbAddress::new_from_parts(
            parts[0].trim().to_string(),
            port,
            lat,
            lon,
        ))
    }

    pub fn new_from_parts(address: String, port: u32, lat: f64, lon: f64) -> Self {
        AdsbAddress {
            address,
            port,
            lat,
            lon,
        }
    }
}
