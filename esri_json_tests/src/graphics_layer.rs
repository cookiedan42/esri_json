/// Test Simple Marker,Line,Fill Symbols
#[cfg(test)]
mod test_simple_symbol {

    use crate::fixtures;
    use esri_json::geometry::CoordXy;
    use esri_json::js_sdk::graphics_layer;
    use serde_json::Map;

    #[test]
    fn simple_marker_symbol() {
        use esri_json::geometry::Point;
        use esri_json::webmap::esri_sms::*;

        let geometry: Point<CoordXy> = fixtures::default_point::<CoordXy>();

        let sms = EsriSMS::default()
            .angle(1.0)
            .color(Color::Rgb(1, 1, 1))
            .outline(Outline::default())
            .size(1.0)
            .style(Style::esriSMSCircle)
            .xoffset(1.0)
            .yoffset(1.0);

        let _g = graphics_layer::Graphic::<Point<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(sms);
    }

    #[test]
    fn simple_line_symbol() {
        use esri_json::geometry::Polyline;
        use esri_json::webmap::esri_sls::*;

        let geometry = fixtures::default_polyline::<CoordXy>();

        let marker = Marker::default().placement(Placement::Begin);

        let sls = EsriSLS::default()
            .color(Color::Rgb(1, 1, 1))
            .marker(marker)
            .style(Style::esriSLSDot)
            .width(1.0);

        let _g = graphics_layer::Graphic::<Polyline<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(sls);
    }

    #[test]
    fn simple_polygon_symbol() {
        use esri_json::geometry::Polygon;
        use esri_json::webmap::esri_sfs::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let sfs = EsriSFS::default()
            .color(Color::Rgb(1, 1, 1))
            .outline(Outline::default())
            .style(Style::esriSFSSolid);

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(sfs);
    }
}

/// Test Polygon geometry against PolygonSymboll3D
#[cfg(test)]
mod test_polygon_symbol_3d {

    use crate::fixtures;
    use esri_json::geometry::{CoordXy, Polygon};
    use esri_json::js_sdk::graphics_layer;
    use esri_json::webscene::polygon_symbol_3d::{PolygonSymbol3D, StyleOrigin};
    use serde_json::Map;

    #[test]
    fn test_polygon_extrude() {
        use esri_json::webscene::extrude_symbol_3d_layer::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let edges = Edges::new_sketch()
            .color(Color::Rgb(1, 1, 1))
            .transparency(50.0)
            .size(1.0)
            .extension_length(1.0);

        let layer = ExtrudeSymbol3DLayer::default()
            .cast_shadows(true)
            .edges(edges)
            .material(Material::default())
            .size(1.0);

        let layers: PolygonSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polygon_fill() {
        use esri_json::webscene::fill_symbol_3d_layer::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .color_mix_mode(ColorMixMode::Replace);

        let outline = Outline::default()
            .color(Color::Rgb(1, 1, 1))
            .pattern(LinePattern::Dash)
            .pattern_cap(LineCap::Round)
            .size(1.0)
            .unwrap()
            .transparency(1.0)
            .unwrap();

        let layer = FillSymbol3DLayer::default()
            .material(material)
            .outline(outline)
            .pattern(PolygonPattern::BackwardDiagonal);

        let layers: PolygonSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polygon_icon() {
        use esri_json::webscene::icon_symbol_3d_layer::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let outline = Outline::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0);

        let resource = Resource::default()
            .data_uri("data_uri".to_string())
            .href("href".to_string())
            .primitive(IconPrimitive::Circle);

        let layer = IconSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0])
            .angle(0.0)
            .material(material)
            .outline(outline)
            .resource(resource)
            .size(1.0);

