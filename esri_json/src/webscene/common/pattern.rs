//! Representation of a [PolygonPattern](https://developers.arcgis.com/web-scene-specification/objects/polygonPattern/)
//!
//! The pattern used to render the fill of the polygon (only applies to PolygonSymbol3D).

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

marker_type!(T, "style");
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub(crate) struct PolygonPatternInternal {
    #[serde(rename = "type")]
    _type: T,
    style: Option<PolygonPattern>,
}

impl From<PolygonPattern> for PolygonPatternInternal {
    fn from(style: PolygonPattern) -> Self {
        Self {
            _type: T,
            style: Some(style),
        }
    }
}

/// Representation of a [polygonPattern](https://developers.arcgis.com/web-scene-specification/objects/polygonPattern/)  
///
/// The pattern used to render the fill of the polygon (only applies to PolygonSymbol3D).
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum PolygonPattern {
    BackwardDiagonal,
    Cross,
    DiagonalCross,
    ForwardDiagonal,
    Horizontal,
    None,
    Solid,
    Vertical,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polygon_pattern() {
        let polygon_pattern = r#"
        {
            "type": "style",
            "style": "cross"
        }
        "#;
        let de: PolygonPatternInternal = serde_json::from_str(polygon_pattern).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PolygonPatternInternal = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
