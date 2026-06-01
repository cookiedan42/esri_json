use crate::CoordNumber;
use crate::geometry::Coord;
use geo_traits::CoordTrait;
use serde::{Deserialize, Serialize};

/// Base Coordinate type with X and Y coordinates
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(into = "Vec<T>", from = "Vec<T>")]
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

impl<T> From<&CoordXy<T>> for Vec<T>
where
    T: CoordNumber,
{
    fn from(val: &CoordXy<T>) -> Self {
        vec![val.x, val.y]
    }
}
impl<T> From<CoordXy<T>> for Vec<T>
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
