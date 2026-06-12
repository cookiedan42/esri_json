use crate::CoordNumber;
use crate::geometry::{Coord, FromCoordTrait};
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X and Y coordinates and an optional Measure value
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<T>", from = "Vec<T>")]
pub struct CoordXym<T>
where
    T: CoordNumber,
{
    x: T,
    y: T,
    m: Option<T>,
}

impl<T> From<&CoordXym<T>> for Vec<T>
where
    T: CoordNumber,
{
    fn from(val: &CoordXym<T>) -> Self {
        if let Some(m) = val.m {
            vec![val.x, val.y, m]
        } else {
            vec![val.x, val.y]
        }
    }
}
impl<T> From<CoordXym<T>> for Vec<T>
where
    T: CoordNumber,
{
    fn from(val: CoordXym<T>) -> Self {
        (&val).into()
    }
}

impl<T> From<Vec<T>> for CoordXym<T>
where
    T: CoordNumber,
{
    fn from(array: Vec<T>) -> Self {
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

impl<T> Coord for CoordXym<T>
where
    T: CoordNumber,
{
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xym
    }
    fn has_z() -> bool {
        false
    }
    fn has_m() -> bool {
        true
    }
    fn m(&self) -> Option<T> {
        self.m
    }
    fn from_coord_fields<C>(x: C, y: C, _z: Option<C>, m: Option<C>) -> Self
    where
        C: Into<T>,
    {
        Self {
            x: x.into(),
            y: y.into(),
            m: m.map(Into::into),
        }
    }
}

impl<T> CoordTrait for CoordXym<T>
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
        geo_traits::Dimensions::Xym
    }
    fn nth_or_panic(&self, n: usize) -> T {
        match n {
            0 => self.x,
            1 => self.y,
            2 => self.m.unwrap_or_else(T::zero),
            _ => panic!("Expected 3 values, got {}", n),
        }
    }
}

impl<T, C> FromCoordTrait<C> for CoordXym<T>
where
    T: CoordNumber,
    C: CoordTrait<T = T>,
{
    fn from_coord_trait(c: C) -> Self {
        use geo_traits::Dimensions;
        match c.dim() {
            Dimensions::Xy => Self::from_coord_fields(c.x(), c.y(), None, None),
            Dimensions::Xym => Self::from_coord_fields(c.x(), c.y(), Some(T::zero()), c.nth(3)),
            Dimensions::Xyz => Self::from_coord_fields(c.x(), c.y(), c.nth(3), None),
            Dimensions::Xyzm => Self::from_coord_fields(c.x(), c.y(), c.nth(3), c.nth(4)),
            Dimensions::Unknown(_) => panic!(
                "Unknown dimension, use `from_coord_fields` to explicity choose dimensions to assign to z and m"
            ),
        }
    }
}
