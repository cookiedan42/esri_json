//! Representation of a [StyleOrigin](https://developers.arcgis.com/web-scene-specification/objects/styleOrigin/)
//!
//! The origin of the style from which the symbol was originally referenced. A reference to the style origin can be either by styleName or by styleUrl (but not both). It may be used to understand where a symbol was originally sourced from, but does not affect actual appearance or rendering of the symbol.

use serde::{Deserialize, Serialize};

/// Representation of a [StyleOrigin](https://developers.arcgis.com/web-scene-specification/objects/styleOrigin/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct StyleOrigin {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    style_url: Option<String>,
}

/// Builder pattern
impl StyleOrigin {
    /// Identifies a symbol in the style by name.
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    /// A registered web style name, such as `EsriThematicShapesStyle`
    pub fn style_name(mut self, style_name: String) -> Self {
        self.style_name = Some(style_name);
        self
    }
    /// URL to a style definition.
    pub fn style_url(mut self, style_url: String) -> Self {
        self.style_url = Some(style_url);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn style_origin_example() {
        let style_origin = r#"{
            "styleName": "EsriThematicTreesStyle",
            "name": "Frangula"
        }"#;
        let de: StyleOrigin = serde_json::from_str(style_origin).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: StyleOrigin = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
