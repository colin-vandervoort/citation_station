use serde::{Deserialize, Serialize};

use crate::api::{
    date::{PublishDate, ieee_abbreviated_month_name},
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
                if let Some(apa_authors) = book.author.as_apa_string() {
                    parts.push(apa_authors);
                }

                // Year
                if let Some(datetime_published) = &book.common_data.published {
                    parts.push(format!("({}).", datetime_published.year()));
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
        match self {
            Citation::Book(book) => {
                let mut parts = Vec::new();

                // Authors
                if let Some(ieee_authors) = book.author.as_ieee_string() {
                    parts.push(ieee_authors);
                }

                // Title
                parts.push(book.title_as_ieee_string());

                // Year
                if let Some(publish_date) = &book.common_data.published {
                    match (publish_date.month(), publish_date.day()) {
                        (None, None) => parts.push(format!("{}.", publish_date.year())),
                        (None, Some(_day)) => panic!(),
                        (Some(month), None) => parts.push(format!(
                            "{}, {}.",
                            ieee_abbreviated_month_name(&month),
                            publish_date.year()
                        )),
                        (Some(month), Some(day)) => parts.push(format!(
                            "{} {}, {}.",
                            ieee_abbreviated_month_name(&month),
                            day,
                            publish_date.year()
                        )),
                    }
                }

                parts.join(" ")
            }
            Citation::ConferencePaperOnline(_paper) => todo!(),
            Citation::ConferenceProceedingsOnline(_proceedings) => todo!(),
            Citation::OnlineManual(_online_manual) => todo!(),
            Citation::OnlineVideo(_online_video) => todo!(),
        }
    }
}
