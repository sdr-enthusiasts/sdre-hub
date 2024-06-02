// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use sh_common::UserWssMessage;
use yew::prelude::*;

#[derive(Properties, Clone, PartialEq)]
pub struct WssCommunicationProps {
    pub send_message: Callback<UserWssMessage>,
}
