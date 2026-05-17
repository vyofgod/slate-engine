//! Dedicated Worker implementation.

use crate::{Result, WorkerError, WorkerState, Message};
use crossbeam_channel::{Sender, Receiver, unbounded};
use parking_lot::RwLock;
use std::sync::Arc;
use std::thread;

/// Dedicated Worker.
///
/// A dedicated worker is owned by a single script and can only communicate
/// with that script.
pub struct DedicatedWorker {
    /// Worker ID
    id: u32,
    
    /// Worker state
    state: Arc<RwLock<WorkerState>>,
    
    /// Message sender (to worker)
    tx: Sender<Message>,
    
    /// Message receiver (from worker)
    rx: Receiver<Message>,
    
    /// Worker thread handle
    thread_handle: Option<thread::JoinHandle<()>>,
}

impl DedicatedWorker {
    /// Create a new dedicated worker.
    pub fn new(id: u32, script_url: String) -> Result<Self> {
        let (main_tx, worker_rx) = unbounded();
        let (worker_tx, main_rx) = unbounded();
        
        let state = Arc::new(RwLock::new(WorkerState::Initializing));
        let state_clone = state.clone();
        
        // Spawn worker thread
        let thread_handle = thread::spawn(move || {
            // Initialize worker
            *state_clone.write() = WorkerState::Running;
            
            // TODO: Load and execute script
            // For now, just echo messages
            loop {
                match worker_rx.recv() {
                    Ok(msg) => {
                        // Process message
                        if let Err(_) = worker_tx.send(msg) {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            
            *state_clone.write() = WorkerState::Terminated;
        });
        
        Ok(Self {
            id,
            state,
            tx: main_tx,
            rx: main_rx,
            thread_handle: Some(thread_handle),
        })
    }
    
    /// Get worker ID.
    pub fn id(&self) -> u32 {
        self.id
    }
    
    /// Get worker state.
    pub fn state(&self) -> WorkerState {
        *self.state.read()
    }
    
    /// Post a message to the worker.
    pub fn post_message(&self, message: Message) -> Result<()> {
        self.tx.send(message)
            .map_err(|e| WorkerError::SendFailed(e.to_string()))
    }
    
    /// Receive a message from the worker (non-blocking).
    pub fn try_recv_message(&self) -> Option<Message> {
        self.rx.try_recv().ok()
    }
    
    /// Receive a message from the worker (blocking).
    pub fn recv_message(&self) -> Result<Message> {
        self.rx.recv()
            .map_err(|e| WorkerError::ReceiveFailed(e.to_string()))
    }
    
    /// Terminate the worker.
    pub fn terminate(&mut self) {
        *self.state.write() = WorkerState::Terminated;
        
        if let Some(handle) = self.thread_handle.take() {
            let _ = handle.join();
        }
    }
}

impl Drop for DedicatedWorker {
    fn drop(&mut self) {
        self.terminate();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_worker() {
        let worker = DedicatedWorker::new(1, "worker.js".to_string());
        assert!(worker.is_ok());
    }
    
    #[test]
    fn worker_state() {
        let worker = DedicatedWorker::new(1, "worker.js".to_string()).unwrap();
        // Give worker time to initialize
        std::thread::sleep(std::time::Duration::from_millis(10));
        assert_eq!(worker.state(), WorkerState::Running);
    }
}
