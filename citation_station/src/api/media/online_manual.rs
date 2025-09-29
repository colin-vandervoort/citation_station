use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::api::media::common::CommonCitationData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnlineManual {
    pub common_data: CommonCitationData,
    /// URL
    pub url: Option<String>,
    pub accessed: NaiveDate,
}
