use crate::geometry::Coord;
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};
/// Base Coordinate type with X and Y coordinates and an optional Measure value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<f64>", from = "Vec<f64>")]
pub struct CoordXym {
    x: f64,
    y: f64,
    m: Option<f64>,
}

impl From<&CoordXym> for Vec<f64> {
    fn from(val: &CoordXym) -> Self {
        if let Some(m) = val.m {
            vec![val.x, val.y, m]
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
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xym
    }
    fn has_z() -> bool {
        false
    }
    fn has_m() -> bool {
        true
    }
    fn m(&self) -> Option<f64> {
        self.m
    }
    fn from_coord_fields(x: f64, y: f64, _z: Option<f64>, m: Option<f64>) -> Self {
        Self { x, y, m }
    }
}

impl CoordTrait for CoordXym {
    type T = f64;
    fn x(&self) -> Self::T {
        self.x
    }
    fn y(&self) -> Self::T {
        self.y
    }
    fn dim(&self) -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xym
    }
    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.m.unwrap_or(0.0),
            _ => panic!("Expected 3 values, got {}", n),
        }
    }
}
