// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

use crate::{
    map::ShMapConfig,
    source::{DataSources, EnabledDataSources},
};

use super::sh_web_sdrehub::ShWebSDREHub;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ShWebConfig {
    pub app: ShWebSDREHub,
    pub enabled_data_sources: EnabledDataSources,
    pub data_sources: DataSources,
    pub map: ShMapConfig,
}
