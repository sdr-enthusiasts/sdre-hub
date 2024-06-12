// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::fmt::Display;

use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum InputFieldType {
    Text,
    Select,
}

impl Display for InputFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Select => write!(f, "select"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub input_value: String,
    // pub on_cautious_change: Callback<ChangeData>,
    pub label: String,
    pub field_type: InputFieldType,
    pub select_options: Option<Vec<String>>,
    pub name: String,
    pub input_node_ref: NodeRef,
    #[prop_or(false)]
    pub read_only: bool,
}

#[function_component(InputField)]
pub fn input_field(props: &InputFieldProps) -> Html {
    let InputFieldProps {
        input_value,
        // on_cautious_change,
        label,
        field_type,
        select_options,
        name,
        input_node_ref,
        read_only,
    } = props;

    html! {
        <>
        <div><label for={name.clone()}>
        { label }</label></div>
            <div>
                {
                    match field_type {
                        InputFieldType::Text => {
                            if select_options.is_some() {
                                log::warn!("Select options were provided for a text field. Ignoring them.");
                            }

                            html! {
                                <input
                                    /* onchange={on_cautious_change} */
                                    type={field_type.to_string()}
                                    value={input_value.clone()}
                                    name={name.clone()}
                                    ref={input_node_ref.clone()}
                                    class="text-black"
                                    readonly={*read_only}
                                />
                            }
                        },
                        InputFieldType::Select => {
                            if select_options.is_none() {
                                log::error!("Select options must be provided for a select field");

                                return html! {
                                    { "Field type is select but no select options were provided" }
                                }
                            }

                            html! {
                                <select
                                    /* onchange={on_cautious_change} */
                                    value={input_value.clone()}
                                    name={name.clone()}
                                    ref={input_node_ref.clone()}
                                    class="text-black"
                                    disabled={*read_only}
                                >
                                    {
                                        select_options.as_ref().unwrap().iter().map(|option| {
                                            let selected = option == input_value;
                                            html! {
                                                <option value={option.clone()} {selected}>{ option }</option>
                                            }
                                        }).collect::<Html>()
                                    }
                                </select>
                            }
                        }
                    }
                }
            </div>
        </>
    }
}
