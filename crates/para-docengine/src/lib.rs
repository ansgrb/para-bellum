//! Document rendering engines for Para Bellum.
//!
//! This crate will provide:
//! - Markdown preview (pulldown-cmark → ratatui widgets)
//! - Shared diagram AST for all diagram formats
//! - Mermaid parser (hand-written recursive descent)
//! - LaTeX math-block compilation (tectonic)
//! - `PlantUML` wrapper (shell-out, v1)
//! - Export pipeline: AST → SVG → PNG/PDF
//!
//! Implementation begins in Week 9.
