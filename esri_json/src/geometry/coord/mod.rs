mod xy;
mod xym;
mod xyz;
mod xyzm;

pub use xy::CoordXy;
pub use xym::CoordXym;
pub use xyz::CoordXyz;
pub use xyzm::CoordXyzm;

use crate::geo_types_n::CoordNumber;
use geo_traits::CoordTrait;

/// Coord is the primitive type for all coordinate types
/// It has no spatial reference
/// Optional Z and M values
pub trait Coord: Copy + CoordTrait<T: CoordNumber> {
    fn dim() -> geo_traits::Dimensions;
    fn has_z() -> bool;
    fn has_m() -> bool;

    fn has_m_field() -> Option<bool> {
        if Self::has_m() { Some(true) } else { None }
    }
    fn has_z_field() -> Option<bool> {
        if Self::has_z() { Some(true) } else { None }
    }

    fn z(&self) -> Option<<Self as CoordTrait>::T> {
        None
    }
    fn m(&self) -> Option<<Self as CoordTrait>::T> {
        None
    }
    fn set_z(self, z: <Self as CoordTrait>::T) -> Self {
        Self::from_coord_fields(self.x(), self.y(), Some(z), self.m())
    }
    fn from_coord<C>(c: &C) -> Self
    where
        C: Coord,
        <C as CoordTrait>::T: Into<<Self as CoordTrait>::T>,
    {
        Self::from_coord_fields(
            c.x().into(),
            c.y().into(),
            c.z().map(Into::into),
            c.m().map(Into::into),
        )
    }
    fn from_coord_fields<C>(x: C, y: C, z: Option<C>, m: Option<C>) -> Self
    where
        C: Into<<Self as CoordTrait>::T>;
}
