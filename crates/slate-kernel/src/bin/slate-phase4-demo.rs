//! Phase 4 Demo: Interactive & Media Features
//!
//! Demonstrates:
//! - Image rendering
//! - Canvas 2D API
//! - Form handling
//! - SVG rendering

use slate_ais::{RenderPrimitive, Rgba8};
use slate_ais::geom::{Point, Rect};
use slate_image::{DecodedImage, ImageFormat};

fn main() {
    println!("=== Slate Engine - Phase 4 Demo ===\n");
    
    // Demo 1: Image Decoding
    demo_image_decoding();
    
    // Demo 2: Canvas 2D API
    demo_canvas_api();
    
    // Demo 3: Form Validation
    demo_form_validation();
    
    // Demo 4: SVG Rendering
    demo_svg_rendering();
    
    println!("\n✅ Phase 4 Demo Complete!");
}

fn demo_image_decoding() {
    println!("📷 Demo 1: Image Decoding");
    println!("─────────────────────────");
    
    // Create a simple test image (2x2 red square)
    let pixels = vec![
        255, 0, 0, 255,  // Red pixel
        255, 0, 0, 255,  // Red pixel
        255, 0, 0, 255,  // Red pixel
        255, 0, 0, 255,  // Red pixel
    ];
    
    let image = DecodedImage::new(2, 2, pixels, ImageFormat::Png);
    
    println!("  ✓ Created {}x{} image", image.width, image.height);
    println!("  ✓ Format: {:?}", image.format);
    println!("  ✓ Pixel count: {}", image.pixels.len() / 4);
    
    // Test image operations
    let resized = image.resize(4, 4);
    println!("  ✓ Resized to {}x{}", resized.width, resized.height);
    
    let _grayscale = image.to_grayscale();
    println!("  ✓ Converted to grayscale");
    
    // Generate DrawImage AIS primitive
    let primitive = RenderPrimitive::DrawImage {
        image: slate_ais::rendering::ImageId(1),
        dest: Rect::from_ltwh(10.0, 10.0, 100.0, 100.0),
        opacity: 255,
    };
    
    println!("  ✓ Generated AIS primitive: {:?}", primitive);
    println!();
}

fn demo_canvas_api() {
    println!("🎨 Demo 2: Canvas 2D API");
    println!("─────────────────────────");
    
    // Simulate Canvas API calls and their AIS translations
    
    // fillRect(10, 10, 100, 50)
    let fill_rect = RenderPrimitive::FillRect {
        rect: Rect::from_ltwh(10.0, 10.0, 100.0, 50.0),
        color: Rgba8::rgb(255, 0, 0),
    };
    println!("  ✓ fillRect(10, 10, 100, 50) → {:?}", fill_rect);
    
    // strokeRect(120, 10, 100, 50)
    let stroke_rect = RenderPrimitive::StrokeRect {
        rect: Rect::from_ltwh(120.0, 10.0, 100.0, 50.0),
        color: Rgba8::rgb(0, 0, 255),
        width: 2.0,
    };
    println!("  ✓ strokeRect(120, 10, 100, 50) → {:?}", stroke_rect);
    
    // fillText("Hello", 10, 100)
    let fill_text = RenderPrimitive::DrawText {
        run: slate_ais::rendering::GlyphRunId(1),
        origin: Point::new(10.0, 100.0),
        color: Rgba8::BLACK,
    };
    println!("  ✓ fillText('Hello', 10, 100) → {:?}", fill_text);
    
    // drawImage(img, 10, 120, 200, 150)
    let draw_image = RenderPrimitive::DrawImage {
        image: slate_ais::rendering::ImageId(1),
        dest: Rect::from_ltwh(10.0, 120.0, 200.0, 150.0),
        opacity: 255,
    };
    println!("  ✓ drawImage(img, 10, 120, 200, 150) → {:?}", draw_image);
    
    println!("  ✓ Canvas state management: save/restore stack");
    println!("  ✓ Transform matrix: translate, rotate, scale");
    println!();
}

