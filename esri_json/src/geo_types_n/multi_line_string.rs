use super::Coord;
use super::line_string::LineString;
use geo_traits::MultiLineStringTrait;

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct MultiLineString<C: Coord>(pub Vec<LineString<C>>);

impl<ILS, C> From<&[ILS]> for MultiLineString<C>
where
    for<'a> &'a ILS: Into<LineString<C>>,
    C: Coord,
{
    fn from(val: &[ILS]) -> Self {
        MultiLineString(val.iter().map(|ls| ls.into()).collect())
    }
}

impl<C: Coord> MultiLineStringTrait for MultiLineString<C> {
    type InnerLineStringType<'a>
        = LineString<C>
    where
        C: 'a;

    unsafe fn line_string_unchecked(&self, i: usize) -> Self::InnerLineStringType<'_> {
        self.0[i].clone()
    }
    fn num_line_strings(&self) -> usize {
        self.0.len()
    }
}

impl<C: Coord> From<&MultiLineString<C>> for geo_types::MultiLineString<C::T>
where
    geo_types::LineString<C::T>: for<'a> From<&'a LineString<C>>,
{
    fn from(value: &MultiLineString<C>) -> Self {
        Self::from_iter(
            value
                .0
                .iter()
                .map(Into::<geo_types::LineString<C::T>>::into),
        )
    }
}
impl<C: Coord> From<&geo_types::MultiLineString<C::T>> for MultiLineString<C>
where
    LineString<C>: for<'a> From<&'a geo_types::LineString<C::T>>,
{
    fn from(value: &geo_types::MultiLineString<C::T>) -> Self {
        Self(value.0.iter().map(Into::into).collect())
    }
}
impl_from!(geo_types::MultiLineString<C::T>, MultiLineString<C>);
impl_from!(MultiLineString<C>, geo_types::MultiLineString<C::T>);
