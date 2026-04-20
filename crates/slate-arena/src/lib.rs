//! # Slate Arena
//!
//! Per-page arena allocator. Every page (tab, iframe, bridge context)
//! gets one `PageArena`; every allocation inside that page goes into
//! it; when the page navigates away, the whole arena is dropped in
//! O(1).
//!
//! This replaces tracing garbage collection *and* reference counting
//! in the hot path. The cost of freeing a billion intermediate
//! tokens, style values, and layout nodes is one pointer write.
//!
//! `unsafe` is not used here: `bumpalo` encapsulates the pointer
//! arithmetic and the borrow checker enforces aliasing. That leaves
//! the door open to reserving `unsafe` for the actually-hot paths
//! identified by the `criterion` suite in `slate-kernel`.

use bumpalo::Bump;

/// An arena scoped to a single page lifecycle.
#[derive(Debug, Default)]
pub struct PageArena {
    bump: Bump,
}

impl PageArena {
    pub fn new() -> Self { Self { bump: Bump::new() } }

    /// Pre-size the arena. Useful when the page's working-set bound is
    /// known (e.g. a known-small widget embedded in Centrion).
    pub fn with_capacity(bytes: usize) -> Self {
        Self { bump: Bump::with_capacity(bytes) }
    }

    #[inline(always)]
    pub fn alloc<T>(&self, value: T) -> &mut T {
        self.bump.alloc(value)
    }

    #[inline(always)]
    pub fn alloc_str<'a>(&'a self, s: &str) -> &'a str {
        self.bump.alloc_str(s)
    }

    /// Allocate a slice by copying `src`. The returned reference lives
    /// as long as the arena.
    #[inline]
    pub fn alloc_slice_copy<'a, T: Copy>(&'a self, src: &[T]) -> &'a mut [T] {
        self.bump.alloc_slice_copy(src)
    }

    /// Free every allocation in O(1). Call this on page navigation.
    pub fn reset(&mut self) { self.bump.reset(); }

    pub fn bytes_allocated(&self) -> usize {
        self.bump.allocated_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_reclaims() {
        let mut a = PageArena::new();
        for i in 0..1000u32 { let _ = a.alloc(i); }
        let before = a.bytes_allocated();
        assert!(before > 0);
        a.reset();
        // After reset the chunks are retained but the cursor is at 0,
        // so further allocations reuse memory.
        for i in 0..1000u32 { let _ = a.alloc(i); }
        assert!(a.bytes_allocated() <= before + 4096);
    }
}
