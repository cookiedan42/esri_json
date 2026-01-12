use geo::CoordsIter;

use crate::geometry::{Coord, MultiPoint};

impl<N: Coord> From<&MultiPoint<N>> for geo_types::MultiPoint<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: &MultiPoint<N>) -> Self {
        let a = val.points().iter().map(Into::<geo_types::Coord>::into);
        geo_types::MultiPoint::<f64>::from_iter(a)
    }
}
impl<N: Coord> From<MultiPoint<N>> for geo_types::MultiPoint<f64>
where
    for<'a> &'a N: Into<geo_types::Coord<f64>>,
{
    fn from(val: MultiPoint<N>) -> Self {
        (&val).into()
    }
}

impl<N: Coord> From<&geo_types::MultiPoint<f64>> for MultiPoint<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(point: &geo_types::MultiPoint<f64>) -> Self {
        let a: Vec<N> = point.coords_iter().map(|c| (&c).into()).collect::<Vec<_>>();
        MultiPoint::new(a, None)
    }
}
impl<N: Coord> From<geo_types::MultiPoint<f64>> for MultiPoint<N>
where
    for<'a> N: From<&'a geo_types::Coord<f64>>,
{
    fn from(point: geo_types::MultiPoint<f64>) -> Self {
        (&point).into()
    }
}
