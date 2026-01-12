//! Representation of a [TextSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/textSymbol3DLayer/)
//!
//! Symbol layer for text and font definitions.
//!
//! Includes re-exports of relevant types
//!

use crate::common::marker_type;
use serde::{Deserialize, Serialize};

pub use crate::common::Color;
/// Backgorund has the same fields as Material
pub use crate::webscene::common::Material as Background;
pub use crate::webscene::common::Material;
pub use crate::webscene::common::font;

marker_type!(T, "Text");

/// Representation of a [TextSymbol3DLayer](https://developers.arcgis.com/web-scene-specification/objects/textSymbol3DLayer/)
///
/// Symbol layer for text and font definitions.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct TextSymbol3DLayer {
    #[serde(rename = "type")]
    _type: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    // background has identical properties to material
    background: Option<Background>,
    #[serde(skip_serializing_if = "Option::is_none")]
    font: Option<font::Font>,
    #[serde(skip_serializing_if = "Option::is_none")]
    halo: Option<Halo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_alignment: Option<HorizontalAlignment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line_height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    material: Option<Material>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vertical_alignment: Option<VerticalAlignment>,
}
/// Builder pattern
impl TextSymbol3DLayer {
    /// Text background definition.
    pub fn background(mut self, background: Material) -> Self {
        self.background = Some(background);
        self
    }
    /// Font used for text symbols.
    pub fn font(mut self, font: font::Font) -> Self {
        self.font = Some(font);
        self
    }
    /// The halo surrounding the text
    pub fn halo(mut self, halo: Halo) -> Self {
        self.halo = Some(halo);
        self
    }
    /// Values representing the horizontal alignment of the text.
    pub fn horizontal_alignment(mut self, horizontal_alignment: HorizontalAlignment) -> Self {
        self.horizontal_alignment = Some(horizontal_alignment);
        self
    }
    /// Multiplier to scale the vertical distance between the baselines of text with multiple lines.
    pub fn line_height(mut self, line_height: f64) -> Self {
        self.line_height = Some(line_height);
        self
    }
    /// The material used to shade the geometry.
    pub fn material(mut self, material: Material) -> Self {
        self.material = Some(material);
        self
    }
    /// Font size in points, positive only
    pub fn size(mut self, size: f64) -> Result<Self, String> {
        if size <= 0.0 {
            return Err(format!("Size must be positive, got {}", size));
        }
        self.size = Some(size);
        Ok(self)
    }
    /// Text content in the label. Typically this property is not set, as text content is read from labeling field.
    pub fn text(mut self, text: String) -> Self {
        self.text = Some(text);
        self
    }
    /// Values representing the vertical alignment of the text.
    pub fn vertical_alignment(mut self, vertical_alignment: VerticalAlignment) -> Self {
        self.vertical_alignment = Some(vertical_alignment);
        self
    }
}

/// Values representing the horizontal alignment of the text.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum HorizontalAlignment {
    Center,
    Left,
    Right,
}

/// Values representing the vertical alignment of the text.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum VerticalAlignment {
    Baseline,
    Bottom,
    Middle,
    Top,
}

/// The halo surrounding the text
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Halo {
    color: Option<Color>,
    size: Option<f64>,
    transparency: Option<f64>,
}
/// Builder pattern
impl Halo {
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    pub fn transparency(mut self, transparency: f64) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&transparency) {
            return Err(format!(
                "Transparency must be a value between 0.0 and 100.0, got {}",
                transparency
            ));
        } else {
            self.transparency = Some(transparency);
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example() {
        let symbol = r#"{
            "size": 12,
            "type": "Text",
            "material": {
                "color": [
                255,
                255,
                0
                ]
            },
            "font": {
                "family": "Arial",
                "weight": "bold"
            },
            "lineHeight": 1.2,
            "horizontalAlignment": "left",
            "verticalAlignment": "top"
        }"#;
        let de: TextSymbol3DLayer = serde_json::from_str(symbol).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: TextSymbol3DLayer = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
