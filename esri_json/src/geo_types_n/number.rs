use std::fmt::Debug;

mod private {
    pub trait Sealed {}
}

/// A trait for floating point number types (f32 or f64).
/// This trait is sealed and cannot be implemented outside this crate.
/// roughly equivalent to geo_traits::GeoNum / CoordFloat
#[cfg(feature = "geo_types")]
pub trait CoordNumber:
    private::Sealed
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Debug
    + Default
    + num_traits::Num
    + num_traits::sign::Signed
    + num_traits::Bounded
    + num_traits::FromPrimitive
    + num_traits::ToPrimitive
    + num_traits::NumCast
    + num_traits::Zero
    + geo::GeoFloat
{
}

#[cfg(not(feature = "geo_types"))]
pub trait CoordNumber:
    private::Sealed
    + Copy
    + Clone
    + PartialEq
    + PartialOrd
    + Debug
    + Default
    + num_traits::Num
    + num_traits::sign::Signed
    + num_traits::Bounded
    + num_traits::FromPrimitive
    + num_traits::ToPrimitive
    + num_traits::NumCast
    + num_traits::Zero
{
}

impl private::Sealed for f32 {}
impl private::Sealed for f64 {}

impl CoordNumber for f32 {}
impl CoordNumber for f64 {}
