use scraper::{Html, Selector};

pub struct WebsiteTitle {
    from_title_tag: Option<String>,
    // from_og_title: Option<String>,
    // from_schema_thing_headline: Option<String>,
}

pub struct HtmlParser {
    title_selector: Selector,
}

impl HtmlParser {
    pub fn new() -> Self {
        let title_selector = Selector::parse("title").unwrap();

        Self { title_selector }
    }

    pub fn parse_title(&self, html: Html) -> WebsiteTitle {
        let html_title = html
            .select(&self.title_selector)
            .into_iter()
            .take(1)
            .next()
            .and_then(|title_tag| Some(title_tag.inner_html().to_string()));

        WebsiteTitle {
            from_title_tag: html_title,
            // from_og_title: None,
            // from_schema_thing_headline: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use crate::html::HtmlParser;

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
