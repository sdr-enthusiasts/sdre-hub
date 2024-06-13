// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::fmt::Display;
use web_sys::HtmlInputElement;
use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum InputFieldType {
    Text,
    Number,
    Select,
}

#[derive(Clone, PartialEq, Eq)]
pub enum CoordinateType {
    Latitude,
    Longitude,
}

impl Display for InputFieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Select => write!(f, "select"),
            Self::Number => write!(f, "number"),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct NumberProperties {
    pub max: String,
    pub min: String,
    pub step: String,
    pub value: String,
}

impl NumberProperties {
    #[must_use]
    pub fn new(coordinate_type: &CoordinateType, initial_value: String) -> Self {
        let max = match coordinate_type {
            CoordinateType::Latitude => "90",
            CoordinateType::Longitude => "180",
        }
        .to_string();

        let min = match coordinate_type {
            CoordinateType::Latitude => "-90",
            CoordinateType::Longitude => "-180",
        }
        .to_string();

        Self {
            max,
            min,
            step: "0.00001".to_string(),
            value: initial_value,
        }
    }
}

impl Default for NumberProperties {
    fn default() -> Self {
        Self {
            max: "180".to_string(),
            min: "180".to_string(),
            step: "0.00001".to_string(),
            value: "0.0".to_string(),
        }
    }
}

pub fn make_sure_string_has_five_digits(value: String) -> String {
    let value = value.parse::<f64>().unwrap();
    format!("{:.5}", value)
}

#[derive(Clone, PartialEq, Properties)]
pub struct InputFieldProps {
    pub input_value: String,
    // pub on_cautious_change: Callback<ChangeData>,
    pub label: String,
    pub field_type: InputFieldType,
    #[prop_or_default]
    pub select_options: Option<Vec<String>>,
    #[prop_or_default]
    pub number_properties: Option<NumberProperties>,
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
        number_properties,
    } = props;

    let onchange = {
        // format the number to always have 5 decimal places
        let field_type = field_type.clone();

        Callback::from(move |event: Event| {
            event.prevent_default();

            let target = event
                .target()
                .unwrap()
                .dyn_into::<HtmlInputElement>()
                .unwrap();

            let value = target.value();
            let value = match field_type {
                InputFieldType::Number => {
                    make_sure_string_has_five_digits(value)
                },
                _ => value
            };

            target.set_value(&value);
        })
    };

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
                                    onchange={onchange}
                                />
                            }
                        },
                        InputFieldType::Number => {
                            if number_properties.is_none() {
                                log::error!("Number properties must be provided for a number field");

                                return html! {
                                    { "Field type is number but no number properties were provided" }
                                }
                            }

                            let number_properties = number_properties.as_ref().unwrap();

                            html! {
                                <input
                                    /* onchange={on_cautious_change} */
                                    type={field_type.to_string()}
                                    value={input_value.clone()}
                                    name={name.clone()}
                                    ref={input_node_ref.clone()}
                                    class="text-black"
                                    readonly={*read_only}
                                    max={number_properties.max.clone()}
                                    min={number_properties.min.clone()}
                                    step={number_properties.step.clone()}
                                    onchange={onchange}
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
