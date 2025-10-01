pub mod api;
pub mod html;
mod unicode;

use api::{citation::Citation, date::PublishDate, errors::CitationError};

use chrono::Month;
use serde::{Deserialize, Serialize};
use std::fmt;

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
        author::{GenericAuthor, PersonName},
        media::{book::Book, common::CommonCitationData},
    };

    use super::*;

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

    #[test]
    fn test_bibliography() {
        let mut bib = Bibliography::new();

        let citation = Citation::Book(Book {
            common_data: CommonCitationData {
                id: "test".to_string(),
                published: None,
            },
            author: GenericAuthor::Persons {
                persons: vec![PersonName::from_first_last("Test", "Author").unwrap()],
            },
            title: "Test Title".to_string(),
            doi: None,
            pages: None,
            chapter: None,
            version: None,
        });

        assert!(bib.add_citation(citation).is_ok());
        assert_eq!(bib.citations().len(), 1);

        let found = bib.get_citation("test");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title(), "Test Title");
    }
}
