//! Real CSS parser using cssparser crate.

use crate::{CssRule, Declaration, Property, Selector, Specificity, Stylesheet};
use crate::values::{CssValue, Unit, Keyword};
use cssparser::{Parser, ParserInput, Token};
use slate_ais::{Rgba8, SubPixel};
use std::fmt;

/// CSS parsing error.
#[derive(Debug)]
pub struct ParseError {
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CSS parse error: {}", self.message)
    }
}

impl std::error::Error for ParseError {}

/// Real CSS parser using cssparser.
pub struct CssParserReal {
    // Stateless
}

impl CssParserReal {
    /// Create a new CSS parser.
    pub fn new() -> Self {
        Self {}
    }

    /// Parse a CSS stylesheet.
    pub fn parse_stylesheet(&self, css: &str) -> Result<Stylesheet, ParseError> {
        let mut input = ParserInput::new(css);
        let mut parser = Parser::new(&mut input);
        
        let mut rules = Vec::new();

        // Skip whitespace and comments
        while !parser.is_exhausted() {
            parser.skip_whitespace();
            
            if parser.is_exhausted() {
                break;
            }

            // Try to parse a rule
            match self.parse_rule(&mut parser) {
                Ok(rule) => rules.push(rule),
                Err(_) => {
                    // Skip to next rule on error
                    while !parser.is_exhausted() {
                        if let Ok(Token::CurlyBracketBlock) = parser.next() {
                            let _ = parser.parse_nested_block::<_, _, ()>(|_| Ok(()));
                            break;
                        }
                    }
                }
            }
        }

        Ok(Stylesheet { rules })
    }

    /// Parse a single CSS rule.
    fn parse_rule<'i, 't>(
        &self,
        parser: &mut Parser<'i, 't>,
    ) -> Result<CssRule, ParseError> {
        // Parse selector
        let selector = self.parse_selector(parser)?;
        let specificity = self.calculate_specificity(&selector);

        // Expect opening brace
        parser.expect_curly_bracket_block()
            .map_err(|e| ParseError { message: format!("Expected {{: {:?}", e) })?;

        // Parse declarations
        let declarations = match parser.parse_nested_block(|parser| {
            let mut decls = Vec::new();
            while !parser.is_exhausted() {
                parser.skip_whitespace();
                if parser.is_exhausted() {
                    break;
                }
                
                // Parse property name
                let property_name = match parser.next() {
                    Ok(Token::Ident(name)) => name.to_string(),
                    Ok(_) => continue,
                    Err(_) => break,
                };
                
                // Expect colon
                if parser.expect_colon().is_err() {
                    continue;
                }
                
                parser.skip_whitespace();
                
                // Parse value
                let value = match self.parse_value(parser) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                
                // Check for !important
                let important = self.check_important(parser);
                
                // Expect semicolon or end
                let _ = parser.expect_semicolon();
                
                let property = Property::from_name(&property_name);
                decls.push(Declaration {
                    property,
                    value,
                    important,
                });
            }
            Ok::<_, cssparser::ParseError<()>>(decls)
        }) {
            Ok(decls) => decls,
            Err(_) => return Err(ParseError { message: "Failed to parse declarations".to_string() }),
        };

        Ok(CssRule {
            selector,
            declarations,
            specificity,
        })
    }

    /// Parse a CSS selector (simplified).
    fn parse_selector<'i, 't>(
        &self,
        parser: &mut Parser<'i, 't>,
    ) -> Result<Selector, ParseError> {
        parser.skip_whitespace();

        let token = parser.next()
            .map_err(|e| ParseError { message: format!("Expected selector: {:?}", e) })?;

        match token {
            Token::Delim('*') => Ok(Selector::Universal),
            Token::Delim('.') => {
                // Class selector
                if let Ok(Token::Ident(class)) = parser.next() {
                    Ok(Selector::Class(class.to_string()))
                } else {
                    Err(ParseError { message: "Expected class name".to_string() })
                }
            }
            Token::IDHash(id) | Token::Hash(id) => {
                Ok(Selector::Id(id.to_string()))
            }
            Token::Ident(name) => {
                Ok(Selector::Type(name.to_string()))
            }
            _ => Err(ParseError { message: format!("Unexpected token in selector: {:?}", token) }),
        }
    }

