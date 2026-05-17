//! WebSocket automatic reconnection logic.

use std::time::Duration;

/// Reconnection strategy.
pub struct ReconnectStrategy {
    /// Current attempt number
    attempt: u32,
    
    /// Initial delay
    initial_delay: Duration,
    
    /// Maximum delay
    max_delay: Duration,
    
    /// Backoff multiplier
    backoff: f32,
}

impl ReconnectStrategy {
    /// Create a new reconnection strategy.
    pub fn new(initial_delay: Duration, max_delay: Duration, backoff: f32) -> Self {
        Self {
            attempt: 0,
            initial_delay,
            max_delay,
            backoff,
        }
    }
    
    /// Get next delay.
    pub fn next_delay(&mut self) -> Duration {
        self.attempt += 1;
        
        let delay_ms = self.initial_delay.as_millis() as f32 
            * self.backoff.powi(self.attempt as i32 - 1);
        
        let delay = Duration::from_millis(delay_ms as u64);
        
        delay.min(self.max_delay)
    }
    
    /// Reset the strategy.
    pub fn reset(&mut self) {
        self.attempt = 0;
    }
    
    /// Get current attempt number.
    pub fn attempt(&self) -> u32 {
        self.attempt
    }
}

impl Default for ReconnectStrategy {
    fn default() -> Self {
        Self::new(
            Duration::from_millis(1000),
            Duration::from_millis(30000),
            1.5,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn exponential_backoff() {
        let mut strategy = ReconnectStrategy::new(
            Duration::from_millis(1000),
            Duration::from_millis(10000),
            2.0,
        );
        
        assert_eq!(strategy.next_delay().as_millis(), 1000);
        assert_eq!(strategy.next_delay().as_millis(), 2000);
        assert_eq!(strategy.next_delay().as_millis(), 4000);
        assert_eq!(strategy.next_delay().as_millis(), 8000);
        assert_eq!(strategy.next_delay().as_millis(), 10000); // Capped at max
    }
}
