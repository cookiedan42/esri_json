use super::Coord;
use super::polygon::Polygon;
use geo_traits::MultiPolygonTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct MultiPolygon<C: Coord>(pub Vec<Polygon<C>>);

impl<IP, C> From<&[IP]> for MultiPolygon<C>
where
    for<'a> &'a IP: Into<Polygon<C>>,
    C: Coord,
{
    fn from(val: &[IP]) -> Self {
        MultiPolygon(val.iter().map(|p| p.into()).collect())
    }
}

impl<C: Coord> MultiPolygonTrait for MultiPolygon<C> {
    type InnerPolygonType<'a>
        = Polygon<C>
    where
        Self: 'a;

    unsafe fn polygon_unchecked(&self, i: usize) -> Self::InnerPolygonType<'_> {
        self.0[i].clone()
    }
    fn num_polygons(&self) -> usize {
        self.0.len()
    }
}

impl<C: Coord> From<&MultiPolygon<C>> for geo_types::MultiPolygon<C::T>
where
    geo_types::Polygon<C::T>: for<'a> From<&'a Polygon<C>>,
{
    fn from(value: &MultiPolygon<C>) -> Self {
        Self::from_iter(value.0.iter().map(Into::<geo_types::Polygon<C::T>>::into))
    }
}
impl<C: Coord> From<&geo_types::MultiPolygon<C::T>> for MultiPolygon<C>
where
    Polygon<C>: for<'a> From<&'a geo_types::Polygon<C::T>>,
{
    fn from(value: &geo_types::MultiPolygon<C::T>) -> Self {
        Self(value.0.iter().map(Into::into).collect())
    }
}
impl_from!(geo_types::MultiPolygon<C::T>, MultiPolygon<C>);
impl_from!(MultiPolygon<C>, geo_types::MultiPolygon<C::T>);
