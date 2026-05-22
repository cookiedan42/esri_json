use super::Coord;
use geo_traits::LineStringTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct LineString<C: Coord>(pub Vec<C>);

impl<IC, C> From<&[IC]> for LineString<C>
where
    for<'a> &'a IC: Into<C>,
    C: Coord,
{
    fn from(val: &[IC]) -> Self {
        LineString(val.iter().map(|c| c.into()).collect())
    }
}

impl<C: Coord> LineStringTrait for LineString<C> {
    type CoordType<'a>
        = C
    where
        C: 'a;

    fn num_coords(&self) -> usize {
        self.0.len()
    }
    unsafe fn coord_unchecked(&self, i: usize) -> Self::CoordType<'_> {
        self.0[i]
    }
}

impl<C: Coord> From<&LineString<C>> for geo_types::LineString<C::T>
where
    geo_types::Coord<C::T>: for<'a> From<&'a C>,
{
    fn from(value: &LineString<C>) -> Self {
        Self::from_iter(value.0.iter().map(Into::<geo_types::Coord<C::T>>::into))
    }
}
impl<C: Coord> From<&geo_types::LineString<C::T>> for LineString<C>
where
    C: for<'a> From<&'a geo_types::Coord<C::T>>,
{
    fn from(value: &geo_types::LineString<C::T>) -> Self {
        Self(value.0.iter().map(Into::into).collect())
    }
}
impl_from!(geo_types::LineString<C::T>, LineString<C>);
impl_from!(LineString<C>, geo_types::LineString<C::T>);
