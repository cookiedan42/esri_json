//! Geometry Primitives representing ESRI geometries

use serde::{Deserialize, Serialize};

mod coord;
mod linestring;
mod multipoint;
mod point;
mod polygon;
mod polyline;
mod spatial_reference;

pub use coord::{Coord, CoordXy, CoordXym, CoordXyz, CoordXyzm, FromCoordTrait};
use geo_traits::CoordTrait;
pub use linestring::LineString;
pub use multipoint::MultiPoint;
pub use point::Point;
pub use polygon::Polygon;
pub use polyline::Polyline;
pub use spatial_reference::SpatialReference;

/// Union type of all Geometry types
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(bound(
    serialize = "C: Serialize, <C as CoordTrait>::T: Serialize",
    deserialize = "C: Deserialize<'de>, <C as CoordTrait>::T: Deserialize<'de>"
))]
#[serde(untagged)]
pub enum Geometry<C: Coord> {
    Point(Point<C>),
    MultiPoint(MultiPoint<C>),
    Polyline(Polyline<C>),
    Polygon(Polygon<C>),
}
