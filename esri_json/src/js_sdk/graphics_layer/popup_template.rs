//! <https://developers.arcgis.com/javascript/latest/api-reference/esri-PopupTemplate.html>

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct PopupTemplate {
    title: Option<String>,
    content: Option<String>,
}

/// Builder pattern
impl PopupTemplate {
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    pub fn content(mut self, content: String) -> Self {
        self.content = Some(content);
        self
    }
}
