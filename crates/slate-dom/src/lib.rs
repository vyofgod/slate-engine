//! # Slate DOM Engine
//!
//! Full DOM implementation with mutation tracking and dirty flags.

use slate_ais::NodeId;
use slotmap::{SlotMap, SecondaryMap};
use std::collections::HashMap;

pub mod node;
pub mod mutation;
pub mod query;

pub use node::{Node, NodeType, Element, Text, Comment, Document};
pub use mutation::{MutationObserver, MutationRecord, MutationType};
pub use query::QueryEngine;

/// The DOM tree with mutation tracking.
pub struct Dom {
    nodes: SlotMap<NodeId, Node>,
    parents: SecondaryMap<NodeId, NodeId>,
    children: SecondaryMap<NodeId, Vec<NodeId>>,
    dirty: SecondaryMap<NodeId, DirtyFlags>,
    document: Option<NodeId>,
    id_map: HashMap<String, NodeId>,
    class_map: HashMap<String, Vec<NodeId>>,
}

/// Dirty flags for incremental layout.
#[derive(Debug, Clone, Copy, Default)]
pub struct DirtyFlags {
    pub layout: bool,
    pub style: bool,
    pub paint: bool,
}

impl Dom {
    /// Create a new empty DOM.
    pub fn new() -> Self {
        let mut dom = Self {
            nodes: SlotMap::with_key(),
            parents: SecondaryMap::new(),
            children: SecondaryMap::new(),
            dirty: SecondaryMap::new(),
            document: None,
            id_map: HashMap::new(),
            class_map: HashMap::new(),
        };

        // Create document node
        let doc = dom.create_document();
        dom.document = Some(doc);
        dom
    }

    /// Create a document node.
    pub fn create_document(&mut self) -> NodeId {
        let node = Node {
            node_type: NodeType::Document(Document),
        };
        self.nodes.insert(node)
    }

    /// Create an element node.
    pub fn create_element(&mut self, tag_name: String) -> NodeId {
        let node = Node {
            node_type: NodeType::Element(Element {
                tag_name,
                attributes: HashMap::new(),
                namespace: None,
            }),
        };
        let id = self.nodes.insert(node);
        self.mark_dirty(id, DirtyFlags {
            layout: true,
            style: true,
            paint: true,
        });
        id
    }

    /// Create a text node.
    pub fn create_text(&mut self, data: String) -> NodeId {
        let node = Node {
            node_type: NodeType::Text(Text { data }),
        };
        let id = self.nodes.insert(node);
        self.mark_dirty(id, DirtyFlags {
            layout: true,
            style: false,
            paint: true,
        });
        id
    }

    /// Create a comment node.
    pub fn create_comment(&mut self, data: String) -> NodeId {
        let node = Node {
            node_type: NodeType::Comment(Comment { data }),
        };
        self.nodes.insert(node)
    }

    /// Get a node by ID.
    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    /// Get a mutable node by ID.
    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    /// Get the document node.
    pub fn document(&self) -> Option<NodeId> {
        self.document
    }

