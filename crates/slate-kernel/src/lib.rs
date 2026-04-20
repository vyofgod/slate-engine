//! # Slate Kernel
//!
//! The orchestrator. The Kernel is the only thing that owns *all
//! three* of the Dispatcher, the state Store, and the per-page Arena.
//! Everything else in the engine talks to one of those subsystems in
//! isolation — the Kernel is where they meet.
//!
//! Phase 1 exposes a synchronous `submit` entry point. Phase 2 will
//! sharded it onto crossbeam/flume channels so the Dispatcher,
//! state-apply, and GPU-submit stages can each run on their own core.

use slate_ais::{AtomicInstruction, Stream};
use slate_arena::PageArena;
use slate_dispatcher::{dispatch, DispatchError, OwnedWebCall, WebCall};
use slate_state::{Snapshot, Store};

pub mod parse;

pub struct Kernel {
    store: Store,
    arena: PageArena,
}

impl Default for Kernel {
    fn default() -> Self { Self::new() }
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            store: Store::new(),
            arena: PageArena::new(),
        }
    }

    /// Translate a Web call and apply the resulting AIS to the store.
    /// Returns the AIS stream so callers can also forward it to the
    /// GPU pipeline.
    pub fn submit(&mut self, call: WebCall<'_>) -> Result<Stream, DispatchError> {
        let stream = dispatch(call)?;
        for instr in stream.iter() {
            self.store.apply(instr);
        }
        Ok(stream)
    }

    /// Submit an owned Web call (from JS runtime or network layer).
    /// Thin borrow-adapter over [`Kernel::submit`].
    pub fn submit_owned(&mut self, call: &OwnedWebCall) -> Result<Stream, DispatchError> {
        self.submit(call.as_web_call())
    }

    /// Drain a batch of owned calls into the kernel and return the
    /// concatenated AIS stream — what you feed into the renderer.
    pub fn submit_batch<'a, I>(&mut self, calls: I) -> Result<Vec<AtomicInstruction>, DispatchError>
    where
        I: IntoIterator<Item = &'a OwnedWebCall>,
    {
        let mut all: Vec<AtomicInstruction> = Vec::new();
        for c in calls {
            let stream = self.submit_owned(c)?;
            all.extend(stream.into_iter());
        }
        Ok(all)
    }

    /// Apply a pre-built instruction stream. Useful for replaying a
    /// captured session byte-for-byte.
    pub fn replay(&mut self, stream: &[AtomicInstruction]) {
        for instr in stream {
            self.store.apply(instr);
        }
    }

    pub fn snapshot(&self) -> Snapshot { self.store.snapshot() }

    pub fn arena(&self) -> &PageArena { &self.arena }

    /// Navigate away from the current page: drops all per-page
    /// allocations in O(1). The state store is *not* reset — the
    /// caller decides whether to keep history.
    pub fn navigate(&mut self) {
        self.arena.reset();
    }
}
