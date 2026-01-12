use crate::geometry::Coord;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X and Y coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(into = "Vec<f64>", from = "Vec<f64>")]
pub struct CoordXy {
    x: f64,
    y: f64,
}

impl CoordXy {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<&CoordXy> for Vec<f64> {
    fn from(val: &CoordXy) -> Self {
        vec![val.x, val.y]
    }
}
impl From<CoordXy> for Vec<f64> {
    fn from(val: CoordXy) -> Self {
        (&val).into()
    }
}
impl From<Vec<f64>> for CoordXy {
    fn from(array: Vec<f64>) -> Self {
        match array.len() {
            2 => Self {
                x: array[0],
                y: array[1],
            },
            _ => panic!("Expected array of length 2 or 3, got {}", array.len()),
        }
    }
}

impl Coord for CoordXy {
    fn has_z() -> bool {
        false
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
    fn from_coord<C: Coord>(c: C) -> Self {
        Self { x: c.x(), y: c.y() }
    }
    fn from_coord_fields(x: f64, y: f64, _z: Option<f64>, _m: Option<f64>) -> Self {
        Self { x, y }
    }
}
