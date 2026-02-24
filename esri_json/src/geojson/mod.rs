// use crate::geometry::{Coord, Point};

// impl<C: Coord> TryFrom<&geojson::Value> for Point<C> {
//     type Error = geojson::Error;

//     fn try_from(val: &geojson::Value) -> Result<Self, Self::Error> {
//         match val {
//             geojson::Value::Point(g) => Ok(g.into()),
//             _ => Err(geojson::Error::InvalidGeometryType),
//         }
//     }
// }

// impl<C: Coord> TryFrom<geojson::Value> for Point<C> {
//     type Error = geojson::Error;
//     fn try_from(val: geojson::Value) -> Result<Self, Self::Error> {
//         (&val).try_into()
//     }
// }
