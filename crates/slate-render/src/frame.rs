//! Deterministic frame scheduler.
//!
//! We deliberately do **not** use V-Sync or any platform refresh
//! callback here — the kernel must drive the clock so that replays
//! are bit-identical. The scheduler hands out a monotonically
//! increasing `frame_index` and reports the intended wall-time gap
//! to the next frame; the embedder decides how to wait (sleep,
//! spin, or block on the compositor).

use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct FrameScheduler {
    target_hz:   f32,
    period:      Duration,
    started_at:  Instant,
    frame_index: u64,
}

impl FrameScheduler {
    pub fn new(target_hz: f32) -> Self {
        let period = Duration::from_secs_f32(1.0 / target_hz.max(1.0));
        Self {
            target_hz,
            period,
            started_at:  Instant::now(),
            frame_index: 0,
        }
    }

    #[inline] pub fn target_hz(&self) -> f32 { self.target_hz }
    #[inline] pub fn period(&self)    -> Duration { self.period }
    #[inline] pub fn frame_index(&self) -> u64 { self.frame_index }

    /// Advance one frame. Returns the scheduled deadline for the
    /// following frame (embedder should park until then).
    pub fn tick(&mut self) -> Instant {
        self.frame_index += 1;
        self.started_at + self.period * self.frame_index as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ticks_monotonically() {
        let mut s = FrameScheduler::new(300.0);
        let a = s.tick();
        let b = s.tick();
        assert!(b > a);
        assert_eq!(s.frame_index(), 2);
        assert!((s.period().as_secs_f32() - (1.0 / 300.0)).abs() < 1e-6);
    }
}
