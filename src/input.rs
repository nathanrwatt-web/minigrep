// =============================================================================
// input.rs — Input acquisition
// =============================================================================
// Responsible for turning Config's file_paths (and eventually stdin, recursive
// directory walks, --include/--exclude globs, binary detection, CRLF handling)
// into a stream of `(source_name, contents)` that the runner can hand to the
// search layer. Currently empty — file reading still lives inline in runner.rs
// and should be migrated here as the -r / stdin / multi-file work lands.
// =============================================================================

use crate::config::Config;

use std::fs; 
use std::error::Error;

// consider propogating errors
pub fn load_sources(config: &Config) -> Vec<(String, String)> {
    config.file_paths.iter().filter_map(|f| {
        match fs::read_to_string(f) {
            Ok(text) => Some((f.clone(), text)),
            Err(e) => { 
                eprintln!("{f}: {e}");
                None 
            }
        }
    }).collect()
}

