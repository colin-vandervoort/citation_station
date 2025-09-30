use ordinal::ToOrdinal as _;
use serde::{Deserialize, Serialize};

use crate::{
    api::{media::common::CommonCitationData, page_range::PageRange},
    unicode::{EMDASH, LEFT_QUOTE, RIGHT_QUOTE},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookVersion {
    DigitalEdition { number: u16 },
    Edition { number: u16 },
    Volume { number: u16 },
    VolumeRange { start: u16, end: u16 },
}

impl BookVersion {
    pub fn as_ieee_string(&self) -> String {
        match self {
            BookVersion::DigitalEdition { number } => {
                format!("{} digital ed.", number.to_ordinal_string())
            }
            BookVersion::Edition { number } => format!("{} ed.", number.to_ordinal_string()),
            BookVersion::Volume { number } => format!("vol. {}", number),
            BookVersion::VolumeRange { start, end } => format!("vols. {}{}{}", start, EMDASH, end),
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            BookVersion::DigitalEdition { number } => {
                format!("({} digital ed.)", number.to_ordinal_string())
            }
            BookVersion::Edition { number } => format!("({} ed.)", number.to_ordinal_string()),
            BookVersion::Volume { number } => format!("(Vol. {})", number),
            BookVersion::VolumeRange { start, end } => {
                format!("(Vols. {}{}{})", start, EMDASH, end)
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub common_data: CommonCitationData,
    /// Book title
    pub title: String,
    /// Chapter
    pub chapter: Option<String>,
    /// Edition
    pub version: Option<BookVersion>,
    /// DOI (Digital Object Identifier)
    pub doi: Option<String>,
    /// Page range
    pub pages: Option<PageRange>,
}

impl Book {
    pub fn title_as_ieee_string(&self) -> String {
        match (&self.chapter, &self.version) {
            (None, None) => format!("{}.", self.title),
            (None, Some(version)) => format!("{}, {}", self.title, version.as_ieee_string()),
            (Some(chapter), None) => format!(
                "{}{},{} in {}.",
                LEFT_QUOTE, chapter, RIGHT_QUOTE, self.title
            ),
            (Some(chapter), Some(version)) => format!(
                "{}{},{} in {}, {}.",
                LEFT_QUOTE,
                chapter,
                RIGHT_QUOTE,
                self.title,
                version.as_ieee_string()
            ),
        }
    }

    pub fn title_as_apa_string(&self) -> String {
        if let Some(version) = &self.version {
            format!("{} {}.", self.title, version.as_apa_string())
        } else {
            format!("{}.", self.title)
        }
    }
}
