//! Web API to AIS Translator (Wine-like compatibility layer)
//!
//! This module acts like Wine for Web APIs: it intercepts high-level
//! Web API calls and translates them into Slate's Atomic Instruction Set (AIS).

use slate_ais::{
    AtomicInstruction, LayoutPrimitive, RenderPrimitive, StatePrimitive,
    NodeId, Point, Rect, Rgba8, Size,
};
use slate_dispatcher::OwnedWebCall;
use std::collections::HashMap;

/// Web API translator - converts high-level Web APIs to AIS primitives.
pub struct WebApiTranslator {
    /// Pending WebCalls to be translated
    pending_calls: Vec<OwnedWebCall>,
    
    /// Node metadata for translation context
    node_metadata: HashMap<NodeId, NodeMetadata>,
}

/// Metadata about a node needed for translation.
#[derive(Debug, Clone)]
struct NodeMetadata {
    #[allow(dead_code)]
    tag: String,
    attributes: HashMap<String, String>,
    styles: HashMap<String, String>,
}

impl WebApiTranslator {
    /// Create new translator.
    pub fn new() -> Self {
        Self {
            pending_calls: Vec::new(),
            node_metadata: HashMap::new(),
        }
    }

    /// Translate a Web API call to AIS primitives.
    pub fn translate(&mut self, call: OwnedWebCall) -> Vec<AtomicInstruction> {
        match call {
            OwnedWebCall::CreateElement { node, ref tag } => {
                self.translate_create_element(node, tag)
            }
            OwnedWebCall::CreateTextNode { node, ref text } => {
                self.translate_create_text_node(node, text)
            }
            OwnedWebCall::AppendChild { parent, child, .. } => {
                self.translate_append_child(parent, child)
            }
            OwnedWebCall::RemoveChild { parent, child } => {
                self.translate_remove_child(parent, child)
            }
            OwnedWebCall::InsertBefore { parent, new_child, ref_child } => {
                self.translate_insert_before(parent, new_child, ref_child)
            }
            OwnedWebCall::SetAttribute { node, ref name, ref value } => {
                self.translate_set_attribute(node, name, value)
            }
            OwnedWebCall::RemoveAttribute { node, ref name } => {
                self.translate_remove_attribute(node, name)
            }
            OwnedWebCall::AddClass { node, ref class } => {
                self.translate_add_class(node, class)
            }
            OwnedWebCall::SetInlineStyle { node, ref css } => {
                self.translate_set_inline_style(node, css)
            }
            OwnedWebCall::AnchorRect { node, rect } => {
                self.translate_anchor_rect(node, rect)
            }
        }
    }