        let layers: PolygonSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polygon_object() {
        use esri_json::webscene::object_symbol_3d_layer::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let resource = Resource::default()
            .href("href".to_string())
            .primitive(ObjectPrimitive::Cone);

        let layer = ObjectSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0, 0.0])
            .cast_shadows(true)
            .depth(1.0)
            .unwrap()
            .heading(1.0)
            .height(1.0)
            .unwrap()
            .material(material)
            .resource(resource)
            .roll(1.0)
            .tilt(1.0)
            .width(1.0)
            .unwrap();

        let layers: PolygonSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polygon_water() {
        use esri_json::webscene::water_symbol_3d_layer::*;

        let geometry = fixtures::default_polygon::<CoordXy>();

        let layer = WaterSymbol3DLayer::default()
            .color(Color::Rgb(1, 1, 1))
            .waterbody_size(WaterbodySize::Large)
            .wave_direction(1.0)
            .wave_strength(WaveStrength::Calm);

        let layers: PolygonSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polygon_multi() {
        let geometry = fixtures::default_polygon::<CoordXy>();

        let water_layer = {
            use esri_json::webscene::water_symbol_3d_layer::*;

            WaterSymbol3DLayer::default()
                .color(Color::Rgb(1, 1, 1))
                .waterbody_size(WaterbodySize::Large)
                .wave_direction(1.0)
                .wave_strength(WaveStrength::Calm)
        };

        let extrude_layer = {
            use esri_json::webscene::extrude_symbol_3d_layer::*;

            let edges = Edges::new_sketch()
                .color(Color::Rgb(1, 1, 1))
                .transparency(50.0)
                .size(1.0)
                .extension_length(1.0);

            ExtrudeSymbol3DLayer::default()
                .cast_shadows(true)
                .edges(edges)
                .material(Material::default())
                .size(1.0)
        };

        let style_origin = StyleOrigin::default()
            .name("style_origin".to_string())
            .style_name("style_name".to_string())
            .style_url("style_url".to_string());

        let layers = PolygonSymbol3D::default()
            .symbol_layers(vec![extrude_layer.into(), water_layer.into()])
            .style_origin(style_origin);

        let _g = graphics_layer::Graphic::<Polygon<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }
}

/// Test Polyline geometry against LineSymbol3D
#[cfg(test)]
mod test_polyline_symbol_3d {
    use crate::fixtures;
    use esri_json::geometry::{CoordXy, Polyline};
    use esri_json::js_sdk::graphics_layer;
    use esri_json::webscene::line_symbol_3d::LineSymbol3D;
    use serde_json::Map;

    #[test]
    fn test_polyline_line() {
        use esri_json::webscene::line_symbol_3d_layer::*;

        let geometry = fixtures::default_polyline::<CoordXy>();

        let line_marker = LineMarker::default()
            .placement(Placement::Begin)
            .style(LineSymbolMarkerStyle::Arrow);

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let layer = LineSymbol3DLayer::default()
            .cap(LineCap::Round)
            .join(LineJoin::Round)
            .marker(line_marker)
            .material(material)
            .pattern(LinePattern::Dash)
            .size(PxOrPt::Points(10.0));

        let layers: LineSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polyline<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_polyline_path() {
        use esri_json::webscene::path_symbol_3d_layer::*;

        let geometry = fixtures::default_polyline::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let layer = PathSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .cap(Cap::Butt)
            .cast_shadows(true)
            .height(1.0)
            .join(LineJoin::Bevel)
            .material(material)
            .profile(Profile::Circle)
            .profile_rotation(ProfileRotation::All)
            .size(1.0)
            .width(1.0);

        let layers: LineSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Polyline<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }
}

/// Test Point geometry against PointSymbol3D
#[cfg(test)]
mod test_point_symbol_3d {
    use crate::fixtures;
    use esri_json::geometry::{CoordXy, Point};
    use esri_json::js_sdk::graphics_layer;
    use esri_json::webscene::point_symbol_3d::PointSymbol3D;
    use serde_json::Map;

    #[test]
    fn test_point_icon() {
        use esri_json::webscene::icon_symbol_3d_layer::*;

        let geometry = fixtures::default_point::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let outline = Outline::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0);

        let resource = Resource::default()
            .data_uri("data_uri".to_string())
            .href("href".to_string())
            .primitive(IconPrimitive::Circle);

        let layer = IconSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0])
            .angle(1.0)
            .material(material)
            .outline(outline)
            .resource(resource)
            .size(1.0);

        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Point<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_point_object() {
        use esri_json::webscene::object_symbol_3d_layer::*;

        let geometry = fixtures::default_point::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let resource = Resource::default()
            .href("href".to_string())
            .primitive(ObjectPrimitive::Cone);

        let layer = ObjectSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0, 0.0])
            .cast_shadows(true)
            .depth(1.0)
            .unwrap()
            .heading(1.0)
            .height(1.0)
            .unwrap()
            .material(material)
            .resource(resource)
            .roll(1.0)
            .tilt(1.0)
            .width(1.0)
            .unwrap();

        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Point<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_point_text() {
        use esri_json::webscene::text_symbol_3d_layer::*;

        let geometry = fixtures::default_point::<CoordXy>();

        let background = Background::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let font = font::Font::default()
            .decoration(font::Decoration::LineThrough)
            .family("Arial".to_string())
            .size(12.0)
            .style(font::Style::Italic)
            .weight(font::Weight::Bolder);

        let halo = Halo::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0)
            .unwrap();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let layer = TextSymbol3DLayer::default()
            .background(background)
            .font(font)
            .halo(halo)
            .horizontal_alignment(HorizontalAlignment::Center)
            .line_height(12.0)
            .material(material)
            .size(1.0)
            .unwrap()
            .text("abc".to_string())
            .vertical_alignment(VerticalAlignment::Middle);

        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<Point<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }
}

