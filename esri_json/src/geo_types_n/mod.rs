//! Intermediate representation of geo_types but which allow for up to xyzm
macro_rules! impl_from {
    ($Source:ty, $Target:ty) => {
        impl<C: Coord> From<$Source> for $Target
        where
            Self: for<'a> From<&'a $Source>,
        {
            fn from(value: $Source) -> Self {
                (&value).into()
            }
        }
    };
}

pub(crate) mod coord;
pub(crate) mod geometry;
pub(crate) mod geometry_collection;
pub(crate) mod line;
pub(crate) mod line_string;
pub(crate) mod multi_line_string;
pub(crate) mod multi_point;
pub(crate) mod multi_polygon;
pub(crate) mod point;
pub(crate) mod polygon;
pub(crate) mod rect;
pub(crate) mod triangle;

// re-export all the geometry variants:
pub use coord::Coord;
pub use geometry::Geometry;
pub use geometry_collection::GeometryCollection;
pub use line::Line;
pub use line_string::LineString;
pub use multi_line_string::MultiLineString;
pub use multi_point::MultiPoint;
pub use multi_polygon::MultiPolygon;
pub use point::Point;
pub use polygon::Polygon;
pub use rect::Rect;
pub use triangle::Triangle;
