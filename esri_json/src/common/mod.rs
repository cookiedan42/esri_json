//! Common types used by multiple modules
//! this covers types that cross web scene and web map

use serde::{Deserialize, Serialize};
/// Marker structs for type field  
///
/// Usage: `marker_type!(EsriSFSType, "esriSFS");`
///
macro_rules! marker_type {
    ($name:ident, $value:literal) => {
        #[derive(Clone, Debug, PartialEq, Default)]
        struct $name;

        impl serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str($value)
            }
        }

        impl<'de> serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                match String::deserialize(deserializer)? {
                    s if s == $value => Ok($name),
                    _ => Err(serde::de::Error::custom(format!("Expected '{}'", $value))),
                }
            }
        }
    };
}
pub(crate) use marker_type;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[serde(deny_unknown_fields)]
/// A color is either RGB or RGBA
pub enum Color {
    Rgba(u8, u8, u8, u8),
    Rgb(u8, u8, u8),
}
