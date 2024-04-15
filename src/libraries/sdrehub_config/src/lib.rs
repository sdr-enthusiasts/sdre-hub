// Copyright (C) 2024 Fred Clausen

// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version 3
// of the License, or (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA

// SDR-E Hub will be configured via using the following logic for ordering of configuration values:
//  * Default values (least priority)
//  * Configuration file (toml format)
//  * Environment variables
//  * Command line arguments (highest priority)
// TODO: Implement command line arguments. The config crate doesn't do this out of the box
//  If no config file is provided whatever value is picked will be written to the default config file
//  If a config file is provided, and a higher priority value is provided via environment variable or command line argument, the value will be written to the config file

use std::{fmt, marker::PhantomData, str::FromStr};

/// SDR-E Hub valid configuration options
/// database_url: The URL to the database
/// enable_acars: Enable ACARS processing
/// enable_vdlm2: Enable VDL-M2 processing
/// enable_hfdl: Enable HFDL processing
/// enable_iridium: Enable Iridium processing
/// enable_inmarsat: Enable Inmarsat processing
/// enable_adsb: Enable ADS-B processing
/// log_level: The log level. Valid values are: trace, debug, info, warn, error. Default is info. List is ordered from most verbose to least verbos
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use log::debug;
use sdre_rust_logging::SetupLogging;
use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use serde_inline_default::serde_inline_default;
use void::Void;

// FIXME: env variables require a dot between the prefix and the variable name. This is not ideal. Should be able to use underscores

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize)]
pub struct SDREHub {
    #[serde_inline_default("sqlite://sdre-hub.db".to_string())]
    pub database_url: String,
    #[serde_inline_default("info".to_string())]
    pub log_level: String,
    #[serde_inline_default("./data".to_string())]
    pub data_path: String,
    #[serde_inline_default(ShConfig::get_file_path())]
    #[serde(skip_serializing)]
    pub config_file: String,
}

impl Default for SDREHub {
    fn default() -> Self {
        SDREHub {
            database_url: "sqlite://sdre-hub.db".to_string(),
            data_path: "./data".to_string(),
            log_level: "info".to_string(),
            config_file: ShConfig::get_file_path(),
        }
    }
}

trait SourceTrait {
    fn new() -> Self;
    fn insert(&mut self, value: Address);
}

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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Source {
    addresses: Vec<Address>,
}

impl FromStr for Source {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Source::new();

        for address in s.split(',') {
            if let Some(address) = Address::new(address.to_string()) {
                output.insert(address);
            }
        }

        Ok(output)
    }
}

impl SourceTrait for Source {
    fn new() -> Self {
        Source {
            addresses: Vec::new(),
        }
    }

    fn insert(&mut self, value: Address) {
        self.addresses.push(value);
    }
}

#[serde_inline_default]
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct DataSources {
    #[serde_inline_default(Source::default())]
    #[serde(deserialize_with = "string_or_struct")]
    pub acars_routers: Source,
    #[serde_inline_default(Source::default())]
    #[serde(deserialize_with = "string_or_struct")]
    pub adsb_sources: Source,
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void> + SourceTrait,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void> + SourceTrait,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // is the seq a string?

            let length = seq.size_hint().unwrap_or(0);

            if length == 0 {
                return Ok(T::new());
            }

            let mut source = T::new();

            while let Some(value) = seq.next_element::<String>()? {
                if let Some(address) = Address::new(value) {
                    source.insert(address);
                }
            }

            Ok(source)
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MapConfig {
    center_latitude: f64,
    center_longitude: f64,
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShConfig {
    #[serde_inline_default(SDREHub::default())]
    pub app: SDREHub,
    #[serde_inline_default(EnabledDataSources::default())]
    pub enabled_data_sources: EnabledDataSources,
    #[serde_inline_default(DataSources::default())]
    pub data_sources: DataSources,
    #[serde_inline_default(MapConfig::default())]
    pub map: MapConfig,
}

impl ShConfig {
    pub fn new() -> Self {
        ShConfig::get_and_validate_config()
    }

    fn get_file_path() -> String {
        // if we are in a test env (denoted with AH_TEST_ENV_PATH) we will use the test config file
        // from the env variable. Otherwise, detect the platform and use "./ah_config.toml" for the config file

        if let Ok(path) = std::env::var("AH_TEST_ENV_PATH") {
            path
        } else if let Ok(path) = std::env::var("AH_CONFIG_PATH") {
            // this match arm is for docker specifically
            path
        } else {
            // FIXME: we should use platform specific paths
            match std::env::consts::OS {
                "linux" => "./ah_config.toml",
                "macos" => "./ah_config.toml",
                "windows" => "./ah_config.toml",
                _ => "./ah_config.toml",
            }
            .to_string()
        }
    }

    fn get_config(file_path: &str) -> ShConfig {
        match Figment::new()
            .merge(Toml::file(file_path))
            .merge(Env::prefixed("AH_"))
            .extract()
        {
            Ok(config) => config,
            Err(e) => {
                println!("Error reading config file: {}", e);
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }

    fn get_and_validate_config() -> ShConfig {
        let file_path = ShConfig::get_file_path();
        ShConfig::get_config(&file_path)
    }

    pub fn show_config(&self) {
        debug!("Config: {:#?}", self);
    }

    pub fn enable_logging(&self) {
        self.app.log_level.enable_logging();
    }

    pub fn stringify_array_for_config_file(array: &[String]) -> String {
        array
            .iter()
            .map(|s| format!("\"{}\"", s))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn get_config_as_yaml_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    pub fn write_config(&self) {
        let file_path = ShConfig::get_file_path();
        let config = self.get_config_as_yaml_string();

        match std::fs::write(file_path, config) {
            Ok(_) => (),
            Err(e) => {
                println!("Error writing config file: {}", e);
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }
}
