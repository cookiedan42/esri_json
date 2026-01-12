//! Representation of a [Font](https://developers.arcgis.com/web-scene-specification/objects/font/)
//!
//! Font used for text symbols.

use serde::{Deserialize, Serialize};

/// Representation of a [Font](https://developers.arcgis.com/web-scene-specification/objects/font/)
//
// Font used for text symbols.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Font {
    #[serde(skip_serializing_if = "Option::is_none")]
    decoration: Option<Decoration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<Style>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight: Option<Weight>,
}

/// Builder pattern
impl Font {
    /// The text decoration.
    pub fn decoration(mut self, decoration: Decoration) -> Self {
        self.decoration = Some(decoration);
        self
    }
    /// The font family.
    pub fn family(mut self, family: String) -> Self {
        self.family = Some(family);
        self
    }
    /// The font size in points. Ignored when font is used on TextSymbol3DLayer.
    pub fn size(mut self, size: f64) -> Self {
        self.size = Some(size);
        self
    }
    /// The text style.
    pub fn style(mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
    /// The text weight.
    pub fn weight(mut self, weight: Weight) -> Self {
        self.weight = Some(weight);
        self
    }
}

/// The text decoration.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Decoration {
    LineThrough,
    None,
    Underline,
}

/// The text style.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Style {
    Italic,
    Normal,
    Oblique,
}

/// The text weight.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Weight {
    Bold,
    Bolder,
    Lighter,
    Normal,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn font_example() {
        let font = r#"{
            "family": "Arial",
            "size": 12,
            "weight": "bold",
            "style": "italic"
        }"#;
        let de: Font = serde_json::from_str(font).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Font = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
