// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

use crate::services::temp_state::{MessageContext, WebAppStateTemp};

#[derive(Properties, Debug, PartialEq)]
pub struct MessageProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(MessageProvider)]
pub fn provider(props: &MessageProviderProps) -> Html {
    let msg_tempstate = use_reducer(|| WebAppStateTemp::default());

    html! {
        <ContextProvider<MessageContext> context={msg_tempstate}>
            {props.children.clone()}
        </ContextProvider<MessageContext>>
    }
}