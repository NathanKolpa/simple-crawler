use select::document::Document;
use select::predicate::Name;

pub fn get_links(html: &str) -> Vec<String> {
    let document = Document::from(html);
    document
        .find(Name("a"))
        .map(|e| e.attr("href").and_then(|x| Some(String::from(x))))
        .flatten()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_html_document() {
        let html = "
        <!DOCTYPE>
        <html>
            <head>
            </head>
            <body>
            </body>
        </html>
            ";

        let links = get_links(html);

        assert_eq!(0, links.len());
    }

    #[test]
    fn flat_html_document_with_one_link() {
        let html = "
        <!DOCTYPE>
        <html>
            <head>
            </head>
            <body>
                <a href=\"https://google.com\"></a>
            </body>
        </html>
            ";

        let links = get_links(html);

        assert_eq!(1, links.len());
        assert_eq!("https://google.com", links.first().unwrap());
    }

    #[test]
    fn flat_html_document_with_multiple_link() {
        let html = "
        <!DOCTYPE>
        <html>
            <head>
            </head>
            <body>
                <a href=\"https://google.com\"></a>
                <a href=\"http://stackoverflow.com\"></a>
                <a href=\"https://getmonero.com\"></a>
                <a href=\"https://fsf.org\"></a>
            </body>
        </html>
            ";

        let links = get_links(html);

        assert_eq!(4, links.len());
        assert_eq!(
            vec![
                "https://google.com",
                "https://stackoverflow.com",
                "https://getmonero.com,",
                "https://fsf.org,",
            ]
            .sort(),
            links.clone().sort()
        );
    }
}
