use crate::geometry::Coord;
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X, Y and Z coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<f64>", from = "Vec<f64>")]
pub struct CoordXyz {
    x: f64,
    y: f64,
    z: f64,
}
impl From<&CoordXyz> for Vec<f64> {
    fn from(val: &CoordXyz) -> Self {
        vec![val.x, val.y, val.z]
    }
}
impl From<CoordXyz> for Vec<f64> {
    fn from(val: CoordXyz) -> Self {
        (&val).into()
    }
}
impl From<Vec<f64>> for CoordXyz {
    fn from(array: Vec<f64>) -> Self {
        Self {
            x: array[0],
            y: array[1],
            z: array[2],
        }
    }
}

impl Coord for CoordXyz {
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }
    fn has_z() -> bool {
        true
    }
    fn has_m() -> bool {
        false
    }
    fn z(&self) -> Option<f64> {
        Some(self.z)
    }
    fn from_coord_fields(x: f64, y: f64, z: Option<f64>, _m: Option<f64>) -> Self {
        Self {
            x,
            y,
            z: z.unwrap_or(0.0),
        }
    }
}

impl CoordTrait for CoordXyz {
    type T = f64;
    fn x(&self) -> Self::T {
        self.x
    }
    fn y(&self) -> Self::T {
        self.y
    }
    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyz
    }
    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!("Expected 3 values, got {}", n),
        }
    }
}
