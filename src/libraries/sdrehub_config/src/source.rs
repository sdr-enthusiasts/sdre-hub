// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::{acars_router_source::AcarsRouterSource, adsb_source::AdsbSource};

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnabledDataSources {
    #[serde_inline_default(false)]
    pub adsb: bool,
    #[serde_inline_default(false)]
    pub acars: bool,
    #[serde_inline_default(false)]
    pub hfdl: bool,
    #[serde_inline_default(false)]
    pub inmarsat: bool,
    #[serde_inline_default(false)]
    pub iridium: bool,
    #[serde_inline_default(false)]
    pub vdlm2: bool,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct DataSources {
    #[serde_inline_default(AcarsRouterSource::default())]
    #[serde(deserialize_with = "crate::acars_router_source::string_or_struct")]
    pub acars_routers: AcarsRouterSource,
    #[serde_inline_default(AdsbSource::default())]
    #[serde(deserialize_with = "crate::adsb_source::string_or_struct")]
    pub adsb_sources: AdsbSource,
}
