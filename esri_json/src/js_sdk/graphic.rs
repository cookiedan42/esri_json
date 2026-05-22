//! Working with [Graphics Layer](https://developers.arcgis.com/javascript/latest/api-reference/esri-layers-GraphicsLayer.html)

use std::fmt::Debug;

use crate::geometry::{Coord, MultiPoint, Point, Polygon, Polyline};
use crate::js_sdk::popup_template::PopupTemplate;
use crate::webmap::{esri_sfs::EsriSFS, esri_sls::EsriSLS, esri_sms::EsriSMS};
use crate::webscene::symbol_3d::{
    label_symbol_3d::LabelSymbol3D, line_symbol_3d::LineSymbol3D, mesh_symbol_3d::MeshSymbol3D,
    point_symbol_3d::PointSymbol3D, polygon_symbol_3d::PolygonSymbol3D,
};

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

/// Used for pairing a geometry type with a symbol type.
pub trait GeometrySymbolPair
where
    Self::Symbol: Serialize + for<'de> Deserialize<'de> + Debug + Clone,
{
    type Symbol;
}
impl<C: Coord> GeometrySymbolPair for Point<C> {
    /// [`PointSymbolSceneView`](crate::js_sdk::graphics_layer::PointSymbolSceneView)
    type Symbol = PointSymbolSceneView;
}
impl<C: Coord> GeometrySymbolPair for MultiPoint<C> {
    /// [`PointSymbolSceneView`](crate::js_sdk::graphics_layer::PointSymbolSceneView)
    type Symbol = PointSymbolSceneView;
}
impl<C: Coord> GeometrySymbolPair for Polyline<C> {
    /// [`PolylineSymbolSceneView`](crate::js_sdk::graphics_layer::PolylineSymbolSceneView)
    type Symbol = PolylineSymbolSceneView;
}
impl<C: Coord> GeometrySymbolPair for Polygon<C> {
    /// [`PolygonSymbolSceneView`](crate::js_sdk::graphics_layer::PolygonSymbolSceneView)
    type Symbol = PolygonSymbolSceneView;
}

/// Representation of a [Graphic](https://developers.arcgis.com/javascript/latest/api-reference/esri-Graphic.html) in Esri JS SDK
///
/// [Symbol compatibility](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-Symbol.html)
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct Graphic<G>
where
    G: GeometrySymbolPair,
{
    pub geometry: G,
    attributes: Map<String, Value>,
    // aggregateGeometries: Option<String>, // ignoring this field
    symbol: Option<G::Symbol>,
    #[serde(rename = "popupTemplate")]
    popup_template: Option<PopupTemplate>,
}

/// Builder pattern
impl<G> Graphic<G>
where
    G: GeometrySymbolPair,
{
    /// Create a new [`Graphic`] with the given geometry
    pub fn new(geometry: G) -> Self {
        Self {
            geometry,
            attributes: Map::new(),
            symbol: None,
            popup_template: None,
        }
    }

    /// Name-value pairs of fields and field values associated with the graphic.
    pub fn attributes(mut self, attributes: Map<String, Value>) -> Self {
        self.attributes = attributes;
        self
    }
    /// The `Symbol` for the graphic.
    ///
    /// Must be a Symbol of the correct type for the geometry which can be turned into one of:
    /// - [PointSymbolSceneView],
    /// - [PolylineSymbolSceneView],
    /// - [PolygonSymbolSceneView]
    ///
    /// Refer to the [JS SDK](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-Symbol.html) for compatability list
    pub fn symbol<S>(mut self, symbol: S) -> Self
    where
        S: Into<G::Symbol>,
    {
        self.symbol = Some(symbol.into());
        self
    }
    /// The template for displaying content in a Popup when the graphic is selected.
    pub fn popup_template(mut self, popup_template: PopupTemplate) -> Self {
        self.popup_template = Some(popup_template);
        self
    }
}

/// Symbols which can be used with a [`Point`] geometry in a [`Graphic`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PointSymbolSceneView {
    // from 2d
    /// [Simple Marker Symbol](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-SimpleMarkerSymbol.html)
    SimpleMarkerSymbol(EsriSMS),
    // PictureMarkerSymbol,
    // TextSymbol,
    // WebStyleSymbol,
    // CIMSymbol

    // from 3d
    /// [PointSymbol3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-PointSymbol3D.html)
    PointSymbol3D(PointSymbol3D),
    /// [Label Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-LabelSymbol3D.html)
    LabelSymbol3D(LabelSymbol3D),
    // WebStyleSymbol,
    // CIMSymbol
}
impl From<EsriSMS> for PointSymbolSceneView {
    fn from(val: EsriSMS) -> Self {
        PointSymbolSceneView::SimpleMarkerSymbol(val)
    }
}
impl From<LabelSymbol3D> for PointSymbolSceneView {
    fn from(val: LabelSymbol3D) -> Self {
        PointSymbolSceneView::LabelSymbol3D(val)
    }
}
impl From<PointSymbol3D> for PointSymbolSceneView {
    fn from(val: PointSymbol3D) -> Self {
        PointSymbolSceneView::PointSymbol3D(val)
    }
}

