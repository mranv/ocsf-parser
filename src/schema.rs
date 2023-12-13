use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct OCSFEvent {
    pub class_uid: u32,
    pub category_uid: u32,
    pub time: String,
    pub message: String,
    pub severity: String,
    pub activity_id: u32,
    pub activity_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}
