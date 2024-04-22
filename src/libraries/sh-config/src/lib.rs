// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#![deny(
    clippy::pedantic,
//    clippy::cargo,
    clippy::nursery,
    clippy::style,
    clippy::correctness,
    clippy::all
)]

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
#[macro_use]
extern crate log;
use map::ShMapConfig;
use sdre_rust_logging::SetupLogging;
use sdrehub::SDREHub;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use source::{DataSources, EnabledDataSources};

pub mod acars_router_source;
pub mod address;
pub mod adsb_source;
pub mod map;
pub mod sdrehub;
pub mod source;
// TODO: Implement command line arguments. The config crate doesn't do this out of the box
// FIXME: env variables require a dot between the prefix and the variable name. This is not ideal. Should be able to use underscores

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ShConfig {
    #[serde_inline_default(SDREHub::default())]
    pub app: SDREHub,
    #[serde_inline_default(EnabledDataSources::default())]
    pub enabled_data_sources: EnabledDataSources,
    #[serde_inline_default(DataSources::default())]
    pub data_sources: DataSources,
    #[serde_inline_default(ShMapConfig::default())]
    pub map: ShMapConfig,
}

impl ShConfig {
    #[must_use]
    pub fn new() -> Self {
        Self::get_and_validate_config()
    }

    fn get_file_path() -> String {
        "./sh_config.toml".to_string() // FIXME: commented out the logic below to make clippy happy until we featurize the code
                                       // match std::env::consts::OS {
                                       //     "linux" => "./sh_config.toml",
                                       //     "macos" => "./sh_config.toml",
                                       //     "windows" => "./sh_config.toml",
                                       //     _ => "./sh_config.toml",
                                       // }
                                       // .to_string()
    }

    fn get_config(file_path: &str) -> Self {
        match Figment::new()
            .merge(Toml::file(file_path))
            .merge(Env::prefixed("SH_"))
            .extract()
        {
            Ok(config) => config,
            Err(e) => {
                println!("Error reading config file: {e}");
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }

    fn get_and_validate_config() -> Self {
        let file_path = Self::get_file_path();
        Self::get_config(&file_path)
    }

    pub fn show_config(&self) {
        debug!("Config: {:#?}", self);
    }

    pub fn enable_logging(&self) {
        self.app.log_level.enable_logging();
    }

    fn get_config_as_toml_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    pub fn write_config(&self) {
        let file_path = Self::get_file_path();
        let config = self.get_config_as_toml_string();

        match std::fs::write(file_path, config) {
            Ok(()) => (),
            Err(e) => {
                println!("Error writing config file: {e}");
                println!("Exiting");
                std::process::exit(1);
            }
        }
    }
}
