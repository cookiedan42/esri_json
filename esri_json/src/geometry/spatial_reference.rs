use serde::{Deserialize, Serialize};

/// Representation of a [SpatialReference](https://developers.arcgis.com/web-scene-specification/objects/spatialReference/)
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpatialReference {
    #[serde(skip_serializing_if = "Option::is_none")]
    wkid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "latestVcsWkid")]
    latest_vcs_wkid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "latestWkid")]
    latest_wkid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "vcsWkid")]
    vcs_wkid: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wkt: Option<String>,
}

/// Builder pattern
impl SpatialReference {
    pub const fn builder() -> Self {
        Self {
            wkid: None,
            latest_vcs_wkid: None,
            latest_wkid: None,
            vcs_wkid: None,
            wkt: None,
        }
    }
    pub const fn wkid(mut self, wkid: u32) -> Self {
        self.wkid = Some(wkid);
        self
    }
    pub const fn latest_vcs_wkid(mut self, latest_vcs_wkid: u32) -> Self {
        self.latest_vcs_wkid = Some(latest_vcs_wkid);
        self
    }
    pub const fn latest_wkid(mut self, latest_wkid: u32) -> Self {
        self.latest_wkid = Some(latest_wkid);
        self
    }
    pub const fn vcs_wkid(mut self, vcs_wkid: u32) -> Self {
        self.vcs_wkid = Some(vcs_wkid);
        self
    }
    pub fn wkt(mut self, wkt: String) -> Self {
        self.wkt = Some(wkt);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spatial_reference_example() {
        let spatial_reference = r#"{
            "wkid": 102100,
            "latestWkid": 3857
        }"#;
        let de: SpatialReference = serde_json::from_str(spatial_reference).unwrap();
        let ser = serde_json::to_string(&de).unwrap();
        let serde: SpatialReference = serde_json::from_str(&ser).unwrap();
        assert_eq!(serde, de);
    }
}
