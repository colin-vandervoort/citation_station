use serde::{Deserialize, Serialize};

use crate::api::date::PublishDate;

/// Data that is shared between all types of sources.
///
/// The title of sources is not stored here because some types of
/// media have additional data associated (e.g. conference names)
/// and media-dependent formatting rules.
///
/// Similarly, different types of media have different conventions
/// around author attribution and formatting.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CommonCitationData {
    /// Unique identifier for the citation
    pub id: String,
    /// Date published
    pub published: Option<PublishDate>,
}
