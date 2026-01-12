# esri_json
A crate for reading and writing Esri JSON

This crate provides representations of Esri JSON objects with [`serde::Deserialize`](https://docs.rs/serde/1.0.228/serde/trait.Deserialize.html) and [`serde::Serialize`](https://docs.rs/serde/1.0.228/serde/trait.Serialize.html) trait implementations.

The primary purpose of this crate is to convert geospatial data processed with [`geo`](https://docs.rs/geo/latest/geo/) into `Graphic` objects for display in a [`WebScene`](https://developers.arcgis.com/javascript/latest/api-reference/esri-WebScene.html) component of an [`ArcGIS Maps SDK for JavaScript`](https://developers.arcgis.com/javascript/) application.

Inspired by the [`serde_esri`](https://github.com/JosiahParry/serde_esri) crate

## Feature Overview

1. Serialization and deserialization of Esri JSON objects
    - Basic Geometry types
    - Reading and writing a [Graphic](https://developers.arcgis.com/javascript/latest/api-reference/esri-Graphic.html) for display in a [WebScene](https://developers.arcgis.com/javascript/latest/api-reference/esri-WebScene.html)
1. Geometry conversion between Esri JSON and [`geo_types`](https://docs.rs/geo-types/0.7.18/geo_types/)


## Example
```rust 
use esri_json::geometry::{Coord,CoordXy,Point};
use esri_json::js_sdk::graphics_layer;
use esri_json::webmap::esri_sms::*;
use serde_json::Map;

let geometry: Point<CoordXy> = Point::new(CoordXy::new(0.0, 0.0), None);

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
```  

More examples are in the [tests crate](/esri_json_tests/src/graphics_layer/mod.rs) 

## Supported Esri Formats
### Coordinate Formats
1. XY
1. XYZ
1. XYM
1. XYZM

---
### Geometries
1. Point
1. MultiPoint
1. Polyline
1. Polygon
<!-- 1. (Unsupported) Envelope -->
<!-- 1. (Unsupported) Mesh -->

id field not supported 

---

### Spatial reference
fields are not validated 

---

### Graphic Types
Symbologies needed for visualizing supported Geometries can be serialized, deserialized and edited too

- `aggregateGeometries` not supported
- `popupTemplate` partially supported, only Title and Content are serialized as `String`


### Symbology Types

Symbologies for Point,Polygon,Polyline,Multipoint Geometries are included  
Symbologies are built up using the Builder pattern, starting from a `::default()` method and adding fields one by one  

## License
Licensed under Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)

