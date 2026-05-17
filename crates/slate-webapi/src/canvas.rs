//! Canvas 2D API implementation.
//!
//! Provides full Canvas 2D Context API for drawing graphics, text, and images.
//! 
//! ## Features
//! - Rectangle drawing (fill, stroke, clear)
//! - Path drawing (lines, curves, arcs)
//! - Text rendering
//! - Image drawing
//! - Transformations (translate, rotate, scale)
//! - Compositing and blending
//! - Gradients and patterns
//! - State management (save/restore)

use boa_engine::{Context, JsResult, JsValue, NativeFunction, JsObject, JsArgs, JsString};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Canvas 2D rendering context state.
#[derive(Debug, Clone)]
pub struct CanvasState {
    /// Fill style (color or gradient)
    pub fill_style: String,
    
    /// Stroke style (color or gradient)
    pub stroke_style: String,
    
    /// Line width
    pub line_width: f64,
    
    /// Line cap style
    pub line_cap: LineCap,
    
    /// Line join style
    pub line_join: LineJoin,
    
    /// Global alpha (opacity)
    pub global_alpha: f64,
    
    /// Global composite operation
    pub global_composite_operation: String,
    
    /// Font string
    pub font: String,
    
    /// Text align
    pub text_align: TextAlign,
    
    /// Text baseline
    pub text_baseline: TextBaseline,
    
    /// Shadow blur
    pub shadow_blur: f64,
    
    /// Shadow color
    pub shadow_color: String,
    
    /// Shadow offset X
    pub shadow_offset_x: f64,
    
    /// Shadow offset Y
    pub shadow_offset_y: f64,
    
    /// Current transformation matrix
    pub transform: [f64; 6], // [a, b, c, d, e, f]
}

impl Default for CanvasState {
    fn default() -> Self {
        Self {
            fill_style: "#000000".to_string(),
            stroke_style: "#000000".to_string(),
            line_width: 1.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            global_alpha: 1.0,
            global_composite_operation: "source-over".to_string(),
            font: "10px sans-serif".to_string(),
            text_align: TextAlign::Start,
            text_baseline: TextBaseline::Alphabetic,
            shadow_blur: 0.0,
            shadow_color: "transparent".to_string(),
            shadow_offset_x: 0.0,
            shadow_offset_y: 0.0,
            transform: [1.0, 0.0, 0.0, 1.0, 0.0, 0.0], // Identity matrix
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineCap {
    Butt,
    Round,
    Square,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineJoin {
    Miter,
    Round,
    Bevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Start,
    End,
    Left,
    Right,
    Center,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextBaseline {
    Top,
    Hanging,
    Middle,
    Alphabetic,
    Ideographic,
    Bottom,
}

/// Canvas 2D rendering context.
pub struct Canvas2DContext {
    /// Canvas ID
    pub id: u32,
    
    /// Canvas width
    pub width: u32,
    
    /// Canvas height
    pub height: u32,
    
    /// Current state
    pub state: CanvasState,
    
    /// State stack for save/restore
    pub state_stack: Vec<CanvasState>,
    
    /// Current path
    pub current_path: Vec<PathCommand>,
}

#[derive(Debug, Clone)]
pub enum PathCommand {
    MoveTo { x: f64, y: f64 },
    LineTo { x: f64, y: f64 },
    BezierCurveTo { cp1x: f64, cp1y: f64, cp2x: f64, cp2y: f64, x: f64, y: f64 },
    QuadraticCurveTo { cpx: f64, cpy: f64, x: f64, y: f64 },
    Arc { x: f64, y: f64, radius: f64, start_angle: f64, end_angle: f64, anticlockwise: bool },
    ArcTo { x1: f64, y1: f64, x2: f64, y2: f64, radius: f64 },
    Rect { x: f64, y: f64, width: f64, height: f64 },
    ClosePath,
}

impl Canvas2DContext {
    pub fn new(id: u32, width: u32, height: u32) -> Self {
        Self {
            id,
            width,
            height,
            state: CanvasState::default(),
            state_stack: Vec::new(),
            current_path: Vec::new(),
        }
    }
}

/// Global canvas context registry.
static CANVAS_CONTEXTS: Mutex<Option<HashMap<u32, Arc<Mutex<Canvas2DContext>>>>> = Mutex::new(None);

fn get_contexts() -> Arc<Mutex<HashMap<u32, Arc<Mutex<Canvas2DContext>>>>> {
    let mut guard = CANVAS_CONTEXTS.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    Arc::new(Mutex::new(guard.as_ref().unwrap().clone()))
}

/// Canvas API bindings.
pub struct CanvasApi;

impl CanvasApi {
    /// Install Canvas API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // Create canvas element
        let create_canvas = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let width = args.get_or_undefined(0).to_number(ctx)? as u32;
            let height = args.get_or_undefined(1).to_number(ctx)? as u32;
            
            let canvas_id = rand::random::<u32>();
            let canvas_ctx = Arc::new(Mutex::new(Canvas2DContext::new(canvas_id, width, height)));
            
            let contexts = get_contexts();
            contexts.lock().unwrap().insert(canvas_id, canvas_ctx);
            
            Ok(JsValue::from(canvas_id))
        });

        ctx.register_global_property(JsString::from("__slate_canvas_create"), create_canvas.to_js_function(ctx.realm()), Default::default())?;

        // getContext('2d')
        let get_context = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _canvas_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let context_type = args.get_or_undefined(1).to_string(ctx)?;
            
            if context_type.to_std_string_escaped() == "2d" {
                // Return context object with methods
                let obj = JsObject::default();
                Ok(JsValue::from(obj))
            } else {
                Ok(JsValue::undefined())
            }
        });

        ctx.register_global_property(JsString::from("__slate_canvas_getContext"), get_context.to_js_function(ctx.realm()), Default::default())?;

        // Install 2D context methods
        Self::install_2d_context(ctx)?;

        Ok(())
    }

