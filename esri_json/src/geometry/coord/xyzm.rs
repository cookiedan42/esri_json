use crate::geometry::Coord;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X, Y and Z coordinates and an optional Measure value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<f64>", from = "Vec<f64>")]
pub struct CoordXyzm {
    x: f64,
    y: f64,
    z: f64,
    m: Option<f64>,
}
impl From<&CoordXyzm> for Vec<f64> {
    fn from(val: &CoordXyzm) -> Self {
        if val.m.is_some() {
            vec![val.x, val.y, val.z, val.m.unwrap()]
        } else {
            vec![val.x, val.y, val.z]
        }
    }
}
impl From<CoordXyzm> for Vec<f64> {
    fn from(val: CoordXyzm) -> Self {
        (&val).into()
    }
}

impl From<Vec<f64>> for CoordXyzm {
    fn from(array: Vec<f64>) -> Self {
        match array.len() {
            4 => Self {
                x: array[0],
                y: array[1],
                z: array[2],
                m: Some(array[3]),
            },
            3 => Self {
                x: array[0],
                y: array[1],
                z: array[2],
                m: None,
            },
            _ => panic!("Expected array of length 3 or 4, got {}", array.len()),
        }
    }
}

impl Coord for CoordXyzm {
    fn has_z() -> bool {
        true
    }
    fn has_m() -> bool {
        true
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
    fn m(&self) -> Option<f64> {
        self.m
    }
    fn from_coord_fields(x: f64, y: f64, z: Option<f64>, m: Option<f64>) -> Self {
        Self {
            x,
            y,
            z: z.unwrap_or(0.0),
            m,
        }
    }
}
