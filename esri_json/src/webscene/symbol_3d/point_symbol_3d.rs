//! Representation of a [PointSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/pointSymbol3D/)
//!
//! PointSymbol3D is used to render features with Point geometry in a 3D SceneView.

use crate::common::marker_type;
use crate::webscene::common::{Callout, StyleOrigin, VerticalOffset};
use crate::webscene::{
    icon_symbol_3d_layer::IconSymbol3DLayer, object_symbol_3d_layer::ObjectSymbol3DLayer,
    text_symbol_3d_layer::TextSymbol3DLayer,
};
use serde::{Deserialize, Serialize};

/// Representation Symbol Layers usable in a a [`PointSymbol3D`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PointSymbol3DLayers {
    Icon(IconSymbol3DLayer),
    Object(ObjectSymbol3DLayer),
    Text(TextSymbol3DLayer),
}
impl From<IconSymbol3DLayer> for PointSymbol3DLayers {
    fn from(val: IconSymbol3DLayer) -> Self {
        PointSymbol3DLayers::Icon(val)
    }
}
impl From<ObjectSymbol3DLayer> for PointSymbol3DLayers {
    fn from(val: ObjectSymbol3DLayer) -> Self {
        PointSymbol3DLayers::Object(val)
    }
}
impl From<TextSymbol3DLayer> for PointSymbol3DLayers {
    fn from(val: TextSymbol3DLayer) -> Self {
        PointSymbol3DLayers::Text(val)
    }
}

marker_type!(T, "PointSymbol3D");

/// Representation of a [PointSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/pointSymbol3D/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct PointSymbol3D {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none")]
    callout: Option<Callout>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleOrigin")]
    style_origin: Option<StyleOrigin>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolLayers")]
    symbol_layers: Option<Vec<PointSymbol3DLayers>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "verticalOffset")]
    vertical_offset: Option<VerticalOffset>,
}
impl<Layer> From<Layer> for PointSymbol3D
where
    Layer: Into<PointSymbol3DLayers>,
{
    fn from(val: Layer) -> Self {
        Self {
            _type: T,
            callout: None,
            style_origin: None,
            symbol_layers: Some(vec![val.into()]),
            vertical_offset: None,
        }
    }
}

/// Builder pattern
impl PointSymbol3D {
    /// Callout configuration for a symbol.
    pub fn callout(mut self, callout: Callout) -> Self {
        self.callout = Some(callout);
        self
    }
    /// The origin of the style from which the symbol was originally referenced.
    pub fn style_origin(mut self, style_origin: StyleOrigin) -> Self {
        self.style_origin = Some(style_origin);
        self
    }
    /// A Collection of Symbol3DLayer objects used to visualize the graphic or feature.
    pub fn symbol_layers(mut self, symbol_layers: Vec<PointSymbol3DLayers>) -> Self {
        self.symbol_layers = Some(symbol_layers);
        self
    }
    /// Shifts the symbol along the vertical world axis by a given length.
    pub fn vertical_offset(mut self, vertical_offset: VerticalOffset) -> Self {
        self.vertical_offset = Some(vertical_offset);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_symbol_3d_example() {
        let point_symbol_3d = r#"{
            "type": "PointSymbol3D",
            "symbolLayers": [
                {
                "width": 30000,
                "height": 160000,
                "resource": {
                    "primitive": "cylinder"
                },
                "type": "Object",
                "material": {
                    "color": [
                    0,
                    255,
                    0
                    ]
                }
                }
            ]
        }"#;
        let de: PointSymbol3D = serde_json::from_str(point_symbol_3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PointSymbol3D = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[test]
    fn icon_point_symbol_with_vertical_offset_and_leader_line_callout() {
        let point_symbol_3d = r#"{
            "type": "PointSymbol3D",
            "symbolLayers": [
                {
                "size": 16,
                "resource": {
                    "primitive": "circle"
                },
                "type": "Icon",
                "material": {
                    "color": [
                    0,
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
        let de: PointSymbol3D = serde_json::from_str(point_symbol_3d).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: PointSymbol3D = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}

/*
Point Symbol 3D Example



Icon Point Symbol with verticalOffset and leader line callout Example



*/
