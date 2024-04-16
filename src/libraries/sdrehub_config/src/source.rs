// Copyright (C) 2024 Fred Clausen
// Use of this source code is governed by an MIT-style
// license that can be found in the LICENSE file or at
// https://opensource.org/licenses/MIT.

use std::{fmt, marker::PhantomData, str::FromStr};

use serde::{
    de::{self, MapAccess, SeqAccess, Visitor},
    Deserialize, Deserializer, Serialize,
};
use serde_inline_default::serde_inline_default;
use void::Void;

use crate::address::Address;

trait SourceTrait {
    fn new() -> Self;
    fn insert(&mut self, value: Address);
}

#[serde_inline_default]
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct EnabledDataSources {
    #[serde_inline_default(false)]
    pub adsb: bool,
    #[serde_inline_default(false)]
    pub acars: bool,
    #[serde_inline_default(false)]
    pub hfdl: bool,
    #[serde_inline_default(false)]
    pub inmarsat: bool,
    #[serde_inline_default(false)]
    pub iridium: bool,
    #[serde_inline_default(false)]
    pub vdlm2: bool,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Source {
    addresses: Vec<Address>,
}

impl FromStr for Source {
    type Err = Void;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut output = Source::new();

        for address in s.split(',') {
            if let Some(address) = Address::new(address.to_string()) {
                output.insert(address);
            }
        }

        Ok(output)
    }
}

impl SourceTrait for Source {
    fn new() -> Self {
        Source {
            addresses: Vec::new(),
        }
    }

    fn insert(&mut self, value: Address) {
        self.addresses.push(value);
    }
}

#[serde_inline_default]
#[derive(Debug, Serialize, Default, Deserialize)]
pub struct DataSources {
    #[serde_inline_default(Source::default())]
    #[serde(deserialize_with = "string_or_struct")]
    pub acars_routers: Source,
    #[serde_inline_default(Source::default())]
    #[serde(deserialize_with = "string_or_struct")]
    pub adsb_sources: Source,
}

fn string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
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
                match map.next_value::<Vec<String>>() {
                    Ok(value) => {
                        for address in value {
                            if let Some(address) = Address::new(address) {
                                source.insert(address);
                            }
                        }

                        return Ok(source);
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

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
                if let Some(address) = Address::new(value) {
                    source.insert(address);
                }
            }

            Ok(source)
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}
