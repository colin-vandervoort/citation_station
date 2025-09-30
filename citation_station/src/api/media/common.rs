use serde::{Deserialize, Serialize};

use crate::api::{author::Author, date::PublishDate};

/// Data that is shared between all types of sources
///
/// The title of sources is not stored here because some types of
/// media have additional data associated (e.g. conference names)
/// and media-dependent formatting rules
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonCitationData {
    /// Unique identifier for the citation
    pub id: String,
    /// Source author
    pub author: Author,
    /// Date published
    pub published: Option<PublishDate>,
}
