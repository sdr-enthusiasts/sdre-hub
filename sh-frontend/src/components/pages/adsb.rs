// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use gloo_utils::document;
use leaflet::{LatLng, Map, MapOptions, TileLayer};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, Node};
use yew::{html::ImplicitClone, prelude::*};

pub enum Msg {}

pub struct ShMapComponent {
    map: Map,
    lat: Point,
    container: HtmlElement,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point(pub f64, pub f64);

#[derive(PartialEq, Clone, Debug)]
pub struct City {
    pub name: String,
    pub lat: Point,
}

impl ImplicitClone for City {}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub city: City,
}

impl ShMapComponent {
    fn render_map(&self) -> Html {
        let node: &Node = &self.container.clone().into();
        Html::VRef(node.clone())
    }
}

impl Component for ShMapComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();

        let container: Element = document().create_element("div").unwrap();
        let container: HtmlElement = container.dyn_into().unwrap();
        container.set_class_name("map");
        let leaflet_map = Map::new_with_element(&container, &MapOptions::default());
        Self {
            map: leaflet_map,
            container,
            lat: props.city.lat,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.map
                .set_view(&LatLng::new(self.lat.0, self.lat.1), 11.0);
            add_tile_layer(&self.map);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let props = ctx.props();

        if self.lat == props.city.lat {
            false
        } else {
            self.lat = props.city.lat;
            self.map
                .set_view(&LatLng::new(self.lat.0, self.lat.1), 11.0);
            true
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        log::debug!("Rendering map.");

        html! {
            <div class="map-container component-container h-full w-full">
                {self.render_map()}
            </div>
        }
    }
}

fn add_tile_layer(map: &Map) {
    TileLayer::new("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png").add_to(map);
}
