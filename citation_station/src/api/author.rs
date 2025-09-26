use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::api::errors::NameError;

fn first_grapheme_from_str(s: &str) -> Option<&str> {
    UnicodeSegmentation::graphemes(s, true).take(1).next()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthorName {
    SurnameOnly {
        surname: String,
    },
    SurnameAndFirstName {
        surname: String,
        first_name: String,
    },
    SurnameAndFirstNameAndMiddleName {
        surname: String,
        first_name: String,
        middle_name: String,
    }
}

impl AuthorName {
    pub fn from_first_middle_last(first: &str, middle: &str, last: &str) -> Result<AuthorName, NameError> {
        if first.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if middle.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(AuthorName::SurnameAndFirstNameAndMiddleName {
            first_name: first.to_string(),
            middle_name: middle.to_string(),
            surname: last.to_string(),
        })
    }

    pub fn from_first_last(first: &str, last: &str) -> Result<AuthorName, NameError> {
        if first.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(AuthorName::SurnameAndFirstName {
            first_name: first.to_string(),
            surname: last.to_string(),
        })
    }

    pub fn from_last(last: &str) -> Result<AuthorName, NameError> {
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(AuthorName::SurnameOnly {
            surname: last.to_string(),
        })
    }

    pub fn as_ieee_string(&self) -> String {
        match self {
            AuthorName::SurnameOnly { surname } => surname.clone(),
            AuthorName::SurnameAndFirstName { surname, first_name } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                if let Some(first_initial) = maybe_first_initial {
                    format!("{}. {}", first_initial, surname)
                } else {
                    surname.clone()
                }
            }
            AuthorName::SurnameAndFirstNameAndMiddleName { surname, first_name, middle_name } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                let maybe_middle_initial = first_grapheme_from_str(middle_name);
                match (maybe_first_initial, maybe_middle_initial) {
                    (None, None) => surname.clone(),
                    (None, Some(_middle_initial)) => panic!(),
                    (Some(_first_initial), None) => panic!(),
                    (Some(first_initial), Some(middle_initial)) => format!("{}. {}. {}", first_initial, middle_initial, surname)
                }
            }
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            AuthorName::SurnameOnly { surname } => surname.clone(),
            AuthorName::SurnameAndFirstName { surname, first_name } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                if let Some(first_initial) = maybe_first_initial {
                    format!("{}, {}.", surname, first_initial)
                } else {
                    surname.clone()
                }
            }
            AuthorName::SurnameAndFirstNameAndMiddleName { surname, first_name, middle_name } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                let maybe_middle_initial = first_grapheme_from_str(middle_name);
                match (maybe_first_initial, maybe_middle_initial) {
                    (None, None) => surname.clone(),
                    (None, Some(_middle_initial)) => panic!(),
                    (Some(_first_initial), None) => panic!(),
                    (Some(first_initial), Some(middle_initial)) => format!("{}, {}.{}.", surname, first_initial, middle_initial)
                }
            }
        }
    }
}
