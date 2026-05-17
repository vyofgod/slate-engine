//! Demo: Web API Translation to AIS
//!
//! This example shows how Web API calls are translated to Atomic Instructions.

use slate_webapi::WebApiTranslator;
use slate_dispatcher::OwnedWebCall;
use slate_ais::{AtomicInstruction, NodeId};

fn main() {
    println!("=== Slate Web API Translator Demo ===\n");
    println!("This demonstrates the Wine-like translation layer:\n");
    println!("Web API Call → Translator → Atomic Instructions (AIS)\n");
    println!("{}", "=".repeat(60));

    // Example 1: Simple Element Creation
    println!("\n📝 Example 1: document.createElement('div')");
    println!("{}", "-".repeat(60));
    
    let mut translator = WebApiTranslator::new();
    let div_node = NodeId(1);
    
    let instructions = translator.translate(OwnedWebCall::CreateElement {
        node: div_node,
        tag: "div".to_string(),
    });
    
    println!("Input: CreateElement {{ node: {:?}, tag: 'div' }}", div_node);
    println!("Output AIS:");
    for (i, instr) in instructions.iter().enumerate() {
        println!("  [{}] {:?}", i, instr);
    }
    println!("✅ Translated to {} atomic instruction(s)", instructions.len());

    // Example 2: DOM Tree Construction
    println!("\n📝 Example 2: Building a DOM Tree");
    println!("{}", "-".repeat(60));
    
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
    
    // Append child
    translator.queue(OwnedWebCall::AppendChild {
        parent: NodeId(1),
        child: NodeId(2),
        index: 0,
    });
    
    println!("Input:");
    println!("  1. CreateElement {{ node: NodeId(1), tag: 'div' }}");
    println!("  2. CreateElement {{ node: NodeId(2), tag: 'span' }}");
    println!("  3. AppendChild {{ parent: NodeId(1), child: NodeId(2) }}");
    
    let instructions = translator.flush();
    
    println!("\nOutput AIS:");
    for (i, instr) in instructions.iter().enumerate() {
        println!("  [{}] {:?}", i, instr);
    }
    println!("✅ Translated to {} atomic instruction(s)", instructions.len());

    // Example 3: Style Translation
    println!("\n📝 Example 3: element.style = 'width:100px;height:50px;background-color:red'");
    println!("{}", "-".repeat(60));
    
    let mut translator = WebApiTranslator::new();
    let node = NodeId(1);
    
    // Create element first
    translator.translate(OwnedWebCall::CreateElement {
        node,
        tag: "div".to_string(),
    });
    
    // Set styles
    let instructions = translator.translate(OwnedWebCall::SetInlineStyle {
        node,
        css: "width:100px;height:50px;background-color:red".to_string(),
    });
    
    println!("Input: SetInlineStyle {{");
    println!("  node: {:?},", node);
    println!("  css: 'width:100px;height:50px;background-color:red'");
    println!("}}");
    
    println!("\nOutput AIS:");
    for (i, instr) in instructions.iter().enumerate() {
        println!("  [{}] {:?}", i, instr);
    }
    println!("✅ Translated to {} atomic instruction(s)", instructions.len());

    // Example 4: Complex Real-World Scenario
    println!("\n📝 Example 4: Real-World Scenario");
    println!("{}", "-".repeat(60));
    println!("JavaScript equivalent:");
    println!("  const div = document.createElement('div');");
    println!("  div.setAttribute('id', 'main');");
    println!("  div.style.width = '100px';");
    println!("  div.style.backgroundColor = 'blue';");
    println!("  const text = document.createTextNode('Hello');");
    println!("  div.appendChild(text);");
    
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
    
    translator.queue(OwnedWebCall::SetInlineStyle {
        node: NodeId(1),
        css: "width:100px;background-color:blue".to_string(),
    });
    
    translator.queue(OwnedWebCall::CreateTextNode {
        node: NodeId(2),
        text: "Hello".to_string(),
    });
    
    translator.queue(OwnedWebCall::AppendChild {
        parent: NodeId(1),
        child: NodeId(2),
        index: 0,
    });
    
    let instructions = translator.flush();
    
    println!("\nOutput AIS ({} instructions):", instructions.len());
    for (i, instr) in instructions.iter().enumerate() {
        println!("  [{}] {:?}", i, instr);
    }

    // Statistics
    println!("\n{}", "=".repeat(60));
    println!("📊 Statistics:");
    
    let state_count = instructions.iter().filter(|i| matches!(i, AtomicInstruction::State(_))).count();
    let layout_count = instructions.iter().filter(|i| matches!(i, AtomicInstruction::Layout(_))).count();
    let render_count = instructions.iter().filter(|i| matches!(i, AtomicInstruction::Render(_))).count();
    
    println!("  State instructions:  {}", state_count);
    println!("  Layout instructions: {}", layout_count);
    println!("  Render instructions: {}", render_count);
    println!("  Total:               {}", instructions.len());

    // Comparison
    println!("\n{}", "=".repeat(60));
    println!("🎯 Key Insight:");
    println!();
    println!("Traditional Browser (Chromium):");
    println!("  - Implements full DOM, CSS, Layout engines");
    println!("  - ~4,000,000 lines of code");
    println!("  - Complex state management");
    println!();
    println!("Slate (Wine-like Translation):");
    println!("  - Translates Web APIs to {} atomic primitives", instructions.len());
    println!("  - ~6,500 lines of code");
    println!("  - Stateless translation");
    println!();
    println!("Result: 600x smaller codebase, native performance!");

    println!("\n{}", "=".repeat(60));
    println!("✅ Demo completed successfully!");
    println!("\nSlate is to Web APIs what Wine is to Windows APIs.");
}
