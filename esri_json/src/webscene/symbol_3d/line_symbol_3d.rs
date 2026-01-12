//! Representation of a [LineSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/lineSymbol3D/)
//!
//! LineSymbol3D is used to render features with Polyline geometry in a 3D SceneView.

use crate::common::marker_type;
use crate::webscene::common::StyleOrigin;
use crate::webscene::{
    line_symbol_3d_layer::LineSymbol3DLayer, path_symbol_3d_layer::PathSymbol3DLayer,
};
use serde::{Deserialize, Serialize};

/// Representation Symbol Layers usable in a a [`LineSymbol3D`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum LineSymbol3DLayers {
    Line(LineSymbol3DLayer),
    Path(PathSymbol3DLayer),
}
impl From<LineSymbol3DLayer> for LineSymbol3DLayers {
    fn from(val: LineSymbol3DLayer) -> Self {
        LineSymbol3DLayers::Line(val)
    }
}
impl From<PathSymbol3DLayer> for LineSymbol3DLayers {
    fn from(val: PathSymbol3DLayer) -> Self {
        LineSymbol3DLayers::Path(val)
    }
}

marker_type!(T, "LineSymbol3D");

/// Representation of a [LineSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/lineSymbol3D/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct LineSymbol3D {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolLayers")]
    symbol_layers: Option<Vec<LineSymbol3DLayers>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleOrigin")]
    style_origin: Option<StyleOrigin>,
}
impl<Layer> From<Layer> for LineSymbol3D
where
    Layer: Into<LineSymbol3DLayers>,
{
    fn from(val: Layer) -> Self {
        Self {
            _type: T,
            symbol_layers: Some(vec![val.into()]),
            style_origin: None,
        }
    }
}
