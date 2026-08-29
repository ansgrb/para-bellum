//! Plugin host and capability-based security model for Para Bellum.
//!
//! This crate will provide:
//! - wasmtime-based WASM sandbox
//! - WIT-defined plugin API (Component Model)
//! - Explicit, per-capability permission grants
//! - No plugin — including first-party — skips the capability check
//!
//! Implementation begins at M4 (Week 28).
