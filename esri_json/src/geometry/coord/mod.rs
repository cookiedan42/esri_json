mod xy;
mod xym;
mod xyz;
mod xyzm;

pub use xy::CoordXy;
pub use xym::CoordXym;
pub use xyz::CoordXyz;
pub use xyzm::CoordXyzm;

/// Coord is the primitive type for all coordinate types
/// It has no spatial reference
/// Optional Z and M values
pub trait Coord: Clone {
    fn has_z() -> bool;
    fn has_m() -> bool;

    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> Option<f64> {
        None
    }
    fn m(&self) -> Option<f64> {
        None
    }
    fn from_coord<C: Coord>(c: C) -> Self;
    fn from_coord_fields(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self;
}
