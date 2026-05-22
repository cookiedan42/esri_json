use super::Coord;
use geo_traits::TriangleTrait;

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Triangle<C: Coord>(pub C, pub C, pub C);

impl<C> TriangleTrait for Triangle<C>
where
    C: Coord,
{
    type CoordType<'a>
        = C
    where
        C: 'a;

    fn first(&self) -> Self::CoordType<'_> {
        self.0
    }
    fn second(&self) -> Self::CoordType<'_> {
        self.1
    }
    fn third(&self) -> Self::CoordType<'_> {
        self.2
    }
}

impl<C: Coord> From<&Triangle<C>> for geo_types::Triangle<C::T>
where
    geo_types::Coord<C::T>: From<C>,
{
    fn from(val: &Triangle<C>) -> Self {
        Self(val.0.into(), val.1.into(), val.2.into())
    }
}
impl<C: Coord> From<&geo_types::Triangle<C::T>> for Triangle<C>
where
    C: From<geo_types::Coord<C::T>>,
{
    fn from(val: &geo_types::Triangle<C::T>) -> Self {
        Self(val.0.into(), val.1.into(), val.2.into())
    }
}
impl_from!(geo_types::Triangle<C::T>, Triangle<C>);
impl_from!(Triangle<C>, geo_types::Triangle<C::T>);