    /// Translate document.createElement() to AIS.
    fn translate_create_element(&mut self, node: NodeId, tag: &str) -> Vec<AtomicInstruction> {
        // Store metadata
        self.node_metadata.insert(node, NodeMetadata {
            tag: tag.to_string(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
        });

        vec![
            AtomicInstruction::State(StatePrimitive::NodeCreate { node }),
        ]
    }

    /// Translate document.createTextNode() to AIS.
    fn translate_create_text_node(&mut self, node: NodeId, _text: &str) -> Vec<AtomicInstruction> {
        self.node_metadata.insert(node, NodeMetadata {
            tag: "#text".to_string(),
            attributes: HashMap::new(),
            styles: HashMap::new(),
        });

        vec![
            AtomicInstruction::State(StatePrimitive::NodeCreate { node }),
        ]
    }

    /// Translate appendChild() to AIS.
    fn translate_append_child(&mut self, parent: NodeId, child: NodeId) -> Vec<AtomicInstruction> {
        vec![
            AtomicInstruction::State(StatePrimitive::NodeAttach { 
                node: child,
                parent, 
                index: 0 // Will be calculated properly
            }),
        ]
    }

    /// Translate removeChild() to AIS.
    fn translate_remove_child(&mut self, _parent: NodeId, child: NodeId) -> Vec<AtomicInstruction> {
        vec![
            AtomicInstruction::State(StatePrimitive::NodeDetach { node: child }),
        ]
    }

    /// Translate insertBefore() to AIS.
    fn translate_insert_before(
        &mut self, 
        parent: NodeId, 
        new_child: NodeId, 
        _ref_child: NodeId
    ) -> Vec<AtomicInstruction> {
        vec![
            AtomicInstruction::State(StatePrimitive::NodeAttach { 
                node: new_child,
                parent, 
                index: 0 // Should calculate proper index
            }),
        ]
    }

    /// Translate setAttribute() to AIS.
    fn translate_set_attribute(
        &mut self, 
        node: NodeId, 
        name: &str, 
        value: &str
    ) -> Vec<AtomicInstruction> {
        // Update metadata
        if let Some(meta) = self.node_metadata.get_mut(&node) {
            meta.attributes.insert(name.to_string(), value.to_string());
        }

        let mut instructions = Vec::new();

        // Some attributes affect layout/rendering
        match name {
            "width" => {
                if let Ok(width) = self.parse_length(value) {
                    instructions.push(AtomicInstruction::Layout(
                        LayoutPrimitive::SetSize { 
                            node, 
                            size: Size { w: width.into(), h: 0.0.into() } 
                        }
                    ));
                }
            }
            "height" => {
                if let Ok(height) = self.parse_length(value) {
                    instructions.push(AtomicInstruction::Layout(
                        LayoutPrimitive::SetSize { 
                            node, 
                            size: Size { w: 0.0.into(), h: height.into() } 
                        }
                    ));
                }
            }
            _ => {}
        }

        instructions
    }

    /// Translate removeAttribute() to AIS.
    fn translate_remove_attribute(&mut self, node: NodeId, name: &str) -> Vec<AtomicInstruction> {
        if let Some(meta) = self.node_metadata.get_mut(&node) {
            meta.attributes.remove(name);
        }
        vec![]
    }

    /// Translate classList.add() to AIS.
    fn translate_add_class(&mut self, node: NodeId, class: &str) -> Vec<AtomicInstruction> {
        let current_class = self.node_metadata
            .get(&node)
            .and_then(|m| m.attributes.get("class"))
            .map(|s| s.as_str())
            .unwrap_or("");

        let new_class = if current_class.is_empty() {
            class.to_string()
        } else {
            format!("{} {}", current_class, class)
        };

        self.translate_set_attribute(node, "class", &new_class)
    }

    /// Translate style.setProperty() to AIS.
    fn translate_set_inline_style(&mut self, node: NodeId, css: &str) -> Vec<AtomicInstruction> {
        let mut instructions = Vec::new();

        for property in css.split(';') {
            let property = property.trim();
            if property.is_empty() {
                continue;
            }

            if let Some((name, value)) = property.split_once(':') {
                let name = name.trim();
                let value = value.trim();

                if let Some(meta) = self.node_metadata.get_mut(&node) {
                    meta.styles.insert(name.to_string(), value.to_string());
                }

                instructions.extend(self.translate_css_property(node, name, value));
            }
        }

        instructions
    }

    /// Translate a single CSS property to AIS primitives.
    fn translate_css_property(
        &self, 
        node: NodeId, 
        name: &str, 
        value: &str
    ) -> Vec<AtomicInstruction> {
        match name {
            "width" => {
                if let Ok(width) = self.parse_length(value) {
                    vec![AtomicInstruction::Layout(
                        LayoutPrimitive::SetSize { 
                            node, 
                            size: Size { w: width.into(), h: 0.0.into() } 
                        }
                    )]
                } else {
                    vec![]
                }
            }
            "height" => {
                if let Ok(height) = self.parse_length(value) {
                    vec![AtomicInstruction::Layout(
                        LayoutPrimitive::SetSize { 
                            node, 
                            size: Size { w: 0.0.into(), h: height.into() } 
                        }
                    )]
                } else {
                    vec![]
                }
            }
            "background-color" | "background" => {
                if let Ok(color) = self.parse_color(value) {
                    vec![AtomicInstruction::Render(
                        RenderPrimitive::FillRect { 
                            rect: Rect::from_ltwh(0.0, 0.0, 0.0, 0.0),
                            color 
                        }
                    )]
                } else {
                    vec![]
                }
            }
            _ => vec![]
        }
    }

    /// Translate AnchorRect to AIS.
    fn translate_anchor_rect(&mut self, node: NodeId, rect: Rect) -> Vec<AtomicInstruction> {
        vec![
            AtomicInstruction::Layout(LayoutPrimitive::SetPosition { 
                node, 
                point: Point { x: rect.origin.x, y: rect.origin.y } 
            }),
            AtomicInstruction::Layout(LayoutPrimitive::SetSize { 
                node, 
                size: rect.size
            }),
        ]
    }

    // Helper functions

    fn parse_length(&self, value: &str) -> Result<f32, ()> {
        let value = value.trim();
        if value.ends_with("px") {
            value[..value.len()-2].parse().map_err(|_| ())
        } else {
            value.parse().map_err(|_| ())
        }
    }

    fn parse_color(&self, value: &str) -> Result<Rgba8, ()> {
        let value = value.trim();
        
        // Named colors
        match value {
            "red" => return Ok(Rgba8 { r: 255, g: 0, b: 0, a: 255 }),
            "green" => return Ok(Rgba8 { r: 0, g: 255, b: 0, a: 255 }),
            "blue" => return Ok(Rgba8 { r: 0, g: 0, b: 255, a: 255 }),
            "white" => return Ok(Rgba8 { r: 255, g: 255, b: 255, a: 255 }),
            "black" => return Ok(Rgba8 { r: 0, g: 0, b: 0, a: 255 }),
            _ => {}
        }
        
        // Hex colors
        if value.starts_with('#') {
            let hex = &value[1..];
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| ())?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| ())?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| ())?;
                return Ok(Rgba8 { r, g, b, a: 255 });
            }
        }
        
        Err(())
    }

    /// Queue a WebCall for batch translation.
    pub fn queue(&mut self, call: OwnedWebCall) {
        self.pending_calls.push(call);
    }

    /// Translate all queued WebCalls to AIS in one batch.
    pub fn flush(&mut self) -> Vec<AtomicInstruction> {
        let mut all_instructions = Vec::new();
        
        let calls = std::mem::take(&mut self.pending_calls);
        for call in calls {
            all_instructions.extend(self.translate(call));
        }
        
        all_instructions
    }
}

