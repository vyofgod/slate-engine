//! # Slate Web API - Wine-like Compatibility Layer
//!
//! **Slate does NOT implement Web APIs.** Instead, it provides a **Wine-like
//! translation layer** that converts high-level Web API calls into Slate's
//! Atomic Instruction Set (AIS).
//!
//! ## Philosophy
//!
//! Just like Wine translates Windows API calls to Linux syscalls without
//! implementing Windows, Slate translates Web API calls to atomic primitives
//! without implementing the Web platform.
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────┐
//! │  Wine: Windows APIs → Linux syscalls                    │
//! │  Slate: Web APIs → Atomic Instructions                  │
//! └─────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Architecture
//!
//! ```text
//! JavaScript Code
//!      ↓
//! Web API Call (e.g., createElement)
//!      ↓
//! WebApiTranslator ← Wine-like translation layer
//!      ↓
//! Atomic Instructions (AIS)
//!      ↓
//! Kernel → GPU
//! ```
//!
//! ## Core Principle: Translation, Not Implementation
//!
//! **Traditional Browser (Chromium):**
//! ```ignore
//! // Implements full DOM
//! fn create_element(tag: &str) -> Element {
//!     let element = Element::new(tag);
//!     element.attach_to_document();
//!     element.setup_event_listeners();
//!     // ... 100+ lines of implementation
//!     element
//! }
//! ```
//!
//! **Slate (Translates to AIS):**
//! ```ignore
//! // Pure translation
//! fn translate_create_element(node: NodeId, tag: &str) -> Vec<AtomicInstruction> {
//!     vec![
//!         AtomicInstruction::CreateNode { id: node },
//!         AtomicInstruction::SetNodeType { id: node, node_type: parse_tag(tag) },
//!     ]
//! }
//! ```
//!
//! ## Modules
//!
//! - `translator`: Core translation engine (Wine-like syscall handler)
//! - `dom`: DOM API bindings (capture calls, don't implement)
//! - `console`: Console API bindings
//! - `canvas`: Canvas 2D API bindings
//! - `events`: Event API bindings
//! - `timers`: Timer API bindings
//! - `storage`: Storage API bindings
//! - `fetch`: Fetch API bindings
//! - `url`: URL API bindings
//! - `crypto`: Crypto API bindings
//! - `performance`: Performance API bindings
//! - `geolocation`: Geolocation API bindings
//! - `notification`: Notification API bindings
//! - `websocket`: WebSocket API bindings
//!
//! ## Benefits
//!
//! 1. **No Interpretation Overhead**: Single translation, then native speed
//! 2. **Embarrassingly Parallel**: AIS are pure functions
//! 3. **Deterministic**: Same input → same AIS → same result
//! 4. **Minimal Codebase**: ~6,500 lines vs ~3,500,000 in Chromium
//! 5. **Small Attack Surface**: 200-500 primitives vs thousands of APIs

pub mod translator;  // Core translation engine (Wine-like)

// Phase 4: Interactive & Media APIs
pub mod canvas;      // Canvas 2D API
pub mod forms;       // Form elements and validation
pub mod svg;         // SVG rendering

// Boa integration modules (disabled for now - focus on translator)
// pub mod console;
// pub mod dom;
// pub mod events;
// pub mod fetch;
// pub mod storage;
// pub mod timers;
// pub mod url;
// pub mod crypto;
// pub mod performance;
// pub mod geolocation;
// pub mod notification;
// pub mod websocket;
// pub mod bindings;
// pub mod runtime;

pub use translator::WebApiTranslator;
pub use canvas::CanvasApi;
pub use forms::FormApi;
pub use svg::SvgApi;
// pub use bindings::WebApiBindings;
// pub use runtime::WebApiRuntime;

/// Web API error types.
#[derive(Debug)]
pub enum WebApiError {
    /// JavaScript error.
    JsError(String),
    /// Not implemented yet.
    NotImplemented(String),
    /// Invalid argument.
    InvalidArgument(String),
    /// Security error.
    SecurityError(String),
    /// Network error.
    NetworkError(String),
}

impl std::fmt::Display for WebApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WebApiError::JsError(e) => write!(f, "JavaScript error: {}", e),
            WebApiError::NotImplemented(api) => write!(f, "Not implemented: {}", api),
            WebApiError::InvalidArgument(msg) => write!(f, "Invalid argument: {}", msg),
            WebApiError::SecurityError(msg) => write!(f, "Security error: {}", msg),
            WebApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
        }
    }
}

impl std::error::Error for WebApiError {}
