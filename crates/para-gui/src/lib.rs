//! Native GUI frontend for Para Bellum.
//!
//! This crate is intentionally empty. It exists to enforce the TUI/GUI
//! architectural boundary from day one: `para-core` must never depend
//! on terminal-specific crates, because this crate will eventually
//! provide an egui-based frontend over the same core.
//!
//! Do NOT add code here until the core API is stable.
