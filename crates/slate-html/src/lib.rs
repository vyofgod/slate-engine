//! # Slate HTML5 Parser
//!
//! Full HTML5 spec-compliant parser with error recovery, quirks mode,
//! and streaming support.

use slate_dispatcher::OwnedWebCall;

pub mod parser;
pub mod tree;
pub mod html5_parser;

pub use parser::{HtmlParser, ParseOptions};
pub use tree::{DomTree, Element, Node, NodeData};
pub use html5_parser::Html5Parser;

/// HTML parsing result.
#[derive(Debug)]
pub struct ParseResult {
    pub tree: DomTree,
    pub web_calls: Vec<OwnedWebCall>,
}

/// HTML document mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentMode {
    NoQuirks,
    LimitedQuirks,
    Quirks,
}

/// HTML namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Namespace {
    Html,
    Svg,
    MathML,
}

/// HTML parsing errors.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("invalid UTF-8")]
    InvalidUtf8,

    #[error("unexpected end of input")]
    UnexpectedEof,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_html() {
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head><title>Test</title></head>
                <body><div>Hello</div></body>
            </html>
        "#;

        let mut parser = HtmlParser::new();
        let result = parser.parse(html).unwrap();

        assert!(!result.web_calls.is_empty());
    }
}
