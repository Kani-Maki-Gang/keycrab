use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TabQueryArgs {
    pub active: bool,
}

impl TabQueryArgs {
    pub fn new(active: bool) -> Self {
        Self { active }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Tab {
    pub id: Option<i32>,
    pub url: Option<String>,
}
