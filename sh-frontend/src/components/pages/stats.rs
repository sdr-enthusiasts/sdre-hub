// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

/// Home page
#[function_component(ShStatistics)]
pub fn statistics() -> Html {
    log::debug!("Rendering statistics page.");

    html! {
        {
            "Stats"
        }
    }
}