    /// Append a child to a parent.
    pub fn append_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), DomError> {
        if !self.nodes.contains_key(parent) {
            return Err(DomError::NodeNotFound);
        }
        if !self.nodes.contains_key(child) {
            return Err(DomError::NodeNotFound);
        }

        // Remove from old parent if exists
        if let Some(old_parent) = self.parents.get(child).copied() {
            self.remove_child(old_parent, child)?;
        }

        // Add to new parent
        self.parents.insert(child, parent);
        self.children
            .entry(parent)
            .unwrap()
            .or_insert_with(Vec::new)
            .push(child);

        // Mark dirty
        self.mark_dirty(parent, DirtyFlags {
            layout: true,
            style: false,
            paint: true,
        });

        Ok(())
    }

    /// Remove a child from a parent.
    pub fn remove_child(&mut self, parent: NodeId, child: NodeId) -> Result<(), DomError> {
        if let Some(children) = self.children.get_mut(parent) {
            if let Some(pos) = children.iter().position(|&c| c == child) {
                children.remove(pos);
                self.parents.remove(child);

                // Mark dirty
                self.mark_dirty(parent, DirtyFlags {
                    layout: true,
                    style: false,
                    paint: true,
                });

                return Ok(());
            }
        }
        Err(DomError::NodeNotFound)
    }

    /// Get children of a node.
    pub fn children(&self, node: NodeId) -> &[NodeId] {
        self.children.get(node).map(|v| v.as_slice()).unwrap_or(&[])
    }

    /// Get parent of a node.
    pub fn parent(&self, node: NodeId) -> Option<NodeId> {
        self.parents.get(node).copied()
    }

    /// Set an attribute on an element.
    pub fn set_attribute(&mut self, node: NodeId, name: String, value: String) -> Result<(), DomError> {
        let Some(n) = self.nodes.get_mut(node) else {
            return Err(DomError::NodeNotFound);
        };

        if let NodeType::Element(ref mut elem) = n.node_type {
            // Handle special attributes
            if name == "id" {
                self.id_map.insert(value.clone(), node);
            } else if name == "class" {
                for class in value.split_whitespace() {
                    self.class_map
                        .entry(class.to_string())
                        .or_insert_with(Vec::new)
                        .push(node);
                }
            }

            elem.attributes.insert(name, value);

            // Mark dirty
            self.mark_dirty(node, DirtyFlags {
                layout: true,
                style: true,
                paint: true,
            });

            Ok(())
        } else {
            Err(DomError::NotAnElement)
        }
    }

    /// Get an attribute from an element.
    pub fn get_attribute(&self, node: NodeId, name: &str) -> Option<&str> {
        let n = self.nodes.get(node)?;
        if let NodeType::Element(ref elem) = n.node_type {
            elem.attributes.get(name).map(|s| s.as_str())
        } else {
            None
        }
    }

    /// Get element by ID.
    pub fn get_element_by_id(&self, id: &str) -> Option<NodeId> {
        self.id_map.get(id).copied()
    }

    /// Get elements by class name.
    pub fn get_elements_by_class_name(&self, class: &str) -> Vec<NodeId> {
        self.class_map.get(class).cloned().unwrap_or_default()
    }

    /// Query selector (basic implementation).
    pub fn query_selector(&self, selector: &str) -> Option<NodeId> {
        let engine = QueryEngine::new(self);
        engine.query_selector(selector)
    }

    /// Query selector all.
    pub fn query_selector_all(&self, selector: &str) -> Vec<NodeId> {
        let engine = QueryEngine::new(self);
        engine.query_selector_all(selector)
    }

    /// Mark a node as dirty.
    pub fn mark_dirty(&mut self, node: NodeId, flags: DirtyFlags) {
        if let Some(existing) = self.dirty.get_mut(node) {
            existing.layout |= flags.layout;
            existing.style |= flags.style;
            existing.paint |= flags.paint;
        } else {
            self.dirty.insert(node, flags);
        }

        // Propagate to ancestors
        if flags.layout {
            let mut current = self.parent(node);
            while let Some(parent) = current {
                if let Some(parent_flags) = self.dirty.get_mut(parent) {
                    if parent_flags.layout {
                        break; // Already marked
                    }
                    parent_flags.layout = true;
                }
                current = self.parent(parent);
            }
        }
    }

    /// Get dirty nodes.
    pub fn dirty_nodes(&self) -> Vec<(NodeId, DirtyFlags)> {
        self.dirty
            .iter()
            .filter(|(_, flags)| flags.layout || flags.style || flags.paint)
            .map(|(id, flags)| (id, *flags))
            .collect()
    }

    /// Clear dirty flags.
    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }

    /// Get text content of a node (recursive).
    pub fn text_content(&self, node: NodeId) -> String {
        let mut result = String::new();
        self.collect_text(node, &mut result);
        result
    }

    fn collect_text(&self, node: NodeId, result: &mut String) {
        if let Some(n) = self.nodes.get(node) {
            match &n.node_type {
                NodeType::Text(text) => result.push_str(&text.data),
                NodeType::Element(_) => {
                    for &child in self.children(node) {
                        self.collect_text(child, result);
                    }
                }
                _ => {}
            }
        }
    }

    /// Set inner HTML (simplified).
    pub fn set_inner_html(&mut self, node: NodeId, html: &str) -> Result<(), DomError> {
        // Remove all children
        let children: Vec<_> = self.children(node).to_vec();
        for child in children {
            self.remove_child(node, child)?;
        }

        // Parse and add new children (simplified)
        // TODO: Use real HTML parser
        let text_node = self.create_text(html.to_string());
        self.append_child(node, text_node)?;

        Ok(())
    }
}

impl Default for Dom {
    fn default() -> Self {
        Self::new()
    }
}

/// DOM errors.
#[derive(Debug, thiserror::Error)]
pub enum DomError {
    #[error("node not found")]
    NodeNotFound,

    #[error("not an element")]
    NotAnElement,

    #[error("hierarchy request error")]
    HierarchyRequest,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_and_append() {
        let mut dom = Dom::new();
        let doc = dom.document().unwrap();
        let div = dom.create_element("div".to_string());
        let text = dom.create_text("Hello".to_string());

        dom.append_child(doc, div).unwrap();
        dom.append_child(div, text).unwrap();

        assert_eq!(dom.children(doc).len(), 1);
        assert_eq!(dom.children(div).len(), 1);
        assert_eq!(dom.text_content(div), "Hello");
    }

    #[test]
    fn attributes() {
        let mut dom = Dom::new();
        let div = dom.create_element("div".to_string());

        dom.set_attribute(div, "id".to_string(), "test".to_string()).unwrap();
        dom.set_attribute(div, "class".to_string(), "foo bar".to_string()).unwrap();

        assert_eq!(dom.get_attribute(div, "id"), Some("test"));
        assert_eq!(dom.get_element_by_id("test"), Some(div));
    }

    #[test]
    fn dirty_tracking() {
        let mut dom = Dom::new();
        let div = dom.create_element("div".to_string());

        assert!(!dom.dirty_nodes().is_empty());

        dom.clear_dirty();
        assert!(dom.dirty_nodes().is_empty());

        dom.set_attribute(div, "style".to_string(), "color:red".to_string()).unwrap();
        assert!(!dom.dirty_nodes().is_empty());
    }
}