/// Symbols which can be used with a [`Polyline`] geometry in a [`Graphic`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PolylineSymbolSceneView {
    // from 2d
    /// [Simple Line Symbol](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-SimpleLineSymbol.html)
    SimpleLineSymbol(EsriSLS),
    // TextSymbol
    // CIM Symbol

    // from 3d
    /// [Line Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-LineSymbol3D.html)
    LineSymbol3D(LineSymbol3D),
    /// [Label Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-LabelSymbol3D.html)
    LabelSymbol3D(LabelSymbol3D),
}
impl From<EsriSLS> for PolylineSymbolSceneView {
    fn from(val: EsriSLS) -> Self {
        PolylineSymbolSceneView::SimpleLineSymbol(val)
    }
}
impl From<LineSymbol3D> for PolylineSymbolSceneView {
    fn from(val: LineSymbol3D) -> Self {
        PolylineSymbolSceneView::LineSymbol3D(val)
    }
}
impl From<LabelSymbol3D> for PolylineSymbolSceneView {
    fn from(val: LabelSymbol3D) -> Self {
        PolylineSymbolSceneView::LabelSymbol3D(val)
    }
}

/// Symbols which can be used with a [`Polygon`] geometry in a [`Graphic`]
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
pub enum PolygonSymbolSceneView {
    // from 2d
    /// [Simple Fill Symbol](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-SimpleFillSymbol.html)
    SimpleFillSymbol(EsriSFS),
    // PictureFillSymbol,
    // SimpleMarkerSymbol,
    // TextSymbol,
    // CIMSymbol

    // from 3d
    /// [Polygon Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-PolygonSymbol3D.html)
    PolygonSymbol3D(PolygonSymbol3D),
    /// [Label Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-LabelSymbol3D.html)
    LabelSymbol3D(LabelSymbol3D),
}
impl From<EsriSFS> for PolygonSymbolSceneView {
    fn from(val: EsriSFS) -> Self {
        PolygonSymbolSceneView::SimpleFillSymbol(val)
    }
}
impl From<PolygonSymbol3D> for PolygonSymbolSceneView {
    fn from(val: PolygonSymbol3D) -> Self {
        PolygonSymbolSceneView::PolygonSymbol3D(val)
    }
}
impl From<LabelSymbol3D> for PolygonSymbolSceneView {
    fn from(val: LabelSymbol3D) -> Self {
        PolygonSymbolSceneView::LabelSymbol3D(val)
    }
}

/// Symbols which can be used with a `Mesh` geometry in a [`Graphic`]  
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(untagged)]
#[allow(dead_code)] // Not useful without Mesh Geometry implementation
enum MeshSymbolSceneView {
    // from 3d
    /// [Mesh Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-MeshSymbol3D.html)
    MeshSymbol3D(MeshSymbol3D),
    /// [Label Symbol 3D](https://developers.arcgis.com/javascript/latest/api-reference/esri-symbols-LabelSymbol3D.html)
    LabelSymbol3D(LabelSymbol3D),
}
impl From<LabelSymbol3D> for MeshSymbolSceneView {
    fn from(val: LabelSymbol3D) -> Self {
        MeshSymbolSceneView::LabelSymbol3D(val)
    }
}
impl From<MeshSymbol3D> for MeshSymbolSceneView {
    fn from(val: MeshSymbol3D) -> Self {
        MeshSymbolSceneView::MeshSymbol3D(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geo_types_n::CoordNumber;
    use crate::geometry::*;
    use esri_json_test_fixtures::graphic::*;
    use rstest::rstest;
    use serde::{Serialize, de::DeserializeOwned};

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn test_point<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32> + Serialize + DeserializeOwned,
    {
        let de: Graphic<Point<CoordXy<T>>> = serde_json::from_str(&point()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Graphic<Point<CoordXy<T>>> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn test_polyline<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32> + Serialize + DeserializeOwned,
    {
        let de: Graphic<Polyline<CoordXy<T>>> = serde_json::from_str(&polyline()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Graphic<Polyline<CoordXy<T>>> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }

    #[rstest]
    #[case::f32(std::marker::PhantomData::<f32>)]
    #[case::f64(std::marker::PhantomData::<f64>)]
    fn test_polygon<T>(#[case] _phantom: std::marker::PhantomData<T>)
    where
        T: CoordNumber + From<f32> + Serialize + DeserializeOwned,
    {
        let de: Graphic<Polygon<CoordXy<T>>> = serde_json::from_str(&polygon()).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: Graphic<Polygon<CoordXy<T>>> = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
