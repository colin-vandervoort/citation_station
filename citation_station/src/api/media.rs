use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::api::{author::Author, date::PublishDate};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonCitationData {
    /// Unique identifier for the citation
    pub id: String,
    /// Title of the work
    pub title: String,
    /// List of authors
    pub authors: Vec<Author>,
    /// Date published
    pub published: Option<PublishDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PageRange {
    pub start: u32,
    pub end: u32,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct LocationData {
//     pub city: String,
//     pub state: Option<String>,
//     pub country: String,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub common_data: CommonCitationData,
    /// DOI (Digital Object Identifier)
    pub doi: Option<String>,
    /// Page range
    pub pages: Option<PageRange>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConferencePaperOnline {
    pub common_data: CommonCitationData,
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
pub struct OnlineManual {
    pub common_data: CommonCitationData,
    /// URL
    pub url: Option<String>,
    pub accessed: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OnlineVideo {
    pub common_data: CommonCitationData,
    /// URL
    pub url: Option<String>,
    pub accessed: NaiveDate,
}

/// A bibliographic entry representing a citable work
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Citation {
    Book(Book),
    ConferencePaperOnline(ConferencePaperOnline),
    ConferenceProceedingsOnline(ConferenceProceedingsOnline),
    OnlineManual(OnlineManual),
    OnlineVideo(OnlineVideo),
}

impl Citation {
    pub fn id(&self) -> String {
        match self {
            Citation::Book(book) => book.common_data.id.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.common_data.id.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.common_data.id.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.common_data.id.clone(),
            Citation::OnlineVideo(online_video) => online_video.common_data.id.clone(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            Citation::Book(book) => book.common_data.title.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.common_data.title.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.common_data.title.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.common_data.title.clone(),
            Citation::OnlineVideo(online_video) => online_video.common_data.title.clone(),
        }
    }

    pub fn authors(&self) -> Vec<Author> {
        match self {
            Citation::Book(book) => book.common_data.authors.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.common_data.authors.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.common_data.authors.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.common_data.authors.clone(),
            Citation::OnlineVideo(online_video) => online_video.common_data.authors.clone(),
        }
    }

    pub fn published(&self) -> Option<PublishDate> {
        match self {
            Citation::Book(book) => book.common_data.published.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.common_data.published.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.common_data.published.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.common_data.published.clone(),
            Citation::OnlineVideo(online_video) => online_video.common_data.published.clone(),
        }
    }
}
