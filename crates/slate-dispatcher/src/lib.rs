//! # Slate Dispatcher
//!
//! A stateless translation bridge. Inspired by the "syscall translation"
//! model (Wine, WSL1, Rosetta) rather than the "interpret and execute"
//! model of Blink/WebKit.
//!
//! The pipeline is three steps, each monomorphic and inline-friendly:
//!
//! ```text
//!   WebCall        ──(normalize)──▶  NormalizedCall
//!   NormalizedCall ──(decompose)──▶  slate_ais::Stream
//! ```
//!
//! The Dispatcher holds no state. Determinism comes for free: two
//! identical input sequences always produce identical AIS streams.

use slate_ais::Stream;

pub mod decompose;
pub mod ir;
pub mod normalize;
pub mod style;

pub use ir::{NormalizedCall, OwnedWebCall, WebCall};

/// Errors that can occur during translation. A parse-side problem in
/// the Web call is rejected early; the kernel never sees malformed IR.
#[derive(Debug, thiserror::Error)]
pub enum DispatchError {
    #[error("unsupported Web API surface: {0}")]
    Unsupported(&'static str),

    #[error("malformed style declaration: {0}")]
    BadStyle(String),
}

/// The bridge entry point. Takes a single raw Web call and produces
/// its AIS decomposition.
///
/// `#[inline]` is load-bearing: we want the call sites in the kernel
/// to collapse into straight-line code after LTO.
#[inline]
pub fn dispatch(call: WebCall<'_>) -> Result<Stream, DispatchError> {
    let normalized = normalize::normalize(call)?;
    Ok(decompose::decompose(&normalized))
}

/// Dispatch a batch. Separate from [`dispatch`] because batches get a
/// single AIS stream (cheaper than concatenating) and because Phase 2
/// will parallelize this across cores.
pub fn dispatch_batch<'a, I>(calls: I) -> Result<Stream, DispatchError>
where
    I: IntoIterator<Item = WebCall<'a>>,
{
    let mut out: Stream = smallvec::SmallVec::new();
    for call in calls {
        let normalized = normalize::normalize(call)?;
        decompose::decompose_into(&normalized, &mut out);
    }
    Ok(out)
}
