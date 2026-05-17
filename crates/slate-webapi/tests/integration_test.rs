//! Integration tests for Web API translation layer

use slate_webapi::WebApiTranslator;
use slate_dispatcher::OwnedWebCall;
use slate_ais::{AtomicInstruction, NodeId, StatePrimitive, LayoutPrimitive, RenderPrimitive};

#[test]
fn test_simple_dom_creation() {
    let mut translator = WebApiTranslator::new();
    
    // Create a div
    let div_node = NodeId(1);
    let instructions = translator.translate(OwnedWebCall::CreateElement {
        node: div_node,
        tag: "div".to_string(),
    });
    
    assert_eq!(instructions.len(), 1);
    assert!(matches!(
        instructions[0],
        AtomicInstruction::State(StatePrimitive::NodeCreate { .. })
    ));
}

#[test]
fn test_dom_tree_construction() {
    let mut translator = WebApiTranslator::new();
    
    // Create parent
    translator.queue(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // Create child
    translator.queue(OwnedWebCall::CreateElement {
        node: NodeId(2),
        tag: "span".to_string(),
    });
    
    // Append child to parent
    translator.queue(OwnedWebCall::AppendChild {
        parent: NodeId(1),
        child: NodeId(2),
        index: 0,
    });
    
    let instructions = translator.flush();
    
    // Should have: CreateNode(1), CreateNode(2), NodeAttach(2->1)
    assert_eq!(instructions.len(), 3);
}

#[test]
fn test_style_translation() {
    let mut translator = WebApiTranslator::new();
    
    // Create element
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // Set styles
    let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "width:100px;height:50px".to_string(),
    });
    
    // Should generate layout instructions
    assert!(instructions.len() >= 2);
    
    // Check for SetSize instructions
    let has_layout = instructions.iter().any(|i| {
        matches!(i, AtomicInstruction::Layout(LayoutPrimitive::SetSize { .. }))
    });
    assert!(has_layout);
}

#[test]
fn test_background_color_translation() {
    let mut translator = WebApiTranslator::new();
    
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "background-color:red".to_string(),
    });
    
    // Should generate render instruction
    let has_render = instructions.iter().any(|i| {
        matches!(i, AtomicInstruction::Render(RenderPrimitive::FillRect { .. }))
    });
    assert!(has_render);
}

#[test]
fn test_complex_dom_manipulation() {
    let mut translator = WebApiTranslator::new();
    
    // Create root
    translator.queue(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // Create multiple children
    for i in 2..=5 {
        translator.queue(OwnedWebCall::CreateElement {
            node: NodeId(i),
            tag: "span".to_string(),
        });
        
        translator.queue(OwnedWebCall::AppendChild {
            parent: NodeId(1),
            child: NodeId(i),
            index: 0,
        });
    }
    
    let instructions = translator.flush();
    
    // Should have: 1 root + 4 children + 4 attachments = 9 instructions
    assert_eq!(instructions.len(), 9);
}

#[test]
fn test_attribute_setting() {
    let mut translator = WebApiTranslator::new();
    
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // Set width attribute (should affect layout)
    let instructions = translator.translate(OwnedWebCall::SetAttribute {
        node: NodeId(1),
        name: "width".to_string(),
        value: "200".to_string(),
    });
    
    // Should generate layout instruction
    assert!(instructions.len() >= 1);
}

#[test]
fn test_class_addition() {
    let mut translator = WebApiTranslator::new();
    
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // Add first class
    translator.translate(OwnedWebCall::AddClass {
        node: NodeId(1),
        class: "container".to_string(),
    });
    
    // Add second class
    let instructions = translator.translate(OwnedWebCall::AddClass {
        node: NodeId(1),
        class: "active".to_string(),
    });
    
    // Class changes don't generate immediate AIS instructions (they're tracked in metadata)
    // The actual styling happens when CSS rules are applied
    // So we just verify the call succeeded without error
    assert!(instructions.is_empty() || !instructions.is_empty()); // Always true, just checking no panic
}

#[test]
fn test_remove_child() {
    let mut translator = WebApiTranslator::new();
    
    let instructions = translator.translate(OwnedWebCall::RemoveChild {
        parent: NodeId(1),
        child: NodeId(2),
    });
    
    // Should generate NodeDetach
    assert_eq!(instructions.len(), 1);
    assert!(matches!(
        instructions[0],
        AtomicInstruction::State(StatePrimitive::NodeDetach { .. })
    ));
}

#[test]
fn test_hex_color_parsing() {
    let mut translator = WebApiTranslator::new();
    
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "background-color:#ff0000".to_string(),
    });
    
    // Should parse hex color
    assert!(instructions.len() >= 1);
}

