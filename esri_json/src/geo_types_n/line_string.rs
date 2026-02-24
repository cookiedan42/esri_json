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

impl<C: Coord> From<&LineString<C>> for geo_types::LineString<f64>
where
    geo_types::Coord<f64>: for<'a> From<&'a C>,
{
    fn from(value: &LineString<C>) -> Self {
        Self::from_iter(value.0.iter().map(Into::<geo_types::Coord<f64>>::into))
    }
}
impl<C: Coord> From<&geo_types::LineString<f64>> for LineString<C>
where
    C: for<'a> From<&'a geo_types::Coord<f64>>,
{
    fn from(value: &geo_types::LineString<f64>) -> Self {
        Self(value.0.iter().map(Into::into).collect())
    }
}
impl_from!(geo_types::LineString<f64>, LineString<C>);
impl_from!(LineString<C>, geo_types::LineString<f64>);
