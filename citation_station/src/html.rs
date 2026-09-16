use scraper::{Html, Selector};

pub struct WebpageTitle {
    from_title_tag: Option<String>,
    from_og_title: Option<String>,
    from_schema_thing_headline: Option<String>,
}

impl WebpageTitle {
    pub fn all_title_sources_match(&self) -> bool {
        let titles: Vec<&String> = vec![
            &self.from_title_tag,
            &self.from_og_title,
            &self.from_schema_thing_headline,
        ]
        .into_iter()
        .flatten()
        .collect();

        titles.windows(2).all(|pair| pair[0] == pair[1])
    }
}

pub struct HtmlParser {
    title_selector: Selector,
}

impl HtmlParser {
    pub fn new() -> Self {
        let title_selector = Selector::parse("title").unwrap();

        Self { title_selector }
    }

    pub fn parse_title(&self, html: Html) -> WebpageTitle {
        let html_title = html
            .select(&self.title_selector)
            .into_iter()
            .take(1)
            .next()
            .and_then(|title_tag| Some(title_tag.inner_html().to_string()));

        WebpageTitle {
            from_title_tag: html_title,
            from_og_title: None,
            from_schema_thing_headline: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use crate::html::{HtmlParser, WebpageTitle};

    #[test]
    fn test_all_title_sources_match() {
        let cases: Vec<(&str, WebpageTitle, bool)> = vec![
            (
                "all none",
                WebpageTitle {
                    from_title_tag: None,
                    from_og_title: None,
                    from_schema_thing_headline: None,
                },
                true,
            ),
            (
                "single source",
                WebpageTitle {
                    from_title_tag: None,
                    from_og_title: Some("The Solar System".to_string()),
                    from_schema_thing_headline: None,
                },
                true,
            ),
            (
                "two sources match",
                WebpageTitle {
                    from_title_tag: Some("The Solar System".to_string()),
                    from_og_title: Some("The Solar System".to_string()),
                    from_schema_thing_headline: None,
                },
                true,
            ),
            (
                "two sources mismatch",
                WebpageTitle {
                    from_title_tag: Some("The Solar System".to_string()),
                    from_og_title: Some("Solar System Facts".to_string()),
                    from_schema_thing_headline: None,
                },
                false,
            ),
            (
                "three sources match",
                WebpageTitle {
                    from_title_tag: Some("The Solar System".to_string()),
                    from_og_title: Some("The Solar System".to_string()),
                    from_schema_thing_headline: Some("The Solar System".to_string()),
                },
                true,
            ),
            (
                "three sources mismatch",
                WebpageTitle {
                    from_title_tag: Some("The Solar System".to_string()),
                    from_og_title: Some("The Solar System".to_string()),
                    from_schema_thing_headline: Some("Solar System Facts".to_string()),
                },
                false,
            ),
        ];

        for (name, webpage_title, expected) in cases {
            assert_eq!(
                webpage_title.all_title_sources_match(),
                expected,
                "case failed: {name}"
            );
        }
    }

    #[test]
    fn test_parse_title_missing() {
        let html_str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
</head>
</html>
"#;
        let html = Html::parse_document(html_str);
        let html_parser = HtmlParser::new();

        let title = html_parser.parse_title(html);

        assert_eq!(title.from_title_tag, None);
    }

    #[test]
    fn test_parse_invalid_html_two_titles() {
        let html_str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>First</title>
    <title>Second</title>
</head>
</html>
"#;
        let html = Html::parse_document(html_str);
        let html_parser = HtmlParser::new();

        let title = html_parser.parse_title(html);

        assert_eq!(title.from_title_tag, Some("First".to_string()));
    }

    #[test]
    fn test_parse_title() {
        let html_str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Document</title>
</head>
</html>
"#;
        let html = Html::parse_document(html_str);
        let html_parser = HtmlParser::new();

        let title = html_parser.parse_title(html);

        assert_eq!(title.from_title_tag, Some("Document".to_string()))
    }
}
