use serde::{Deserialize, Serialize};

use crate::api::{author::Author, date::PublishDate};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonCitationData {
    /// Unique identifier for the citation
    pub id: String,
    /// Title of the work
    pub title: String,
    /// Source author
    pub author: Author,
    /// Date published
    pub published: Option<PublishDate>,
}
