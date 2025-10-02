// cSpell: ignore Breimann
use serde::{Deserialize, Serialize};

use crate::api::{
    author::GenericAuthor,
    citation::{ApaFormatting, IeeeFormatting},
    date::AccessDate,
    media::{common::CommonCitationData, version::GenericMediaVersion},
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
pub enum OnlineManualAvailability {
    #[default]
    NotAvailable,
    DOI(String),
    URL(String),
    LibraryDatabaseProvider(String),
}

/// A manual that was accessed via the internet.
///
/// IEEE formatting rules taken from:
/// * https://journals.ieeeauthorcenter.ieee.org/wp-content/uploads/sites/7/IEEE_Reference_Guide.pdf
///
/// APA does not explicitly define formatting for online manuals citations.
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
    pub accessed: AccessDate,
}

impl IeeeFormatting for OnlineManual {
    fn citation_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(author_formatted) = self.author.as_ieee_string() {
            parts.push(format!("{}.", author_formatted));
        }

        if let Some(version) = &self.version {
            parts.push(format!("{} {}.", self.title, version.as_ieee_string()));
        } else {
            parts.push(format!("{}.", self.title));
        }

        if let Some(published) = &self.common_data.published {
            parts.push(format!("({}).", published.fmt_for_apa_citation()));
        }

        parts.push(format!(
            "Accessed: {}. [Online].",
            self.accessed.fmt_for_ieee_citation()
        ));

        match &self.available_at {
            OnlineManualAvailability::NotAvailable => (),
            OnlineManualAvailability::DOI(_) => todo!(),
            OnlineManualAvailability::URL(url) => parts.push(format!("Available: {}", url.clone())),
            OnlineManualAvailability::LibraryDatabaseProvider(_) => todo!(),
        }

        parts.join(" ")
    }
}

impl ApaFormatting for OnlineManual {
    fn citation_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(author_formatted) = self.author.as_apa_string() {
            parts.push(format!("{}.", author_formatted));
        }

        if let Some(published) = &self.common_data.published {
            parts.push(format!("({}).", published.fmt_for_apa_citation()));
        }

        parts.push(format!("{}.", self.title));

        match &self.available_at {
            OnlineManualAvailability::NotAvailable => (),
            OnlineManualAvailability::DOI(_) => todo!(),
            OnlineManualAvailability::URL(url) => parts.push(url.clone()),
            OnlineManualAvailability::LibraryDatabaseProvider(_) => todo!(),
        }

        parts.join(" ")
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use crate::api::{
        author::{GenericAuthor, PersonName},
        citation::IeeeFormatting,
        date::PublishDate,
        media::{
            common::CommonCitationData,
            online_manual::{OnlineManual, OnlineManualAvailability},
            version::{GenericMediaVersion, SemVer},
        },
    };

    #[test]
    fn test_format_online_manual_ieee() {
        let manual = OnlineManual {
            common_data: CommonCitationData {
                id: "foo".to_string(),
                published: Some(PublishDate::from_year(2003)),
            },
            author: GenericAuthor::Persons {
                persons: vec![PersonName::from_first_last("L", "Breimann").unwrap()],
            },
            title: "Manual on Setting Up, Using, and Understanding Random Forests".to_string(),
            version: Some(GenericMediaVersion::SemVer(SemVer::from_major_minor(4, 0))),
            available_at: OnlineManualAvailability::URL(
                "http://oz.berkeley.edu/users/breiman/Using_random_forests_v4.0.pdf".to_string(),
            ),
            accessed: NaiveDate::from_ymd_opt(2014, 4, 16).unwrap().into(),
        };

        let expect = "L. Breimann. Manual on Setting Up, Using, and Understanding Random Forests \
                                    v4.0. (2003). Accessed: Apr. 16, 2014. [Online]. Available: \
                                    http://oz.berkeley.edu/users/breiman/Using_random_forests_v4.0.pdf";

        assert_eq!(IeeeFormatting::citation_string(&manual), expect)
    }
}
