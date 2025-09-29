use serde::{Deserialize, Serialize};

use crate::api::{media::common::CommonCitationData, page_range::PageRange};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub common_data: CommonCitationData,
    /// DOI (Digital Object Identifier)
    pub doi: Option<String>,
    /// Page range
    pub pages: Option<PageRange>,
}
