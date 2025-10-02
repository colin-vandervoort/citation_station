use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LocationData {
    pub city: String,
    pub state: Option<String>,
    pub country: String,
}
