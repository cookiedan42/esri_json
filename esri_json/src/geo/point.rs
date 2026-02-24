use crate::geo_types_n;
use crate::geometry::{Coord, Point};

impl<C: Coord> From<Point<C>> for geo_types_n::Point<C> {
    fn from(value: Point<C>) -> Self {
        Self(*value.coord())
    }
}

impl<C: Coord> From<geo_types_n::Point<C>> for Point<C> {
    fn from(value: geo_types_n::Point<C>) -> Self {
        Self::new(value.0, None)
    }
}
