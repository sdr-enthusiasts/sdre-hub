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

use crate::address::ShAcarsRouterConfig;

pub trait SourceTrait {
    fn new() -> Self;
    fn insert(&mut self, value: ShAcarsRouterConfig);
}
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
pub struct AcarsRouterSource {
    addresses: Vec<ShAcarsRouterConfig>,
}

impl FromStr for AcarsRouterSource {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Self::new();

        for address in s.split(',') {
            if let Some(address) = ShAcarsRouterConfig::new(address) {
                output.insert(address);
            }
        }

        Ok(output)
    }
}

impl SourceTrait for AcarsRouterSource {
    fn new() -> Self {
        Self {
            addresses: Vec::new(),
        }
    }

    fn insert(&mut self, value: ShAcarsRouterConfig) {
        self.addresses.push(value);
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone)]
#[serde(untagged)]
enum FieldTypes {
    #[serde(rename = "address")]
    Address(String),
    #[serde(rename = "port")]
    Port(u32),
}

/// # Errors
/// Returns an error if the input fails to deserialize
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
                                FieldTypes::Address(_) => {
                                    return Err(de::Error::custom("Port not found"));
                                }
                            };

                            let address = match item.get("address").unwrap().to_owned() {
                                FieldTypes::Address(address) => address,
                                FieldTypes::Port(_) => {
                                    return Err(de::Error::custom("Address not found"));
                                }
                            };

                            let address = ShAcarsRouterConfig::new_from_parts(address, port);

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
                if let Some(address) = ShAcarsRouterConfig::new(&value) {
                    source.insert(address);
                }
            }

            Ok(source)
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
