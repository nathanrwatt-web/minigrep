// =============================================================================
// search/mod.rs — Search module root
// =============================================================================
// Declares the search submodules and re-exports their public surface so the
// rest of the crate can `use crate::search::*` without caring which file a
// given matcher lives in.
//
//   fixed  — literal / non-regex substring matching
//   regex  — regex-backed matchers (default, -i, -w, -x, -v, multi-pattern)
//   plan   — Matcher enum + MatcherPlan, chooses which search fn to dispatch to
// =============================================================================

#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

pub mod fixed;
pub mod regex;
pub mod plan;

pub use fixed::search_fixed;
pub use regex::{
    search_regex,
    search_case_insensitive,
    search_word,
    search_line,
    search_inverted,
    search_multi_pattern,
};
pub use plan::{Matcher, MatcherPlan};
