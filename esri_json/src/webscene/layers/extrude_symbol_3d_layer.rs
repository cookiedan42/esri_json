//! Representation of a [ExtrudeSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/extrudeSymbol3DLayer/)
//!
//! ExtrudeSymbol3DLayer is used to render Polygon geometries by extruding them upward from the ground, creating a 3D volumetric object.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::{Edges, Material};

marker_type!(TypeString, "Extrude");

/// Representation of a [ExtrudeSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/extrudeSymbol3DLayer/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ExtrudeSymbol3DLayer {
    #[serde(rename = "type")]
    _type: TypeString,
    #[serde(skip_serializing_if = "Option::is_none", rename = "castShadows")]
    cast_shadows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    edges: Option<Edges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
}

/// Builder pattern
impl ExtrudeSymbol3DLayer {
    pub fn cast_shadows(mut self, cast_shadows: bool) -> Self {
        self.cast_shadows = Some(cast_shadows);
        self
    }
    pub fn edges(mut self, edges: Edges) -> Self {
        self.edges = Some(edges);
        self
    }
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrude_symbol_3d_layer() {
        let extrude_symbol_3d_layer = r#"
       {
        "type": "Extrude",
        "size": 100.5,
        "material": {
          "color": [
            244,
            247,
            134
          ]
        },
        "castShadows": false,
        "edges": {
          "type": "solid",
          "color": [
            50,
            50,
            50
          ],
          "transparency": 50,
          "extensionLength": 10,
          "size": 20
        }
      }
        "#;
        let de: ExtrudeSymbol3DLayer = serde_json::from_str(extrude_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: ExtrudeSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
