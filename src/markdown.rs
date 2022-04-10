
use pulldown_cmark::{html, Options, Parser};

pub fn parse_markdown_to_html(markdown: &str) -> String {
    let mut output = String::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    html::push_html(&mut output, Parser::new_ext(markdown, options));
    output
}

#[cfg(test)]
mod tests {
    use super::parse_markdown_to_html;

    #[test]
    fn simple_test_that_markdown_parser_seems_to_work() {
        let markdown_input = "Hello world, this is a ~~complicated~~ *very simple* example.";
        let html_output = parse_markdown_to_html(&markdown_input);
        let expected_html =
            "<p>Hello world, this is a <del>complicated</del> <em>very simple</em> example.</p>\n";
        assert_eq!(expected_html, &html_output);
    }
}
