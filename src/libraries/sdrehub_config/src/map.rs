// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

/// MapConfig is a struct for storing global map values
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MapConfig {
    /// center_latitude is the latitude of the center of the map
    /// This value will be used to center the map on the web interface
    /// Default value is 0.0
    center_latitude: f64,
    /// center_longitude is the longitude of the center of the map
    /// This value will be used to center the map on the web interface
    /// Default value is 0.0
    center_longitude: f64,
}
