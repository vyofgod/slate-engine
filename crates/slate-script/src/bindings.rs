//! JavaScript bindings for DOM API.

use super::runtime::{JsRuntime, ScriptError};
use slate_ais::NodeId;
use slate_dispatcher::OwnedWebCall;
use std::sync::{Arc, Mutex};

/// DOM bindings for JavaScript.
pub struct DomBindings {
    web_calls: Arc<Mutex<Vec<OwnedWebCall>>>,
    next_node_id: Arc<Mutex<u32>>,
}

impl DomBindings {
    /// Create new DOM bindings.
    pub fn new() -> Self {
        Self {
            web_calls: Arc::new(Mutex::new(Vec::new())),
            next_node_id: Arc::new(Mutex::new(1000)), // Start from 1000 to avoid conflicts
        }
    }

    /// Install bindings into JavaScript runtime.
    pub fn install(&self, runtime: &mut JsRuntime) -> Result<(), ScriptError> {
        // Install document object
        self.install_document(runtime)?;

        // Install element prototype
        self.install_element_prototype(runtime)?;

        // Install console object
        self.install_console(runtime)?;

        Ok(())
    }

    /// Install document object.
    fn install_document(&self, runtime: &mut JsRuntime) -> Result<(), ScriptError> {
        let web_calls = Arc::clone(&self.web_calls);
        let next_node_id = Arc::clone(&self.next_node_id);

        // document.createElement
        runtime.register_function("__slate_createElement", move |args| {
            let tag_name = args.get(0)
                .and_then(|v| v.as_string())
                .ok_or("createElement requires tag name")?;

            let mut node_id_lock = next_node_id.lock().unwrap();
            let node_id = NodeId(*node_id_lock);
            *node_id_lock += 1;

            web_calls.lock().unwrap().push(OwnedWebCall::CreateElement {
                node: node_id,
                tag: tag_name.to_string(),
            });

            Ok(JsValue::Number(node_id.0 as f64))
        })?;

        let _web_calls = Arc::clone(&self.web_calls);

        // document.getElementById
        runtime.register_function("__slate_getElementById", move |args| {
            let _id = args.get(0)
                .and_then(|v| v.as_string())
                .ok_or("getElementById requires id")?;

            // In real implementation, query the DOM
            // For now, return null
            Ok(JsValue::Null)
        })?;

        let _web_calls = Arc::clone(&self.web_calls);

        // document.querySelector
        runtime.register_function("__slate_querySelector", move |args| {
            let _selector = args.get(0)
                .and_then(|v| v.as_string())
                .ok_or("querySelector requires selector")?;

            // In real implementation, query the DOM
            Ok(JsValue::Null)
        })?;

        Ok(())
    }

    /// Install element prototype methods.
    fn install_element_prototype(&self, runtime: &mut JsRuntime) -> Result<(), ScriptError> {
        let web_calls = Arc::clone(&self.web_calls);

        // element.appendChild
        runtime.register_function("__slate_appendChild", move |args| {
            let parent = args.get(0)
                .and_then(|v| v.as_number())
                .ok_or("appendChild requires parent node")?;

            let child = args.get(1)
                .and_then(|v| v.as_number())
                .ok_or("appendChild requires child node")?;

            web_calls.lock().unwrap().push(OwnedWebCall::AppendChild {
                parent: NodeId(parent as u32),
                child: NodeId(child as u32),
                index: 0, // Will be calculated by DOM
            });

            Ok(JsValue::Undefined)
        })?;

        let web_calls = Arc::clone(&self.web_calls);

        // element.setAttribute
        runtime.register_function("__slate_setAttribute", move |args| {
            let node = args.get(0)
                .and_then(|v| v.as_number())
                .ok_or("setAttribute requires node")?;

            let name = args.get(1)
                .and_then(|v| v.as_string())
                .ok_or("setAttribute requires attribute name")?;

            let value = args.get(2)
                .and_then(|v| v.as_string())
                .ok_or("setAttribute requires attribute value")?;

            web_calls.lock().unwrap().push(OwnedWebCall::SetAttribute {
                node: NodeId(node as u32),
                name: name.to_string(),
                value: value.to_string(),
            });

            Ok(JsValue::Undefined)
        })?;

        let web_calls = Arc::clone(&self.web_calls);

        // element.style.setProperty
        runtime.register_function("__slate_setStyle", move |args| {
            let node = args.get(0)
                .and_then(|v| v.as_number())
                .ok_or("setStyle requires node")?;

            let css = args.get(1)
                .and_then(|v| v.as_string())
                .ok_or("setStyle requires CSS")?;

            web_calls.lock().unwrap().push(OwnedWebCall::SetInlineStyle {
                node: NodeId(node as u32),
                css: css.to_string(),
            });

            Ok(JsValue::Undefined)
        })?;

        let _web_calls = Arc::clone(&self.web_calls);

        // element.innerHTML setter
        runtime.register_function("__slate_setInnerHTML", move |args| {
            let _node = args.get(0)
                .and_then(|v| v.as_number())
                .ok_or("setInnerHTML requires node")?;

            let _html = args.get(1)
                .and_then(|v| v.as_string())
                .ok_or("setInnerHTML requires HTML")?;

            // Parse HTML and generate WebCalls
            // For now, just create a text node
            Ok(JsValue::Undefined)
        })?;

        Ok(())
    }

    /// Install console object.
    fn install_console(&self, runtime: &mut JsRuntime) -> Result<(), ScriptError> {
        // console.log
        runtime.register_function("__slate_console_log", move |args| {
            let message = args.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            println!("[JS] {}", message);
            Ok(JsValue::Undefined)
        })?;

        // console.error
        runtime.register_function("__slate_console_error", move |args| {
            let message = args.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            eprintln!("[JS ERROR] {}", message);
            Ok(JsValue::Undefined)
        })?;

        // console.warn
        runtime.register_function("__slate_console_warn", move |args| {
            let message = args.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(" ");

            println!("[JS WARN] {}", message);
            Ok(JsValue::Undefined)
        })?;

        Ok(())
    }

    /// Take all pending WebCalls.
    pub fn take_web_calls(&self) -> Vec<OwnedWebCall> {
        std::mem::take(&mut *self.web_calls.lock().unwrap())
    }
}

impl Default for DomBindings {
    fn default() -> Self {
        Self::new()
    }
}

/// JavaScript value types.
#[derive(Debug, Clone)]
pub enum JsValue {
    Undefined,
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Object,
}

impl JsValue {
    /// Convert to string.
    pub fn to_string(&self) -> String {
        match self {
            JsValue::Undefined => "undefined".to_string(),
            JsValue::Null => "null".to_string(),
            JsValue::Boolean(b) => b.to_string(),
            JsValue::Number(n) => n.to_string(),
            JsValue::String(s) => s.clone(),
            JsValue::Object => "[object Object]".to_string(),
        }
    }

    /// Try to get as string.
    pub fn as_string(&self) -> Option<&str> {
        match self {
            JsValue::String(s) => Some(s),
            _ => None,
        }
    }

    /// Try to get as number.
    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    /// Try to get as boolean.
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            JsValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_bindings() {
        let bindings = DomBindings::new();
        assert_eq!(bindings.take_web_calls().len(), 0);
    }

    #[test]
    fn js_value_conversions() {
        let num = JsValue::Number(42.0);
        assert_eq!(num.as_number(), Some(42.0));
        assert_eq!(num.to_string(), "42");

        let str_val = JsValue::String("hello".to_string());
        assert_eq!(str_val.as_string(), Some("hello"));
    }
}
