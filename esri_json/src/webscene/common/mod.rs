use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use std::fmt;

mod cap;
pub use cap::{LineCap, LineJoin};

mod edges;
pub use edges::Edges;

mod line_pattern;
pub use line_pattern::LinePattern;
pub(crate) use line_pattern::LinePatternInternal;

mod outline;
pub use outline::Outline;

mod material;
pub use material::Material;

mod material_color_mix_mode;
pub use material_color_mix_mode::{ColorMixMode, MaterialColorMixMode};

mod pattern;
pub use pattern::PolygonPattern;
pub(crate) use pattern::PolygonPatternInternal;

pub(crate) mod line_marker;

pub mod font;

mod callout;
pub use callout::Callout;

mod style_origin;
pub use style_origin::StyleOrigin;

mod vertical_offset;
pub use vertical_offset::VerticalOffset;

/// Enum which represents a field which can be Pixels, Points or a number (treated as points)
#[derive(Debug, Clone, PartialEq)]
pub enum PxOrPt {
    Pixels(f64),
    Points(f64),
    Numeric(f64),
}

impl Serialize for PxOrPt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            PxOrPt::Pixels(value) => serializer.serialize_str(&format!("{}px", value)),
            PxOrPt::Points(value) => serializer.serialize_str(&format!("{}pt", value)),
            PxOrPt::Numeric(value) => serializer.serialize_f64(*value),
        }
    }
}

impl<'de> Deserialize<'de> for PxOrPt {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(PxOrPtVisitor)
    }
}

struct PxOrPtVisitor;

impl<'de> de::Visitor<'de> for PxOrPtVisitor {
    type Value = PxOrPt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a number, or a string ending with 'px' or 'pt'")
    }

    fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PxOrPt::Numeric(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PxOrPt::Numeric(value as f64))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(PxOrPt::Numeric(value as f64))
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if let Some(px_value) = value.strip_suffix("px") {
            let num = px_value
                .parse::<f64>()
                .map_err(|_| E::custom(format!("Invalid pixel value: {}", px_value)))?;
            Ok(PxOrPt::Pixels(num))
        } else if let Some(pt_value) = value.strip_suffix("pt") {
            let num = pt_value
                .parse::<f64>()
                .map_err(|_| E::custom(format!("Invalid point value: {}", pt_value)))?;
            Ok(PxOrPt::Points(num))
        } else {
            Err(E::custom(format!(
                "Expected string ending with 'px' or 'pt', got: {}",
                value
            )))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct T {
        val: PxOrPt,
    }

    #[test]
    fn test_deserialize_size_value() {
        let t_num = T {
            val: PxOrPt::Numeric(10.0),
        };
        let t_px = T {
            val: PxOrPt::Pixels(10.0),
        };
        let t_pt = T {
            val: PxOrPt::Points(10.0),
        };

        println! {"{:?}",serde_json::to_string(&t_num).unwrap()};
        println! {"{:?}",serde_json::to_string(&t_px).unwrap()};
        println! {"{:?}",serde_json::to_string(&t_pt).unwrap()};

        let _t_f64_deser: T = serde_json::from_str(r#"{"val":10.0}"#).unwrap();
        let _t_f32_deser: T = serde_json::from_str(r#"{"val":10}"#).unwrap();
        let _t_px_deser: T = serde_json::from_str(r#"{"val":"10px"}"#).unwrap();
        let _t_pt_deser: T = serde_json::from_str(r#"{"val":"10pt"}"#).unwrap();
    }
}
