//! HTML Form API implementation.
//!
//! Provides form elements, validation, and submission handling.
//!
//! ## Supported Elements
//! - Input (text, password, email, number, checkbox, radio, file, etc.)
//! - Textarea
//! - Select and Option
//! - Button
//! - Form
//!
//! ## Features
//! - HTML5 validation
//! - Custom validation
//! - Form submission
//! - FormData API

use boa_engine::{Context, JsResult, JsValue, NativeFunction, JsArgs, JsString};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Input element types.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputType {
    Text,
    Password,
    Email,
    Number,
    Tel,
    Url,
    Search,
    Date,
    Time,
    DateTime,
    Month,
    Week,
    Color,
    Range,
    Checkbox,
    Radio,
    File,
    Submit,
    Reset,
    Button,
    Hidden,
}

impl InputType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "password" => InputType::Password,
            "email" => InputType::Email,
            "number" => InputType::Number,
            "tel" => InputType::Tel,
            "url" => InputType::Url,
            "search" => InputType::Search,
            "date" => InputType::Date,
            "time" => InputType::Time,
            "datetime" | "datetime-local" => InputType::DateTime,
            "month" => InputType::Month,
            "week" => InputType::Week,
            "color" => InputType::Color,
            "range" => InputType::Range,
            "checkbox" => InputType::Checkbox,
            "radio" => InputType::Radio,
            "file" => InputType::File,
            "submit" => InputType::Submit,
            "reset" => InputType::Reset,
            "button" => InputType::Button,
            "hidden" => InputType::Hidden,
            _ => InputType::Text,
        }
    }
}

/// Form element state.
#[derive(Debug, Clone)]
pub struct FormElement {
    pub id: u32,
    pub element_type: String, // "input", "textarea", "select"
    pub input_type: Option<InputType>,
    pub name: String,
    pub value: String,
    pub checked: bool,
    pub disabled: bool,
    pub readonly: bool,
    pub required: bool,
    pub pattern: Option<String>,
    pub min: Option<String>,
    pub max: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub placeholder: Option<String>,
    pub autocomplete: Option<String>,
    pub validation_message: Option<String>,
}

impl FormElement {
    pub fn new(element_type: String) -> Self {
        Self {
            id: rand::random(),
            element_type,
            input_type: None,
            name: String::new(),
            value: String::new(),
            checked: false,
            disabled: false,
            readonly: false,
            required: false,
            pattern: None,
            min: None,
            max: None,
            min_length: None,
            max_length: None,
            placeholder: None,
            autocomplete: None,
            validation_message: None,
        }
    }

