pub use crate::geometry::Coord;
pub use crate::geometry::{CoordXy, CoordXym, CoordXyz, CoordXyzm};
use geo_traits::CoordTrait;

macro_rules! FromIntoGeoCoord {
    ($coordtype:ty) => {
        impl From<$coordtype> for geo_types::Coord<f64> {
            fn from(value: $coordtype) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                }
            }
        }

        impl From<&$coordtype> for geo_types::Coord<f64> {
            fn from(value: &$coordtype) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                }
            }
        }

        impl From<geo_types::Coord<f64>> for $coordtype {
            fn from(value: geo_types::Coord<f64>) -> Self {
                <$coordtype>::from_coord_fields(value.x, value.y, None, None)
            }
        }
        impl From<&geo_types::Coord<f64>> for $coordtype {
            fn from(value: &geo_types::Coord<f64>) -> Self {
                <$coordtype>::from_coord_fields(value.x, value.y, None, None)
            }
        }
    };
}

FromIntoGeoCoord!(CoordXy);
FromIntoGeoCoord!(CoordXyz);
FromIntoGeoCoord!(CoordXym);
FromIntoGeoCoord!(CoordXyzm);
