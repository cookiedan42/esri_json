//! Representation of a [MeshSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/meshSymbol3D/)
//!
//! MeshSymbol3D is used to render 3D mesh features in a SceneLayer in a 3D SceneView.

use crate::common::marker_type;
use crate::webscene::common::StyleOrigin;
use crate::webscene::fill_symbol_3d_layer::FillSymbol3DLayer;
use serde::{Deserialize, Serialize};

/// Representation Symbol Layers usable in a a [`MeshSymbol3D`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum MeshSymbol3DLayers {
    Fill(FillSymbol3DLayer),
}
impl From<FillSymbol3DLayer> for MeshSymbol3DLayers {
    fn from(val: FillSymbol3DLayer) -> Self {
        MeshSymbol3DLayers::Fill(val)
    }
}

marker_type!(T, "MeshSymbol3D");

/// Representation of a [MeshSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/meshSymbol3D/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct MeshSymbol3D {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none", rename = "styleOrigin")]
    style_origin: Option<StyleOrigin>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolLayers")]
    symbol_layers: Option<Vec<MeshSymbol3DLayers>>,
}
impl<Layer> From<Layer> for MeshSymbol3D
where
    Layer: Into<MeshSymbol3DLayers>,
{
    fn from(val: Layer) -> Self {
        Self {
            _type: T,
            style_origin: None,
            symbol_layers: Some(vec![val.into()]),
        }
    }
}

/// Builder pattern
impl MeshSymbol3D {
    pub fn style_origin(mut self, style_origin: StyleOrigin) -> Self {
        self.style_origin = Some(style_origin);
        self
    }
    pub fn symbol_layers(mut self, symbol_layers: Vec<MeshSymbol3DLayers>) -> Self {
        self.symbol_layers = Some(symbol_layers);
        self
    }
}
