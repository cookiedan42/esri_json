use serde::{Deserialize, Serialize};

/// Shape of the tips at the start and end of each line geometry. This also applies to the tips of each pattern segment along the line
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

/// Shape of the intersection of two line segments
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}
