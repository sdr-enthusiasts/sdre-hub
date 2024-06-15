// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use sh_common::UserWssMessage;
use yew::prelude::*;

use super::alert_boxes::AlertBoxToShow;

// FIXME: This should probably be renamed and refactored to a different file name
#[derive(Properties, Clone, PartialEq)]
pub struct WssCommunicationProps {
    pub send_message: Callback<UserWssMessage>,
    pub request_alert_box: Callback<AlertBoxToShow>,
}
