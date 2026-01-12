//! Representation of a [PolygonSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/polygonSymbol3D/)
//!
//! PolygonSymbol3D is used to render features with Polygon geometry in a 3D SceneView. Polygon features may also be rendered as points with icons or objects at the centroid of each polygon.

use crate::common::marker_type;
use crate::webscene::{
    extrude_symbol_3d_layer::ExtrudeSymbol3DLayer, fill_symbol_3d_layer::FillSymbol3DLayer,
    icon_symbol_3d_layer::IconSymbol3DLayer, line_symbol_3d_layer::LineSymbol3DLayer,
    object_symbol_3d_layer::ObjectSymbol3DLayer, text_symbol_3d_layer::TextSymbol3DLayer,
    water_symbol_3d_layer::WaterSymbol3DLayer,
};
use serde::{Deserialize, Serialize};

pub use crate::webscene::common::StyleOrigin;

/// Representation Symbol Layers usable in a a [`PolygonSymbol3D`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PolygonSymbol3DLayers {
    Extrude(ExtrudeSymbol3DLayer),
    Fill(FillSymbol3DLayer),
    Icon(IconSymbol3DLayer),
    #[deprecated = "use [`fillSymbol3DLayer`] with outline instead"]
    Line(LineSymbol3DLayer),
    Object(ObjectSymbol3DLayer),
    #[deprecated = "use [`labelSymbol3D`] for TextSymbol3dLayers"]
    Text(TextSymbol3DLayer),
    Water(WaterSymbol3DLayer),
}

impl From<ExtrudeSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: ExtrudeSymbol3DLayer) -> Self {
        PolygonSymbol3DLayers::Extrude(val)
    }
}
impl From<FillSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: FillSymbol3DLayer) -> Self {
        PolygonSymbol3DLayers::Fill(val)
    }
}
impl From<IconSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: IconSymbol3DLayer) -> Self {
        PolygonSymbol3DLayers::Icon(val)
    }
}
impl From<LineSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: LineSymbol3DLayer) -> Self {
        #[allow(deprecated)]
        PolygonSymbol3DLayers::Line(val)
    }
}
impl From<ObjectSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: ObjectSymbol3DLayer) -> Self {
        PolygonSymbol3DLayers::Object(val)
    }
}
impl From<TextSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: TextSymbol3DLayer) -> Self {
        #[allow(deprecated)]
        PolygonSymbol3DLayers::Text(val)
    }
}
impl From<WaterSymbol3DLayer> for PolygonSymbol3DLayers {
    fn from(val: WaterSymbol3DLayer) -> Self {
        PolygonSymbol3DLayers::Water(val)
    }
}

marker_type!(T, "PolygonSymbol3D");

/// Representation of a [PolygonSymbol3D](https://developers.arcgis.com/web-scene-specification/objects/polygonSymbol3D/)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PolygonSymbol3D {
    #[serde(rename = "type")]
    _type: T,

    #[serde(skip_serializing_if = "Option::is_none", rename = "symbolLayers")]
    symbol_layers: Option<Vec<PolygonSymbol3DLayers>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "styleOrigin")]
    style_origin: Option<StyleOrigin>,
}

impl<Layer> From<Layer> for PolygonSymbol3D
where
    Layer: Into<PolygonSymbol3DLayers>,
{
    fn from(val: Layer) -> Self {
        Self {
            _type: T,
            symbol_layers: Some(vec![val.into()]),
            style_origin: None,
        }
    }
}

///Builder pattern
impl PolygonSymbol3D {
    /// A Collection of Symbol3DLayer objects used to visualize the graphic or feature.
    pub fn symbol_layers(mut self, symbol_layers: Vec<PolygonSymbol3DLayers>) -> Self {
        self.symbol_layers = Some(symbol_layers);
        self
    }
    /// The origin of the style from which the symbol was originally referenced.
    pub fn style_origin(mut self, style_origin: StyleOrigin) -> Self {
        self.style_origin = Some(style_origin);
        self
    }
}
