use ordinal::ToOrdinal as _;
use serde::{Deserialize, Serialize};

use crate::unicode::EMDASH;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BookVersion {
    DigitalEdition { number: u16 },
    Edition { number: u16 },
    Volume { number: u16 },
    VolumeRange { start: u16, end: u16 },
}

impl BookVersion {
    pub fn as_ieee_string(&self) -> String {
        match self {
            BookVersion::DigitalEdition { number } => {
                format!("{} digital ed.", number.to_ordinal_string())
            }
            BookVersion::Edition { number } => format!("{} ed.", number.to_ordinal_string()),
            BookVersion::Volume { number } => format!("vol. {}", number),
            BookVersion::VolumeRange { start, end } => format!("vols. {}{}{}", start, EMDASH, end),
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            BookVersion::DigitalEdition { number } => {
                format!("({} digital ed.)", number.to_ordinal_string())
            }
            BookVersion::Edition { number } => format!("({} ed.)", number.to_ordinal_string()),
            BookVersion::Volume { number } => format!("(Vol. {})", number),
            BookVersion::VolumeRange { start, end } => {
                format!("(Vols. {}{}{})", start, EMDASH, end)
            }
        }
    }
}
