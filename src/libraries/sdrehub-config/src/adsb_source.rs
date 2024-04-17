// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::{collections::HashMap, fmt, marker::PhantomData, str::FromStr};

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use void::Void;

use crate::address::SHAdsbConfig;

pub trait SourceTrait {
    fn new() -> Self;
    fn insert(&mut self, value: SHAdsbConfig);
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct AdsbSource {
    addresses: Vec<SHAdsbConfig>,
}

impl FromStr for AdsbSource {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self::new();

        for address in s.split(',') {
            if let Some(address) = SHAdsbConfig::new(address) {
                output.insert(address);
            }
        }

        Ok(output)
    }
}

impl SourceTrait for AdsbSource {
    fn new() -> Self {
        Self {
            addresses: Vec::new(),
        }
    }

    fn insert(&mut self, value: SHAdsbConfig) {
        self.addresses.push(value);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum FieldTypes {
    Address(String),
    Port(u32),
    Position(f64)
}

///# Errors
/// will return an error if the input fails to deserialize
 pub fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void> + SourceTrait,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void> + SourceTrait,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, mut map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.

            let mut source = T::new();

            if let Some(_key) = map.next_key::<String>()? {
                // loop through the keys and values

                match map.next_value::<Vec<HashMap<String, FieldTypes>>>() {
                    Ok(value) => {
                        if value.is_empty() {
                            return Ok(source);
                        }

                        for item in &value {
                            let port = match item.get("port").unwrap().to_owned() {
                                FieldTypes::Port(port) => {
                                    if (0..=65535).contains(&port) {
                                        port
                                    } else {
                                        return Err(de::Error::custom("Port out of range"));
                                    }
                                }
                                _ => {
                                    return Err(de::Error::custom("Port not found"));
                                }
                            };

                            let FieldTypes::Address(address) = item.get("address").unwrap().to_owned() else {
                                return Err(de::Error::custom("Address not found"));
                            };

                            let latitude = match item.get("latitude").unwrap().to_owned() {
                                FieldTypes::Position(latitutde) => {
                                    if (-90.0..=90.0).contains(&latitutde) {
                                        latitutde
                                    } else {
                                        return Err(de::Error::custom("Latitude out of range"));
                                    }
                                }
                                _ => {
                                    return Err(de::Error::custom("Latitude not found"));
                                }
                            };

                            let longitude = match item.get("longitude").unwrap().to_owned() {
                                FieldTypes::Position(longitude) => {
                                    if (-180.0..=180.0).contains(&longitude) {
                                        longitude
                                    } else {
                                        return Err(de::Error::custom("Longitude out of range"));
                                    }
                                }
                                _ => {
                                    return Err(de::Error::custom("Longitude not found"));
                                }
                            };

                            let address = SHAdsbConfig::new_from_parts(address, port, latitude, longitude);

                            source.insert(address);
                        }
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }
            //return Err(de::Error::custom("error"));
            Ok(source)
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            // is the seq a string?

            let length = seq.size_hint().unwrap_or(0);

            if length == 0 {
                return Ok(T::new());
            }

            let mut source = T::new();

            while let Some(value) = seq.next_element::<String>()? {
                if let Some(address) = SHAdsbConfig::new(&value) {
                    source.insert(address);
                }
            }

            Ok(source)
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
