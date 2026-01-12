use crate::geometry::Coord;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X, Y and Z coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

// impl<C: Coord> From<C> for CoordXyz {
//     fn from(value: C) -> Self::from_coord_fields(
//         value.x(),
//         value.y(),
//         value.z(),
//         value.m(),
//     )
// }

impl Coord for CoordXyz {
    fn has_z() -> bool {
        true
    }
    fn has_m() -> bool {
        false
    }
    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> Option<f64> {
        Some(self.z)
    }
    fn from_coord<C: Coord>(c: C) -> Self {
        Self {
            x: c.x(),
            y: c.y(),
            z: c.z().unwrap_or(0.0),
        }
    }
    fn from_coord_fields(x: f64, y: f64, z: Option<f64>, _m: Option<f64>) -> Self {
        Self {
            x,
            y,
            z: z.unwrap_or(0.0),
        }
    }
}
