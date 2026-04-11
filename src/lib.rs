// =============================================================================
// lib.rs — Crate root
// =============================================================================
// Declares the module tree and re-exports the public API for main.rs
// =============================================================================

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub mod config;
pub mod cli;
pub mod search;
pub mod input;
pub mod output;
pub mod runner;

pub use config::Config;
pub use runner::run;
pub use search::{
    search_fixed,
    search_regex,
    search_case_insensitive,
    search_word,
    search_line,
    search_inverted,
    search_multi_pattern,
};
