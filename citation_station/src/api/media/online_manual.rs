use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::{
    api::{
        author::GenericAuthor,
        media::{common::CommonCitationData, version::GenericMediaVersion},
    },
    unicode::{LEFT_QUOTE, RIGHT_QUOTE},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum OnlineManualAvailability {
    #[default]
    NotAvailable,
    DOI(String),
    URL(String),
    LibraryDatabaseProvider(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnlineManual {
    /// Universal data shared between different media types
    pub common_data: CommonCitationData,
    /// Author
    pub author: GenericAuthor,
    /// Title
    pub title: String,
    /// Version
    pub version: Option<GenericMediaVersion>,
    /// DOI, library database provider, or URL
    pub available_at: OnlineManualAvailability,
    /// When the resource was viewed
    pub accessed: NaiveDate,
}

impl OnlineManual {
    pub fn title_as_ieee_string(&self) -> String {
        match &self.version {
            None => format!("{}.", self.title),
            Some(version) => format!("{}, {}.", self.title, version.as_ieee_string()),
        }
    }

    pub fn title_as_apa_string(&self) -> String {
        todo!();
        // format!("{}{}{}", LEFT_QUOTE, self.title, RIGHT_QUOTE)
    }
}
