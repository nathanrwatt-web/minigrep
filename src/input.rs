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
use walkdir::WalkDir;
use std::fs; 
use std::error::Error;
use std::io::{self, Write};

// consider propogating errors
// Returns a vector of tuples: (filename, filecontents)
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

// read from stdin when no file paths are provided
pub fn load_stdin() -> Vec<(String, String)> {
    todo!()
}
pub fn load_recursive(config: &Config) -> Vec<(String, String)> {
    config.file_paths
        .iter()
        .flat_map(|root| WalkDir::new(root).into_iter()) // recurse over each dir
        .filter_map( |entry| { entry.map_err(|e| eprintln!("minigrep: {e}")).ok() }) //filter error
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| {
            let path = entry.path();
            fs::read_to_string(path)
                .map(|contents| (path.to_string_lossy().to_string(), contents))
                .map_err(|e| eprintln!("minigrep: {}: {}", path.display(), e))
                .ok()
        })
        .collect()
}

// -r/-R: recursively walk directories in config.file_paths and collect all files
pub fn load_recursive_no_pipe(config: &Config) -> Vec<(String, String)> {
    let mut results: Vec<(String, String)> = Vec::new();
    let stderr = io::stderr();
    for file_name in &config.file_paths {
        for entry in WalkDir::new(file_name) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    let _ = writeln!(stderr.lock(), "minigrep: {e}");
                    continue;
                }            
            };
            // skip if not a valid readable file type
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            match fs::read_to_string(path) {
                Ok(contents) => {
                    results.push((path.to_string_lossy().to_string(), contents));
                }
                Err(e) => {
                    let _ = writeln!(stderr.lock(), "minigrep: {}: {}", path.display(), e);
                }
            }
        }
    }
    results
}

// binary_skip: return true if the file contents appear to be binary (non-text)
pub fn detect_binary(_contents: &str) -> bool {
    todo!()
}

// -f: read query patterns from a file, one per line
pub fn load_from_file(_path: &str) -> Vec<String> {
    todo!()
}

// -A/-B/-C: expand matched line numbers into ranges including surrounding context lines
pub fn apply_context(_line_count: usize, _before: Option<u32>, _after: Option<u32>, _matched: &[usize]) -> Vec<usize> {
    todo!()
}
