//! Enables convertion between Esri geometry types and geometries from [`geo_types`]   
//!
//! # Coord
//! [`geo_types::Coord`] can be converted into any of [`Coord`](crate::geometry::Coord)  ( [`CoordXy`](crate::geometry::CoordXy), [`CoordXyz`](crate::geometry::CoordXyz), [`CoordXym`](crate::geometry::CoordXym), [`CoordXyzm`](crate::geometry::CoordXyzm) )  
//!  - Missing Z coordinates are filled with `0.0_f64` and missing M coordinates are filled with `None`  
//!
//! [`Coord`](crate::geometry::Coord) can be converted into [`geo_types::Coord`]  
//!  - M and Z coordinates are ignored  
//!
//! These changes apply to the coordinates when converting all other geometries
//!
//! # Points
//! Point types are easily convertible  
//! [`geo_types::Point`] and [`Point`](crate::geometry::Point) can be converted to each other  
//!
//! [`geo_types::MultiPoint`] and [`MultiPoint`](crate::geometry::MultiPoint) can be converted to each other  
//!
//! # Lines
//! [`geo_types::Line`], [`geo_types::LineString`] and [`geo_types::MultiLineString`] can be converted to [`LineString`](crate::geometry::Polyline)  
//!
//! [`LineString`](crate::geometry::Polyline) can only be converted to [`geo_types::MultiLineString`]  
//!
//! # Polygons
//! [`geo_types::Polygon`], [`geo_types::MultiPolygon`], [`geo_types::Rect`] and [`geo_types::Triangle`] can be converted to [`Polygon`](crate::geometry::Polygon)  
//!
//! [`Polygon`](crate::geometry::Polygon) can only be converted to [`geo_types::MultiPolygon`]  
//!
//! # Geometry
//! [`geo_types::Geometry`] can mostly be converted to [`Geometry`](crate::geometry::Geometry), but [`geo_types::GeometryCollection`] cannot be converted
//!
//! [`Geometry`](crate::geometry::Geometry) can only be converted to [`geo_types::Geometry`]
//!
//! # Spatial Reference
//! The [`SpatialReference`](crate::geometry::SpatialReference) type is ignored when converting and will need to be set separately  
//!

mod coord;
mod geometry;
mod linestring;
mod multipoint;
mod point;
mod polygon;
mod polyline;
