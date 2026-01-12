//! Representation of a [LineSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/lineSymbol3DLayer/)  
//!
//! LineSymbol3DLayer renders Polyline geometries using a flat 2D line with a LineSymbol3D in a 3D SceneView.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use crate::webscene::common::LinePatternInternal;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::line_marker::{LineMarker, LineSymbolMarkerStyle, Placement};
pub use crate::webscene::common::{LineCap, LineJoin, LinePattern, Material, PxOrPt};

marker_type!(T, "Line");

/// Representation of a [LineSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/lineSymbol3DLayer/)  
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct LineSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    cap: Option<LineCap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<LineJoin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<LineMarker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<LinePatternInternal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<PxOrPt>,
}

/// Builder pattern
impl LineSymbol3DLayer {
    /// Shape of the tips at the start and end of each line geometry. This also applies to the tips of each pattern segment along the line
    pub fn cap(mut self, cap: LineCap) -> Self {
        self.cap = Some(cap);
        self
    }
    /// Shape of the intersection of two line segments
    pub fn join(mut self, join: LineJoin) -> Self {
        self.join = Some(join);
        self
    }
    /// Represents markers placed at the start and end of each line geometry, or both
    pub fn marker(mut self, marker: LineMarker) -> Self {
        self.marker = Some(marker);
        self
    }
    /// The material used to shade the geometry
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    /// A pattern used to render a line
    pub fn pattern(mut self, pattern: LinePattern) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
    /// Line width in points, positive only
    pub fn size(mut self, size: PxOrPt) -> Self {
        // Todo: enforce this constraint
        self.size = Some(size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_symbol_3d_layer() {
        let line_symbol_3d_layer = r#"{
        "type": "Line",
        "material": {
          "color": [
            0,
            0,
            0
          ]
        },
        "join": "round",
        "cap": "round",
        "size": 2,
        "pattern": {
          "type": "style",
          "style": "dash"
        },
        "marker": {
          "type": "style",
          "placement": "end",
          "color": [
            255,
            0,
            0,
            255
          ]
        }
      }"#;
        let de: LineSymbol3DLayer = serde_json::from_str(line_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: LineSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
