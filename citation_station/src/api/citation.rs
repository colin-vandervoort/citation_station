use serde::{Deserialize, Serialize};

use crate::api::{
    author::Author,
    date::PublishDate,
    media::{
        book::Book,
        conference_paper::{ConferencePaperOnline, ConferenceProceedingsOnline},
        online_manual::OnlineManual,
        online_video::OnlineVideo,
    },
};

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct LocationData {
//     pub city: String,
//     pub state: Option<String>,
//     pub country: String,
// }

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
            Citation::Book(book) => book.title.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.title.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.title.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.title.clone(),
            Citation::OnlineVideo(online_video) => online_video.title.clone(),
        }
    }

    pub fn author(&self) -> Author {
        match self {
            Citation::Book(book) => book.common_data.author.clone(),
            Citation::ConferencePaperOnline(conference_paper_online) => {
                conference_paper_online.common_data.author.clone()
            }
            Citation::ConferenceProceedingsOnline(conference_proceedings_online) => {
                conference_proceedings_online.common_data.author.clone()
            }
            Citation::OnlineManual(online_manual) => online_manual.common_data.author.clone(),
            Citation::OnlineVideo(online_video) => online_video.common_data.author.clone(),
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

    /// Format the citation in APA style
    pub fn format_apa(&self) -> String {
        let mut parts = Vec::new();

        match self {
            Citation::Book(book) => {
                // Authors
                if let Some(apa_authors) = book.common_data.author.as_apa_string() {
                    parts.push(apa_authors);
                }

                // Year
                if let Some(datetime_published) = &book.common_data.published {
                    parts.push(format!("({})", datetime_published.year()));
                }

                // Title
                parts.push(book.title_as_apa_string());
            }
            Citation::ConferencePaperOnline(_paper) => todo!(),
            Citation::ConferenceProceedingsOnline(_proceedings) => todo!(),
            Citation::OnlineManual(_online_manual) => todo!(),
            Citation::OnlineVideo(_online_video) => todo!(),
        }

        parts.join(" ")
    }

    pub fn format_ieee(&self) -> String {
        todo!();
    }
}
