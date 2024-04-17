// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

use crate::{acars_router_source::AcarsRouterSource, adsb_source::AdsbSource};

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "snake_case", try_from = "String")]
pub enum ShEnabledDataSources {
    Acars,
    Adsb,
    Hfdl,
    Inmarsat,
    Iridium,
    Vdlm2,
    #[default]
    None
}

impl TryFrom<String> for ShEnabledDataSources {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "acars" => Ok(Self::Acars),
            "adsb" => Ok(Self::Adsb),
            "hfdl" => Ok(Self::Hfdl),
            "inmarsat" => Ok(Self::Inmarsat),
            "iridium" => Ok(Self::Iridium),
            "vdlm2" => Ok(Self::Vdlm2),
            _ => Err(format!("Invalid value for ShEnabledDataSources: {}", value)),
        }
    }
}

impl Serialize for ShEnabledDataSources {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Acars => serializer.serialize_str("acars"),
            Self::Adsb => serializer.serialize_str("adsb"),
            Self::Hfdl => serializer.serialize_str("hfdl"),
            Self::Inmarsat => serializer.serialize_str("inmarsat"),
            Self::Iridium => serializer.serialize_str("iridium"),
            Self::Vdlm2 => serializer.serialize_str("vdlm2"),
            // None should not write out anything
            Self::None => serializer.serialize_none(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnabledDataSources {
    pub enabled_sources: Vec<ShEnabledDataSources>,
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
