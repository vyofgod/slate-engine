//! DOM tree representation.

use slate_ais::NodeId;
use std::collections::HashMap;

/// A DOM tree.
#[derive(Debug, Clone)]
pub struct DomTree {
    pub nodes: HashMap<NodeId, Node>,
    pub root: Option<NodeId>,
}

impl DomTree {
    /// Create a new empty DOM tree.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            root: None,
        }
    }

    /// Add a node to the tree.
    pub fn add_node(&mut self, node: Node) {
        if self.root.is_none() {
            self.root = Some(node.id);
        }
        self.nodes.insert(node.id, node);
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    /// Get a mutable node by ID.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(&id)
    }

    /// Append a child to a parent node.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) {
        if let Some(parent_node) = self.nodes.get_mut(&parent) {
            parent_node.children.push(child);
        }
        if let Some(child_node) = self.nodes.get_mut(&child) {
            child_node.parent = Some(parent);
        }
    }
}

impl Default for DomTree {
    fn default() -> Self {
        Self::new()
    }
}

/// A DOM node.
#[derive(Debug, Clone)]
pub struct Node {
    pub id: NodeId,
    pub data: NodeData,
    pub parent: Option<NodeId>,
    pub children: Vec<NodeId>,
}

/// Node data variants.
#[derive(Debug, Clone)]
pub enum NodeData {
    Document,
    Element(Element),
    Text(String),
    Comment(String),
    Doctype(Doctype),
}

/// An HTML element.
#[derive(Debug, Clone)]
pub struct Element {
    pub tag_name: String,
    pub attributes: HashMap<String, String>,
    pub namespace: super::Namespace,
}

impl Element {
    /// Get an attribute value.
    pub fn get_attr(&self, name: &str) -> Option<&str> {
        self.attributes.get(name).map(|s| s.as_str())
    }

    /// Set an attribute value.
    pub fn set_attr(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    /// Check if element has an attribute.
    pub fn has_attr(&self, name: &str) -> bool {
        self.attributes.contains_key(name)
    }
}

/// A DOCTYPE declaration.
#[derive(Debug, Clone)]
pub struct Doctype {
    pub name: String,
    pub public_id: Option<String>,
    pub system_id: Option<String>,
}
