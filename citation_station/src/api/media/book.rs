// cSpell: ignore Popov

use serde::{Deserialize, Serialize};

use crate::{
    api::{
        author::GenericAuthor,
        citation::{ApaFormatting, IeeeFormatting},
        date::ieee_abbreviated_month_name,
        media::{common::CommonCitationData, version::GenericMediaVersion},
        page_range::PageRange,
    },
    unicode::{LEFT_QUOTE, RIGHT_QUOTE},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Book {
    pub common_data: CommonCitationData,
    /// Author
    pub author: GenericAuthor,
    /// Book title
    pub title: String,
    /// Chapter
    pub chapter: Option<String>,
    /// Edition
    pub version: Option<GenericMediaVersion>,
    /// DOI (Digital Object Identifier)
    pub doi: Option<String>,
    /// Page range
    pub pages: Option<PageRange>,
}

impl IeeeFormatting for Book {
    fn citation_string(&self) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(authors) = &self.author.as_ieee_string() {
            parts.push(format!("{},", authors));
        }

        parts.push(match (&self.chapter, &self.version) {
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
        });

        if let Some(published) = &self.common_data.published {
            parts.push(format!("{}.", published.fmt_for_ieee_citation()));
        }

        parts.join(" ")
    }
}

impl ApaFormatting for Book {
    fn citation_string(&self) -> String {
        let authors_editors = if let Some(authors) = &self.author.as_apa_string() {
            format!("{} ", authors)
        } else {
            "".to_string()
        };
        let published_title_version = match (&self.common_data.published, &self.version) {
            (None, None) => format!("{}.", self.title),
            (None, Some(version)) => {
                format!("{} {}.", self.title, version.as_apa_string())
            }
            (Some(published), None) => {
                format!("({}). {}.", published.year(), self.title)
            }
            (Some(published), Some(version)) => {
                format!(
                    "({}). {}. {}.",
                    published.year(),
                    self.title,
                    version.as_apa_string()
                )
            }
        };

        format!("{}{}", authors_editors, published_title_version)
    }
}

#[cfg(test)]
mod tests {
    use chrono::Month;

    use crate::api::{
        author::{GenericAuthor, PersonName},
        citation::Citation,
        date::PublishDate,
        media::{book::Book, common::CommonCitationData},
    };

    #[test]
    fn test_book_apa_formatting_minimal() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![PersonName::from_first_last("J", "Smith").unwrap()],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_apa();
        assert_eq!(formatted, "Smith, J. (2023). A Great Paper.");
    }

    #[test]
    fn test_book_apa_formatting_two_authors() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                ],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_apa();
        assert_eq!(formatted, "Smith, J., & Fuentes, H. (2023). A Great Paper.");
    }

    #[test]
    fn test_book_apa_formatting_three_authors() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                    PersonName::from_first_last("Isabel", "Popov").unwrap(),
                ],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_apa();
        assert_eq!(formatted, "Smith, J. et al. (2023). A Great Paper.");
    }

    #[test]
    fn test_book_ieee_formatting_minimal() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![PersonName::from_first_last("J", "Smith").unwrap()],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_ieee();
        assert_eq!(formatted, "J. Smith, A Great Paper. 2023.");
    }

    #[test]
    fn test_book_ieee_formatting_two_authors() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                ],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_ieee();
        assert_eq!(formatted, "J. Smith and H. Fuentes, A Great Paper. 2023.");
    }

    #[test]
    fn test_book_ieee_formatting_three_authors() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year(2023)),
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                    PersonName::from_first_last("Isabel", "Popov").unwrap(),
                ],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_ieee();
        assert_eq!(
            formatted,
            "J. Smith, H. Fuentes, and I. Popov, A Great Paper. 2023."
        );
    }

    #[test]
    fn test_book_ieee_formatting_three_authors_full_date() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: Some(PublishDate::from_year_month_day(2023, Month::January, 1).unwrap()),
            },
            author: GenericAuthor::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                    PersonName::from_first_last("Isabel", "Popov").unwrap(),
                ],
            },
            title: "A Great Paper".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        let formatted = citation.format_ieee();
        assert_eq!(
            formatted,
            "J. Smith, H. Fuentes, and I. Popov, A Great Paper. Jan. 1, 2023."
        );
    }
}
