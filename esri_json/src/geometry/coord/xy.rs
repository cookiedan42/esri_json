use crate::CoordNumber;
use crate::geometry::{Coord, FromCoordTrait};
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X and Y coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "[T;2]", from = "Vec<T>")]
pub struct CoordXy<T>
where
    T: CoordNumber,
{
    x: T,
    y: T,
}

impl<T> CoordXy<T>
where
    T: CoordNumber,
{
    pub fn new(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T> From<&CoordXy<T>> for [T; 2]
where
    T: CoordNumber,
{
    fn from(val: &CoordXy<T>) -> Self {
        [val.x, val.y]
    }
}
impl<T> From<CoordXy<T>> for [T; 2]
where
    T: CoordNumber,
{
    fn from(val: CoordXy<T>) -> Self {
        (&val).into()
    }
}
impl<T> From<Vec<T>> for CoordXy<T>
where
    T: CoordNumber,
{
    fn from(array: Vec<T>) -> Self {
        match array.len() {
            2 | 3 | 4 => Self {
                x: array[0],
                y: array[1],
            },
            _ => panic!("Expected array of length 2 or 3, got {}", array.len()),
        }
    }
}

impl<T> Coord for CoordXy<T>
where
    T: CoordNumber,
{
    fn dim() -> geo_traits::Dimensions {
        geo_traits::Dimensions::Xy
    }
    fn has_z() -> bool {
        false
    }
    fn has_m() -> bool {
        false
    }

    fn from_coord_fields<C>(x: C, y: C, _z: Option<C>, _m: Option<C>) -> Self
    where
        C: Into<Self::T>,
    {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl<T> CoordTrait for CoordXy<T>
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
        geo_traits::Dimensions::Xy
    }
    fn nth_or_panic(&self, n: usize) -> Self::T {
        match n {
            0 => self.x,
            1 => self.y,
            _ => panic!("Expected 2 values, got {}", n),
        }
    }
}

impl<T, C> FromCoordTrait<C> for CoordXy<T>
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
