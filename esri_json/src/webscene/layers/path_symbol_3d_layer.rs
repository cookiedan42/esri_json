//! Representation of a [PathSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/pathSymbol3DLayer/)
//!
//! PathSymbol3DLayer renders polyline geometries by extruding a 2D profile along the line, resulting in visualizations like tubes, walls, etc.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::{LineJoin, Material};

marker_type!(T, "Path");

/// Representation of a [PathSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/pathSymbol3DLayer/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PathSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cap: Option<Cap>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "castShadows")]
    cast_shadows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    join: Option<LineJoin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    profile: Option<Profile>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "profileRotation")]
    profile_rotation: Option<ProfileRotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

/// Builder pattern
impl PathSymbol3DLayer {
    /// The position of the extrusion profile with respect to the polyline geometry.
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = Some(anchor);
        self
    }
    /// Shape of the tips at the start and end of each path geometry
    pub fn cap(mut self, cap: Cap) -> Self {
        self.cap = Some(cap);
        self
    }
    /// Boolean to control the shadow casting behaviour of the rendered geometries
    pub fn cast_shadows(mut self, cast_shadows: bool) -> Self {
        self.cast_shadows = Some(cast_shadows);
        self
    }
    // Path height in meters. If unspecified, it is equal to `width`
    pub fn height(mut self, height: f64) -> Self {
        self.height = Some(height);
        self
    }
    /// Shape of the intersection of two line segments
    pub fn join(mut self, join: LineJoin) -> Self {
        self.join = Some(join);
        self
    }
    /// The material used to shade the geometry
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    /// The shape which is extruded along the line
    pub fn profile(mut self, profile: Profile) -> Self {
        self.profile = Some(profile);
        self
    }
    /// Specifies the axes about which the profile may be rotated at the joins. Constraining the rotation axes leads to a fixed orientation of the profile for the specified directions
    pub fn profile_rotation(mut self, profile_rotation: ProfileRotation) -> Self {
        self.profile_rotation = Some(profile_rotation);
        self
    }
    /// Path size (diameter) in meters. Ignored if either `width` or `height` are present
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    /// Path width in meters. If unspecified, it is equal to `height`
    pub fn width(mut self, width: f64) -> Self {
        self.width = Some(width);
        self
    }
}

/// The position of the extrusion profile with respect to the polyline geometry.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Anchor {
    Bottom,
    Center,
    Top,
}

/// Shape of the tips at the start and end of each path geometry
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Cap {
    Butt,
    None,
    Round,
    Square,
}

/// The shape which is extruded along the line
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Profile {
    Circle,
    Quad,
}

/// Specifies the axes about which the profile may be rotated at the joins. Constraining the rotation axes leads to a fixed orientation of the profile for the specified directions
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ProfileRotation {
    All,
    Heading,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn red_tube_example() {
        let path_symbol_3d_layer = r#"{
            "type": "Path",
            "material": {
                "color": [
                255,
                0,
                0
                ]
            },
            "size": 12
        }"#;

        let de: PathSymbol3DLayer = serde_json::from_str(path_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PathSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn thin_wall_example() {
        let path_symbol_3d_layer = r#"{
            "type": "Path",
            "profile": "quad",
            "width": 0,
            "height": 50,
            "anchor": "bottom",
            "profileRotation": "heading"
        }"#;

        let de: PathSymbol3DLayer = serde_json::from_str(path_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PathSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn strip_example() {
        let path_symbol_3d_layer = r#"{
            "type": "Path",
            "profile": "quad",
            "width": 10,
            "height": 1,
            "anchor": "center",
            "profileRotation": "heading"
        }"#;

        let de: PathSymbol3DLayer = serde_json::from_str(path_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PathSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
