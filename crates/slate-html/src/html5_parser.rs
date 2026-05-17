//! Real HTML5 parser using html5ever.

use crate::{DomTree, Element, Node, NodeData, ParseError, ParseResult};
use html5ever::tendril::TendrilSink;
use html5ever::{parse_document, ParseOpts};
use markup5ever_rcdom::{Handle, NodeData as RcNodeData, RcDom};
use slate_dispatcher::OwnedWebCall;
use slate_ais::NodeId;
use std::collections::HashMap;

/// HTML5-compliant parser using html5ever.
pub struct Html5Parser {
    options: ParseOpts,
    next_node_id: u32,
}

impl Html5Parser {
    /// Create a new HTML5 parser with default options.
    pub fn new() -> Self {
        Self {
            options: ParseOpts::default(),
            next_node_id: 1,
        }
    }

    /// Get next node ID.
    fn next_id(&mut self) -> NodeId {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        id
    }

    /// Parse HTML string into DOM tree and WebCalls.
    pub fn parse(&mut self, html: &str) -> Result<ParseResult, ParseError> {
        // Parse with html5ever
        let dom = parse_document(RcDom::default(), self.options.clone())
            .from_utf8()
            .read_from(&mut html.as_bytes())
            .map_err(|e| ParseError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;

        // Convert to our DOM tree
        let mut tree = DomTree::new();
        let mut web_calls = Vec::new();

        // Process the document
        self.process_node(&dom.document, &mut tree, &mut web_calls, None);

        Ok(ParseResult { tree, web_calls })
    }

    /// Process a node recursively.
    fn process_node(
        &mut self,
        handle: &Handle,
        tree: &mut DomTree,
        web_calls: &mut Vec<OwnedWebCall>,
        parent_id: Option<NodeId>,
    ) -> Option<NodeId> {
        let node_data = &handle.data;

        match node_data {
            RcNodeData::Document => {
                // Process children
                for child in handle.children.borrow().iter() {
                    self.process_node(child, tree, web_calls, None);
                }
                None
            }
            RcNodeData::Doctype { name, public_id, system_id } => {
                let node_id = self.next_id();
                let node = Node {
                    id: node_id,
                    data: NodeData::Doctype(crate::tree::Doctype {
                        name: name.to_string(),
                        public_id: Some(public_id.to_string()),
                        system_id: Some(system_id.to_string()),
                    }),
                    parent: parent_id,
                    children: Vec::new(),
                };
                tree.add_node(node);
                
                if let Some(parent) = parent_id {
                    tree.append_child(parent, node_id);
                }
                
                Some(node_id)
            }
            RcNodeData::Text { contents } => {
                let text = contents.borrow().to_string();
                if !text.trim().is_empty() {
                    let node_id = self.next_id();
                    let node = Node {
                        id: node_id,
                        data: NodeData::Text(text.clone()),
                        parent: parent_id,
                        children: Vec::new(),
                    };
                    tree.add_node(node);
                    
                    if let Some(parent) = parent_id {
                        tree.append_child(parent, node_id);
                    }

                    // Generate WebCall for text node
                    web_calls.push(OwnedWebCall::CreateTextNode {
                        node: node_id,
                        text,
                    });
                    
                    if let Some(parent) = parent_id {
                        web_calls.push(OwnedWebCall::AppendChild {
                            parent,
                            child: node_id,
                            index: 0,
                        });
                    }
                    
                    Some(node_id)
                } else {
                    None
                }
            }
            RcNodeData::Comment { contents } => {
                let node_id = self.next_id();
                let node = Node {
                    id: node_id,
                    data: NodeData::Comment(contents.to_string()),
                    parent: parent_id,
                    children: Vec::new(),
                };
                tree.add_node(node);
                
                if let Some(parent) = parent_id {
                    tree.append_child(parent, node_id);
                }
                
                Some(node_id)
            }
            RcNodeData::Element {
                name,
                attrs,
                ..
            } => {
                let tag_name = name.local.to_string();
                let node_id = self.next_id();
                
                // Convert attributes
                let mut attributes = HashMap::new();
                for attr in attrs.borrow().iter() {
                    attributes.insert(
                        attr.name.local.to_string(),
                        attr.value.to_string(),
                    );
                }

                let element = Element {
                    tag_name: tag_name.clone(),
                    attributes: attributes.clone(),
                    namespace: crate::Namespace::Html, // TODO: Handle SVG/MathML
                };

                let node = Node {
                    id: node_id,
                    data: NodeData::Element(element),
                    parent: parent_id,
                    children: Vec::new(),
                };
                tree.add_node(node);
                
                if let Some(parent) = parent_id {
                    tree.append_child(parent, node_id);
                }

                // Generate WebCall for element creation
                web_calls.push(OwnedWebCall::CreateElement {
                    node: node_id,
                    tag: tag_name.clone(),
                });

                // Generate WebCalls for attributes
                for (key, value) in attributes.iter() {
                    web_calls.push(OwnedWebCall::SetAttribute {
                        node: node_id,
                        name: key.clone(),
                        value: value.clone(),
                    });
                }

                // Append to parent
                if let Some(parent) = parent_id {
                    web_calls.push(OwnedWebCall::AppendChild {
                        parent,
                        child: node_id,
                        index: 0,
                    });
                }

                // Process children
                for child in handle.children.borrow().iter() {
                    self.process_node(child, tree, web_calls, Some(node_id));
                }
                
                Some(node_id)
            }
            RcNodeData::ProcessingInstruction { .. } => {
                // Ignore processing instructions for now
                None
            }
        }
    }
}

impl Default for Html5Parser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_html() {
        let mut parser = Html5Parser::new();
        let html = r#"<!DOCTYPE html><html><body><h1>Hello</h1></body></html>"#;
        let result = parser.parse(html);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_with_attributes() {
        let mut parser = Html5Parser::new();
        let html = r#"<div class="container" id="main">Content</div>"#;
        let result = parser.parse(html);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_nested_elements() {
        let mut parser = Html5Parser::new();
        let html = r#"
            <div>
                <p>Paragraph 1</p>
                <p>Paragraph 2</p>
            </div>
        "#;
        let result = parser.parse(html);
        assert!(result.is_ok());
    }
}

