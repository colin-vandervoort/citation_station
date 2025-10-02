use ordinal::ToOrdinal as _;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::unicode::EMDASH;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub enum SemVer {
    Major { major: u32 },
    MajorMinor { major: u32, minor: u32 },
    MajorMinorPatch { major: u32, minor: u32, patch: u32 },
}

impl SemVer {
    pub fn from_major(major: u32) -> Self {
        SemVer::Major { major }
    }

    pub fn from_major_minor(major: u32, minor: u32) -> Self {
        SemVer::MajorMinor { major, minor }
    }

    pub fn from_major_minor_patch(major: u32, minor: u32, patch: u32) -> Self {
        SemVer::MajorMinorPatch {
            major,
            minor,
            patch,
        }
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SemVer::Major { major } => write!(f, "{}", major),
            SemVer::MajorMinor { major, minor } => write!(f, "{}.{}", major, minor),
            SemVer::MajorMinorPatch {
                major,
                minor,
                patch,
            } => write!(f, "{}.{}.{}", major, minor, patch),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GenericMediaVersion {
    DigitalEdition { number: u16 },
    Edition { number: u16 },
    SemVer(SemVer),
    Volume { number: u16 },
    VolumeRange { start: u16, end: u16 },
}

impl GenericMediaVersion {
    pub fn as_ieee_string(&self) -> String {
        match self {
            GenericMediaVersion::DigitalEdition { number } => {
                format!("{} digital ed.", number.to_ordinal_string())
            }
            GenericMediaVersion::Edition { number } => {
                format!("{} ed.", number.to_ordinal_string())
            }
            GenericMediaVersion::SemVer(sem_ver) => format!("v{}", sem_ver),
            GenericMediaVersion::Volume { number } => format!("vol. {}", number),
            GenericMediaVersion::VolumeRange { start, end } => {
                format!("vols. {}{}{}", start, EMDASH, end)
            }
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            GenericMediaVersion::DigitalEdition { number } => {
                format!("({} digital ed.)", number.to_ordinal_string())
            }
            GenericMediaVersion::Edition { number } => {
                format!("({} ed.)", number.to_ordinal_string())
            }
            GenericMediaVersion::SemVer(sem_ver) => format!("(v{})", sem_ver),
            GenericMediaVersion::Volume { number } => format!("(Vol. {})", number),
            GenericMediaVersion::VolumeRange { start, end } => {
                format!("(Vols. {}{}{})", start, EMDASH, end)
            }
        }
    }
}
