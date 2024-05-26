// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use crate::components::{
    map_components::control::Cities,
    pages::adsb::{City, Point, ShMapComponent},
};
use yew::prelude::*;

pub enum Msg {
    SelectCity(City),
}

pub struct ShMap {
    city: City,
    cities: Cities,
}

impl Component for ShMap {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let aachen = City {
            name: "Aachen".to_string(),
            lat: Point(50.7597f64, 6.0967f64),
        };
        let stuttgart = City {
            name: "Stuttgart".to_string(),
            lat: Point(48.7784f64, 9.1742f64),
        };
        let cities: Cities = Cities {
            list: vec![aachen, stuttgart],
        };
        let city = cities.list[0].clone();
        Self { city, cities }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SelectCity(city) => {
                self.city = self
                    .cities
                    .list
                    .iter()
                    .find(|c| c.name == city.name)
                    .unwrap()
                    .clone();
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                <ShMapComponent city={&self.city}  />
            </>
        }
    }
}
