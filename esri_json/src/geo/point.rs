use crate::geometry::{Coord, Point};

impl<C: Coord> From<Point<C>> for geo_types::Point<f64> {
    fn from(val: Point<C>) -> Self {
        geo_types::Point::<f64>::new(val.x(), val.y())
    }
}

impl<C> From<geo_types::Point<f64>> for Point<C>
where
    C: Coord + From<geo::Coord>,
{
    fn from(point: geo_types::Point<f64>) -> Self {
        let c: C = point.0.into();
        Point::new(c, None)
    }
}
