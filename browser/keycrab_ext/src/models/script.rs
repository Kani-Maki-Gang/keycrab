use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct InjectionTarget {
    #[serde(rename = "tabId")]
    pub tab_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ExecuteScriptArgs {
    pub target: InjectionTarget,
    pub files: Vec<String>,
}
