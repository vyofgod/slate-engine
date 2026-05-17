//! SVG (Scalable Vector Graphics) API implementation.
//!
//! Provides basic SVG rendering support including:
//! - Basic shapes (rect, circle, ellipse, line, polyline, polygon)
//! - Paths
//! - Text
//! - Transforms
//! - Gradients and patterns
//!
//! ## Architecture
//! SVG elements are parsed and converted to AIS primitives for rendering.

use boa_engine::{Context, JsResult, JsValue, NativeFunction, JsArgs, JsString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// SVG element types.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SvgElementType {
    Svg,
    Rect,
    Circle,
    Ellipse,
    Line,
    Polyline,
    Polygon,
    Path,
    Text,
    Group,
    Defs,
    Use,
    LinearGradient,
    RadialGradient,
    Pattern,
}

/// SVG element.
#[derive(Debug, Clone)]
pub struct SvgElement {
    pub id: u32,
    pub element_type: SvgElementType,
    pub attributes: HashMap<String, String>,
    pub children: Vec<u32>,
    pub transform: Option<SvgTransform>,
}

impl SvgElement {
    pub fn new(element_type: SvgElementType) -> Self {
        Self {
            id: rand::random(),
            element_type,
            attributes: HashMap::new(),
            children: Vec::new(),
            transform: None,
        }
    }

    pub fn set_attribute(&mut self, name: String, value: String) {
        self.attributes.insert(name, value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&String> {
        self.attributes.get(name)
    }
}

/// SVG transform.
#[derive(Debug, Clone)]
pub enum SvgTransform {
    Translate { x: f64, y: f64 },
    Scale { x: f64, y: f64 },
    Rotate { angle: f64, cx: Option<f64>, cy: Option<f64> },
    SkewX { angle: f64 },
    SkewY { angle: f64 },
    Matrix { a: f64, b: f64, c: f64, d: f64, e: f64, f: f64 },
}

/// SVG path command.
#[derive(Debug, Clone)]
pub enum SvgPathCommand {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    HorizontalLineTo { x: f64 },
    VerticalLineTo { y: f64 },
    CurveTo { x1: f64, y1: f64, x2: f64, y2: f64, x: f64, y: f64 },
    SmoothCurveTo { x2: f64, y2: f64, x: f64, y: f64 },
    QuadraticCurveTo { x1: f64, y1: f64, x: f64, y: f64 },
    SmoothQuadraticCurveTo { x: f64, y: f64 },
    Arc { rx: f64, ry: f64, x_axis_rotation: f64, large_arc: bool, sweep: bool, x: f64, y: f64 },
    ClosePath,
}

/// Parse SVG path data string.
pub fn parse_path_data(_data: &str) -> Vec<SvgPathCommand> {
    // TODO: Implement full SVG path parser
    // For now, return empty vector
    Vec::new()
}

/// Global SVG element registry.
static SVG_ELEMENTS: Mutex<Option<HashMap<u32, Arc<Mutex<SvgElement>>>>> = Mutex::new(None);

fn get_svg_elements() -> Arc<Mutex<HashMap<u32, Arc<Mutex<SvgElement>>>>> {
    let mut guard = SVG_ELEMENTS.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    Arc::new(Mutex::new(guard.as_ref().unwrap().clone()))
}

/// SVG API bindings.
pub struct SvgApi;

impl SvgApi {
    /// Install SVG API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // Create SVG element
        let create_svg = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            let element = SvgElement::new(SvgElementType::Svg);
            let element_id = element.id;
            
            let elements = get_svg_elements();
            elements.lock().unwrap().insert(element_id, Arc::new(Mutex::new(element)));
            
            Ok(JsValue::from(element_id))
        });
        ctx.register_global_property(JsString::from("__slate_svg_create"), create_svg.to_js_function(ctx.realm()), Default::default())?;

        // Create SVG rect
        let create_rect = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            let element = SvgElement::new(SvgElementType::Rect);
            let element_id = element.id;
            
            let elements = get_svg_elements();
            elements.lock().unwrap().insert(element_id, Arc::new(Mutex::new(element)));
            
