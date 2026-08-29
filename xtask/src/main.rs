//! Build automation for Para Bellum (cargo-xtask pattern).
//!
//! Usage: `cargo xtask <command>`
//!
//! Commands will include:
//! - `check-boundary`: Verify para-core has no terminal crate dependencies
//! - `fuzz`: Run fuzz targets
//! - `release`: Build release binaries

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("check-boundary") => check_boundary(),
        Some(cmd) => {
            eprintln!("Unknown command: {cmd}");
            eprintln!("Available: check-boundary");
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: cargo xtask <command>");
            eprintln!("Available: check-boundary");
            std::process::exit(1);
        }
    }
}

/// Verify that para-core does not depend on any terminal/UI crate.
/// This enforces the TUI/GUI architectural boundary.
fn check_boundary() {
    let forbidden = ["ratatui", "crossterm", "termion", "cursive", "egui"];

    let output = Command::new("cargo")
        .args(["tree", "-p", "para-core", "--prefix", "none"])
        .output()
        .expect("Failed to run `cargo tree`");

    let tree_output = String::from_utf8_lossy(&output.stdout);

    let mut violations = Vec::new();
    for dep in &forbidden {
        if tree_output.lines().any(|line| line.starts_with(dep)) {
            violations.push(*dep);
        }
    }

    if violations.is_empty() {
        println!("✓ para-core boundary check passed: no terminal/UI dependencies found.");
    } else {
        eprintln!("✗ para-core boundary VIOLATED! Found forbidden dependencies:");
        for v in &violations {
            eprintln!("  - {v}");
        }
        std::process::exit(1);
    }
}
