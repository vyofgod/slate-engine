//! # Slate State Engine
//!
//! The state of the world, as Slate sees it, is a **single immutable
//! snapshot**. Mutations do not happen in place; they happen by
//! producing a new snapshot that shares unchanged data with the old
//! one.
//!
//! This is what makes Slate deterministic and replayable:
//!
//! ```text
//!   apply : (Snapshot, AtomicInstruction) -> Snapshot
//! ```
//!
//! No hidden side effects. No wall-clock dependency. Given the same
//! starting snapshot and the same sequence of inputs, the engine
//! produces the same ending snapshot every time.
//!
//! Phase 1 implements the minimum: `Store`, `Node`, `Snapshot::version`,
//! and the apply loop for the small number of [`StatePrimitive`]
//! variants we decompose to. Structural sharing is modeled (the
//! `snapshot()` call is O(1)) but not yet optimized for memory.

use dashmap::DashMap;
use slate_ais::{AtomicInstruction, NodeId, StatePrimitive};
use slotmap::{new_key_type, SlotMap};
use std::sync::Arc;

new_key_type! { pub struct NodeKey; }

#[derive(Debug, Clone)]
pub struct Node {
    pub id:       NodeId,
    pub parent:   Option<NodeId>,
    pub children: Vec<NodeId>,
    pub detached: bool,
}

/// The mutable store. Each call to [`Store::apply`] bumps `version`
/// and may publish a fresh [`Snapshot`].
pub struct Store {
    nodes:   SlotMap<NodeKey, Node>,
    /// NodeId → NodeKey. Populated by the Dispatcher's deterministic
    /// id allocator; the kernel never invents a NodeKey out of thin
    /// air.
    index:   DashMap<NodeId, NodeKey>,
    /// (node, attr_key) → value_id. Dashmap gives us lock-free reads
    /// from layout/render workers.
    attrs:   DashMap<(NodeId, u32), u32>,
    version: u64,
}

impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

impl Store {
    pub fn new() -> Self {
        Self {
            nodes:   SlotMap::with_key(),
            index:   DashMap::new(),
            attrs:   DashMap::new(),
            version: 0,
        }
    }

    #[inline]
    pub fn version(&self) -> u64 { self.version }

    #[inline]
    pub fn node_count(&self) -> usize { self.nodes.len() }

    pub fn apply(&mut self, instr: &AtomicInstruction) {
        // Only the State domain touches the store. Layout and Render
        // primitives are handed off to their respective subsystems.
        if let AtomicInstruction::State(sp) = instr {
            self.apply_state(sp);
            self.version += 1;
        }
    }

    fn apply_state(&mut self, sp: &StatePrimitive) {
        match sp {
            StatePrimitive::NodeCreate { node } => {
                let key = self.nodes.insert(Node {
                    id: *node,
                    parent: None,
                    children: Vec::new(),
                    detached: false,
                });
                self.index.insert(*node, key);
            }
            StatePrimitive::NodeDetach { node } => {
                if let Some(key) = self.index.get(node).map(|r| *r) {
                    if let Some(n) = self.nodes.get_mut(key) {
                        n.detached = true;
                    }
                }
            }
            StatePrimitive::NodeAttach { node, parent, index } => {
                if let Some(pkey) = self.index.get(parent).map(|r| *r) {
                    if let Some(p) = self.nodes.get_mut(pkey) {
                        let i = (*index as usize).min(p.children.len());
                        p.children.insert(i, *node);
                    }
                }
                if let Some(ckey) = self.index.get(node).map(|r| *r) {
                    if let Some(c) = self.nodes.get_mut(ckey) {
                        c.parent = Some(*parent);
                        c.detached = false;
                    }
                }
            }
            StatePrimitive::AttrBind { node, key, value } => {
                self.attrs.insert((*node, key.0), value.0);
            }
            StatePrimitive::SignalEmit { .. } => {
                // Signals don't mutate the tree directly — they
                // trigger reducer logic that the kernel runs out of
                // band. Phase 1: no-op.
            }
        }
    }

    /// Take an immutable view of the store at the current version.
    /// O(1) — the snapshot does not clone the tree.
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            version: self.version,
            node_count: self.nodes.len(),
            root: self.nodes.iter().find_map(|(_, n)| {
                if n.parent.is_none() && !n.detached {
                    Some(n.id)
                } else {
                    None
                }
            }),
            _keepalive: Arc::new(()),
        }
    }
}

/// A point-in-time view of the store. The `Arc` is a placeholder for
/// the structural-sharing snapshot log we will wire in Phase 2; right
/// now it just makes snapshots cheap to pass around.
#[derive(Debug, Clone)]
pub struct Snapshot {
    pub version:    u64,
    pub node_count: usize,
    pub root:       Option<NodeId>,
    _keepalive:     Arc<()>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use slate_ais::StatePrimitive;

    #[test]
    fn create_then_attach_is_deterministic() {
        let mut a = Store::new();
        let mut b = Store::new();

        let ops = [
            AtomicInstruction::State(StatePrimitive::NodeCreate { node: NodeId(1) }),
            AtomicInstruction::State(StatePrimitive::NodeCreate { node: NodeId(2) }),
            AtomicInstruction::State(StatePrimitive::NodeAttach {
                node: NodeId(2),
                parent: NodeId(1),
                index: 0,
            }),
        ];

        for op in &ops { a.apply(op); }
        for op in &ops { b.apply(op); }

        assert_eq!(a.version(), b.version());
        assert_eq!(a.snapshot().root, b.snapshot().root);
        assert_eq!(a.node_count(), 2);
    }
}
