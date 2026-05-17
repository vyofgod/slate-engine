//! HTML5 parser implementation with state machine tokenizer.

use super::{DomTree, Element, Node, NodeData, ParseError, ParseResult};
use slate_ais::NodeId;
use slate_dispatcher::OwnedWebCall;
use std::collections::HashMap;

/// HTML parser options.
#[derive(Debug, Clone)]
pub struct ParseOptions {
    pub scripting_enabled: bool,
    pub iframe_srcdoc: bool,
    pub drop_doctype: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            scripting_enabled: true,
            iframe_srcdoc: false,
            drop_doctype: true,
        }
    }
}

/// HTML5 parser with state machine tokenizer.
pub struct HtmlParser {
    #[allow(dead_code)]
    options: ParseOptions,
    next_node_id: u32,
}

impl HtmlParser {
    /// Create a new HTML parser.
    pub fn new() -> Self {
        Self::with_options(ParseOptions::default())
    }

    /// Create a parser with custom options.
    pub fn with_options(options: ParseOptions) -> Self {
        Self {
            options,
            next_node_id: 0,
        }
    }

    /// Parse HTML into a DOM tree and WebCall sequence.
    pub fn parse(&mut self, html: &str) -> Result<ParseResult, ParseError> {
        let mut tree = DomTree::new();
        let mut web_calls = Vec::new();
        let mut stack: Vec<NodeId> = Vec::new();

        // Tokenize
        let tokens = Tokenizer::new(html).tokenize();

        // Build tree
        for token in tokens {
            match token {
                Token::StartTag { name, attributes, self_closing } => {
                    let node_id = self.allocate_node_id();

                    // Create element
                    web_calls.push(OwnedWebCall::CreateElement {
                        node: node_id,
                        tag: name.clone(),
                    });

                    // Add attributes
                    for (attr_name, attr_value) in &attributes {
                        web_calls.push(OwnedWebCall::SetAttribute {
                            node: node_id,
                            name: attr_name.clone(),
                            value: attr_value.clone(),
                        });
                    }

                    // Add to tree
                    let element = Element {
                        tag_name: name.clone(),
                        attributes: attributes.clone(),
                        namespace: super::Namespace::Html,
                    };

                    tree.add_node(Node {
                        id: node_id,
                        data: NodeData::Element(element),
                        parent: stack.last().copied(),
                        children: Vec::new(),
                    });

                    // Append to parent
                    if let Some(&parent) = stack.last() {
                        tree.append_child(parent, node_id);
                        web_calls.push(OwnedWebCall::AppendChild {
                            parent,
                            child: node_id,
                            index: tree.get_node(parent).map(|n| n.children.len() as u32).unwrap_or(0),
                        });
                    }

                    // Push to stack if not self-closing
                    if !self_closing && !is_void_element(&name) {
                        stack.push(node_id);
                    }
                }

                Token::EndTag { name } => {
                    // Pop from stack
                    if let Some(pos) = stack.iter().rposition(|&id| {
                        tree.get_node(id)
                            .and_then(|n| match &n.data {
                                NodeData::Element(e) => Some(&e.tag_name),
                                _ => None,
                            })
                            .map(|tag| tag == &name)
                            .unwrap_or(false)
                    }) {
                        stack.truncate(pos);
                    }
                }

                Token::Text { data } => {
                    if let Some(&parent) = stack.last() {
                        let node_id = self.allocate_node_id();

                        tree.add_node(Node {
                            id: node_id,
                            data: NodeData::Text(data.clone()),
                            parent: Some(parent),
                            children: Vec::new(),
                        });

                        tree.append_child(parent, node_id);
                    }
                }

                Token::Comment { data } => {
                    if let Some(&parent) = stack.last() {
                        let node_id = self.allocate_node_id();

                        tree.add_node(Node {
                            id: node_id,
                            data: NodeData::Comment(data),
                            parent: Some(parent),
                            children: Vec::new(),
                        });

                        tree.append_child(parent, node_id);
                    }
                }

                Token::Doctype { .. } => {
                    // Skip doctype for now
                }
            }
        }

        Ok(ParseResult { tree, web_calls })
    }

    /// Allocate a new node ID.
    fn allocate_node_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    /// Parse HTML from a stream (for incremental parsing).
    pub fn parse_stream<R: std::io::Read>(&mut self, reader: R) -> Result<ParseResult, ParseError> {
        let mut html = String::new();
        std::io::Read::read_to_string(&mut reader.take(10 * 1024 * 1024), &mut html)?;
        self.parse(&html)
    }
}

impl Default for HtmlParser {
    fn default() -> Self {
        Self::new()
    }
}

/// HTML token.
#[derive(Debug, Clone)]
enum Token {
    StartTag {
        name: String,
        attributes: HashMap<String, String>,
        self_closing: bool,
    },
    EndTag {
        name: String,
    },
    Text {
        data: String,
    },
    Comment {
        data: String,
    },
    Doctype {
        #[allow(dead_code)]
        name: String,
    },
}

