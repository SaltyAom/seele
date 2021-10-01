use std::{ fmt, num::NonZeroI32 };

use serde::{ Serialize, de::{Deserialize, Deserializer, Visitor} };

use async_graphql::*;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub struct NumericNonZeroU32(NonZeroI32);

impl<'de> Deserialize<'de> for NumericNonZeroU32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct MyVisitor;

        impl<'de> Visitor<'de> for MyVisitor {
            type Value = NumericNonZeroU32;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("integer or string")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match NonZeroI32::new(val as i32) {
                    Some(val) => Ok(NumericNonZeroU32(val)),
                    None => Err(E::custom("invalid integer value")),
                }
            }

            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match val.parse::<u64>() {
                    Ok(val) => self.visit_u64(val),
                    Err(_) => Err(E::custom("failed to parse integer")),
                }
            }
        }

        deserializer.deserialize_any(MyVisitor)
    }
}