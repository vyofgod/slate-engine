//! Query selector engine (basic implementation).

use super::Dom;
use slate_ais::NodeId;

/// Query engine for selector matching.
pub struct QueryEngine<'a> {
    dom: &'a Dom,
}

impl<'a> QueryEngine<'a> {
    /// Create a new query engine.
    pub fn new(dom: &'a Dom) -> Self {
        Self { dom }
    }

    /// Query selector (returns first match).
    pub fn query_selector(&self, selector: &str) -> Option<NodeId> {
        let selector = selector.trim();

        // ID selector
        if let Some(id) = selector.strip_prefix('#') {
            return self.dom.get_element_by_id(id);
        }

        // Class selector
        if let Some(class) = selector.strip_prefix('.') {
            return self.dom.get_elements_by_class_name(class).first().copied();
        }

        // Tag selector
        self.find_by_tag(selector)
    }

    /// Query selector all (returns all matches).
    pub fn query_selector_all(&self, selector: &str) -> Vec<NodeId> {
        let selector = selector.trim();

        // ID selector
        if let Some(id) = selector.strip_prefix('#') {
            return self.dom.get_element_by_id(id).into_iter().collect();
        }

        // Class selector
        if let Some(class) = selector.strip_prefix('.') {
            return self.dom.get_elements_by_class_name(class);
        }

        // Tag selector
        self.find_all_by_tag(selector)
    }

    /// Find first element by tag name.
    fn find_by_tag(&self, tag: &str) -> Option<NodeId> {
        if let Some(doc) = self.dom.document() {
            self.find_by_tag_recursive(doc, tag)
        } else {
            None
        }
    }

    fn find_by_tag_recursive(&self, node: NodeId, tag: &str) -> Option<NodeId> {
        if let Some(n) = self.dom.get_node(node) {
            if let super::NodeType::Element(ref elem) = n.node_type {
                if elem.tag_name.eq_ignore_ascii_case(tag) {
                    return Some(node);
                }
            }

            for &child in self.dom.children(node) {
                if let Some(found) = self.find_by_tag_recursive(child, tag) {
                    return Some(found);
                }
            }
        }
        None
    }

    /// Find all elements by tag name.
    fn find_all_by_tag(&self, tag: &str) -> Vec<NodeId> {
        let mut results = Vec::new();
        if let Some(doc) = self.dom.document() {
            self.find_all_by_tag_recursive(doc, tag, &mut results);
        }
        results
    }

    fn find_all_by_tag_recursive(&self, node: NodeId, tag: &str, results: &mut Vec<NodeId>) {
        if let Some(n) = self.dom.get_node(node) {
            if let super::NodeType::Element(ref elem) = n.node_type {
                if elem.tag_name.eq_ignore_ascii_case(tag) {
                    results.push(node);
                }
            }

            for &child in self.dom.children(node) {
                self.find_all_by_tag_recursive(child, tag, results);
            }
        }
    }
}
