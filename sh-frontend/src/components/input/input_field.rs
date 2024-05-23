// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub input_value: String,
    // pub on_cautious_change: Callback<ChangeData>,
    pub label: String,
    pub field_type: String,
    pub name: String,
    pub input_node_ref: NodeRef,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        input_value,
        // on_cautious_change,
        label,
        field_type,
        name,
        input_node_ref,
    } = props;

    log::debug!("InputField: {:?}", input_value);
    html! {
        <label for="cautious-input">
                { label }
                <input
                    /* onchange={on_cautious_change} */
                    type={field_type.clone()}
                    value={input_value.clone()}
                    name={name.clone()}
                    ref={input_node_ref.clone()}
                    class="text-black"
                />
        </label>
    }
}