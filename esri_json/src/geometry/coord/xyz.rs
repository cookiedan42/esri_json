use crate::CoordNumber;
use crate::geometry::Coord;
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X, Y and Z coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "[T;3]", from = "Vec<T>")]
pub struct CoordXyz<T>
where
    T: CoordNumber,
{
    x: T,
    y: T,
    z: T,
}
impl<T> From<&CoordXyz<T>> for [T; 3]
where
    T: CoordNumber,
{
    fn from(val: &CoordXyz<T>) -> Self {
        [val.x, val.y, val.z]
    }
}

impl<T> From<CoordXyz<T>> for [T; 3]
where
    T: CoordNumber,
{
    fn from(val: CoordXyz<T>) -> Self {
        [val.x, val.y, val.z]
    }
}

impl<T> From<Vec<T>> for CoordXyz<T>
where
    T: CoordNumber,
{
    fn from(array: Vec<T>) -> Self {
        match array.len() {
            2 => Self {
                x: array[0],
                y: array[1],
                z: T::zero(),
            },
            3 => Self {
                x: array[0],
                y: array[1],
                z: array[2],
            },
            4 => Self {
                x: array[0],
                y: array[1],
                z: array[2],
            },
            _ => panic!("Expected array of length 3, got {}", array.len()),
        }
    }
}

impl<T> Coord for CoordXyz<T>
where
    T: CoordNumber,
{
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }
    fn has_z() -> bool {
        true
    }
    fn has_m() -> bool {
        false
    }
    fn z(&self) -> Option<T> {
        Some(self.z)
    }
    fn from_coord_fields<C>(x: C, y: C, z: Option<C>, _m: Option<C>) -> Self
    where
        C: Into<T>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.map_or_else(T::zero, Into::into),
        }
    }
}

impl<T> CoordTrait for CoordXyz<T>
where
    T: CoordNumber,
{
    type T = T;
    fn x(&self) -> T {
        self.x
    }
    fn y(&self) -> T {
        self.y
    }
    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }
    fn nth_or_panic(&self, n: usize) -> T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Expected 3 values, got {}", n),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coord_xyz() {
        let _val2: CoordXyz<f64> = vec![0.0, 0.0].into();
        let _val3: CoordXyz<f64> = vec![0.0, 0.0, 0.0].into();
        let _val4: CoordXyz<f64> = vec![0.0, 0.0, 0.0, 0.0].into();
    }
}