    /// Parse a CSS value.
    fn parse_value<'i, 't>(
        &self,
        parser: &mut Parser<'i, 't>,
    ) -> Result<CssValue, ParseError> {
        parser.skip_whitespace();

        let token = parser.next()
            .map_err(|e| ParseError { message: format!("Expected value: {:?}", e) })?;

        match token {
            Token::Ident(keyword) => {
                // Parse keyword
                let kw = match keyword.as_ref() {
                    "auto" => Keyword::Auto,
                    "none" => Keyword::None,
                    "normal" => Keyword::Normal,
                    "inherit" => Keyword::Inherit,
                    "initial" => Keyword::Initial,
                    "unset" => Keyword::Unset,
                    _ => return Ok(CssValue::String(keyword.to_string())),
                };
                Ok(CssValue::Keyword(kw))
            }
            Token::Number { value, .. } => {
                Ok(CssValue::Number(*value))
            }
            Token::Percentage { unit_value, .. } => {
                Ok(CssValue::Percentage(*unit_value * 100.0))
            }
            Token::Dimension { value, unit, .. } => {
                let unit_enum = match unit.as_ref() {
                    "px" => Unit::Px,
                    "em" => Unit::Em,
                    "rem" => Unit::Rem,
                    "vh" => Unit::Vh,
                    "vw" => Unit::Vw,
                    "%" => Unit::Percent,
                    _ => Unit::Px,
                };
                Ok(CssValue::Length(SubPixel(*value), unit_enum))
            }
            Token::Hash(hex) | Token::IDHash(hex) => {
                // Parse hex color
                if let Some(color) = self.parse_hex_color(hex) {
                    Ok(CssValue::Color(color))
                } else {
                    Ok(CssValue::String(hex.to_string()))
                }
            }
            Token::QuotedString(s) => {
                Ok(CssValue::String(s.to_string()))
            }
            _ => Err(ParseError { message: format!("Unexpected value token: {:?}", token) }),
        }
    }

    /// Parse hex color.
    fn parse_hex_color(&self, hex: &str) -> Option<Rgba8> {
        let hex = hex.trim_start_matches('#');
        
        match hex.len() {
            3 => {
                // #RGB -> #RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Rgba8 { r, g, b, a: 255 })
            }
            6 => {
                // #RRGGBB
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Rgba8 { r, g, b, a: 255 })
            }
            8 => {
                // #RRGGBBAA
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
                Some(Rgba8 { r, g, b, a })
            }
            _ => None,
        }
    }

    /// Check for !important flag.
    fn check_important<'i, 't>(&self, parser: &mut Parser<'i, 't>) -> bool {
        parser.skip_whitespace();
        
        if let Ok(Token::Delim('!')) = parser.next() {
            parser.skip_whitespace();
            if let Ok(Token::Ident(ident)) = parser.next() {
                return ident.eq_ignore_ascii_case("important");
            }
        }
        
        false
    }

    /// Calculate selector specificity.
    fn calculate_specificity(&self, selector: &Selector) -> Specificity {
        match selector {
            Selector::Universal => Specificity::new(0, 0, 0, 0),
            Selector::Type(_) => Specificity::new(0, 0, 0, 1),
            Selector::Class(_) => Specificity::new(0, 0, 1, 0),
            Selector::Id(_) => Specificity::new(0, 1, 0, 0),
            Selector::Attribute { .. } => Specificity::new(0, 0, 1, 0),
            Selector::PseudoClass(_) => Specificity::new(0, 0, 1, 0),
            Selector::PseudoElement(_) => Specificity::new(0, 0, 0, 1),
            Selector::Combinator { left, right, .. } => {
                let spec_a = self.calculate_specificity(left);
                let spec_b = self.calculate_specificity(right);
                Specificity::new(
                    spec_a.inline + spec_b.inline,
                    spec_a.ids + spec_b.ids,
                    spec_a.classes + spec_b.classes,
                    spec_a.elements + spec_b.elements,
                )
            }
            Selector::Compound(selectors) => {
                selectors.iter().fold(Specificity::new(0, 0, 0, 0), |acc, sel| {
                    let spec = self.calculate_specificity(sel);
                    Specificity::new(
                        acc.inline + spec.inline,
                        acc.ids + spec.ids,
                        acc.classes + spec.classes,
                        acc.elements + spec.elements,
                    )
                })
            }
        }
    }
}

impl Default for CssParserReal {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_rule() {
        let parser = CssParserReal::new();
        let css = "div { color: red; }";
        let result = parser.parse_stylesheet(css);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_class_selector() {
        let parser = CssParserReal::new();
        let css = ".container { width: 100px; }";
        let result = parser.parse_stylesheet(css);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_id_selector() {
        let parser = CssParserReal::new();
        let css = "#main { height: 50%; }";
        let result = parser.parse_stylesheet(css);
        assert!(result.is_ok());
    }
}
