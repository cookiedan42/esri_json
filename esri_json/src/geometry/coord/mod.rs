mod xy;
mod xym;
mod xyz;
mod xyzm;

pub use xy::CoordXy;
pub use xym::CoordXym;
pub use xyz::CoordXyz;
pub use xyzm::CoordXyzm;

use geo_traits::CoordTrait;

/// Coord is the primitive type for all coordinate types
/// It has no spatial reference
/// Optional Z and M values
pub trait Coord: Copy + CoordTrait<T = f64> {
    fn dim() -> geo_traits::Dimensions;
    fn has_z() -> bool;
    fn has_m() -> bool;

    fn has_m_field() -> Option<bool> {
        if Self::has_m() { Some(true) } else { None }
    }
    fn has_z_field() -> Option<bool> {
        if Self::has_z() { Some(true) } else { None }
    }

    fn z(&self) -> Option<f64> {
        None
    }
    fn m(&self) -> Option<f64> {
        None
    }
    fn set_z(self, z: f64) -> Self {
        Self::from_coord_fields(self.x(), self.y(), Some(z), self.m())
    }
    fn from_coord<C: Coord>(c: &C) -> Self {
        Self::from_coord_fields(c.x(), c.y(), c.z(), c.m())
    }
    fn from_coord_fields(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self;
}
