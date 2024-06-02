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

use std::env;

use directories::ProjectDirs;
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
use web::sh_web_config::ShWebConfig;

pub mod acars_router_source;
pub mod address;
pub mod adsb_source;
pub mod map;
pub mod sdrehub;
pub mod source;
pub mod web;

// FIXME: env variables require a dot between the prefix and the variable name. This is not ideal. Should be able to use underscores

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
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

    #[must_use]
    pub fn to_web_config(&self) -> ShWebConfig {
        ShWebConfig {
            app: self.app.to_web_sdrehub(),
            enabled_data_sources: self.enabled_data_sources.clone(),
            data_sources: self.data_sources.clone(),
            map: self.map.clone(),
        }
    }

    fn get_application_data_path() -> String {
        if env::var("SH_DATA_PATH").is_ok() {
            let path = env::var("SH_DATA_PATH").unwrap();
            if std::path::Path::new(&path).exists() {
                // canonicalize the path
                match std::fs::canonicalize(&path) {
                    Ok(canonical_path) => {
                        if let Some(canonical_path_str) = canonical_path.to_str() {
                            return canonical_path_str.to_string();
                        }
                    }
                    Err(e) => {
                        println!("Error getting config file path: {e}");
                        println!("Exiting");
                        std::process::exit(1);
                    }
                }
            }

            println!("Error getting config file path");
            println!("Exiting");
            std::process::exit(1);
        }

        // Otherwise, use the OS default pathing
        ProjectDirs::from("org", "sdre-e", "sdr-e-hub").map_or_else(
            || {
                println!("Error getting config file path");
                println!("Exiting");
                std::process::exit(1);
            },
            |proj_dirs| {
                proj_dirs.config_dir().to_str().map_or_else(
                    || {
                        println!("Error getting config file path");
                        println!("Exiting");
                        std::process::exit(1);
                    },
                    |path| {
                        // make the directory if it doesn't exist
                        if std::path::Path::new(&path).exists() {
                            path.to_string()
                        } else {
                            match std::fs::create_dir_all(path) {
                                Ok(()) => path.to_string(),
                                Err(e) => {
                                    println!("Error creating config directory: {e}");
                                    println!("Exiting");
                                    std::process::exit(1);
                                }
                            }
                        }
                    },
                )
            },
        )
    }

    fn get_config_file_path() -> String {
        let path = Self::get_application_data_path();
        format!("{path}/sh_config.toml")
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
        let file_path = Self::get_config_file_path();
        Self::get_config(&file_path)
    }

    pub fn show_config(&self) {
        debug!("Config: {:#?}", self);
    }

    pub fn enable_logging(&self) {
        println!("Enabling logging with level: {}", self.app.log_level);
        self.app.log_level.enable_logging();
    }

    fn get_config_as_toml_string(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    pub fn write_config(&self) {
        let file_path = Self::get_config_file_path();
        let config = self.get_config_as_toml_string();
        println!("Writing config file to: {file_path}");
        println!("Config: {config}");

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
