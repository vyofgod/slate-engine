//! Worker pool for efficient resource management.

use crate::{DedicatedWorker, Result, Message};
use parking_lot::RwLock;
use std::sync::Arc;
use std::collections::VecDeque;

/// Worker pool.
///
/// Manages a pool of workers for efficient task distribution.
pub struct WorkerPool {
    /// Pool size
    size: usize,
    
    /// Workers
    workers: Arc<RwLock<Vec<Arc<DedicatedWorker>>>>,
    
    /// Task queue
    queue: Arc<RwLock<VecDeque<Message>>>,
    
    /// Next worker index (round-robin)
    next_worker: Arc<RwLock<usize>>,
}

impl WorkerPool {
    /// Create a new worker pool.
    pub fn new(size: usize, script_url: String) -> Result<Self> {
        let mut workers = Vec::new();
        
        for i in 0..size {
            let worker = DedicatedWorker::new(i as u32, script_url.clone())?;
            workers.push(Arc::new(worker));
        }
        
        Ok(Self {
            size,
            workers: Arc::new(RwLock::new(workers)),
            queue: Arc::new(RwLock::new(VecDeque::new())),
            next_worker: Arc::new(RwLock::new(0)),
        })
    }
    
    /// Get pool size.
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Submit a task to the pool.
    pub fn submit(&self, message: Message) -> Result<()> {
        // Get next worker (round-robin)
        let mut next = self.next_worker.write();
        let worker_idx = *next;
        *next = (*next + 1) % self.size;
        drop(next);
        
        // Post message to worker
        let workers = self.workers.read();
        if let Some(worker) = workers.get(worker_idx) {
            worker.post_message(message)?;
        }
        
        Ok(())
    }
    
    /// Try to receive a result from any worker.
    pub fn try_recv(&self) -> Option<Message> {
        let workers = self.workers.read();
        
        for worker in workers.iter() {
            if let Some(msg) = worker.try_recv_message() {
                return Some(msg);
            }
        }
        
        None
    }
    
    /// Shutdown the pool.
    pub fn shutdown(&self) {
        let mut workers = self.workers.write();
        workers.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MessageData;
    
    #[test]
    fn create_pool() {
        let pool = WorkerPool::new(4, "worker.js".to_string());
        assert!(pool.is_ok());
        assert_eq!(pool.unwrap().size(), 4);
    }
    
    #[test]
    fn submit_task() {
        let pool = WorkerPool::new(2, "worker.js".to_string()).unwrap();
        let msg = Message::new(MessageData::String("test".to_string()));
        assert!(pool.submit(msg).is_ok());
    }
}
