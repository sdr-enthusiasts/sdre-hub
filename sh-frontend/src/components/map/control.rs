// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::pages::adsb::City;
use yew::{html::ImplicitClone, prelude::*};

pub enum Msg {
    CityChosen(City),
}

pub struct Control {
    cities: Vec<City>,
}

#[derive(PartialEq, Clone)]
pub struct Cities {
    pub list: Vec<City>,
}

impl ImplicitClone for Cities {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub cities: Cities,
    pub select_city: Callback<City>,
}

impl Control {
    fn button(ctx: &Context<Self>, city: City) -> Html {
        let name = city.name.clone();
        let cb = ctx.link().callback(move |_| Msg::CityChosen(city.clone()));
        html! {
            <button onclick={cb}>{name}</button>
        }
    }
}

impl Component for Control {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            cities: ctx.props().cities.list.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::CityChosen(city) => {
                log::info!("Update: {:?}", city.name);
                ctx.props().select_city.emit(city);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="control component-container">
                <h1>{"Choose a city"}</h1>
                <div>
                    {for self.cities.iter().map(|city| Self::button(ctx, city.clone()))}
                    </div>

            </div>
        }
    }
}
