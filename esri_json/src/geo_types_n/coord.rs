use crate::geo_types_n::CoordNumber;
pub use crate::geometry::{Coord, CoordXy, CoordXym, CoordXyz, CoordXyzm};
use geo_traits::CoordTrait;
macro_rules! FromIntoGeoCoord {
    ($coordtype:ty) => {
        impl<T> From<$coordtype> for geo_types::Coord<T>
        where
            T: CoordNumber,
        {
            fn from(value: $coordtype) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                }
            }
        }

        impl<T> From<&$coordtype> for geo_types::Coord<T>
        where
            T: CoordNumber,
        {
            fn from(value: &$coordtype) -> Self {
                Self {
                    x: value.x(),
                    y: value.y(),
                }
            }
        }

        impl<T> From<geo_types::Coord<T>> for $coordtype
        where
            T: CoordNumber,
        {
            fn from(value: geo_types::Coord<T>) -> Self {
                <$coordtype>::from_coord_fields(value.x, value.y, None, None)
            }
        }
        impl<T> From<&geo_types::Coord<T>> for $coordtype
        where
            T: CoordNumber,
        {
            fn from(value: &geo_types::Coord<T>) -> Self {
                <$coordtype>::from_coord_fields(value.x, value.y, None, None)
            }
        }
    };
}

FromIntoGeoCoord!(CoordXy<T>);
FromIntoGeoCoord!(CoordXyz<T>);
FromIntoGeoCoord!(CoordXym<T>);
FromIntoGeoCoord!(CoordXyzm<T>);
