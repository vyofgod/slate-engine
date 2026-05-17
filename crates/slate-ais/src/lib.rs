//! # Slate Atomic Instruction Set (AIS)
//!
//! The AIS is the *machine code* of the Slate engine. Every high-level
//! Web API call — no matter how baroque — must be decomposed by the
//! Dispatcher into a finite sequence of [`AtomicInstruction`]s before
//! the kernel will execute anything.
//!
//! The set is intentionally small (target: 200–500 variants across all
//! three domains). Growth of this set is a design failure; shrinking it
//! is a design success.
//!
//! See [`MANIFEST.md`](../../../MANIFEST.md) in the repo root for the
//! invariants these types exist to enforce.

#![cfg_attr(not(test), no_std)]
extern crate alloc;

pub mod geom;
pub mod layout;
pub mod rendering;
pub mod state;

pub use geom::{NodeId, Point, Rect, Size, SubPixel};
pub use layout::LayoutPrimitive;
pub use rendering::{LayerId, RenderPrimitive, Rgba8};
pub use state::{SignalId, StatePrimitive};

/// A single atomic operation. The discriminant is `u8`-tagged so the
/// kernel dispatch loop can be a jump table rather than a chain of
/// conditionals.
#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum AtomicInstruction {
    Layout(LayoutPrimitive)  = 0x01,
    Render(RenderPrimitive)  = 0x02,
    State(StatePrimitive)    = 0x03,
}

impl AtomicInstruction {
    /// The domain tag — useful for bucketing instructions before handing
    /// them to domain-specific executors (e.g. GPU command buffer vs.
    /// state mutator).
    #[inline(always)]
    pub const fn domain(&self) -> Domain {
        match self {
            AtomicInstruction::Layout(_) => Domain::Layout,
            AtomicInstruction::Render(_) => Domain::Render,
            AtomicInstruction::State(_)  => Domain::State,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Domain {
    Layout = 1,
    Render = 2,
    State  = 3,
}

/// A dense buffer of atomic instructions. Small-vector optimized for the
/// common case where a single WebCall decomposes to a handful of ops.
pub type Stream = smallvec::SmallVec<[AtomicInstruction; 8]>;
