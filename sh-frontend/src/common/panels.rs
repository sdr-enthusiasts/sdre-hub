// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Default, Deserialize, Serialize)]
pub enum Panels {
    Messages,
    Map,
    Stats,
    Settings,
    Help,
    #[default]
    None,
}

// implement to_string for Panels
impl std::fmt::Display for Panels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Messages => write!(f, "Messages"),
            Self::Map => write!(f, "Map"),
            Self::Stats => write!(f, "Stats"),
            Self::Settings => write!(f, "Settings"),
            Self::Help => write!(f, "Help"),
            Self::None => write!(f, "None"),
        }
    }
}

// implement from for Panels
impl From<&str> for Panels {
    fn from(s: &str) -> Self {
        match s {
            "Messages" => Self::Messages,
            "Map" => Self::Map,
            "Stats" => Self::Stats,
            "Settings" => Self::Settings,
            "Help" => Self::Help,
            _ => Self::None,
        }
    }
}

impl Panels {
    #[must_use]
    pub fn next(&self, skip: Self) -> Self {
        // go to the next panel, skipping the one we're currently on

        match self {
            Self::Messages => {
                if skip == Self::Map {
                    Self::Stats
                } else {
                    Self::Map
                }
            }
            Self::Map => {
                if skip == Self::Stats {
                    Self::Settings
                } else {
                    Self::Map
                }
            }
            Self::Stats => {
                if skip == Self::Settings {
                    Self::Help
                } else {
                    Self::Settings
                }
            }
            Self::Settings => {
                if skip == Self::Help {
                    Self::Messages
                } else {
                    Self::Help
                }
            }
            Self::Help => {
                if skip == Self::Messages {
                    Self::Map
                } else {
                    Self::Messages
                }
            }

            Self::None => Self::None,
        }
    }

    #[must_use]
    pub fn previous(&self, skip: Self) -> Self {
        // go to the previous panel, skipping the one we're currently on

        match self {
            Self::Messages => {
                if skip == Self::Help {
                    Self::Settings
                } else {
                    Self::Help
                }
            }
            Self::Map => {
                if skip == Self::Messages {
                    Self::Help
                } else {
                    Self::Messages
                }
            }
            Self::Stats => {
                if skip == Self::Map {
                    Self::Messages
                } else {
                    Self::Map
                }
            }
            Self::Settings => {
                if skip == Self::Stats {
                    Self::Map
                } else {
                    Self::Stats
                }
            }
            Self::Help => {
                if skip == Self::Settings {
                    Self::Stats
                } else {
                    Self::Settings
                }
            }

            Self::None => Self::None,
        }
    }
}
