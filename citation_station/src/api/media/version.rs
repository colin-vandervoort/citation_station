use ordinal::ToOrdinal as _;
use serde::{Deserialize, Serialize};

use crate::unicode::EMDASH;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenericMediaVersion {
    DigitalEdition { number: u16 },
    Edition { number: u16 },
    Volume { number: u16 },
    VolumeRange { start: u16, end: u16 },
}

impl GenericMediaVersion {
    pub fn as_ieee_string(&self) -> String {
        match self {
            GenericMediaVersion::DigitalEdition { number } => {
                format!("{} digital ed.", number.to_ordinal_string())
            }
            GenericMediaVersion::Edition { number } => format!("{} ed.", number.to_ordinal_string()),
            GenericMediaVersion::Volume { number } => format!("vol. {}", number),
            GenericMediaVersion::VolumeRange { start, end } => format!("vols. {}{}{}", start, EMDASH, end),
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            GenericMediaVersion::DigitalEdition { number } => {
                format!("({} digital ed.)", number.to_ordinal_string())
            }
            GenericMediaVersion::Edition { number } => format!("({} ed.)", number.to_ordinal_string()),
            GenericMediaVersion::Volume { number } => format!("(Vol. {})", number),
            GenericMediaVersion::VolumeRange { start, end } => {
                format!("(Vols. {}{}{})", start, EMDASH, end)
            }
        }
    }
}
