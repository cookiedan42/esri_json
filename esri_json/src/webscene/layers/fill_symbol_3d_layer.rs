//! Representation of a [FillSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/fillSymbol3DLayer/)
//!
//! FillSymbol3DLayer is used to render the surfaces of flat 2D Polygon geometries and 3D volumetric meshes in a SceneView.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use crate::webscene::common::PolygonPatternInternal;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::{
    ColorMixMode, Edges, LineCap, LinePattern, MaterialColorMixMode as Material, Outline,
    PolygonPattern,
};

marker_type!(T, "Fill");

/// Representation of a [FillSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/fillSymbol3DLayer/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct FillSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none", rename = "castShadows")]
    cast_shadows: Option<bool>, // only used by mesh 3d
    #[serde(skip_serializing_if = "Option::is_none")]
    edges: Option<Edges>, // only used by mesh 3d
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outline: Option<Outline>, // only used by polygon 3d
    #[serde(skip_serializing_if = "Option::is_none")]
    pattern: Option<PolygonPatternInternal>,
}

// Builder pattern
impl FillSymbol3DLayer {
    /// Only used by mesh 3d
    pub fn cast_shadows(mut self, cast_shadows: bool) -> Self {
        self.cast_shadows = Some(cast_shadows);
        self
    }
    /// Specifies an edge visualization style (only applies to [`MeshSymbol3D`](crate::webscene::mesh_symbol_3d::MeshSymbol3DLayers)). Edges describe the style applied to visually important edges of 3D objects.
    pub fn edges(mut self, edges: Edges) -> Self {
        self.edges = Some(edges);
        self
    }
    /// The material used to shade the geometry, including colorMixMode options.
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    /// The outline of the symbol layer (only applies to [`PolygonSymbol3D`](crate::webscene::polygon_symbol_3d::PolygonSymbol3DLayers)).
    pub fn outline(mut self, outline: Outline) -> Self {
        self.outline = Some(outline);
        self
    }
    /// The pattern used to render the fill of the polygon [`PolygonSymbol3D`](crate::webscene::polygon_symbol_3d::PolygonSymbol3DLayers)).
    pub fn pattern(mut self, pattern: PolygonPattern) -> Self {
        self.pattern = Some(pattern.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let fill_symbol_3d_layer = r#"{
        "type": "Fill",
          "material": {
            "color": [
              255,
              0,
              0
            ],
            "colorMixMode": "replace"
          },
          "outline": {
            "size": 1,
            "color": [
              255,
              0,
              0
            ],
            "pattern": {
              "type": "style",
              "style": "dash"
            }
          }
        }"#;
        let de: FillSymbol3DLayer = serde_json::from_str(fill_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: FillSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
