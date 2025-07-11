use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TabQueryArgs {
    pub active: bool,
    #[serde(rename = "lastFocusedWindow")]
    pub last_focused_window: bool,
}

impl TabQueryArgs {
    pub fn new(active: bool, last_focused_window: bool) -> Self {
        Self {
            active,
            last_focused_window,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    pub id: Option<i32>,
    pub url: Option<String>,
}