    fn install_2d_context(ctx: &mut Context) -> JsResult<()> {
        // === Rectangle Methods ===
        
        // fillRect(x, y, width, height)
        let fill_rect = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _x = args.get_or_undefined(1).to_number(ctx)?;
            let _y = args.get_or_undefined(2).to_number(ctx)?;
            let _width = args.get_or_undefined(3).to_number(ctx)?;
            let _height = args.get_or_undefined(4).to_number(ctx)?;
            
            // TODO: Generate FillRect AIS primitive
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_fillRect"), fill_rect.to_js_function(ctx.realm()), Default::default())?;

        // strokeRect(x, y, width, height)
        let stroke_rect = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _x = args.get_or_undefined(1).to_number(ctx)?;
            let _y = args.get_or_undefined(2).to_number(ctx)?;
            let _width = args.get_or_undefined(3).to_number(ctx)?;
            let _height = args.get_or_undefined(4).to_number(ctx)?;
            
            // TODO: Generate StrokeRect AIS primitive
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_strokeRect"), stroke_rect.to_js_function(ctx.realm()), Default::default())?;

        // clearRect(x, y, width, height)
        let clear_rect = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _x = args.get_or_undefined(1).to_number(ctx)?;
            let _y = args.get_or_undefined(2).to_number(ctx)?;
            let _width = args.get_or_undefined(3).to_number(ctx)?;
            let _height = args.get_or_undefined(4).to_number(ctx)?;
            
            // TODO: Clear rectangle (fill with transparent)
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_clearRect"), clear_rect.to_js_function(ctx.realm()), Default::default())?;

        // === Path Methods ===
        
        // beginPath()
        let begin_path = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.clear();
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_beginPath"), begin_path.to_js_function(ctx.realm()), Default::default())?;

        // closePath()
        let close_path = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.push(PathCommand::ClosePath);
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_closePath"), close_path.to_js_function(ctx.realm()), Default::default())?;

        // moveTo(x, y)
        let move_to = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let x = args.get_or_undefined(1).to_number(ctx)?;
            let y = args.get_or_undefined(2).to_number(ctx)?;
            
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.push(PathCommand::MoveTo { x, y });
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_moveTo"), move_to.to_js_function(ctx.realm()), Default::default())?;

        // lineTo(x, y)
        let line_to = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let x = args.get_or_undefined(1).to_number(ctx)?;
            let y = args.get_or_undefined(2).to_number(ctx)?;
            
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.push(PathCommand::LineTo { x, y });
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_lineTo"), line_to.to_js_function(ctx.realm()), Default::default())?;

        // arc(x, y, radius, startAngle, endAngle, anticlockwise)
        let arc = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let x = args.get_or_undefined(1).to_number(ctx)?;
            let y = args.get_or_undefined(2).to_number(ctx)?;
            let radius = args.get_or_undefined(3).to_number(ctx)?;
            let start_angle = args.get_or_undefined(4).to_number(ctx)?;
            let end_angle = args.get_or_undefined(5).to_number(ctx)?;
            let anticlockwise = args.get_or_undefined(6).to_boolean();
            
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.push(PathCommand::Arc {
                    x, y, radius, start_angle, end_angle, anticlockwise
                });
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_arc"), arc.to_js_function(ctx.realm()), Default::default())?;

        // bezierCurveTo(cp1x, cp1y, cp2x, cp2y, x, y)
        let bezier_curve_to = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let cp1x = args.get_or_undefined(1).to_number(ctx)?;
            let cp1y = args.get_or_undefined(2).to_number(ctx)?;
            let cp2x = args.get_or_undefined(3).to_number(ctx)?;
            let cp2y = args.get_or_undefined(4).to_number(ctx)?;
            let x = args.get_or_undefined(5).to_number(ctx)?;
            let y = args.get_or_undefined(6).to_number(ctx)?;
            
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                canvas_ctx.lock().unwrap().current_path.push(PathCommand::BezierCurveTo {
                    cp1x, cp1y, cp2x, cp2y, x, y
                });
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_bezierCurveTo"), bezier_curve_to.to_js_function(ctx.realm()), Default::default())?;

        // fill()
        let fill = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Fill current path
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_fill"), fill.to_js_function(ctx.realm()), Default::default())?;

        // stroke()
        let stroke = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            // TODO: Stroke current path
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_stroke"), stroke.to_js_function(ctx.realm()), Default::default())?;

        // === Text Methods ===
        
        // fillText(text, x, y, maxWidth?)
        let fill_text = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _text = args.get_or_undefined(1).to_string(ctx)?;
            let _x = args.get_or_undefined(2).to_number(ctx)?;
            let _y = args.get_or_undefined(3).to_number(ctx)?;
            
            // TODO: Generate DrawText AIS primitive
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_fillText"), fill_text.to_js_function(ctx.realm()), Default::default())?;

        // strokeText(text, x, y, maxWidth?)
        let stroke_text = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _text = args.get_or_undefined(1).to_string(ctx)?;
            let _x = args.get_or_undefined(2).to_number(ctx)?;
            let _y = args.get_or_undefined(3).to_number(ctx)?;
            
            // TODO: Stroke text outline
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_strokeText"), stroke_text.to_js_function(ctx.realm()), Default::default())?;

        // measureText(text)
        let measure_text = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _text = args.get_or_undefined(1).to_string(ctx)?;
            
            // TODO: Measure text width
            // Return TextMetrics object
            Ok(JsValue::from(100.0)) // Placeholder width
        });
        ctx.register_global_property(JsString::from("__slate_canvas_measureText"), measure_text.to_js_function(ctx.realm()), Default::default())?;

        // === Image Methods ===
        
        // drawImage(image, dx, dy) or drawImage(image, dx, dy, dw, dh) or 
        // drawImage(image, sx, sy, sw, sh, dx, dy, dw, dh)
        let draw_image = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _image_id = args.get_or_undefined(1).to_number(ctx)? as u32;
            
            // TODO: Handle different overloads and generate DrawImage AIS primitive
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_drawImage"), draw_image.to_js_function(ctx.realm()), Default::default())?;

        // === Transform Methods ===
        
        // save()
        let save = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                let mut ctx_guard = canvas_ctx.lock().unwrap();
                let state_clone = ctx_guard.state.clone();
                ctx_guard.state_stack.push(state_clone);
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_save"), save.to_js_function(ctx.realm()), Default::default())?;

        // restore()
        let restore = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let contexts = get_contexts();
            if let Some(canvas_ctx) = contexts.lock().unwrap().get(&ctx_id) {
                let mut ctx_guard = canvas_ctx.lock().unwrap();
                if let Some(state) = ctx_guard.state_stack.pop() {
                    ctx_guard.state = state;
                }
            }
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_restore"), restore.to_js_function(ctx.realm()), Default::default())?;

        // translate(x, y)
        let translate = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _x = args.get_or_undefined(1).to_number(ctx)?;
            let _y = args.get_or_undefined(2).to_number(ctx)?;
            
            // TODO: Update transformation matrix
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_translate"), translate.to_js_function(ctx.realm()), Default::default())?;

        // rotate(angle)
        let rotate = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _angle = args.get_or_undefined(1).to_number(ctx)?;
            
            // TODO: Update transformation matrix
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_rotate"), rotate.to_js_function(ctx.realm()), Default::default())?;

        // scale(x, y)
        let scale = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _ctx_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _x = args.get_or_undefined(1).to_number(ctx)?;
            let _y = args.get_or_undefined(2).to_number(ctx)?;
            
            // TODO: Update transformation matrix
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_canvas_scale"), scale.to_js_function(ctx.realm()), Default::default())?;

        Ok(())
    }
}