/// Test Multipoint geometry against PointSymbol3D
#[cfg(test)]
mod test_multipoint_symbol_3d {
    use crate::fixtures;
    use esri_json::geometry::{CoordXy, MultiPoint};
    use esri_json::js_sdk::graphics_layer;
    use esri_json::webscene::point_symbol_3d::PointSymbol3D;
    use serde_json::Map;

    #[test]
    fn test_point_icon() {
        use esri_json::webscene::icon_symbol_3d_layer::*;

        let geometry = fixtures::default_multipoint::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let outline = Outline::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0);

        let resource = Resource::default()
            .data_uri("data_uri".to_string())
            .href("href".to_string())
            .primitive(IconPrimitive::Circle);

        let layer = IconSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0])
            .angle(1.0)
            .material(material)
            .outline(outline)
            .resource(resource)
            .size(1.0);
        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<MultiPoint<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_point_object() {
        use esri_json::webscene::object_symbol_3d_layer::*;

        let geometry = fixtures::default_multipoint::<CoordXy>();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let resource = Resource::default()
            .href("href".to_string())
            .primitive(ObjectPrimitive::Cone);

        let layer = ObjectSymbol3DLayer::default()
            .anchor(Anchor::Bottom)
            .anchor_position([0.0, 0.0, 0.0])
            .cast_shadows(true)
            .depth(1.0)
            .unwrap()
            .heading(1.0)
            .height(1.0)
            .unwrap()
            .material(material)
            .resource(resource)
            .roll(1.0)
            .tilt(1.0)
            .width(1.0)
            .unwrap();

        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<MultiPoint<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }

    #[test]
    fn test_point_text() {
        use esri_json::webscene::text_symbol_3d_layer::*;

        let geometry = fixtures::default_multipoint::<CoordXy>();

        let background = Background::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let font = font::Font::default()
            .decoration(font::Decoration::LineThrough)
            .family("Arial".to_string())
            .size(12.0)
            .style(font::Style::Italic)
            .weight(font::Weight::Bolder);

        let halo = Halo::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0)
            .unwrap();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let layer = TextSymbol3DLayer::default()
            .background(background)
            .font(font)
            .halo(halo)
            .horizontal_alignment(HorizontalAlignment::Center)
            .line_height(12.0)
            .material(material)
            .size(1.0)
            .unwrap()
            .text("abc".to_string())
            .vertical_alignment(VerticalAlignment::Middle);

        let layers: PointSymbol3D = layer.into();

        let _g = graphics_layer::Graphic::<MultiPoint<CoordXy>>::new(geometry)
            .attributes(Map::new())
            .symbol(layers);
    }
}

/// Test LabelSymbol3D with all geometry types
#[cfg(test)]
mod test_label_layer {
    use crate::fixtures;
    use esri_json::geometry::CoordXy;
    use esri_json::js_sdk::graphics_layer;
    use esri_json::webscene::label_symbol_3d::*;
    use esri_json::webscene::text_symbol_3d_layer::*;
    use serde_json::Map;

    #[test]
    fn test_point() {
        let background = Background::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let font = font::Font::default()
            .decoration(font::Decoration::LineThrough)
            .family("Arial".to_string())
            .size(12.0)
            .style(font::Style::Italic)
            .weight(font::Weight::Bolder);

        let halo = Halo::default()
            .color(Color::Rgb(1, 1, 1))
            .size(1.0)
            .transparency(1.0)
            .unwrap();

        let material = Material::default()
            .color(Color::Rgb(1, 1, 1))
            .transparency(1.0)
            .unwrap();

        let layer = TextSymbol3DLayer::default()
            .background(background)
            .font(font)
            .halo(halo)
            .horizontal_alignment(HorizontalAlignment::Center)
            .line_height(12.0)
            .material(material)
            .size(1.0)
            .unwrap()
            .text("abc".to_string())
            .vertical_alignment(VerticalAlignment::Middle);

        let layers: LabelSymbol3D = layer.into();

        let _g1 = graphics_layer::Graphic::new(fixtures::default_point::<CoordXy>())
            .attributes(Map::new())
            .symbol(layers.clone());
        let _g2 = graphics_layer::Graphic::new(fixtures::default_multipoint::<CoordXy>())
            .attributes(Map::new())
            .symbol(layers.clone());
        let _g3 = graphics_layer::Graphic::new(fixtures::default_polyline::<CoordXy>())
            .attributes(Map::new())
            .symbol(layers.clone());
        let _g4 = graphics_layer::Graphic::new(fixtures::default_polygon::<CoordXy>())
            .attributes(Map::new())
            .symbol(layers.clone());
    }
}
