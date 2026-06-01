use crate::CoordNumber;
use crate::geometry::Coord;
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X, Y and Z coordinates and an optional Measure value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<T>", from = "Vec<T>")]
pub struct CoordXyzm<T>
where
    T: CoordNumber,
{
    x: T,
    y: T,
    z: T,
    m: Option<T>,
}
impl<T> From<&CoordXyzm<T>> for Vec<T>
where
    T: CoordNumber,
{
    fn from(val: &CoordXyzm<T>) -> Self {
        if let Some(m) = val.m {
            vec![val.x, val.y, val.z, m]
        } else {
            vec![val.x, val.y, val.z]
        }
    }
}
impl<T> From<CoordXyzm<T>> for Vec<T>
where
    T: CoordNumber,
{
    fn from(val: CoordXyzm<T>) -> Self {
        (&val).into()
    }
}

impl<T> From<Vec<T>> for CoordXyzm<T>
where
    T: CoordNumber,
{
    fn from(array: Vec<T>) -> Self {
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
            2 => Self {
                x: array[0],
                y: array[1],
                z: T::zero(),
                m: None,
            },
            _ => panic!("Expected array of length 3 or 4, got {}", array.len()),
        }
    }
}

impl<T> Coord for CoordXyzm<T>
where
    T: CoordNumber,
{
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xyzm
    }
    fn has_z() -> bool {
        true
    }
    fn has_m() -> bool {
        true
    }
    fn z(&self) -> Option<T> {
        Some(self.z)
    }
    fn m(&self) -> Option<T> {
        self.m
    }
    fn from_coord_fields<C>(x: C, y: C, z: Option<C>, m: Option<C>) -> Self
    where
        C: Into<Self::T>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.map_or_else(Self::T::zero, Into::into),
            m: m.map(Into::into),
        }
    }
}

impl<T> CoordTrait for CoordXyzm<T>
where
    T: CoordNumber,
{
    type T = T;
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
            2 => self.z,
            3 => self.m.unwrap_or_else(T::zero),
            _ => panic!("Expected 4 values, got {}", n),
        }
    }
}