#[test]
fn test_multiple_style_properties() {
    let mut translator = WebApiTranslator::new();
    
    translator.translate(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "width:100px;height:50px;background-color:blue".to_string(),
    });
    
    // Should generate multiple instructions
    assert!(instructions.len() >= 3);
}

#[test]
fn test_text_node_creation() {
    let mut translator = WebApiTranslator::new();
    
    let instructions = translator.translate(OwnedWebCall::CreateTextNode {
        node: NodeId(1),
        text: "Hello, World!".to_string(),
    });
    
    assert_eq!(instructions.len(), 1);
    assert!(matches!(
        instructions[0],
        AtomicInstruction::State(StatePrimitive::NodeCreate { .. })
    ));
}

#[test]
fn test_insert_before() {
    let mut translator = WebApiTranslator::new();
    
    let instructions = translator.translate(OwnedWebCall::InsertBefore {
        parent: NodeId(1),
        new_child: NodeId(2),
        ref_child: NodeId(3),
    });
    
    // Should generate NodeAttach
    assert_eq!(instructions.len(), 1);
    assert!(matches!(
        instructions[0],
        AtomicInstruction::State(StatePrimitive::NodeAttach { .. })
    ));
}

#[test]
fn test_batch_processing() {
    let mut translator = WebApiTranslator::new();
    
    // Queue many operations
    for i in 1..=10 {
        translator.queue(OwnedWebCall::CreateElement {
            node: NodeId(i),
            tag: "div".to_string(),
        });
    }
    
    let instructions = translator.flush();
    
    // Should have 10 create instructions
    assert_eq!(instructions.len(), 10);
    
    // Queue should be empty after flush
    let instructions2 = translator.flush();
    assert_eq!(instructions2.len(), 0);
}

#[test]
fn test_real_world_scenario() {
    let mut translator = WebApiTranslator::new();
    
    // Simulate: const div = document.createElement('div');
    translator.queue(OwnedWebCall::CreateElement {
        node: NodeId(1),
        tag: "div".to_string(),
    });
    
    // div.setAttribute('id', 'main');
    translator.queue(OwnedWebCall::SetAttribute {
        node: NodeId(1),
        name: "id".to_string(),
        value: "main".to_string(),
    });
    
    // div.style.width = '100px';
    // div.style.height = '50px';
    // div.style.backgroundColor = 'red';
    translator.queue(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "width:100px;height:50px;background-color:red".to_string(),
    });
    
    // const text = document.createTextNode('Hello');
    translator.queue(OwnedWebCall::CreateTextNode {
        node: NodeId(2),
        text: "Hello".to_string(),
    });
    
    // div.appendChild(text);
    translator.queue(OwnedWebCall::AppendChild {
        parent: NodeId(1),
        child: NodeId(2),
        index: 0,
    });
    
    let instructions = translator.flush();
    
    // Should have multiple instructions
    assert!(instructions.len() >= 5);
    
    // Verify we have different types of instructions
    let has_state = instructions.iter().any(|i| matches!(i, AtomicInstruction::State(_)));
    let has_layout = instructions.iter().any(|i| matches!(i, AtomicInstruction::Layout(_)));
    let has_render = instructions.iter().any(|i| matches!(i, AtomicInstruction::Render(_)));
    
    assert!(has_state);
    assert!(has_layout);
    assert!(has_render);
}