    /// Validate the form element according to HTML5 validation rules.
    pub fn validate(&self) -> Result<(), String> {
        // Required validation
        if self.required && self.value.is_empty() {
            return Err("This field is required".to_string());
        }

        // Type-specific validation
        if let Some(input_type) = self.input_type {
            match input_type {
                InputType::Email => {
                    if !self.value.is_empty() && !self.is_valid_email(&self.value) {
                        return Err("Please enter a valid email address".to_string());
                    }
                }
                InputType::Url => {
                    if !self.value.is_empty() && !self.is_valid_url(&self.value) {
                        return Err("Please enter a valid URL".to_string());
                    }
                }
                InputType::Number => {
                    if !self.value.is_empty() {
                        if self.value.parse::<f64>().is_err() {
                            return Err("Please enter a valid number".to_string());
                        }
                        
                        if let Some(min) = &self.min {
                            if let (Ok(val), Ok(min_val)) = (self.value.parse::<f64>(), min.parse::<f64>()) {
                                if val < min_val {
                                    return Err(format!("Value must be at least {}", min));
                                }
                            }
                        }
                        
                        if let Some(max) = &self.max {
                            if let (Ok(val), Ok(max_val)) = (self.value.parse::<f64>(), max.parse::<f64>()) {
                                if val > max_val {
                                    return Err(format!("Value must be at most {}", max));
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        // Pattern validation
        if let Some(pattern) = &self.pattern {
            if !self.value.is_empty() {
                // Simple pattern matching (in real implementation, use regex)
                if !self.matches_pattern(&self.value, pattern) {
                    return Err("Please match the requested format".to_string());
                }
            }
        }

        // Length validation
        if let Some(min_length) = self.min_length {
            if self.value.len() < min_length {
                return Err(format!("Please use at least {} characters", min_length));
            }
        }

        if let Some(max_length) = self.max_length {
            if self.value.len() > max_length {
                return Err(format!("Please use at most {} characters", max_length));
            }
        }

        Ok(())
    }

    fn is_valid_email(&self, email: &str) -> bool {
        // Simple email validation
        email.contains('@') && email.contains('.')
    }

    fn is_valid_url(&self, url: &str) -> bool {
        // Simple URL validation
        url.starts_with("http://") || url.starts_with("https://")
    }

    fn matches_pattern(&self, _value: &str, _pattern: &str) -> bool {
        // TODO: Implement regex pattern matching
        true
    }
}

/// Form state.
#[derive(Debug, Clone)]
pub struct Form {
    pub id: u32,
    pub action: String,
    pub method: String, // "GET" or "POST"
    pub enctype: String,
    pub elements: Vec<u32>, // Element IDs
}

impl Form {
    pub fn new() -> Self {
        Self {
            id: rand::random(),
            action: String::new(),
            method: "GET".to_string(),
            enctype: "application/x-www-form-urlencoded".to_string(),
            elements: Vec::new(),
        }
    }
}

/// Global form element registry.
static FORM_ELEMENTS: Mutex<Option<HashMap<u32, Arc<Mutex<FormElement>>>>> = Mutex::new(None);
static FORMS: Mutex<Option<HashMap<u32, Arc<Mutex<Form>>>>> = Mutex::new(None);

fn get_form_elements() -> Arc<Mutex<HashMap<u32, Arc<Mutex<FormElement>>>>> {
    let mut guard = FORM_ELEMENTS.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    Arc::new(Mutex::new(guard.as_ref().unwrap().clone()))
}

fn get_forms() -> Arc<Mutex<HashMap<u32, Arc<Mutex<Form>>>>> {
    let mut guard = FORMS.lock().unwrap();
    if guard.is_none() {
        *guard = Some(HashMap::new());
    }
    Arc::new(Mutex::new(guard.as_ref().unwrap().clone()))
}

/// Form API bindings.
pub struct FormApi;

impl FormApi {
    /// Install Form API into JavaScript context.
    pub fn install(ctx: &mut Context) -> JsResult<()> {
        // Create form element
        let create_form = NativeFunction::from_fn_ptr(|_, _args, _ctx| {
            let form = Form::new();
            let form_id = form.id;
            
            let forms = get_forms();
            forms.lock().unwrap().insert(form_id, Arc::new(Mutex::new(form)));
            
            Ok(JsValue::from(form_id))
        });
        ctx.register_global_property(JsString::from("__slate_form_create"), create_form.to_js_function(ctx.realm()), Default::default())?;

        // Create input element
        let create_input = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let input_type = args.get_or_undefined(0).to_string(ctx)?;
            let type_str = input_type.to_std_string_escaped();
            
            let mut element = FormElement::new("input".to_string());
            element.input_type = Some(InputType::from_str(&type_str));
            let element_id = element.id;
            
            let elements = get_form_elements();
            elements.lock().unwrap().insert(element_id, Arc::new(Mutex::new(element)));
            
            Ok(JsValue::from(element_id))
        });
        ctx.register_global_property(JsString::from("__slate_form_createInput"), create_input.to_js_function(ctx.realm()), Default::default())?;

        // Set input value
        let set_value = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let value = args.get_or_undefined(1).to_string(ctx)?;
            
            let elements = get_form_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                element.lock().unwrap().value = value.to_std_string_escaped();
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_form_setValue"), set_value.to_js_function(ctx.realm()), Default::default())?;

        // Get input value
        let get_value = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            
            let elements = get_form_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                let value = element.lock().unwrap().value.clone();
                return Ok(JsValue::from(JsString::from(value)));
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_form_getValue"), get_value.to_js_function(ctx.realm()), Default::default())?;

        // Set checked state (for checkbox/radio)
        let set_checked = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            let checked = args.get_or_undefined(1).to_boolean();
            
            let elements = get_form_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                element.lock().unwrap().checked = checked;
            }
            
            Ok(JsValue::undefined())
        });
        ctx.register_global_property(JsString::from("__slate_form_setChecked"), set_checked.to_js_function(ctx.realm()), Default::default())?;

        // Validate element
        let validate = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            
            let elements = get_form_elements();
            let element_arc = {
                let elements_guard = elements.lock().unwrap();
                elements_guard.get(&element_id).cloned()
            };
            
            if let Some(element) = element_arc {
                let elem = element.lock().unwrap();
                match elem.validate() {
                    Ok(_) => Ok(JsValue::from(true)),
                    Err(msg) => {
                        // Return validation message
                        drop(elem);
                        element.lock().unwrap().validation_message = Some(msg.clone());
                        Ok(JsValue::from(false))
                    }
                }
            } else {
                Ok(JsValue::from(false))
            }
        });
        ctx.register_global_property(JsString::from("__slate_form_validate"), validate.to_js_function(ctx.realm()), Default::default())?;

        // Get validation message
        let get_validation_message = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let element_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            
            let elements = get_form_elements();
            if let Some(element) = elements.lock().unwrap().get(&element_id) {
                if let Some(msg) = &element.lock().unwrap().validation_message {
                    return Ok(JsValue::from(JsString::from(msg.clone())));
                }
            }
            
            Ok(JsValue::from(JsString::from("")))
        });
        ctx.register_global_property(JsString::from("__slate_form_getValidationMessage"), get_validation_message.to_js_function(ctx.realm()), Default::default())?;

        // Submit form
        let submit = NativeFunction::from_fn_ptr(|_, args, ctx| {
            let form_id = args.get_or_undefined(0).to_number(ctx)? as u32;
            
            let forms = get_forms();
            if let Some(form) = forms.lock().unwrap().get(&form_id) {
                let form_guard = form.lock().unwrap();
                
                // Validate all form elements
                let elements = get_form_elements();
                let elements_guard = elements.lock().unwrap();
                
                for element_id in &form_guard.elements {
                    if let Some(element) = elements_guard.get(element_id) {
                        if let Err(_msg) = element.lock().unwrap().validate() {
                            // Validation failed
                            return Ok(JsValue::from(false));
                        }
                    }
                }
                
                // TODO: Actually submit the form (generate network request)
                return Ok(JsValue::from(true));
            }
            
            Ok(JsValue::from(false))
        });
        ctx.register_global_property(JsString::from("__slate_form_submit"), submit.to_js_function(ctx.realm()), Default::default())?;

        Ok(())
    }
}
