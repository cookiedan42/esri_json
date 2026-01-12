//! Geometry Primitives representing ESRI geometries

use serde::{Deserialize, Serialize};

mod coord;
mod linestring;
mod multipoint;
mod point;
mod polygon;
mod polyline;
mod spatial_reference;

pub use coord::{Coord, CoordXy, CoordXym, CoordXyz, CoordXyzm};
pub use linestring::LineString;
pub use multipoint::MultiPoint;
pub use point::Point;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use spatial_reference::SpatialReference;

/// Union type of all Geometry types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
pub enum Geometry<N: Coord> {
    Point(Point<N>),
    MultiPoint(MultiPoint<N>),
    Polyline(Polyline<N>),
    Polygon(Polygon<N>),
}
