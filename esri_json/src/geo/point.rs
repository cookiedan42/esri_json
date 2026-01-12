use crate::geometry::{Coord, Point};

impl<N: Coord> From<&Point<N>> for geo_types::Point<f64> {
    fn from(val: &Point<N>) -> Self {
        geo_types::Point::<f64>::new(val.x(), val.y())
    }
}
impl<N: Coord> From<Point<N>> for geo_types::Point<f64> {
    fn from(val: Point<N>) -> Self {
        (&val).into()
    }
}

impl<N> From<&geo_types::Point<f64>> for Point<N>
where
    N: Coord + for<'a> std::convert::From<&'a geo::Coord>,
{
    fn from(point: &geo_types::Point<f64>) -> Self {
        let c: N = (&point.0).into();
        Point::new(c, None)
    }
}
impl<N: Coord> From<geo_types::Point<f64>> for Point<N>
where
    N: Coord + for<'a> std::convert::From<&'a geo::Coord>,
{
    fn from(point: geo_types::Point<f64>) -> Self {
        (&point).into()
    }
}
