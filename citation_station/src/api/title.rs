use std::str::FromStr;

use serde::{Deserialize, Serialize};

const LEFT_QUOTE: char = '\u{201C}';
const RIGHT_QUOTE: char = '\u{201D}';

#[derive(Debug, Serialize, Deserialize)]
pub struct BookTitle {
    title: String,
}

impl BookTitle {
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBookTitleError;

impl FromStr for BookTitle {
    type Err = ParseBookTitleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(BookTitle {
                title: s.to_string(),
            })
        } else {
            Err(Self::Err {})
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookChapterTitle {
    title: String,
}

impl BookChapterTitle {
    pub fn title(&self) -> String {
        self.title.clone()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseBookChapterTitleError;

impl FromStr for BookChapterTitle {
    type Err = ParseBookChapterTitleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.is_empty() {
            Ok(BookChapterTitle {
                title: s.to_string(),
            })
        } else {
            Err(Self::Err {})
        }
    }
}

pub enum SourceName {
    BookTitle(BookTitle),
    ConferenceName,
}

impl SourceName {
    pub fn as_asciidoc_string(&self) -> String {
        let title = match self {
            SourceName::BookTitle(book_title) => &book_title.title,
            SourceName::ConferenceName => todo!(),
        };
        format!("{}{}{}", LEFT_QUOTE, title, RIGHT_QUOTE)
    }

    pub fn as_markdown_string(&self) -> String {
        let title = match self {
            SourceName::BookTitle(book_title) => &book_title.title,
            SourceName::ConferenceName => todo!(),
        };
        format!("{}{}{}", LEFT_QUOTE, title, RIGHT_QUOTE)
    }
}

pub enum SourceComponent {
    BookChapterTitle(BookChapterTitle),
    ConferencePaperTitle,
}

impl SourceComponent {
    pub fn as_asciidoc_string(&self) -> String {
        let title = match self {
            SourceComponent::BookChapterTitle(book_chapter_title) => &book_chapter_title.title,
            SourceComponent::ConferencePaperTitle => todo!(),
        };
        format!("{}{}{}", LEFT_QUOTE, title, RIGHT_QUOTE)
    }

    pub fn as_markdown_string(&self) -> String {
        let title = match self {
            SourceComponent::BookChapterTitle(book_chapter_title) => &book_chapter_title.title,
            SourceComponent::ConferencePaperTitle => todo!(),
        };
        format!("{}{}{}", LEFT_QUOTE, title, RIGHT_QUOTE)
    }
}