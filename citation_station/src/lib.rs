use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

/// Errors that can occur during citation processing
#[derive(Error, Debug)]
pub enum CitationError {
    #[error("Invalid citation format: {0}")]
    InvalidFormat(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Parsing error: {0}")]
    ParseError(String),
}

/// A bibliographic entry representing a citable work
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Citation {
    /// Unique identifier for the citation
    pub id: String,
    /// Title of the work
    pub title: String,
    /// List of authors
    pub authors: Vec<String>,
    /// Publication year
    pub year: Option<u16>,
    /// Journal or venue name
    pub venue: Option<String>,
    /// Volume number
    pub volume: Option<String>,
    /// Issue or number
    pub number: Option<String>,
    /// Page range
    pub pages: Option<String>,
    /// DOI (Digital Object Identifier)
    pub doi: Option<String>,
    /// URL
    pub url: Option<String>,
}

impl Citation {
    /// Create a new citation with required fields
    pub fn new(id: String, title: String, authors: Vec<String>) -> Self {
        Self {
            id,
            title,
            authors,
            year: None,
            venue: None,
            volume: None,
            number: None,
            pages: None,
            doi: None,
            url: None,
        }
    }

    /// Format the citation in APA style
    pub fn format_apa(&self) -> String {
        let mut parts = Vec::new();

        // Authors
        if !self.authors.is_empty() {
            let authors = if self.authors.len() == 1 {
                self.authors[0].clone()
            } else if self.authors.len() == 2 {
                format!("{} & {}", self.authors[0], self.authors[1])
            } else {
                format!("{}, et al.", self.authors[0])
            };
            parts.push(authors);
        }

        // Year
        if let Some(year) = self.year {
            parts.push(format!("({})", year));
        }

        // Title
        parts.push(format!("{}.", self.title));

        // Venue
        if let Some(venue) = &self.venue {
            parts.push(format!("*{}*", venue));
        }

        parts.join(" ")
    }

    /// Validate that the citation has all required fields
    pub fn validate(&self) -> Result<(), CitationError> {
        if self.id.is_empty() {
            return Err(CitationError::MissingField("id".to_string()));
        }
        if self.title.is_empty() {
            return Err(CitationError::MissingField("title".to_string()));
        }
        if self.authors.is_empty() {
            return Err(CitationError::MissingField("authors".to_string()));
        }
        Ok(())
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
        citation.validate()?;

        // Check for duplicate IDs
        if self.citations.iter().any(|c| c.id == citation.id) {
            return Err(CitationError::InvalidFormat(format!(
                "Citation with ID '{}' already exists",
                citation.id
            )));
        }

        self.citations.push(citation);
        Ok(())
    }

    /// Get a citation by ID
    pub fn get_citation(&self, id: &str) -> Option<&Citation> {
        self.citations.iter().find(|c| c.id == id)
    }

    /// Get all citations
    pub fn citations(&self) -> &[Citation] {
        &self.citations
    }

    /// Sort citations by author's last name
    pub fn sort_by_author(&mut self) {
        self.citations.sort_by(|a, b| {
            let a_author = a.authors.first().unwrap_or(&String::new()).clone();
            let b_author = b.authors.first().unwrap_or(&String::new()).clone();
            a_author.cmp(&b_author)
        });
    }

    /// Sort citations by year (descending)
    pub fn sort_by_year(&mut self) {
        self.citations
            .sort_by(|a, b| b.year.unwrap_or(0).cmp(&a.year.unwrap_or(0)));
    }
}

impl Default for Bibliography {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_citation_creation() {
        let citation = Citation::new(
            "cv_algo_practice".to_string(),
            "algo_practice".to_string(),
            vec!["Colin VanDervoort".to_string()],
        );

        assert_eq!(citation.id, "cv_algo_practice");
        assert_eq!(citation.title, "algo_practice");
        assert_eq!(citation.authors, vec!["Colin VanDervoort"]);
    }

    #[test]
    fn test_citation_validation() {
        let valid_citation = Citation::new(
            "test".to_string(),
            "Test Title".to_string(),
            vec!["Test Author".to_string()],
        );

        assert!(valid_citation.validate().is_ok());

        let invalid_citation = Citation::new(
            "".to_string(),
            "Test Title".to_string(),
            vec!["Test Author".to_string()],
        );

        assert!(invalid_citation.validate().is_err());
    }

    #[test]
    fn test_apa_formatting() {
        let mut citation = Citation::new(
            "test".to_string(),
            "A Great Paper".to_string(),
            vec!["Smith, J.".to_string()],
        );
        citation.year = Some(2023);
        citation.venue = Some("Journal of Testing".to_string());

        let formatted = citation.format_apa();
        assert!(formatted.contains("Smith, J."));
        assert!(formatted.contains("(2023)"));
        assert!(formatted.contains("A Great Paper"));
        assert!(formatted.contains("*Journal of Testing*"));
    }

    #[test]
    fn test_bibliography() {
        let mut bib = Bibliography::new();

        let citation = Citation::new(
            "test".to_string(),
            "Test Title".to_string(),
            vec!["Test Author".to_string()],
        );

        assert!(bib.add_citation(citation).is_ok());
        assert_eq!(bib.citations().len(), 1);

        let found = bib.get_citation("test");
        assert!(found.is_some());
        assert_eq!(found.unwrap().title, "Test Title");
    }
}
