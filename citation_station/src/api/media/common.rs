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
