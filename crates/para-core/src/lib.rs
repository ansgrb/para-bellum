//! Para Bellum core — UI-agnostic editor engine.
//!
//! This crate contains the fundamental editing primitives:
//! - [`buffer::Buffer`] — Rope-backed text buffer with O(log n) edits
//! - [`command::Command`] — Every edit expressed as a dispatchable command
//! - [`cursor::Cursor`] — Cursor position and selection state
//! - [`view::View`] — Buffer view with scroll and viewport state
//! - [`undo::UndoStack`] — Undo/redo history stack
//!
//! **Architectural invariant**: This crate MUST NOT depend on any terminal
//! or GUI crate (ratatui, crossterm, egui, etc.). The TUI and GUI frontends
//! depend on this crate, never the reverse.

pub mod buffer;
pub mod command;
pub mod cursor;
pub mod undo;
pub mod view;
