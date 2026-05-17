//! Mutation observer system.

use slate_ais::NodeId;

/// Mutation observer for tracking DOM changes.
pub struct MutationObserver {
    records: Vec<MutationRecord>,
}

impl MutationObserver {
    /// Create a new mutation observer.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Record a mutation.
    pub fn record(&mut self, record: MutationRecord) {
        self.records.push(record);
    }

    /// Take all recorded mutations.
    pub fn take_records(&mut self) -> Vec<MutationRecord> {
        std::mem::take(&mut self.records)
    }
}

impl Default for MutationObserver {
    fn default() -> Self {
        Self::new()
    }
}

/// A mutation record.
#[derive(Debug, Clone)]
pub struct MutationRecord {
    pub mutation_type: MutationType,
    pub target: NodeId,
    pub added_nodes: Vec<NodeId>,
    pub removed_nodes: Vec<NodeId>,
    pub attribute_name: Option<String>,
    pub old_value: Option<String>,
}

/// Mutation type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationType {
    ChildList,
    Attributes,
    CharacterData,
}
