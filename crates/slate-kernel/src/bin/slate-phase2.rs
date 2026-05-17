//! # Slate Phase 2 Demo
//!
//! Full rendering pipeline: HTML → DOM → Layout → Paint → Raster

use slate_ais::{Point, Rect, Rgba8, Size};
use slate_css::CssParser;
use slate_dom::Dom;
use slate_html::HtmlParser;
use slate_layout::{Constraints, FlexContainer, FlexItem, FlexLayout, LayoutEngine};
use slate_rasterizer::{DisplayCommand, DisplayList, FrameBuffer, Painter};

fn main() {
    println!("🚀 Slate Engine - Phase 2 Demo");
    println!("================================\n");

    // Sample HTML
    let html = r#"
        <!DOCTYPE html>
        <html>
            <head>
                <title>Slate Phase 2</title>
            </head>
            <body>
                <div id="header" style="background-color: #3498db; padding: 20px;">
                    <h1>Welcome to Slate Engine</h1>
                </div>
                <div class="container">
                    <div class="box" style="background-color: #e74c3c;">Box 1</div>
                    <div class="box" style="background-color: #2ecc71;">Box 2</div>
                    <div class="box" style="background-color: #f39c12;">Box 3</div>
                </div>
                <footer style="background-color: #34495e; color: white;">
                    <p>Powered by Rust</p>
                </footer>
            </body>
        </html>
    "#;

    // Sample CSS
    let css = r#"
        body {
            margin: 0;
            padding: 0;
            font-family: Arial, sans-serif;
        }
        
        #header {
            padding: 20px;
            color: white;
        }
        
        .container {
            display: flex;
            padding: 20px;
            gap: 10px;
        }
        
        .box {
            width: 200px;
            height: 150px;
            padding: 10px;
            color: white;
        }
        
        footer {
            padding: 20px;
            text-align: center;
        }
    "#;

    println!("📄 Parsing HTML...");
    let mut parser = HtmlParser::new();
    let parse_result = parser.parse(html).expect("Failed to parse HTML");
    println!("   ✓ Parsed {} nodes", parse_result.tree.nodes.len());
    println!("   ✓ Generated {} WebCalls", parse_result.web_calls.len());

    println!("\n🎨 Parsing CSS...");
    let stylesheet = CssParser::parse_stylesheet(css).expect("Failed to parse CSS");
    println!("   ✓ Parsed {} rules", stylesheet.rules.len());

    println!("\n🌳 Building DOM...");
    let mut dom = Dom::new();
    
    // Build DOM from parse result
    let doc = dom.document().unwrap();
    let body = dom.create_element("body".to_string());
    dom.append_child(doc, body).unwrap();

    // Create header
    let header = dom.create_element("div".to_string());
    dom.set_attribute(header, "id".to_string(), "header".to_string()).unwrap();
    dom.set_attribute(header, "style".to_string(), "background-color: #3498db".to_string()).unwrap();
    dom.append_child(body, header).unwrap();

    let h1 = dom.create_element("h1".to_string());
    let h1_text = dom.create_text("Welcome to Slate Engine".to_string());
    dom.append_child(h1, h1_text).unwrap();
    dom.append_child(header, h1).unwrap();

    // Create container with boxes
    let container = dom.create_element("div".to_string());
    dom.set_attribute(container, "class".to_string(), "container".to_string()).unwrap();
    dom.append_child(body, container).unwrap();

    let colors = ["#e74c3c", "#2ecc71", "#f39c12"];
    let labels = ["Box 1", "Box 2", "Box 3"];
    let mut boxes = Vec::new();

    for (_i, (&color, &label)) in colors.iter().zip(labels.iter()).enumerate() {
        let box_div = dom.create_element("div".to_string());
        dom.set_attribute(box_div, "class".to_string(), "box".to_string()).unwrap();
        dom.set_attribute(box_div, "style".to_string(), format!("background-color: {}", color)).unwrap();
        
        let text = dom.create_text(label.to_string());
        dom.append_child(box_div, text).unwrap();
        dom.append_child(container, box_div).unwrap();
        
        boxes.push(box_div);
    }

    // Create footer
    let footer = dom.create_element("footer".to_string());
    dom.set_attribute(footer, "style".to_string(), "background-color: #34495e".to_string()).unwrap();
    let footer_text = dom.create_text("Powered by Rust".to_string());
    dom.append_child(footer, footer_text).unwrap();
    dom.append_child(body, footer).unwrap();

    println!("   ✓ Built DOM with {} nodes", dom.dirty_nodes().len());

    println!("\n📐 Computing Layout...");
    
    // Setup flexbox layout for container
    let mut flex_layout = FlexLayout::new();
    flex_layout.add_container(FlexContainer {
        node: container,
        direction: slate_layout::flexbox::FlexDirection::Row,
        wrap: slate_layout::flexbox::FlexWrap::NoWrap,
        justify_content: slate_layout::flexbox::JustifyContent::FlexStart,
        align_items: slate_layout::flexbox::AlignItems::Stretch,
        align_content: slate_layout::flexbox::AlignContent::Stretch,
        gap: 10.into(),
    });

    let flex_items: Vec<_> = boxes.iter().map(|&node| FlexItem {
        node,
        flex_grow: 0.0,
        flex_shrink: 0.0,
        flex_basis: 200.into(),
        align_self: None,
        order: 0,
    }).collect();

    flex_layout.add_items(container, flex_items);

    let constraints = Constraints {
        min_width: 0.into(),
        max_width: 800.into(),
        min_height: 0.into(),
        max_height: 600.into(),
    };

    let layout_result = flex_layout.layout(container, constraints);
    println!("   ✓ Generated {} layout primitives", layout_result.primitives.len());

    println!("\n🎨 Building Display List...");
    let mut display_list = DisplayList::new();

    // Background
    display_list.push(DisplayCommand::FillRect {
        rect: Rect {
            origin: Point { x: 0.into(), y: 0.into() },
            size: Size { w: 800.into(), h: 600.into() },
        },
        color: Rgba8::rgb(255, 255, 255),
    });

    // Header
    display_list.push(DisplayCommand::FillRect {
        rect: Rect {
            origin: Point { x: 0.into(), y: 0.into() },
            size: Size { w: 800.into(), h: 80.into() },
        },
        color: Rgba8::rgb(52, 152, 219),
    });

    display_list.push(DisplayCommand::DrawText {
        text: "Welcome to Slate Engine".to_string(),
        position: Point {
            x: 20.into(),
            y: 30.into(),
        },
        color: Rgba8::WHITE,
        font_size: 24.0,
    });

    // Boxes
    let box_colors = [
        Rgba8::rgb(231, 76, 60),
        Rgba8::rgb(46, 204, 113),
        Rgba8::rgb(243, 156, 18),
    ];

    for (i, &color) in box_colors.iter().enumerate() {
        let x = 20 + (i as i32 * 210);
        display_list.push(DisplayCommand::FillRect {
            rect: Rect {
                origin: Point { x: x.into(), y: 100.into() },
                size: Size { w: 200.into(), h: 150.into() },
            },
            color,
        });

        display_list.push(DisplayCommand::DrawText {
            text: format!("Box {}", i + 1),
            position: Point {
                x: (x + 10).into(),
                y: 120.into(),
            },
            color: Rgba8::WHITE,
            font_size: 16.0,
        });
    }

    // Footer
    display_list.push(DisplayCommand::FillRect {
        rect: Rect {
            origin: Point { x: 0.into(), y: 270.into() },
            size: Size { w: 800.into(), h: 60.into() },
        },
        color: Rgba8::rgb(52, 73, 94),
    });

    display_list.push(DisplayCommand::DrawText {
        text: "Powered by Rust".to_string(),
        position: Point {
            x: 320.into(),
            y: 290.into(),
        },
        color: Rgba8::WHITE,
        font_size: 14.0,
    });

    println!("   ✓ Built display list with {} commands", display_list.len());

    println!("\n🖼️  Rasterizing...");
    let mut frame_buffer = FrameBuffer::new(800, 600);
    frame_buffer.clear(Rgba8::WHITE);

    let mut painter = Painter::new();
    painter.paint(&display_list, &mut frame_buffer);

    println!("   ✓ Rasterized to 800x600 frame buffer");

    println!("\n💾 Saving output...");
    frame_buffer
        .save_ppm("output/phase2-demo.ppm")
        .expect("Failed to save frame buffer");

    println!("   ✓ Saved to output/phase2-demo.ppm");

    println!("\n✅ Phase 2 Demo Complete!");
    println!("\nPipeline Summary:");
    println!("  HTML → {} nodes", parse_result.tree.nodes.len());
    println!("  CSS  → {} rules", stylesheet.rules.len());
    println!("  DOM  → {} elements", dom.dirty_nodes().len());
    println!("  Layout → {} primitives", layout_result.primitives.len());
    println!("  Display List → {} commands", display_list.len());
    println!("  Output → 800x600 pixels");
}
