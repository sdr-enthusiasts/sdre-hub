// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

// We need this roundabout nonsense because ShConfig uses some filesystem stuff to get
// the config file. This was causing a load of problems in the WASM code, so we're
// bypassing that by creating a new struct that doesn't have those dependencies.
// The rest of the config can share a common code base

pub mod sh_web_config;
pub mod sh_web_sdrehub;
