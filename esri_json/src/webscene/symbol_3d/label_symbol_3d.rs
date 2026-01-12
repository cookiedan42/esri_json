//! Representation of a [LabelSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/labelSymbol3D/)
//!
//! LabelSymbol3D is used to render labels for features from a FeatureLayer in a 3D SceneView.

use crate::common::marker_type;
use crate::webscene::text_symbol_3d_layer::TextSymbol3DLayer;
use serde::{Deserialize, Serialize};

pub use crate::webscene::common::{Callout, VerticalOffset};

/// Representation Symbol Layers usable in a a [`LabelSymbol3D`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum LabelSymbol3DLayers {
    Text(TextSymbol3DLayer),
}
impl From<TextSymbol3DLayer> for LabelSymbol3DLayers {
    fn from(val: TextSymbol3DLayer) -> Self {
        LabelSymbol3DLayers::Text(val)
    }
}

marker_type!(T, "LabelSymbol3D");

/// Representation of a [LabelSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/labelSymbol3D/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LabelSymbol3D {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    callout: Option<Callout>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolLayers")]
    symbol_layers: Option<Vec<LabelSymbol3DLayers>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "verticalOffset")]
    vertical_offset: Option<VerticalOffset>,
}
impl<Layer> From<Layer> for LabelSymbol3D
where
    Layer: Into<LabelSymbol3DLayers>,
{
    fn from(val: Layer) -> Self {
        Self {
            _type: T,
            callout: None,
            symbol_layers: Some(vec![val.into()]),
            vertical_offset: None,
        }
    }
}

/// Builder pattern
impl LabelSymbol3D {
    pub fn callout(mut self, callout: Callout) -> Self {
        self.callout = Some(callout);
        self
    }
    pub fn symbol_layers(mut self, symbol_layers: Vec<LabelSymbol3DLayers>) -> Self {
        self.symbol_layers = Some(symbol_layers);
        self
    }
    pub fn vertical_offset(mut self, vertical_offset: VerticalOffset) -> Self {
        self.vertical_offset = Some(vertical_offset);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let label_symbol_3d = r#"{
            "type": "LabelSymbol3D",
            "symbolLayers": [
                {
                "size": 12,
                "type": "Text",
                "material": {
                    "color": [
                    255,
                    255,
                    0
                    ]
                }
                }
            ]
        }"#;
        let de: LabelSymbol3D = serde_json::from_str(label_symbol_3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: LabelSymbol3D = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn label_symbol_3d_with_vertical_offset_and_leader_line_callout_example() {
        let label_symbol_3d = r#"{
            "type": "LabelSymbol3D",
            "symbolLayers": [
                {
                "size": 12,
                "type": "Text",
                "material": {
                    "color": [
                    255,
                    255,
                    0
                    ]
                }
                }
            ],
            "callout": {
                "type": "line",
                "color": [
                255,
                255,
                255
                ],
                "size": 0.75,
                "border": {
                "color": [
                    50,
                    50,
                    50
                ]
                }
            },
            "verticalOffset": {
                "screenLength": 100,
                "maxWorldLength": 600,
                "minWorldLength": 10
            }
        }"#;
        let de: LabelSymbol3D = serde_json::from_str(label_symbol_3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: LabelSymbol3D = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
