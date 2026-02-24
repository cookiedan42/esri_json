use super::Coord;
use super::Point;
use geo_traits::MultiPointTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct MultiPoint<C: Coord>(pub Vec<Point<C>>);

impl<IP, C> From<&[IP]> for MultiPoint<C>
where
    for<'a> &'a IP: Into<Point<C>>,
    C: Coord,
{
    fn from(val: &[IP]) -> Self {
        MultiPoint(val.iter().map(|p| p.into()).collect())
    }
}

impl<C: Coord> MultiPointTrait for MultiPoint<C> {
    type InnerPointType<'a>
        = Point<C>
    where
        C: 'a;

    unsafe fn point_unchecked(&self, i: usize) -> Self::InnerPointType<'_> {
        self.0[i]
    }
    fn num_points(&self) -> usize {
        self.0.len()
    }
}

impl<C: Coord> From<&MultiPoint<C>> for geo_types::MultiPoint<f64>
where
    geo_types::Point<f64>: for<'a> From<&'a Point<C>>,
{
    fn from(val: &MultiPoint<C>) -> Self {
        Self::from_iter(val.0.iter().map(Into::<geo_types::Point<f64>>::into))
    }
}
impl<C: Coord> From<&geo_types::MultiPoint<f64>> for MultiPoint<C>
where
    Point<C>: for<'a> From<&'a geo_types::Point<f64>>,
{
    fn from(val: &geo_types::MultiPoint<f64>) -> Self {
        Self(val.0.iter().map(Into::into).collect())
    }
}
impl_from!(geo_types::MultiPoint<f64>, MultiPoint<C>);
impl_from!(MultiPoint<C>, geo_types::MultiPoint<f64>);
