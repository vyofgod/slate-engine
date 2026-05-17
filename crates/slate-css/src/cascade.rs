//! CSS cascade and computed style calculation.

use super::{CssRule, CssValue, Property, SelectorMatcher, Specificity, Stylesheet};
use slate_ais::NodeId;
use std::collections::HashMap;

/// Computed style for a node after cascade resolution.
#[derive(Debug, Clone, Default)]
pub struct ComputedStyle {
    pub properties: HashMap<Property, CssValue>,
}

impl ComputedStyle {
    /// Create a new empty computed style.
    pub fn new() -> Self {
        Self {
            properties: HashMap::new(),
        }
    }

    /// Get a property value.
    pub fn get(&self, property: Property) -> Option<&CssValue> {
        self.properties.get(&property)
    }

    /// Set a property value.
    pub fn set(&mut self, property: Property, value: CssValue) {
        self.properties.insert(property, value);
    }

    /// Merge another style into this one (for cascade).
    pub fn merge(&mut self, other: &ComputedStyle, _specificity: Specificity) {
        for (prop, value) in &other.properties {
            // TODO: Track specificity per property for proper cascade
            self.properties.insert(*prop, value.clone());
        }
    }
}

/// CSS cascade engine.
pub struct CascadeEngine {
    stylesheets: Vec<Stylesheet>,
    matcher: SelectorMatcher,
    computed_cache: HashMap<NodeId, ComputedStyle>,
}

impl CascadeEngine {
    /// Create a new cascade engine.
    pub fn new() -> Self {
        Self {
            stylesheets: Vec::new(),
            matcher: SelectorMatcher::new(),
            computed_cache: HashMap::new(),
        }
    }

    /// Add a stylesheet to the cascade.
    pub fn add_stylesheet(&mut self, stylesheet: Stylesheet) {
        self.stylesheets.push(stylesheet);
    }

    /// Get the selector matcher.
    pub fn matcher_mut(&mut self) -> &mut SelectorMatcher {
        &mut self.matcher
    }

    /// Compute the style for a node.
    pub fn compute_style(&mut self, node: NodeId, parent_style: Option<&ComputedStyle>) -> ComputedStyle {
        // Check cache
        if let Some(cached) = self.computed_cache.get(&node) {
            return cached.clone();
        }

        let mut computed = ComputedStyle::new();

        // 1. Inherit from parent
        if let Some(parent) = parent_style {
            for (prop, value) in &parent.properties {
                if prop.is_inherited() {
                    computed.set(*prop, value.clone());
                }
            }
        }

        // 2. Apply matching rules in specificity order
        let mut matching_rules: Vec<(&CssRule, Specificity)> = Vec::new();

        for stylesheet in &self.stylesheets {
            for rule in &stylesheet.rules {
                if self.matcher.matches(node, &rule.selector) {
                    matching_rules.push((rule, rule.specificity));
                }
            }
        }

        // Sort by specificity
        matching_rules.sort_by_key(|(_, spec)| *spec);

        // Apply rules in order
        for (rule, _) in matching_rules {
            for decl in &rule.declarations {
                computed.set(decl.property, decl.value.clone());
            }
        }

        // Cache result
        self.computed_cache.insert(node, computed.clone());
        computed
    }

    /// Clear the computed style cache.
    pub fn clear_cache(&mut self) {
        self.computed_cache.clear();
    }

    /// Invalidate a specific node's cache.
    pub fn invalidate(&mut self, node: NodeId) {
        self.computed_cache.remove(&node);
    }
}

impl Default for CascadeEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn computed_style_merge() {
        let mut style1 = ComputedStyle::new();
        style1.set(Property::Width, CssValue::Number(100.0));

        let mut style2 = ComputedStyle::new();
        style2.set(Property::Height, CssValue::Number(200.0));

        style1.merge(&style2, Specificity::ZERO);

        assert!(style1.get(Property::Width).is_some());
        assert!(style1.get(Property::Height).is_some());
    }
}
