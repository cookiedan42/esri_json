//! Representation of a [MaterialColorMixMode](https://developers.arcgis.com/web-scene-specification/objects/materialColorMixMode/)
//!
//! The material used to shade the geometry, including colorMixMode options.

use crate::common::Color;
use serde::{Deserialize, Serialize};

/// Representation of a [MaterialColorMixMode](https://developers.arcgis.com/web-scene-specification/objects/materialColorMixMode/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct MaterialColorMixMode {
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color_mix_mode: Option<ColorMixMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transparency: Option<f64>,
}
/// Builder pattern
impl MaterialColorMixMode {
    /// Color is represented as a three or four-element array.
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }
    /// Controls how symbolLayer or visualVariable color is applied onto the underlying geometry color/texture.
    /// This property only applies to [`FillSymbol3DLayer`](crate::webscene::layers::fill_symbol_3d_layer::FillSymbol3DLayer)
    /// within [`MeshSymbol3D`](crate::webscene::mesh_symbol_3d::MeshSymbol3DLayers)).
    pub fn color_mix_mode(mut self, color_mix_mode: ColorMixMode) -> Self {
        self.color_mix_mode = Some(color_mix_mode);
        self
    }
    /// A value between 100 (full transparency) and 0 (full opacity). Ignored if no color is specified.
    pub fn transparency(mut self, transparency: f64) -> Self {
        self.transparency = Some(transparency);
        self
    }
}

/// Controls how symbolLayer or visualVariable color is applied onto the underlying geometry color/texture.
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ColorMixMode {
    Tint,
    Replace,
    Multiply,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let material_color_mix_mode = r#"{
            "color": [
                255,
                0,
                0
            ],
            "transparency": 20,
            "colorMixMode": "tint"
        }"#;
        let de: MaterialColorMixMode = serde_json::from_str(material_color_mix_mode).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: MaterialColorMixMode = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
