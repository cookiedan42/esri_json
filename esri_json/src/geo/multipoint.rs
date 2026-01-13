use geo::CoordsIter;

use crate::geometry::{Coord, MultiPoint};

impl<C: Coord> From<MultiPoint<C>> for geo_types::MultiPoint<f64>
where
    geo_types::Coord<f64>: From<C>,
{
    fn from(val: MultiPoint<C>) -> Self {
        let a = val.into_iter().map(Into::<geo_types::Coord>::into);
        geo_types::MultiPoint::<f64>::from_iter(a)
    }
}

impl<C: Coord> From<geo_types::MultiPoint<f64>> for MultiPoint<C>
where
    C: From<geo_types::Coord<f64>>,
{
    fn from(point: geo_types::MultiPoint<f64>) -> Self {
        let a: Vec<C> = point.coords_iter().map(Into::into).collect::<Vec<_>>();
        MultiPoint::new(a, None)
    }
}
