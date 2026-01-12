//! Representation of a [IconSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/iconSymbol3DLayer/)
//!
//! IconSymbol3DLayer is used to render Point geometries using a flat 2D icon (e.g. a circle) with a PointSymbol3D in a SceneView.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::Material;

marker_type!(T, "Icon");

/// Representation of a [IconSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/iconSymbol3DLayer/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct IconSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor: Option<Anchor>,
    #[serde(skip_serializing_if = "Option::is_none")]
    anchor_position: Option<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    outline: Option<Outline>,
    #[serde(skip_serializing_if = "Option::is_none")]
    resource: Option<Resource>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
}
/// Builder pattern
impl IconSymbol3DLayer {
    /// Anchor Position
    pub fn anchor(mut self, anchor: Anchor) -> Self {
        self.anchor = Some(anchor);
        self
    }
    /// When anchor is set to [`Anchor::Relative`], this property specifies the position within the icon that should coincide with the feature geometry.
    /// Otherwise it is ignored. The position is defined as a factor of the icon dimensions that is added to the
    /// icon center: positionInIcon = (0.5 + anchorPosition) * `size`, where `size` is the original size of the icon resource
    pub fn anchor_position(mut self, position: [f64; 2]) -> Self {
        self.anchor_position = Some(position);
        self
    }
    /// Rotation angle in degrees. The rotation is defined in screen space, with a rotation of 0 degrees (default value) pointing in the direction of the Y-axis. Positive values indicate clockwise rotation.
    pub fn angle(mut self, angle: f64) -> Self {
        self.angle = Some(angle);
        self
    }
    /// The material used to shade the geometry.
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    /// Sets properties of the outline of the IconSymbol3DLayer.
    pub fn outline(mut self, outline: Outline) -> Self {
        self.outline = Some(outline);
        self
    }
    /// The shape (primitive) or image URL (href) used to visualize the features.
    pub fn resource(mut self, resource: Resource) -> Self {
        self.resource = Some(resource);
        self
    }
    /// Icon size in points, positive only
    pub fn size(mut self, size: f64) -> Self {
        // TODO: enforce this constraint
        self.size = Some(size);
        self
    }
}

/// Representation of a [IconSymbol3DLayer Resource](https://developers.arcgis.com/web-scene-specification/objects/iconSymbol3DLayer_resource/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Resource {
    #[serde(skip_serializing_if = "Option::is_none", rename = "dataURI")]
    data_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    primitive: Option<IconPrimitive>,
}
/// Builder pattern
impl Resource {
    /// an image encoded as base64 string, starting with `data:image/`
    pub fn data_uri(mut self, data_uri: String) -> Self {
        self.data_uri = Some(data_uri);
        self
    }
    /// URL to the returned image.
    pub fn href(mut self, href: String) -> Self {
        self.href = Some(href);
        self
    }
    /// Specifies the type of symbol used.
    pub fn primitive(mut self, primitive: IconPrimitive) -> Self {
        self.primitive = Some(primitive);
        self
    }
}

/// Specifies the type of symbol used.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum IconPrimitive {
    Circle,
    Cross,
    Kite,
    Square,
    Triangle,
    X,
}

/// Anchor Position
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
#[serde(rename_all = "camelCase")]
pub enum Anchor {
    Bottom,
    BottomLeft,
    BottomRight,
    Center,
    Left,
    Relative,
    Right,
    Top,
    TopLeft,
    TopRight,
}

/// Sets properties of the outline of the IconSymbol3DLayer.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Outline {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparency: Option<f64>,
}
/// Builder pattern
impl Outline {
    /// Color is represented as a three or four-element array.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// Outline size in points, positive only
    pub fn size(mut self, size: f64) -> Self {
        // TODO: enfornce this constraint
        self.size = Some(size);
        self
    }
    /// The value has to lie between 100 (full transparency) and 0 (full opacity).
    pub fn transparency(mut self, transparency: f64) -> Self {
        // TODO: enforce this constaint
        self.transparency = Some(transparency);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_symbol_3d_layer() {
        let icon_symbol_3d_layer = r#"
            {
            "type": "Icon",
            "resource": {
                "primitive": "kite"
            },
            "material": {
                "color": [
                255,
                255,
                255
                ]
            },
            "size": 12,
            "angle": 90,
            "anchor": "bottom",
            "outline": {
                "size": 1,
                "color": [
                0,
                0,
                0
                ]
            }
        }"#;

        let de: IconSymbol3DLayer = serde_json::from_str(icon_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: IconSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn test_icon_symbol_3d_layer2() {
        let icon_symbol_3d_layer = r#"{
            "type": "Icon",
            "resource": {
                "href": "http://hostname/icon.png"
            },
            "size": 12,
            "anchor": "relative",
            "anchorPosition": [
                0.25,
                0.4
            ]
        }"#;

        let de: IconSymbol3DLayer = serde_json::from_str(icon_symbol_3d_layer).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: IconSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
