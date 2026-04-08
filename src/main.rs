// =============================================================================
// minigrep — Main Entry Point
// =============================================================================
//
// A command-line text search tool inspired by grep. This binary handles
// argument parsing, configuration, and orchestrates the search pipeline.
//
// CURRENT CAPABILITIES:
//   - Parse CLI args: query string and file path
//   - Read file contents into memory
//   - Case-sensitive substring search
//   - Case-insensitive search (via IGNORE_CASE env var)
//   - Print matching lines to stdout
//   - Error messages to stderr (eprintln)
//
// FULL GREP FEATURE SET (not yet implemented):
//
//   Pattern Matching:
//     - Regex support (basic and extended regular expressions)
//     - Fixed-string matching (-F, literal match without regex)
//     - Whole-word matching (-w, match only at word boundaries)
//     - Whole-line matching (-x, entire line must match pattern)
//
//   Output Control:
//     - Line numbers (-n, prefix each match with its line number)
//     - Count only (-c, print count of matching lines instead of lines)
//     - Filenames only (-l, list files that contain a match)
//     - Files without match (-L, list files that do NOT match)
//     - Quiet/silent mode (-q, exit 0 if match found, no output)
//     - Context lines: before (-B N), after (-A N), and around (-C N)
//     - Byte offset (-b, show byte offset of each match)
//     - Color/highlight matching text in output
//     - Print only the matched portion of a line (-o)
//     - Prefix output with filename when searching multiple files
//
//   Search Scope:
//     - Recursive directory search (-r/-R, search all files in a dir tree)
//     - Multiple file arguments (search across many files at once)
//     - Stdin support (pipe input instead of reading a file)
//     - Include/exclude file patterns (--include, --exclude globs)
//     - Follow or skip symlinks
//     - Binary file detection (skip or warn on binary files)
//
//   Matching Behavior:
//     - Invert match (-v, print lines that do NOT match)
//     - Max match count (-m N, stop after N matches)
//     - Multi-pattern search (-e PATTERN, specify multiple patterns)
//     - Pattern from file (-f FILE, read patterns from a file)
//
//   Performance:
//     - Parallel/threaded search across multiple files
//     - Memory-mapped file reading for large files
//     - Line-buffered streaming output
//
//   Exit Codes:
//     - 0: match found
//     - 1: no match found
//     - 2: error (file not found, bad args, etc.)
//
// =============================================================================

use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::*;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}



fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search_regex(&config.query, &contents)
    };

    for (int, line) in results {
        println!("{int}   {line}");
    }
    
    Ok(())
}



// ----- helper functions ------
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,

}

impl Config {
    fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
        args.next(); // skip first item 

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file pathh"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config { query, file_path, ignore_case, })
    }
}
