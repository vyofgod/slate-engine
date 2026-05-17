//! # Slate Network
//!
//! Async fetch layer. The shape we care about is **streaming**: by
//! the time the first MTU of HTML lands, the Dispatcher is already
//! decomposing it into AIS. Nothing blocks on "page fully
//! downloaded".
//!
//! ## Pipeline
//!
//! ```text
//!   reqwest::Response.bytes_stream()
//!     │  (Bytes chunks)
//!     ▼
//!   IncrementalParser::feed(chunk)  ──▶  Vec<OwnedWebCall>
//!     │
//!     ▼
//!   kernel.submit_owned(calls)
//! ```
//!
//! ## HTTP/3
//!
//! The `http3` feature is a placeholder — enabling it signals intent
//! to route through a QUIC transport when the reqwest/h3 integration
//! is stable on this toolchain. Today it compiles but does not alter
//! transport selection.
//!
//! ## Sandbox
//!
//! Each [`Fetcher`] is scoped: it can be created with an allow-list
//! of origins, and requests to other origins fail fast. Real process
//! sandboxing (seccomp, user namespaces) is Centrion's job; we just
//! give it an API boundary to enforce.

pub mod fetch;
pub mod parse;
pub mod sandbox;

pub use fetch::{FetchError, Fetcher};
pub use parse::IncrementalParser;
pub use sandbox::OriginPolicy;
