use crate::geometry::Coord;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X and Y coordinates and an optional Measure value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(into = "Vec<f64>", from = "Vec<f64>")]
pub struct CoordXym {
    x: f64,
    y: f64,
    m: Option<f64>,
}

impl From<&CoordXym> for Vec<f64> {
    fn from(val: &CoordXym) -> Self {
        if val.m.is_some() {
            vec![val.x, val.y, val.m.unwrap()]
        } else {
            vec![val.x, val.y]
        }
    }
}
impl From<CoordXym> for Vec<f64> {
    fn from(val: CoordXym) -> Self {
        (&val).into()
    }
}
impl From<Vec<f64>> for CoordXym {
    fn from(array: Vec<f64>) -> Self {
        match array.len() {
            3 => Self {
                x: array[0],
                y: array[1],
                m: Some(array[2]),
            },
            2 => Self {
                x: array[0],
                y: array[1],
                m: None,
            },
            _ => panic!("Expected array of length 2 or 3, got {}", array.len()),
        }
    }
}

impl Coord for CoordXym {
    fn has_z() -> bool {
        false
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
    fn m(&self) -> Option<f64> {
        self.m
    }
    fn from_coord<C: Coord>(c: C) -> Self {
        Self {
            x: c.x(),
            y: c.y(),
            m: c.m(),
        }
    }
    fn from_coord_fields(x: f64, y: f64, _z: Option<f64>, m: Option<f64>) -> Self {
        Self { x, y, m }
    }
}
