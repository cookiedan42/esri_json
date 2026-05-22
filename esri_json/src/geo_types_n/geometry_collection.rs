use geo_traits::GeometryCollectionTrait;

use super::Coord;
use super::Geometry;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct GeometryCollection<C: Coord>(pub Vec<Geometry<C>>);

impl<C: Coord> GeometryCollectionTrait for GeometryCollection<C> {
    type GeometryType<'a>
        = Geometry<C>
    where
        C: 'a;

    fn num_geometries(&self) -> usize {
        self.0.len()
    }
    unsafe fn geometry_unchecked(&self, i: usize) -> Self::GeometryType<'_> {
        self.0[i].clone()
    }
}

impl<C: Coord> From<&geo_types::GeometryCollection<C::T>> for GeometryCollection<C>
where
    Geometry<C>: for<'a> From<&'a geo_types::Geometry<C::T>>,
{
    fn from(value: &geo_types::GeometryCollection<C::T>) -> Self {
        let a = value.iter().map(Into::into).collect::<Vec<_>>();
        Self(a)
    }
}
impl<C: Coord> From<&GeometryCollection<C>> for geo_types::GeometryCollection<C::T>
where
    geo_types::Geometry<C::T>: for<'a> From<&'a Geometry<C>>,
{
    fn from(value: &GeometryCollection<C>) -> Self {
        Self::from_iter(value.0.iter().map(Into::<geo_types::Geometry<C::T>>::into))
    }
}