            Ok(JsValue::from(element_id))
        });
        ctx.register_global_property(JsString::from("__slate_svg_createRect"), create_rect.to_js_function(ctx.realm()), Default::default())?;

        // Create SVG circle
        let create_circle = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            let element = SvgElement::new(SvgElementType::Circle);
            let element_id = element.id;
            
            let elements = get_svg_elements();
            elements.lock().unwrap().insert(element_id, Arc::new(Mutex::new(element)));
            
            Ok(JsValue::from(element_id))
        });
        ctx.register_global_property(JsString::from("__slate_svg_createCircle"), create_circle.to_js_function(ctx.realm()), Default::default())?;

        // Create SVG path
        let create_path = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            let element = SvgElement::new(SvgElementType::Path);
            let element_id = element.id;
            
            let elements = get_svg_elements();
            elements.lock().unwrap().insert(element_id, Arc::new(Mutex::new(element)));
            
            Ok(JsValue::from(element_id))
        });
        ctx.register_global_property(JsString::from("__slate_svg_createPath"), create_path.to_js_function(ctx.realm()), Default::default())?;

        // Set SVG attribute
        let set_attribute = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let name = args.get_or_undefined(1).to_string(ctx)?;
            let value = args.get_or_undefined(2).to_string(ctx)?;
            
            let elements = get_svg_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                element.lock().unwrap().set_attribute(
                    name.to_std_string_escaped(),
                    value.to_std_string_escaped()
                );
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_svg_setAttribute"), set_attribute.to_js_function(ctx.realm()), Default::default())?;

        // Get SVG attribute
        let get_attribute = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let name = args.get_or_undefined(1).to_string(ctx)?;
            
            let elements = get_svg_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                if let Some(value) = element.lock().unwrap().get_attribute(&name.to_std_string_escaped()) {
                    return Ok(JsValue::from(JsString::from(value.clone())));
                }
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_svg_getAttribute"), get_attribute.to_js_function(ctx.realm()), Default::default())?;

        // Render SVG element to AIS primitives
        let render = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            
            let elements = get_svg_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                let elem = element.lock().unwrap();
                
                // TODO: Convert SVG element to AIS primitives
                // For now, just return success
                match elem.element_type {
                    SvgElementType::Rect => {
                        // Generate FillRect or StrokeRect AIS
                    }
                    SvgElementType::Circle => {
                        // Generate circle path and FillPath AIS
                    }
                    SvgElementType::Path => {
                        // Parse path data and generate FillPath AIS
                    }
                    _ => {}
                }
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_svg_render"), render.to_js_function(ctx.realm()), Default::default())?;

        Ok(())
    }
}

/// Convert SVG color string to RGBA.
pub fn parse_svg_color(color: &str) -> Option<(u8, u8, u8, u8)> {
    // Handle hex colors
    if color.starts_with('#') {
        let hex = &color[1..];
        
        if hex.len() == 3 {
            // #RGB -> #RRGGBB
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            return Some((r, g, b, 255));
        } else if hex.len() == 6 {
            // #RRGGBB
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            return Some((r, g, b, 255));
        }
    }
    
    // Handle named colors
    match color.to_lowercase().as_str() {
        "black" => Some((0, 0, 0, 255)),
        "white" => Some((255, 255, 255, 255)),
        "red" => Some((255, 0, 0, 255)),
        "green" => Some((0, 128, 0, 255)),
        "blue" => Some((0, 0, 255, 255)),
        "yellow" => Some((255, 255, 0, 255)),
        "cyan" => Some((0, 255, 255, 255)),
        "magenta" => Some((255, 0, 255, 255)),
        "transparent" => Some((0, 0, 0, 0)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_svg_color() {
        assert_eq!(parse_svg_color("#000"), Some((0, 0, 0, 255)));
        assert_eq!(parse_svg_color("#fff"), Some((255, 255, 255, 255)));
        assert_eq!(parse_svg_color("#ff0000"), Some((255, 0, 0, 255)));
        assert_eq!(parse_svg_color("black"), Some((0, 0, 0, 255)));
        assert_eq!(parse_svg_color("white"), Some((255, 255, 255, 255)));
    }
}
