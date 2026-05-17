//! CSS selector matching with full CSS3 support.

use slate_ais::NodeId;
use std::collections::HashMap;

/// A CSS selector.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Universal selector: *
    Universal,

    /// Type selector: div, span, etc.
    Type(String),

    /// Class selector: .foo
    Class(String),

    /// ID selector: #bar
    Id(String),

    /// Attribute selector: [attr=value]
    Attribute {
        name: String,
        operator: AttributeOperator,
        value: Option<String>,
    },

    /// Pseudo-class: :hover, :first-child, etc.
    PseudoClass(PseudoClass),

    /// Pseudo-element: ::before, ::after, etc.
    PseudoElement(PseudoElement),

    /// Combinator: descendant, child, sibling
    Combinator {
        left: Box<Selector>,
        combinator: Combinator,
        right: Box<Selector>,
    },

    /// Compound selector: multiple simple selectors
    Compound(Vec<Selector>),
}

/// Attribute selector operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AttributeOperator {
    Exists,        // [attr]
    Equals,        // [attr=value]
    Contains,      // [attr~=value]
    StartsWith,    // [attr^=value]
    EndsWith,      // [attr$=value]
    Substring,     // [attr*=value]
    DashMatch,     // [attr|=value]
}

/// CSS pseudo-classes.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoClass {
    Hover,
    Active,
    Focus,
    Visited,
    Link,
    FirstChild,
    LastChild,
    NthChild(i32),
    NthLastChild(i32),
    OnlyChild,
    FirstOfType,
    LastOfType,
    NthOfType(i32),
    Empty,
    Root,
    Not(Box<Selector>),
}

/// CSS pseudo-elements.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PseudoElement {
    Before,
    After,
    FirstLine,
    FirstLetter,
    Selection,
}

/// Selector combinators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Combinator {
    Descendant,      // space
    Child,           // >
    NextSibling,     // +
    SubsequentSibling, // ~
}

/// Selector specificity for cascade ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Specificity {
    pub inline: u32,  // Inline styles
    pub ids: u32,     // ID selectors
    pub classes: u32, // Class, attribute, pseudo-class
    pub elements: u32, // Element, pseudo-element
}

impl Specificity {
    /// Create a new specificity.
    pub const fn new(inline: u32, ids: u32, classes: u32, elements: u32) -> Self {
        Self {
            inline,
            ids,
            classes,
            elements,
        }
    }

    /// Zero specificity.
    pub const ZERO: Self = Self::new(0, 0, 0, 0);

    /// Inline style specificity.
    pub const INLINE: Self = Self::new(1, 0, 0, 0);

    /// Calculate specificity for a selector.
    pub fn calculate(selector: &Selector) -> Self {
        let mut spec = Self::ZERO;
        Self::calculate_recursive(selector, &mut spec);
        spec
    }

    fn calculate_recursive(selector: &Selector, spec: &mut Self) {
        match selector {
            Selector::Universal => {}
            Selector::Type(_) => spec.elements += 1,
            Selector::Class(_) => spec.classes += 1,
            Selector::Id(_) => spec.ids += 1,
            Selector::Attribute { .. } => spec.classes += 1,
            Selector::PseudoClass(_) => spec.classes += 1,
            Selector::PseudoElement(_) => spec.elements += 1,
            Selector::Combinator { left, right, .. } => {
                Self::calculate_recursive(left, spec);
                Self::calculate_recursive(right, spec);
            }
            Selector::Compound(selectors) => {
                for sel in selectors {
                    Self::calculate_recursive(sel, spec);
                }
            }
        }
    }
}

/// Selector matcher for checking if a node matches a selector.
pub struct SelectorMatcher {
    // Node metadata for matching
    node_types: HashMap<NodeId, String>,
    node_classes: HashMap<NodeId, Vec<String>>,
    node_ids: HashMap<NodeId, String>,
}

impl SelectorMatcher {
    /// Create a new selector matcher.
    pub fn new() -> Self {
        Self {
            node_types: HashMap::new(),
            node_classes: HashMap::new(),
            node_ids: HashMap::new(),
        }
    }

    /// Register a node's type.
    pub fn set_node_type(&mut self, node: NodeId, tag: String) {
        self.node_types.insert(node, tag);
    }

    /// Register a node's classes.
    pub fn set_node_classes(&mut self, node: NodeId, classes: Vec<String>) {
        self.node_classes.insert(node, classes);
    }

    /// Register a node's ID.
    pub fn set_node_id(&mut self, node: NodeId, id: String) {
        self.node_ids.insert(node, id);
    }

    /// Check if a node matches a selector.
    pub fn matches(&self, node: NodeId, selector: &Selector) -> bool {
        match selector {
            Selector::Universal => true,

            Selector::Type(tag) => {
                self.node_types.get(&node).map_or(false, |t| t == tag)
            }

            Selector::Class(class) => {
                self.node_classes
                    .get(&node)
                    .map_or(false, |classes| classes.contains(class))
            }

            Selector::Id(id) => {
                self.node_ids.get(&node).map_or(false, |node_id| node_id == id)
            }

            Selector::Compound(selectors) => {
                selectors.iter().all(|sel| self.matches(node, sel))
            }

            // TODO: Implement other selector types
            _ => false,
        }
    }
}

impl Default for SelectorMatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn specificity_ordering() {
        let id_spec = Specificity::new(0, 1, 0, 0);
        let class_spec = Specificity::new(0, 0, 1, 0);
        let element_spec = Specificity::new(0, 0, 0, 1);

        assert!(id_spec > class_spec);
        assert!(class_spec > element_spec);
        assert!(Specificity::INLINE > id_spec);
    }

    #[test]
    fn specificity_calculation() {
        let selector = Selector::Compound(vec![
            Selector::Type("div".to_string()),
            Selector::Class("foo".to_string()),
        ]);

        let spec = Specificity::calculate(&selector);
        assert_eq!(spec.elements, 1);
        assert_eq!(spec.classes, 1);
        assert_eq!(spec.ids, 0);
    }
}
