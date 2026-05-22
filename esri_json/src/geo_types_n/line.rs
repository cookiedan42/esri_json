use super::Coord;
use geo_traits::CoordTrait;
use geo_traits::LineTrait;
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub struct Line<C: Coord> {
    pub start: C,
    pub end: C,
}

impl<IC, C> From<(IC, IC)> for Line<C>
where
    for<'a> &'a IC: Into<C>,
    C: Coord,
{
    fn from(c: (IC, IC)) -> Self {
        Line {
            start: (&c.0).into(),
            end: (&c.1).into(),
        }
    }
}

impl<C: Coord> LineTrait for Line<C> {
    // type CoordType<'a>: 'a + CoordTrait<T = Self::T>

    type CoordType<'a>
        = C
    where
        C: 'a;

    fn start(&self) -> Self::CoordType<'_> {
        self.start
    }
    fn end(&self) -> Self::CoordType<'_> {
        self.end
    }
}

impl<C: Coord> From<&Line<C>> for geo_types::Line<<C as CoordTrait>::T>
where
    geo_types::Coord<<C as CoordTrait>::T>: From<C>,
{
    fn from(val: &Line<C>) -> Self {
        Self {
            start: val.start.into(),
            end: val.end.into(),
        }
    }
}

impl<C: Coord> From<&geo_types::Line<<C as CoordTrait>::T>> for Line<C>
where
    C: From<geo_types::Coord<<C as CoordTrait>::T>>,
{
    fn from(value: &geo_types::Line<<C as CoordTrait>::T>) -> Self {
        Self {
            start: value.start.into(),
            end: value.end.into(),
        }
    }
}

impl_from!(geo_types::Line<C::T>, Line<C>);
impl_from!(Line<C>, geo_types::Line<C::T>);
