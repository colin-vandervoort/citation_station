use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::media::common::CommonCitationData;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]

pub struct ConferencePaperOnline {
    pub common_data: CommonCitationData,
    pub title: String,
    /// Journal or venue name
    pub venue: Option<String>,
    /// Volume number
    pub volume: Option<String>,
    /// Issue or number
    pub number: Option<String>,
    pub conference_name: String,
    pub conference_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConferenceProceedingsOnline {
    pub common_data: CommonCitationData,
    pub title: String,
    /// Journal or venue name
    pub venue: Option<String>,
    /// Volume number
    pub volume: Option<String>,
    /// Issue or number
    pub number: Option<String>,
    pub conference_name: String,
    pub conference_date: DateTime<Utc>,
}
