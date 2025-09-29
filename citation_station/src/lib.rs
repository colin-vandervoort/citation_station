pub mod api;
pub mod html;

use api::{date::PublishDate, errors::CitationError, media::Citation};

use chrono::Month;
use serde::{Deserialize, Serialize};
use std::fmt;

impl Citation {
    /// Format the citation in APA style
    pub fn format_apa(&self) -> String {
        let mut parts = Vec::new();

        match self {
            Citation::Book(book) => {
                // Authors
                if !book.common_data.authors.is_empty() {
                    let authors = if book.common_data.authors.len() == 1 {
                        let first_author = book.common_data.authors[0].clone();
                        first_author.as_apa_string()
                    } else if book.common_data.authors.len() == 2 {
                        format!(
                            "{} & {}",
                            book.common_data.authors[0].as_apa_string(),
                            book.common_data.authors[1].as_apa_string()
                        )
                    } else {
                        todo!();
                    };
                    parts.push(authors);
                }

                // Year
                if let Some(datetime_published) = &book.common_data.published {
                    parts.push(format!("({})", datetime_published.year()));
                }

                // Title
                parts.push(format!("{}.", book.common_data.title));
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

impl fmt::Display for Citation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_apa())
    }
}

/// A collection of citations forming a bibliography
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bibliography {
    citations: Vec<Citation>,
}

impl Bibliography {
    /// Create a new empty bibliography
    pub fn new() -> Self {
        Self {
            citations: Vec::new(),
        }
    }

    /// Add a citation to the bibliography
    pub fn add_citation(&mut self, citation: Citation) -> Result<(), CitationError> {
        // Check for duplicate IDs
        if self.citations.iter().any(|c| c.id() == citation.id()) {
            return Err(CitationError::InvalidFormat(format!(
                "Citation with ID '{}' already exists",
                citation.id()
            )));
        }

        self.citations.push(citation);
        Ok(())
    }

    /// Get a citation by ID
    pub fn get_citation(&self, id: &str) -> Option<&Citation> {
        self.citations.iter().find(|c| c.id() == id)
    }

    /// Get all citations
    pub fn citations(&self) -> &[Citation] {
        &self.citations
    }

    // pub fn sort_by_author(&mut self) {
    //     self.citations.sort_by(|a, b| {
    //         match(a.authors().first(), b.authors().first()) {
    //             (None, None) => Ordering::Equal,
    //             (None, Some(_)) => Ordering::Less,
    //             (Some(_), None) => Ordering::Greater,
    //             (Some(author_a), Some(author_b)) => author_a.cmp(author_b)
    //         }
    //     });
    // }

    /// Sort citations by year (descending)
    pub fn sort_by_publish_date(&mut self) {
        const DEFAULT_PUBLISH_DATE: PublishDate = PublishDate::from_year_month(0, Month::January);
        self.citations.sort_by(|a, b| {
            b.published()
                .unwrap_or(DEFAULT_PUBLISH_DATE)
                .cmp(&a.published().unwrap_or(DEFAULT_PUBLISH_DATE))
        });
    }
}

impl Default for Bibliography {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {

    use crate::api::{
        author::{Author, PersonName},
        media::{Book, CommonCitationData},
    };

    use super::*;

    #[test]
    fn test_citation_creation() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "cv_algo_practice".to_string(),
                title: "algo_practice".to_string(),
                authors: vec![Author::Person {
                    name: PersonName::from_first_middle_last("Colin", "James", "VanDervoort")
                        .unwrap(),
                }],
                published: Option::None,
            },
            doi: Option::None,
            pages: Option::None,
        });

        assert_eq!(citation.id(), "cv_algo_practice");
        assert_eq!(citation.title(), "algo_practice");
    }

    #[test]
    fn test_apa_formatting() {
        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                title: "A Great Paper".to_string(),
                authors: vec![Author::Person {
                    name: PersonName::from_first_last("J", "Smith").unwrap(),
                }],
                published: Some(PublishDate::from_year(2023)),
            },
            doi: Option::None,
            pages: Option::None,
        });

        let formatted = citation.format_apa();
        assert!(formatted.contains("Smith, J."));
        assert!(formatted.contains("(2023)"));
        assert!(formatted.contains("A Great Paper"));
    }

    #[test]
    fn test_bibliography() {
        let mut bib = Bibliography::new();

        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                title: "Test Title".to_string(),
                authors: vec![Author::Person {
                    name: PersonName::from_first_last("Test", "Author").unwrap(),
                }],
                published: None,
            },
            doi: Option::None,
            pages: Option::None,
        });

        assert!(bib.add_citation(citation).is_ok());
        assert_eq!(bib.citations().len(), 1);

        let found = bib.get_citation("test");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title(), "Test Title");
    }
}