impl Default for WebApiTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_translate_create_element() {
        let mut translator = WebApiTranslator::new();
        let node = NodeId(1);
        
        let instructions = translator.translate(OwnedWebCall::CreateElement {
            node,
            tag: "div".to_string(),
        });
        
        assert_eq!(instructions.len(), 1);
        match &instructions[0] {
            AtomicInstruction::State(StatePrimitive::NodeCreate { node: n }) => {
                assert_eq!(*n, node);
            }
            _ => panic!("Expected NodeCreate"),
        }
    }

    #[test]
    fn test_translate_append_child() {
        let mut translator = WebApiTranslator::new();
        let parent = NodeId(1);
        let child = NodeId(2);
        
        let instructions = translator.translate(OwnedWebCall::AppendChild {
            parent,
            child,
            index: 0,
        });
        
        assert_eq!(instructions.len(), 1);
        match &instructions[0] {
            AtomicInstruction::State(StatePrimitive::NodeAttach { node, parent: p, .. }) => {
                assert_eq!(*node, child);
                assert_eq!(*p, parent);
            }
            _ => panic!("Expected NodeAttach"),
        }
    }

    #[test]
    fn test_translate_set_style() {
        let mut translator = WebApiTranslator::new();
        let node = NodeId(1);
        
        // First create the element
        translator.translate(OwnedWebCall::CreateElement {
            node,
            tag: "div".to_string(),
        });
        
        // Then set style
        let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
            node,
            css: "width:100px;height:50px;background-color:red".to_string(),
        });
        
        assert!(instructions.len() >= 2); // At least width and height
    }

    #[test]
    fn test_batch_translation() {
        let mut translator = WebApiTranslator::new();
        
        translator.queue(OwnedWebCall::CreateElement {
            node: NodeId(1),
            tag: "div".to_string(),
        });
        
        translator.queue(OwnedWebCall::SetAttribute {
            node: NodeId(1),
            name: "id".to_string(),
            value: "main".to_string(),
        });
        
        let instructions = translator.flush();
        assert!(instructions.len() >= 1);
    }

    #[test]
    fn test_parse_color() {
        let translator = WebApiTranslator::new();
        
        assert_eq!(translator.parse_color("red").unwrap(), Rgba8 { r: 255, g: 0, b: 0, a: 255 });
        assert_eq!(translator.parse_color("#ff0000").unwrap(), Rgba8 { r: 255, g: 0, b: 0, a: 255 });
        assert_eq!(translator.parse_color("blue").unwrap(), Rgba8 { r: 0, g: 0, b: 255, a: 255 });
    }

    #[test]
    fn test_parse_length() {
        let translator = WebApiTranslator::new();
        
        assert_eq!(translator.parse_length("100px").unwrap(), 100.0);
        assert_eq!(translator.parse_length("50").unwrap(), 50.0);
        assert_eq!(translator.parse_length("25.5px").unwrap(), 25.5);
    }
}