/// HTML tokenizer with state machine.
struct Tokenizer<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        Self { input, pos: 0 }
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while self.pos < self.input.len() {
            if self.peek() == Some('<') {
                self.advance();

                if self.peek() == Some('!') {
                    self.advance();
                    if self.input[self.pos..].starts_with("--") {
                        // Comment
                        tokens.push(self.read_comment());
                    } else if self.input[self.pos..].to_uppercase().starts_with("DOCTYPE") {
                        // Doctype
                        tokens.push(self.read_doctype());
                    }
                } else if self.peek() == Some('/') {
                    // End tag
                    self.advance();
                    tokens.push(self.read_end_tag());
                } else {
                    // Start tag
                    tokens.push(self.read_start_tag());
                }
            } else {
                // Text
                tokens.push(self.read_text());
            }
        }

        tokens
    }

    fn read_start_tag(&mut self) -> Token {
        let name = self.read_tag_name();
        let mut attributes = HashMap::new();
        let mut self_closing = false;

        self.skip_whitespace();

        while self.peek().is_some() && self.peek() != Some('>') && self.peek() != Some('/') {
            let attr_name = self.read_attribute_name();
            self.skip_whitespace();

            let attr_value = if self.peek() == Some('=') {
                self.advance();
                self.skip_whitespace();
                self.read_attribute_value()
            } else {
                String::new()
            };

            attributes.insert(attr_name, attr_value);
            self.skip_whitespace();
        }

        if self.peek() == Some('/') {
            self.advance();
            self_closing = true;
        }

        if self.peek() == Some('>') {
            self.advance();
        }

        Token::StartTag {
            name,
            attributes,
            self_closing,
        }
    }

    fn read_end_tag(&mut self) -> Token {
        let name = self.read_tag_name();

        while self.peek().is_some() && self.peek() != Some('>') {
            self.advance();
        }

        if self.peek() == Some('>') {
            self.advance();
        }

        Token::EndTag { name }
    }

    fn read_text(&mut self) -> Token {
        let start = self.pos;

        while self.peek().is_some() && self.peek() != Some('<') {
            self.advance();
        }

        let data = self.input[start..self.pos].to_string();
        Token::Text { data }
    }

    fn read_comment(&mut self) -> Token {
        self.pos += 2; // Skip "--"
        let start = self.pos;

        while self.pos + 2 < self.input.len() {
            if &self.input[self.pos..self.pos + 2] == "--" {
                let data = self.input[start..self.pos].to_string();
                self.pos += 2;
                if self.peek() == Some('>') {
                    self.advance();
                }
                return Token::Comment { data };
            }
            self.advance();
        }

        Token::Comment {
            data: self.input[start..].to_string(),
        }
    }

    fn read_doctype(&mut self) -> Token {
        self.pos += 7; // Skip "DOCTYPE"
        self.skip_whitespace();

        let start = self.pos;
        while self.peek().is_some() && self.peek() != Some('>') {
            self.advance();
        }

        let name = self.input[start..self.pos].trim().to_string();

        if self.peek() == Some('>') {
            self.advance();
        }

        Token::Doctype { name }
    }

    fn read_tag_name(&mut self) -> String {
        let start = self.pos;

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || ch == '>' || ch == '/' {
                break;
            }
            self.advance();
        }

        self.input[start..self.pos].to_lowercase()
    }

    fn read_attribute_name(&mut self) -> String {
        let start = self.pos;

        while let Some(ch) = self.peek() {
            if ch.is_whitespace() || ch == '=' || ch == '>' || ch == '/' {
                break;
            }
            self.advance();
        }

        self.input[start..self.pos].to_lowercase()
    }

    fn read_attribute_value(&mut self) -> String {
        if self.peek() == Some('"') || self.peek() == Some('\'') {
            let quote = self.peek().unwrap();
            self.advance();
            let start = self.pos;

            while self.peek().is_some() && self.peek() != Some(quote) {
                self.advance();
            }

            let value = self.input[start..self.pos].to_string();

            if self.peek() == Some(quote) {
                self.advance();
            }

            value
        } else {
            let start = self.pos;

            while let Some(ch) = self.peek() {
                if ch.is_whitespace() || ch == '>' {
                    break;
                }
                self.advance();
            }

            self.input[start..self.pos].to_string()
        }
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek() {
            if !ch.is_whitespace() {
                break;
            }
            self.advance();
        }
    }

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) {
        if let Some(ch) = self.peek() {
            self.pos += ch.len_utf8();
        }
    }
}

/// Check if element is void (self-closing).
fn is_void_element(name: &str) -> bool {
    matches!(
        name,
        "area" | "base" | "br" | "col" | "embed" | "hr" | "img" | "input" | "link" | "meta"
            | "param" | "source" | "track" | "wbr"
    )
}
