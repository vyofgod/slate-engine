//! State & event primitives.
//!
//! State transitions in Slate are pure: `apply(state, primitive) →
//! state'`. There is no hidden mutation. Events from the outside world
//! (clicks, key presses, fetches completing) arrive as a `SignalEmit`
//! primitive — by the time anything can change, the event has already
//! been converted into a deterministic, replayable instruction.

use crate::geom::NodeId;

/// A deterministic input signal. Two signals with the same id + target
/// are interchangeable by definition; the kernel is allowed to coalesce
/// them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct SignalId(pub u32);

/// Interned key-id (e.g. attribute name). The key/value interner lives
/// in the state store so that `AttrBind` is pure `u32`→`u32`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct KeyId(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ValueId(pub u32);

#[derive(Debug, Clone, PartialEq)]
pub enum StatePrimitive {
    /// Allocate a new node. The id is chosen by the Dispatcher, not the
    /// kernel, to keep allocation deterministic across replays.
    NodeCreate { node: NodeId },

    /// Detach a node (and its descendants) from the tree. Reversible
    /// via the snapshot log.
    NodeDetach { node: NodeId },

    /// Re-parent `node` under `parent` at position `index`.
    NodeAttach { node: NodeId, parent: NodeId, index: u32 },

    /// Bind an interned attribute on a node.
    AttrBind { node: NodeId, key: KeyId, value: ValueId },

    /// Emit a bridgeable input signal. Fires exactly once per call.
    SignalEmit { signal: SignalId, target: NodeId },
}
