//! # Slate Script
//!
//! The JS↔engine bridge. A JS runtime (Boa) runs on its own isolated
//! thread and never touches the state store directly. The only thing
//! it can do is push [`OwnedWebCall`]s into a buffer; the kernel
//! drains that buffer on the next frame and feeds the Dispatcher.
//!
//! This enforces two things:
//!
//! 1. **Determinism.** The JS side cannot observe wall time or stash
//!    hidden mutable state in the state store. Everything it wants
//!    to happen must be expressed as a WebCall, so it is replayable.
//!
//! 2. **Isolation.** The boa `Context` sees only the host functions
//!    we register. No `XMLHttpRequest`, no `document` beyond the
//!    exposed surface. The network and DOM live on the Rust side.
//!
//! Wasm + SIMD will attach here in a later phase — the plan is to
//! route heavy numerical code (layout, animation) through a
//! `wasmtime` runtime instead of Boa's interpreter. Phase 2 ships
//! the channel, not the runtime switch.

pub mod bridge;
pub mod host;
pub mod runtime;

pub use runtime::{ScriptError, ScriptRuntime};
