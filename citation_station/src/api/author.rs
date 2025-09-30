use serde::{Deserialize, Serialize};
use unicode_segmentation::UnicodeSegmentation;

use crate::api::errors::NameError;

const IEEE_ET_AL_CUTOFF: usize = 6;

fn first_grapheme_from_str(s: &str) -> Option<&str> {
    UnicodeSegmentation::graphemes(s, true).take(1).next()
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PersonName {
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
    },
}

impl PersonName {
    pub fn from_first_middle_last(
        first: &str,
        middle: &str,
        last: &str,
    ) -> Result<PersonName, NameError> {
        if first.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if middle.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(PersonName::SurnameAndFirstNameAndMiddleName {
            first_name: first.to_string(),
            middle_name: middle.to_string(),
            surname: last.to_string(),
        })
    }

    pub fn from_first_last(first: &str, last: &str) -> Result<PersonName, NameError> {
        if first.len() == 0 {
            return Err(NameError::EmptyString);
        }
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(PersonName::SurnameAndFirstName {
            first_name: first.to_string(),
            surname: last.to_string(),
        })
    }

    pub fn from_last(last: &str) -> Result<PersonName, NameError> {
        if last.len() == 0 {
            return Err(NameError::EmptyString);
        }
        Ok(PersonName::SurnameOnly {
            surname: last.to_string(),
        })
    }

    pub fn as_ieee_string(&self) -> String {
        match self {
            PersonName::SurnameOnly { surname } => surname.clone(),
            PersonName::SurnameAndFirstName {
                surname,
                first_name,
            } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                if let Some(first_initial) = maybe_first_initial {
                    format!("{}. {}", first_initial, surname)
                } else {
                    surname.clone()
                }
            }
            PersonName::SurnameAndFirstNameAndMiddleName {
                surname,
                first_name,
                middle_name,
            } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                let maybe_middle_initial = first_grapheme_from_str(middle_name);
                match (maybe_first_initial, maybe_middle_initial) {
                    (None, None) => surname.clone(),
                    (None, Some(_middle_initial)) => panic!(),
                    (Some(_first_initial), None) => panic!(),
                    (Some(first_initial), Some(middle_initial)) => {
                        format!("{}. {}. {}", first_initial, middle_initial, surname)
                    }
                }
            }
        }
    }

    pub fn as_apa_string(&self) -> String {
        match self {
            PersonName::SurnameOnly { surname } => surname.clone(),
            PersonName::SurnameAndFirstName {
                surname,
                first_name,
            } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                if let Some(first_initial) = maybe_first_initial {
                    format!("{}, {}.", surname, first_initial)
                } else {
                    surname.clone()
                }
            }
            PersonName::SurnameAndFirstNameAndMiddleName {
                surname,
                first_name,
                middle_name,
            } => {
                let maybe_first_initial = first_grapheme_from_str(first_name);
                let maybe_middle_initial = first_grapheme_from_str(middle_name);
                match (maybe_first_initial, maybe_middle_initial) {
                    (None, None) => surname.clone(),
                    (None, Some(_middle_initial)) => panic!(),
                    (Some(_first_initial), None) => panic!(),
                    (Some(first_initial), Some(middle_initial)) => {
                        format!("{}, {}. {}.", surname, first_initial, middle_initial)
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Author {
    Persons { persons: Vec<PersonName> },
    Organization { name: String },
}

impl Author {
    pub fn as_ieee_string(&self) -> Option<String> {
        match self {
            Author::Persons { persons } => match persons.as_slice() {
                [] => None,
                [first] => Some(first.as_ieee_string()),
                [first, second] => Some(format!(
                    "{} and {}",
                    first.as_ieee_string(),
                    second.as_ieee_string()
                )),
                [all @ ..] => {
                    if all.len() > IEEE_ET_AL_CUTOFF {
                        Some(format!("{} et al.", all.first().unwrap().as_ieee_string()))
                    } else {
                        // let mut s = all.into_iter().map(|person| person.as_ieee_string()).collect().join(", ");
                        let mut persons_iter = all.into_iter();
                        let last_person = persons_iter.next_back().unwrap();
                        let persons_except_last = persons_iter
                            .map(|person| person.as_ieee_string())
                            .collect::<Vec<String>>()
                            .join(", ");

                        Some(format!(
                            "{} and {}",
                            persons_except_last,
                            last_person.as_ieee_string()
                        ))
                    }
                }
            },
            Author::Organization { name } => Some(name.clone()),
        }
    }

    pub fn as_apa_string(&self) -> Option<String> {
        match self {
            Author::Persons { persons } => match persons.as_slice() {
                [] => None,
                [first] => Some(first.as_apa_string()),
                [first, second] => Some(format!(
                    "{} & {}",
                    first.as_apa_string(),
                    second.as_apa_string()
                )),
                [all @ ..] => Some(format!("{} et al.", all.first().unwrap().as_apa_string())),
            },
            Author::Organization { name } => Some(name.clone()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Editors {
    persons: Vec<PersonName>,
}

impl Editors {
    pub fn as_ieee_string(&self) -> Option<String> {
        todo!();
    }

    pub fn as_apa_string(&self) -> Option<String> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::api::author::{Author, PersonName};

    #[test]
    fn test_format_person_author_ieee_last_name_only() {
        let author = Author::Persons {
            persons: vec![PersonName::from_last("Doe").unwrap()],
        };

        assert_eq!(author.as_ieee_string(), Some("Doe".to_string()))
    }

    #[test]
    fn test_format_person_author_ieee_first_last() {
        let author = Author::Persons {
            persons: vec![PersonName::from_first_last("Jane", "Doe").unwrap()],
        };

        assert_eq!(author.as_ieee_string(), Some("J. Doe".to_string()))
    }

    #[test]
    fn test_format_person_author_ieee_first_middle_last() {
        let author = Author::Persons {
            persons: vec![PersonName::from_first_middle_last("Jane", "Dilly", "Doe").unwrap()],
        };

        assert_eq!(author.as_ieee_string(), Some("J. D. Doe".to_string()))
    }

    #[test]
    fn test_format_org_author_ieee() {
        let org_name = "The Corporation";
        let author = Author::Organization {
            name: org_name.to_string(),
        };

        assert_eq!(author.as_ieee_string(), Some(org_name.to_string()))
    }

    #[test]
    fn test_format_person_author_apa_last_name_only() {
        let author = Author::Persons {
            persons: vec![PersonName::from_last("Doe").unwrap()],
        };

        assert_eq!(author.as_apa_string(), Some("Doe".to_string()))
    }

    #[test]
    fn test_format_person_author_apa_first_last() {
        let author = Author::Persons {
            persons: vec![PersonName::from_first_last("Jane", "Doe").unwrap()],
        };

        assert_eq!(author.as_apa_string(), Some("Doe, J.".to_string()))
    }

    #[test]
    fn test_format_person_author_apa_first_middle_last() {
        let author = Author::Persons {
            persons: vec![PersonName::from_first_middle_last("Jane", "Dilly", "Doe").unwrap()],
        };

        assert_eq!(author.as_apa_string(), Some("Doe, J. D.".to_string()))
    }

    #[test]
    fn test_format_org_author_apa() {
        let org_name = "The Corporation";
        let author = Author::Organization {
            name: org_name.to_string(),
        };

        assert_eq!(author.as_apa_string(), Some(org_name.to_string()))
    }
}
