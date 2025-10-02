// cSpell: ignore scorpiopede, anomalocaris

use serde::{Deserialize, Serialize};

use crate::api::{
    citation::{ApaFormatting, IeeeFormatting},
    date::{AccessDate, ieee_abbreviated_month_name},
    media::common::CommonCitationData,
};

/// A video that was accessed via the internet.
///
/// IEEE formatting rules taken from:
/// * https://journals.ieeeauthorcenter.ieee.org/wp-content/uploads/sites/7/IEEE_Reference_Guide.pdf
///
/// APA formatting rules taken from:
/// * https://apastyle.apa.org/style-grammar-guidelines/references/examples/youtube-references
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OnlineVideo {
    Generic {
        common_data: CommonCitationData,
        title: String,
        url: Option<String>,
        accessed: AccessDate,
    },
    YouTube {
        common_data: CommonCitationData,
        title: String,
        url: Option<String>,
        channel: String,
        accessed: AccessDate,
    },
}

impl IeeeFormatting for OnlineVideo {
    fn citation_string(&self) -> String {
        match self {
            OnlineVideo::Generic { .. } => todo!(),
            OnlineVideo::YouTube {
                common_data,
                title,
                url: maybe_url,
                channel,
                accessed,
            } => {
                let mut parts: Vec<String> = vec![format!("{}.", channel)];
                // TODO: owner location
                parts.push(format!("{}.", title));
                if let Some(published) = &common_data.published {
                    parts.push(format!("({}).", published.fmt_for_ieee_citation()));
                }
                parts.push(format!(
                    "Accessed: {} {}, {}. [Online Video].",
                    ieee_abbreviated_month_name(&accessed.month()),
                    accessed.day(),
                    accessed.year(),
                ));
                if let Some(url) = maybe_url {
                    parts.push(format!("Available: {}", url.to_string()));
                }
                parts.join(" ")
            }
        }
    }
}

impl ApaFormatting for OnlineVideo {
    fn citation_string(&self) -> String {
        match self {
            OnlineVideo::Generic { .. } => todo!(),
            OnlineVideo::YouTube {
                common_data,
                title,
                url: maybe_url,
                channel,
                accessed,
            } => {
                let mut parts: Vec<String> = vec![format!("{}.", channel)];
                if let Some(published) = &common_data.published {
                    parts.push(format!("({}).", published.fmt_for_apa_citation()));
                }
                parts.push(format!("{} [Video]. YouTube.", title));
                if let Some(url) = maybe_url {
                    parts.push(format!(
                        "Retrieved {} {}, {}, from {}",
                        accessed.month().name(),
                        accessed.day(),
                        accessed.year(),
                        url.to_string()
                    ));
                } else {
                    parts.push(format!(
                        "Retrieved {} {}, {}.",
                        accessed.month().name(),
                        accessed.day(),
                        accessed.year()
                    ));
                }
                parts.join(" ")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Month, NaiveDate};

    use crate::api::{
        citation::{ApaFormatting, IeeeFormatting},
        date::PublishDate,
        media::{common::CommonCitationData, online_video::OnlineVideo},
    };

    #[test]
    fn test_youtube_video_ieee_formatting() {
        let video = OnlineVideo::YouTube {
            common_data: CommonCitationData {
                id: "foo".to_string(),
                published: Some(PublishDate::from_year_month_day(2009, Month::April, 4).unwrap()),
            },
            title: "Tribute to anomalocaris".to_string(),
            url: Some("https://www.youtube.com/watch?v=6YsNRnZRgg8".to_string()),
            channel: "scorpiopede".to_string(),
            accessed: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap().into(),
        };

        assert_eq!(
            IeeeFormatting::citation_string(&video),
            "scorpiopede. Tribute to anomalocaris. (Apr. 4, 2009). Accessed: Oct. 1, 2025. [Online Video]. Available: https://www.youtube.com/watch?v=6YsNRnZRgg8"
        )
    }

    #[test]
    fn test_youtube_video_apa_formatting() {
        let video = OnlineVideo::YouTube {
            common_data: CommonCitationData {
                id: "foo".to_string(),
                published: Some(PublishDate::from_year_month_day(2009, Month::April, 4).unwrap()),
            },
            title: "Tribute to anomalocaris".to_string(),
            url: Some("https://www.youtube.com/watch?v=6YsNRnZRgg8".to_string()),
            channel: "scorpiopede".to_string(),
            accessed: NaiveDate::from_ymd_opt(2025, 10, 1).unwrap().into(),
        };

        assert_eq!(
            ApaFormatting::citation_string(&video),
            "scorpiopede. (2009, April 4). Tribute to anomalocaris [Video]. YouTube. Retrieved October 1, 2025, from https://www.youtube.com/watch?v=6YsNRnZRgg8"
        )
    }
}
