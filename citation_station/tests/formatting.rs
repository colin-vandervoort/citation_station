// cSpell: ignore Popov

use citation_station::api::{
    author::{Author, PersonName},
    citation::Citation,
    date::PublishDate,
    media::{book::Book, common::CommonCitationData},
};

#[test]
fn test_apa_formatting_minimal() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![PersonName::from_first_last("J", "Smith").unwrap()],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_apa();
    assert_eq!(formatted, "Smith, J. (2023). A Great Paper.");
}

#[test]
fn test_apa_formatting_two_authors() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                ],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_apa();
    assert_eq!(formatted, "Smith, J. & Fuentes, H. (2023). A Great Paper.");
}

#[test]
fn test_apa_formatting_three_authors() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                    PersonName::from_first_last("Isabel", "Popov").unwrap(),
                ],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_apa();
    assert_eq!(formatted, "Smith, J. et al. (2023). A Great Paper.");
}

#[test]
fn test_ieee_formatting_minimal() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![PersonName::from_first_last("J", "Smith").unwrap()],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_ieee();
    assert_eq!(formatted, "J. Smith, A Great Paper. 2023");
}

#[test]
fn test_ieee_formatting_two_authors() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                ],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_ieee();
    assert_eq!(formatted, "J. Smith and H. Fuentes, A Great Paper. 2023");
}

#[test]
fn test_ieee_formatting_three_authors() {
    let citation = Citation::Book(Book {
        common_data: CommonCitationData {
            id: "test".to_string(),
            author: Author::Persons {
                persons: vec![
                    PersonName::from_first_last("J", "Smith").unwrap(),
                    PersonName::from_first_last("Humberto", "Fuentes").unwrap(),
                    PersonName::from_first_last("Isabel", "Popov").unwrap(),
                ],
            },
            published: Some(PublishDate::from_year(2023)),
        },
        title: "A Great Paper".to_string(),
        doi: None,
        pages: None,
        chapter: None,
        version: None,
    });

    let formatted = citation.format_ieee();
    assert_eq!(formatted, "J. Smith, A Great Paper. 2023");
}
