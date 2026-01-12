//! Representation of a [Callout](https://developers.arcgis.com/web-scene-specification/objects/callout/)
//!
//! Callout configuration for a symbol.

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
pub use crate::webscene::common::Material;

marker_type!(CalloutType, "line");

/// Representation of a [Callout](https://developers.arcgis.com/web-scene-specification/objects/callout/)
///
/// Callout configuration for a symbol.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Callout {
    #[serde(rename = "type")]
    _type: CalloutType,

    #[serde(skip_serializing_if = "Option::is_none")]
    border: Option<Material>, // Border has the same fields as material
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparency: Option<f64>,
}

/// Builder pattern
impl Callout {
    /// Optional border on the line that is used to improve the contrast of the line color against various background colors.
    pub fn border(mut self, border: Material) -> Self {
        self.border = Some(border);
        self
    }
    /// The color of the line.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// The width of the line in points.
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    /// A value between 100 (full transparency) and 0 (full opacity).
    pub fn transparency(mut self, transparency: f64) -> Self {
        self.transparency = Some(transparency);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leader_line_example() {
        let callout = r#"{
            "type": "line",
            "size": 0.75,
            "color": [
                255,
                255,
                255
            ],
            "transparency": 20,
            "border": {
                "color": [
                50,
                50,
                50
                ],
                "transparency": 50
            }
        }"#;
        let de: Callout = serde_json::from_str(callout).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Callout = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
