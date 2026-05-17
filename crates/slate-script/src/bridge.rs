//! The shared bridge state between Boa host functions and the
//! Rust-side kernel.
//!
//! We use a `thread_local!` because Boa host functions are plain
//! function pointers — they have no captured environment, so shared
//! state must be reachable by pointer, not by closure capture. Every
//! `ScriptRuntime` lives on its own thread (the JS worker), so the
//! thread-local is effectively runtime-local.

use std::cell::RefCell;

use slate_ais::NodeId;
use slate_dispatcher::OwnedWebCall;

thread_local! {
    pub(crate) static BUFFER: RefCell<Vec<OwnedWebCall>> = const { RefCell::new(Vec::new()) };
    pub(crate) static NEXT_ID: RefCell<u32> = const { RefCell::new(1) };
}

pub(crate) fn alloc_id() -> NodeId {
    NEXT_ID.with(|n| {
        let mut n = n.borrow_mut();
        let v = *n;
        *n = v.saturating_add(1);
        NodeId(v)
    })
}

pub(crate) fn push(call: OwnedWebCall) {
    BUFFER.with(|b| b.borrow_mut().push(call));
}

/// Drain the WebCall buffer. Called by the kernel each frame.
pub fn drain() -> Vec<OwnedWebCall> {
    BUFFER.with(|b| std::mem::take(&mut *b.borrow_mut()))
}

/// Reset the id counter. Called at page navigation so two pages in
/// the same runtime thread don't alias ids.
pub fn reset() {
    BUFFER.with(|b| b.borrow_mut().clear());
    NEXT_ID.with(|n| *n.borrow_mut() = 1);
}
