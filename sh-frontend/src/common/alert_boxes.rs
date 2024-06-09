// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

#[derive(Debug, Default)]
pub enum AlertBoxToShow {
    ConfigWriteSuccess,
    ConfigWriteFailure,
    UnsavedChanges,
    #[default]
    None
}