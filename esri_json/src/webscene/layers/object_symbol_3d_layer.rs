//! Representation of a [ObjectSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/objectSymbol3DLayer/)
//!
//! ObjectSymbol3DLayer is used to render Point geometries using a volumetric 3D shape (e.g., a sphere or cylinder) with a Symbol3D in a SceneView.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::Material;

marker_type!(T, "Object");

/// Representation of a [ObjectSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/objectSymbol3DLayer/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct ObjectSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "anchorPosition")]
    anchor_position: Option<[f64; 3]>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "castShadows")]
    cast_shadows: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    depth: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roll: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tilt: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

/// Builder pattern
impl ObjectSymbol3DLayer {
    /// The positioning of the object relative to the geometry
    pub const fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = Some(anchor);
        self
    }

    /// When `anchor` is set to [`Anchor::Relative`], this property specifies the positioning of the object relative to the geometry as a fraction of the symbol layer's bounding box. Otherwise it is ignored.
    pub const fn anchor_position(mut self, position: [f64; 3]) -> Self {
        self.anchor_position = Some(position);
        self
    }

    /// Boolean to control the shadow casting behaviour of the rendered geometries
    pub const fn cast_shadows(mut self, cast_shadows: bool) -> Self {
        self.cast_shadows = Some(cast_shadows);
        self
    }

    /// Object depth in meters, positive only
    pub fn depth(mut self, depth: f64) -> Result<Self, String> {
        if depth <= 0.0 {
            return Err(format!("Depth must be positive, got {}", depth));
        }
        self.depth = Some(depth);
        Ok(self)
    }

    /// Rotation angle around Z axis in degrees. At 0 degrees, the model points in the direction of the Y-axis. Positive values indicate clockwise rotation (when looked at from the top)
    pub const fn heading(mut self, heading: f64) -> Self {
        self.heading = Some(heading);
        self
    }

    /// Object height in meters, positive only
    pub fn height(mut self, height: f64) -> Result<Self, String> {
        if height <= 0.0 {
            return Err(format!("Height must be positive, got {}", height));
        }
        self.height = Some(height);
        Ok(self)
    }

    /// The material used to shade the geometry.
    pub const fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }

    /// The primitive shape (primitive) or external 3D model (href) used to visualize the points.
    pub fn resource(mut self, resource: Resource) -> Self {
        self.resource = Some(resource);
        self
    }

    /// Rotation angle around Y axis in degrees. At 0 degrees, the model is level. A positive value lifts the left part and lowers the right part of the model
    pub const fn roll(mut self, roll: f64) -> Self {
        self.roll = Some(roll);
        self
    }

    /// Rotation angle around X axis in degrees. At 0 degrees, the model is level. A positive value lifts the front and lowers the back of the model
    pub const fn tilt(mut self, tilt: f64) -> Self {
        self.tilt = Some(tilt);
        self
    }

    /// Object width in meters, positive only
    pub fn width(mut self, width: f64) -> Result<Self, String> {
        // format is non-const
        if width <= 0.0 {
            return Err(format!("Width must be positive, got {}", width));
        }
        self.width = Some(width);
        Ok(self)
    }
}

/// The positioning of the object relative to the geometry
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Anchor {
    Bottom,
    Center,
    Origin,
    Relative,
    Top,
}

/// Representation of a [ObjectSymbol3DLayer Resource](https://developers.arcgis.com/web-scene-specification/objects/objectSymbol3DLayer_resource/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primitive: Option<ObjectPrimitive>,
}
/// Builder pattern
impl Resource {
    pub fn href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }
    pub const fn primitive(mut self, primitive: ObjectPrimitive) -> Self {
        self.primitive = Some(primitive);
        self
    }
}

/// Representation of the `primitive` field in  [ObjectSymbol3DLayer Resource](https://developers.arcgis.com/web-scene-specification/objects/objectSymbol3DLayer_resource/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ObjectPrimitive {
    Cone,
    Cube,
    Cylinder,
    Diamond,
    InvertedCone,
    Sphere,
    Tetrahedron,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_symbol_layer() {
        let text = r#"{
            "type": "Object",
            "resource": {
                "primitive": "sphere"
            },
            "width": 50000,
            "height": 50000,
            "material": {
                "color": [
                128,
                255,
                0
                ]
            }
        }"#;

        let de: ObjectSymbol3DLayer = serde_json::from_str(text).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: ObjectSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn object_symbol_layer_with_3_axis_rotation() {
        let text = r#"{
            "type": "Object",
            "resource": {
                "primitive": "diamond"
            },
            "width": 50000,
            "height": 50000,
            "material": {
                "color": [
                128,
                255,
                0
                ]
            },
            "heading": 12,
            "tilt": 10,
            "roll": 5
        }"#;

        let de: ObjectSymbol3DLayer = serde_json::from_str(text).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: ObjectSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn object_symbol_layer_with_relative_anchor() {
        let text = r#"{
            "type": "Object",
            "resource": {
                "primitive": "diamond"
            },
            "width": 10000,
            "anchor": "relative",
            "anchorPosition": [
                0,
                -0.5,
                0
            ],
            "material": {
                "color": [
                128,
                255,
                0
                ]
            }
        }"#;

        let de: ObjectSymbol3DLayer = serde_json::from_str(text).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: ObjectSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn object_symbol_layer_with_casting_shadows_disabled() {
        let text = r#"{
            "type": "Object",
            "resource": {
                "primitive": "cone"
            },
            "width": 50,
            "height": 100,
            "castShadows": false
        }"#;

        let de: ObjectSymbol3DLayer = serde_json::from_str(text).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: ObjectSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
