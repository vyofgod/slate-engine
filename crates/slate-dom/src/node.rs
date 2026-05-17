//! DOM node types.

use std::collections::HashMap;

/// A DOM node.
#[derive(Debug, Clone)]
pub struct Node {
    pub node_type: NodeType,
}

/// Node type variants.
#[derive(Debug, Clone)]
pub enum NodeType {
    Document(Document),
    Element(Element),
    Text(Text),
    Comment(Comment),
}

/// Document node.
#[derive(Debug, Clone)]
pub struct Document;

/// Element node.
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub namespace: Option<String>,
}

impl Element {
    /// Get an attribute value.
    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Check if element has an attribute.
    pub fn has_attr(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }

    /// Get tag name in lowercase.
    pub fn tag_name_lower(&self) -> String {
        self.tag_name.to_lowercase()
    }
}

/// Text node.
#[derive(Debug, Clone)]
pub struct Text {
    pub data: String,
}

/// Comment node.
#[derive(Debug, Clone)]
pub struct Comment {
    pub data: String,
}
