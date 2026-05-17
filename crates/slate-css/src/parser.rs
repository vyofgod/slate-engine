//! CSS parser using hand-written tokenizer.

use super::{CssRule, CssValue, Declaration, Property, Selector, Specificity, Stylesheet};

/// CSS parser.
pub struct CssParser;

impl CssParser {
    /// Parse a complete stylesheet.
    pub fn parse_stylesheet(css: &str) -> Result<Stylesheet, ParseError> {
        let mut stylesheet = Stylesheet::new();
        let mut tokenizer = Tokenizer::new(css);

        while tokenizer.pos < css.len() {
            tokenizer.skip_whitespace();
            if tokenizer.pos >= css.len() {
                break;
            }

            // Parse selector
            let selector_str = tokenizer.read_until('{');
            tokenizer.advance(); // Skip '{'

            // Parse declarations
            let decl_block = tokenizer.read_until('}');
            tokenizer.advance(); // Skip '}'

            let declarations = Self::parse_declarations(&decl_block)?;

            // Create rule
            let selector = Self::parse_selector(&selector_str);
            let specificity = Specificity::calculate(&selector);

            stylesheet.add_rule(CssRule {
                selector,
                declarations,
                specificity,
            });
        }

        Ok(stylesheet)
    }

    /// Parse a selector (simplified).
    fn parse_selector(s: &str) -> Selector {
        let s = s.trim();

        // ID selector
        if let Some(id) = s.strip_prefix('#') {
            return Selector::Id(id.to_string());
        }

        // Class selector
        if let Some(class) = s.strip_prefix('.') {
            return Selector::Class(class.to_string());
        }

        // Universal selector
        if s == "*" {
            return Selector::Universal;
        }

        // Type selector
        Selector::Type(s.to_string())
    }

    /// Parse inline style declarations.
    pub fn parse_inline_style(css: &str) -> Result<Vec<Declaration>, ParseError> {
        Self::parse_declarations(css)
    }

    /// Parse CSS declarations.
    fn parse_declarations(css: &str) -> Result<Vec<Declaration>, ParseError> {
        let mut declarations = Vec::new();

        for part in css.split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }

            if let Some((prop, value)) = part.split_once(':') {
                let prop = prop.trim();
                let value = value.trim();

                // Check for !important
                let (value, important) = if let Some(v) = value.strip_suffix("!important") {
                    (v.trim(), true)
                } else {
                    (value, false)
                };

                let property = Property::from_name(prop);
                let css_value = Self::parse_value(value)?;

                declarations.push(Declaration {
                    property,
                    value: css_value,
                    important,
                });
            }
        }

        Ok(declarations)
    }

    /// Parse a CSS value.
    fn parse_value(s: &str) -> Result<CssValue, ParseError> {
        let s = s.trim();

        // Keywords
        match s {
            "auto" => return Ok(CssValue::Keyword(super::values::Keyword::Auto)),
            "none" => return Ok(CssValue::Keyword(super::values::Keyword::None)),
            "normal" => return Ok(CssValue::Keyword(super::values::Keyword::Normal)),
            "inherit" => return Ok(CssValue::Inherit),
            "initial" => return Ok(CssValue::Initial),
            "unset" => return Ok(CssValue::Unset),
            _ => {}
        }

        // Try color
        if let Some(color) = CssValue::parse_color(s) {
            return Ok(color);
        }

        // Try length
        if let Some(length) = CssValue::parse_length(s) {
            return Ok(length);
        }

        // Try number
        if let Ok(num) = s.parse::<f32>() {
            return Ok(CssValue::Number(num));
        }

        // Multiple values (space-separated)
        if s.contains(' ') {
            let values: Result<Vec<_>, _> = s
                .split_whitespace()
                .map(|part| Self::parse_value(part))
                .collect();
            return Ok(CssValue::List(values?));
        }

        // Fallback to string
        Ok(CssValue::String(s.to_string()))
    }
}

/// CSS parsing errors.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("invalid syntax: {0}")]
    InvalidSyntax(String),

    #[error("unknown property: {0}")]
    UnknownProperty(String),
}

/// Simple tokenizer for CSS.
struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        while self.pos < self.input.len() {
            let ch = self.input.as_bytes()[self.pos];
            if !ch.is_ascii_whitespace() {
                break;
            }
            self.pos += 1;
        }
    }

    fn read_until(&mut self, delimiter: char) -> String {
        let start = self.pos;
        while self.pos < self.input.len() {
            if self.input.as_bytes()[self.pos] == delimiter as u8 {
                break;
            }
            self.pos += 1;
        }
        self.input[start..self.pos].to_string()
    }

    fn advance(&mut self) {
        if self.pos < self.input.len() {
            self.pos += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_inline_style() {
        let css = "color: red; font-size: 16px; margin: 10px 20px";
        let decls = CssParser::parse_inline_style(css).unwrap();

        assert_eq!(decls.len(), 3);
        assert_eq!(decls[0].property, Property::Color);
    }

    #[test]
    fn parse_stylesheet() {
        let css = r#"
            .button {
                background-color: #3498db;
                color: white;
                padding: 10px 20px;
            }
            
            #header {
                font-size: 24px;
            }
        "#;

        let stylesheet = CssParser::parse_stylesheet(css).unwrap();
        assert_eq!(stylesheet.rules.len(), 2);
    }

    #[test]
    fn parse_important() {
        let css = "color: red !important";
        let decls = CssParser::parse_inline_style(css).unwrap();

        assert_eq!(decls.len(), 1);
        assert!(decls[0].important);
    }
}
