use crate::common::marker_type;
use serde::{Deserialize, Serialize};

marker_type!(ArrowStyle, "arrow");

/// <https://developers.arcgis.com/web-map-specification/objects/marker/>
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Marker {
    #[serde(skip_serializing_if = "Option::is_none")]
    placement: Option<Placement>,
    style: ArrowStyle,
}

/// Builder pattern
impl Marker {
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = Some(placement);
        self
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Placement {
    Begin,
    BeginEnd,
    End,
}
