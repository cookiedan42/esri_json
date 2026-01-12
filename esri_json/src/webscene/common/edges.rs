//! Representation of a [Edges](https://developers.arcgis.com/web-scene-specification/objects/edges/)   
//!
//! Specifies an edge visualization style (only applies to MeshSymbol3D). Edges describe the style applied to visually important edges of 3D objects.

use crate::common::Color;
use serde::{Deserialize, Serialize};

// TODO: refactor Eddges into a enum based on sketch/solid
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
enum T {
    Sketch,
    Solid,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
/// Representation of [Sketch Edge and Solid Edge](https://developers.arcgis.com/web-scene-specification/objects/edges/)
pub struct Edges {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparency: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "extensionLength")]
    extension_length: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
}

/// Constructor
impl Edges {
    pub fn new_sketch() -> Self {
        Self {
            _type: T::Sketch,
            color: None,
            transparency: None,
            extension_length: None,
            size: None,
        }
    }
    pub fn new_solid() -> Self {
        Self {
            _type: T::Solid,
            color: None,
            transparency: None,
            extension_length: None,
            size: None,
        }
    }
}

/// Builder pattern
impl Edges {
    /// Color is represented as a three or four-element array.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// The value has to lie between 100 (full transparency) and 0 (full opacity).
    pub fn transparency(mut self, transparency: f64) -> Self {
        // TODO: enforce this constraint
        self.transparency = Some(transparency);
        self
    }
    /// A size in points by which to extend edges beyond their original end points.
    pub fn extension_length(mut self, extension_length: f64) -> Self {
        self.extension_length = Some(extension_length);
        self
    }
    /// Edge size in points, positive only
    pub fn size(mut self, size: f64) -> Self {
        // TODO: enforce this constraint
        self.size = Some(size);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sketch_example() {
        let edges3d = r#"{
            "type": "sketch",
            "color": [
                255,
                0,
                0
            ],
            "size": 0.75,
            "transparency": 10,
            "extensionLength": 5
        }"#;
        let de: Edges = serde_json::from_str(edges3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let _serde: Edges = serde_json::from_str(&ser).unwrap();
    }

    #[test]
    fn solid_example() {
        let edges3d = r#"{
            "type": "solid",
            "color": [
                255,
                0,
                0
            ],
            "size": 0.75,
            "transparency": 10,
            "extensionLength": 5
        }"#;
        let de: Edges = serde_json::from_str(edges3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let _serde: Edges = serde_json::from_str(&ser).unwrap();
    }
}
