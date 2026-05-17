//! DOM API implementation.
//!
//! Implements the Document Object Model APIs including:
//! - Document interface
//! - Element interface
//! - Node interface
//! - HTMLElement and specific element types
//! - DOM manipulation methods

use boa_engine::{Context, JsResult, JsValue, NativeFunction};
use slate_ais::NodeId;
use slate_dispatcher::OwnedWebCall;
use std::sync::{Arc, Mutex};

/// DOM API bindings.
pub struct DomApi {
    web_calls: Arc<Mutex<Vec<OwnedWebCall>>>,
    next_node_id: Arc<Mutex<u32>>,
}

impl DomApi {
    /// Create new DOM API.
    pub fn new() -> Self {
        Self {
            web_calls: Arc::new(Mutex::new(Vec::new())),
            next_node_id: Arc::new(Mutex::new(1000)),
        }
    }

    /// Install DOM APIs into JavaScript context.
    pub fn install(&self, ctx: &mut Context) -> JsResult<()> {
        self.install_document(ctx)?;
        self.install_element(ctx)?;
        self.install_node(ctx)?;
        Ok(())
    }

    /// Install Document interface.
    fn install_document(&self, ctx: &mut Context) -> JsResult<()> {
        let web_calls = Arc::clone(&self.web_calls);
        let next_node_id = Arc::clone(&self.next_node_id);

        // document.createElement
        let create_element = NativeFunction::from_fn_ptr(move |_, args, _| {
            let tag_name = args.get_or_undefined(0).to_string(ctx)?;
            
            let mut node_id_lock = next_node_id.lock().unwrap();
            let node_id = NodeId(*node_id_lock);
            *node_id_lock += 1;

            web_calls.lock().unwrap().push(OwnedWebCall::CreateElement {
                node: node_id,
                tag: tag_name.to_std_string_escaped(),
            });

            Ok(JsValue::from(node_id.0 as f64))
        });

        ctx.register_global_property("__slate_createElement", create_element, Default::default())?;

        // document.getElementById
        let get_element_by_id = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _id = args.get_or_undefined(0).to_string(ctx)?;
            // TODO: Query actual DOM
            Ok(JsValue::null())
        });

        ctx.register_global_property("__slate_getElementById", get_element_by_id, Default::default())?;

        // document.querySelector
        let query_selector = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _selector = args.get_or_undefined(0).to_string(ctx)?;
            // TODO: Query actual DOM with CSS selector
            Ok(JsValue::null())
        });

        ctx.register_global_property("__slate_querySelector", query_selector, Default::default())?;

        // document.querySelectorAll
        let query_selector_all = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _selector = args.get_or_undefined(0).to_string(ctx)?;
            // TODO: Query actual DOM with CSS selector
            // Return NodeList
            Ok(JsValue::null())
        });

        ctx.register_global_property("__slate_querySelectorAll", query_selector_all, Default::default())?;

        // document.createTextNode
        let web_calls = Arc::clone(&self.web_calls);
        let next_node_id = Arc::clone(&self.next_node_id);

        let create_text_node = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let text = args.get_or_undefined(0).to_string(ctx)?;
            
            let mut node_id_lock = next_node_id.lock().unwrap();
            let node_id = NodeId(*node_id_lock);
            *node_id_lock += 1;

            web_calls.lock().unwrap().push(OwnedWebCall::CreateTextNode {
                node: node_id,
                text: text.to_std_string_escaped(),
            });

            Ok(JsValue::from(node_id.0 as f64))
        });

        ctx.register_global_property("__slate_createTextNode", create_text_node, Default::default())?;

        Ok(())
    }

    /// Install Element interface.
    fn install_element(&self, ctx: &mut Context) -> JsResult<()> {
        let web_calls = Arc::clone(&self.web_calls);

        // element.setAttribute
        let set_attribute = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let name = args.get_or_undefined(1).to_string(ctx)?;
            let value = args.get_or_undefined(2).to_string(ctx)?;

            web_calls.lock().unwrap().push(OwnedWebCall::SetAttribute {
                node: NodeId(node),
                name: name.to_std_string_escaped(),
                value: value.to_std_string_escaped(),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_setAttribute", set_attribute, Default::default())?;

        // element.getAttribute
        let get_attribute = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let _node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let _name = args.get_or_undefined(1).to_string(ctx)?;
            // TODO: Query actual DOM
            Ok(JsValue::null())
        });

        ctx.register_global_property("__slate_getAttribute", get_attribute, Default::default())?;

        // element.removeAttribute
        let web_calls = Arc::clone(&self.web_calls);

        let remove_attribute = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let name = args.get_or_undefined(1).to_string(ctx)?;

            web_calls.lock().unwrap().push(OwnedWebCall::RemoveAttribute {
                node: NodeId(node),
                name: name.to_std_string_escaped(),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_removeAttribute", remove_attribute, Default::default())?;

        // element.classList operations
        let web_calls = Arc::clone(&self.web_calls);

        let add_class = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let class_name = args.get_or_undefined(1).to_string(ctx)?;

            // Get current class attribute and append
            web_calls.lock().unwrap().push(OwnedWebCall::AddClass {
                node: NodeId(node),
                class: class_name.to_std_string_escaped(),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_addClass", add_class, Default::default())?;

        // element.style.setProperty
        let web_calls = Arc::clone(&self.web_calls);

        let set_style = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let node = args.get_or_undefined(0).to_number(ctx)? as u32;
            let css = args.get_or_undefined(1).to_string(ctx)?;

            web_calls.lock().unwrap().push(OwnedWebCall::SetInlineStyle {
                node: NodeId(node),
                css: css.to_std_string_escaped(),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_setStyle", set_style, Default::default())?;

        Ok(())
    }

    /// Install Node interface.
    fn install_node(&self, ctx: &mut Context) -> JsResult<()> {
        let web_calls = Arc::clone(&self.web_calls);

        // node.appendChild
        let append_child = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let parent = args.get_or_undefined(0).to_number(ctx)? as u32;
            let child = args.get_or_undefined(1).to_number(ctx)? as u32;

            web_calls.lock().unwrap().push(OwnedWebCall::AppendChild {
                parent: NodeId(parent),
                child: NodeId(child),
                index: 0, // Will be calculated by DOM
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_appendChild", append_child, Default::default())?;

        // node.removeChild
        let web_calls = Arc::clone(&self.web_calls);

        let remove_child = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let parent = args.get_or_undefined(0).to_number(ctx)? as u32;
            let child = args.get_or_undefined(1).to_number(ctx)? as u32;

            web_calls.lock().unwrap().push(OwnedWebCall::RemoveChild {
                parent: NodeId(parent),
                child: NodeId(child),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_removeChild", remove_child, Default::default())?;

        // node.insertBefore
        let web_calls = Arc::clone(&self.web_calls);

        let insert_before = NativeFunction::from_fn_ptr(move |_, args, ctx| {
            let parent = args.get_or_undefined(0).to_number(ctx)? as u32;
            let new_child = args.get_or_undefined(1).to_number(ctx)? as u32;
            let ref_child = args.get_or_undefined(2).to_number(ctx)? as u32;

            web_calls.lock().unwrap().push(OwnedWebCall::InsertBefore {
                parent: NodeId(parent),
                new_child: NodeId(new_child),
                ref_child: NodeId(ref_child),
            });

            Ok(JsValue::undefined())
        });

        ctx.register_global_property("__slate_insertBefore", insert_before, Default::default())?;

        Ok(())
    }

    /// Take all pending WebCalls.
    pub fn take_web_calls(&self) -> Vec<OwnedWebCall> {
        std::mem::take(&mut *self.web_calls.lock().unwrap())
    }
}

impl Default for DomApi {
    fn default() -> Self {
        Self::new()
    }
}
