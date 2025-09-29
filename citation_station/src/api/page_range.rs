use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageRange {
    pub start: u32,
    pub end: u32,
}