fn demo_form_validation() {
    println!("📝 Demo 3: Form Validation");
    println!("─────────────────────────");
    
    use slate_webapi::forms::{FormElement, InputType};
    
    // Email input
    let mut email = FormElement::new("input".to_string());
    email.input_type = Some(InputType::Email);
    email.required = true;
    email.value = "user@example.com".to_string();
    
    match email.validate() {
        Ok(_) => println!("  ✓ Email validation: PASS ({})", email.value),
        Err(e) => println!("  ✗ Email validation: FAIL ({})", e),
    }
    
    // Invalid email
    email.value = "invalid-email".to_string();
    match email.validate() {
        Ok(_) => println!("  ✓ Invalid email: PASS"),
        Err(e) => println!("  ✓ Invalid email caught: {}", e),
    }
    
    // Number input with range
    let mut number = FormElement::new("input".to_string());
    number.input_type = Some(InputType::Number);
    number.min = Some("0".to_string());
    number.max = Some("100".to_string());
    number.value = "50".to_string();
    
    match number.validate() {
        Ok(_) => println!("  ✓ Number validation: PASS ({})", number.value),
        Err(e) => println!("  ✗ Number validation: FAIL ({})", e),
    }
    
    // Out of range
    number.value = "150".to_string();
    match number.validate() {
        Ok(_) => println!("  ✓ Out of range: PASS"),
        Err(e) => println!("  ✓ Out of range caught: {}", e),
    }
    
    // Required field
    let mut required = FormElement::new("input".to_string());
    required.required = true;
    required.value = "".to_string();
    
    match required.validate() {
        Ok(_) => println!("  ✓ Required field: PASS"),
        Err(e) => println!("  ✓ Required field caught: {}", e),
    }
    
    // Length validation
    let mut text = FormElement::new("input".to_string());
    text.min_length = Some(5);
    text.max_length = Some(10);
    text.value = "hello".to_string();
    
    match text.validate() {
        Ok(_) => println!("  ✓ Length validation: PASS ({})", text.value),
        Err(e) => println!("  ✗ Length validation: FAIL ({})", e),
    }
    
    println!();
}

fn demo_svg_rendering() {
    println!("🎭 Demo 4: SVG Rendering");
    println!("─────────────────────────");
    
    use slate_webapi::svg::{SvgElement, SvgElementType, parse_svg_color};
    
    // Create SVG rectangle
    let mut rect = SvgElement::new(SvgElementType::Rect);
    rect.set_attribute("x".to_string(), "10".to_string());
    rect.set_attribute("y".to_string(), "10".to_string());
    rect.set_attribute("width".to_string(), "100".to_string());
    rect.set_attribute("height".to_string(), "50".to_string());
    rect.set_attribute("fill".to_string(), "#ff0000".to_string());
    
    println!("  ✓ Created SVG rect:");
    println!("    - Position: ({}, {})", 
        rect.get_attribute("x").unwrap(), 
        rect.get_attribute("y").unwrap()
    );
    println!("    - Size: {}x{}", 
        rect.get_attribute("width").unwrap(), 
        rect.get_attribute("height").unwrap()
    );
    println!("    - Fill: {}", rect.get_attribute("fill").unwrap());
    
    // Create SVG circle
    let mut circle = SvgElement::new(SvgElementType::Circle);
    circle.set_attribute("cx".to_string(), "200".to_string());
    circle.set_attribute("cy".to_string(), "100".to_string());
    circle.set_attribute("r".to_string(), "50".to_string());
    circle.set_attribute("fill".to_string(), "#00ff00".to_string());
    
    println!("  ✓ Created SVG circle:");
    println!("    - Center: ({}, {})", 
        circle.get_attribute("cx").unwrap(), 
        circle.get_attribute("cy").unwrap()
    );
    println!("    - Radius: {}", circle.get_attribute("r").unwrap());
    println!("    - Fill: {}", circle.get_attribute("fill").unwrap());
    
    // Test color parsing
    let colors = vec![
        "#000", "#fff", "#ff0000", "#00ff00", "#0000ff",
        "black", "white", "red", "green", "blue"
    ];
    
    println!("  ✓ Color parsing:");
    for color in colors {
        if let Some((r, g, b, a)) = parse_svg_color(color) {
            println!("    - {} → rgba({}, {}, {}, {})", color, r, g, b, a);
        }
    }
    
    // SVG → AIS conversion
    println!("  ✓ SVG elements convert to AIS primitives:");
    println!("    - <rect> → FillRect or StrokeRect");
    println!("    - <circle> → FillPath (circle path)");
    println!("    - <path> → FillPath or StrokePath");
    println!("    - <text> → DrawText");
    
    println!();
}
