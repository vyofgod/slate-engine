//! Event dispatcher with bubbling and capturing.

use super::{Event, EventListener, EventPhase, EventType};
use slate_ais::NodeId;
use std::collections::HashMap;
use std::sync::Arc;

/// Event dispatcher manages event listeners and dispatches events.
pub struct EventDispatcher {
    listeners: HashMap<NodeId, Vec<RegisteredListener>>,
    parent_map: HashMap<NodeId, NodeId>,
}

struct RegisteredListener {
    event_type: EventType,
    listener: Arc<dyn EventListener>,
    phase: EventPhase,
}

impl EventDispatcher {
    /// Create a new event dispatcher.
    pub fn new() -> Self {
        Self {
            listeners: HashMap::new(),
            parent_map: HashMap::new(),
        }
    }

    /// Register a parent-child relationship.
    pub fn set_parent(&mut self, child: NodeId, parent: NodeId) {
        self.parent_map.insert(child, parent);
    }

    /// Add an event listener to a node.
    pub fn add_listener(
        &mut self,
        node: NodeId,
        event_type: EventType,
        listener: Arc<dyn EventListener>,
        phase: EventPhase,
    ) {
        let registered = RegisteredListener {
            event_type,
            listener,
            phase,
        };

        self.listeners
            .entry(node)
            .or_insert_with(Vec::new)
            .push(registered);
    }

    /// Remove an event listener from a node.
    pub fn remove_listener(&mut self, node: NodeId, event_type: EventType) {
        if let Some(listeners) = self.listeners.get_mut(&node) {
            listeners.retain(|l| l.event_type != event_type);
        }
    }

    /// Dispatch an event to a target node.
    pub fn dispatch(&self, mut event: Event) {
        // Build ancestor chain for bubbling/capturing
        let ancestors = self.build_ancestor_chain(event.target);

        // 1. Capturing phase (from root to target)
        event.phase = EventPhase::Capturing;
        for &ancestor in ancestors.iter().rev() {
            if event.is_propagation_stopped() {
                return;
            }

            event.current_target = Some(ancestor);
            self.invoke_listeners(ancestor, &event, EventPhase::Capturing);
        }

        // 2. At target phase
        event.phase = EventPhase::AtTarget;
        event.current_target = Some(event.target);
        self.invoke_listeners(event.target, &event, EventPhase::AtTarget);

        if event.is_propagation_stopped() {
            return;
        }

        // 3. Bubbling phase (from target to root)
        if event.bubbles {
            event.phase = EventPhase::Bubbling;
            for &ancestor in &ancestors {
                if event.is_propagation_stopped() {
                    return;
                }

                event.current_target = Some(ancestor);
                self.invoke_listeners(ancestor, &event, EventPhase::Bubbling);
            }
        }
    }

    /// Build the ancestor chain for a node.
    fn build_ancestor_chain(&self, node: NodeId) -> Vec<NodeId> {
        let mut ancestors = Vec::new();
        let mut current = node;

        while let Some(&parent) = self.parent_map.get(&current) {
            ancestors.push(parent);
            current = parent;
        }

        ancestors
    }

    /// Invoke listeners for a node at a specific phase.
    fn invoke_listeners(&self, node: NodeId, event: &Event, phase: EventPhase) {
        if let Some(listeners) = self.listeners.get(&node) {
            for registered in listeners {
                if registered.event_type == event.event_type
                    && (registered.phase == phase || phase == EventPhase::AtTarget)
                {
                    registered.listener.handle_event(event);
                }
            }
        }
    }
}

impl Default for EventDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EventData;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestListener {
        call_count: Arc<AtomicUsize>,
    }

    impl EventListener for TestListener {
        fn handle_event(&self, _event: &Event) {
            self.call_count.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn event_bubbling_order() {
        let mut dispatcher = EventDispatcher::new();

        let root = NodeId(1);
        let child = NodeId(2);
        let grandchild = NodeId(3);

        dispatcher.set_parent(child, root);
        dispatcher.set_parent(grandchild, child);

        let call_count = Arc::new(AtomicUsize::new(0));

        let listener = Arc::new(TestListener {
            call_count: Arc::clone(&call_count),
        });

        dispatcher.add_listener(root, EventType::Click, listener.clone(), EventPhase::Bubbling);
        dispatcher.add_listener(child, EventType::Click, listener.clone(), EventPhase::Bubbling);
        dispatcher.add_listener(grandchild, EventType::Click, listener, EventPhase::AtTarget);

        let event = Event::new(EventType::Click, grandchild, EventData::None);
        dispatcher.dispatch(event);

        // Should be called 3 times: at target, child bubble, root bubble
        assert_eq!(call_count.load(Ordering::SeqCst), 3);
    }
}
