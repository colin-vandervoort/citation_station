use std::fmt;

use serde::{Deserialize, Serialize};

use crate::api::{
    date::PublishDate,
    media::{
        book::Book,
        conference_paper::{ConferencePaperOnline, ConferenceProceedingsOnline},
        online_manual::OnlineManual,
        online_video::OnlineVideo,
    },
};

pub trait IeeeFormatting {
    fn citation_string(&self) -> String;
}

pub trait ApaFormatting {
    fn citation_string(&self) -> String;
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
            Citation::OnlineVideo(online_video) => match online_video {
                OnlineVideo::Generic { common_data, .. } => common_data.id.clone(),
                OnlineVideo::YouTube { common_data, .. } => common_data.id.clone(),
            },
        }
    }

    pub fn title(&self) -> String {
        match self {
            Citation::Book(book) => book.title.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.title.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.title.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.title.clone(),
            Citation::OnlineVideo(online_video) => match online_video {
                OnlineVideo::Generic { title, .. } => title.clone(),
                OnlineVideo::YouTube { title, .. } => title.clone(),
            },
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
            Citation::OnlineVideo(online_video) => match online_video {
                OnlineVideo::Generic { common_data, .. } => common_data.published.clone(),
                OnlineVideo::YouTube { common_data, .. } => common_data.published.clone(),
            },
        }
    }

    /// Format the citation in APA style
    pub fn format_apa(&self) -> String {
        match self {
            Citation::Book(book) => ApaFormatting::citation_string(book),
            Citation::ConferencePaperOnline(_paper) => todo!(),
            Citation::ConferenceProceedingsOnline(_proceedings) => todo!(),
            Citation::OnlineManual(_online_manual) => todo!(),
            Citation::OnlineVideo(_online_video) => todo!(),
        }
    }

    pub fn format_ieee(&self) -> String {
        match self {
            Citation::Book(book) => IeeeFormatting::citation_string(book),
            Citation::ConferencePaperOnline(_paper) => todo!(),
            Citation::ConferenceProceedingsOnline(_proceedings) => todo!(),
            Citation::OnlineManual(_online_manual) => todo!(),
            Citation::OnlineVideo(_online_video) => todo!(),
        }
    }
}

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_apa())
    }
}

#[cfg(test)]
mod tests {
    use crate::api::{
        author::{GenericAuthor, PersonName},
        citation::Citation,
        media::{book::Book, common::CommonCitationData},
    };

    #[test]
    fn test_citation_creation() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "cv_algo_practice".to_string(),
                published: None,
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_middle_last("Colin", "James", "VanDervoort").unwrap(),
                ],
            },
            title: "algo_practice".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        assert_eq!(citation.id(), "cv_algo_practice");
        assert_eq!(citation.title(), "algo_practice");
    }
}
