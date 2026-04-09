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

#[derive(Default)]
pub struct Config {
    pub queries: Vec<String>,
    pub file_paths: Vec<String>,
    pub fixed_string: bool,
    pub whole_word: bool,
    pub whole_line: bool,
    pub case_insensitive: bool,
    pub show_line_numbers: bool,
    pub show_line_numbers_only: bool,
    pub show_filenames_only: bool,
    pub invert_match: bool,
    pub quiet_mode: bool,
    pub before_lines: Option<u32>, // if both are zero, do nothing
    pub after_lines: Option<u32>,
    pub byte_offset: bool,
    pub only_matched_portion: bool,
    pub recursive: bool,
    pub count_mode: bool,  // if 0 ignore
    pub max_count: Option<u32>,
    pub multi_pattern: bool,
    pub from_file: bool,
    pub binary_skip: bool,
}


impl Config {
    fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
        args.next(); // skip first item 
        let mut config = Config::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {

                // check for things that requre something after them first
                "-e" => { 
                    let pattern = args.next().expect("-e requires a pattern"); 
                    config.queries.push(pattern);
                }

                // match count
                "-m" => { 
                    let num_matches = args.next().expect("-m requires a number of matches");
                    config.max_count = Some(num_matches.parse().expect("invalid number"));
                }

                // -A, -B, -C augment the surrounding lines shown
                "-A" => {
                    let lines_after = args.next().expect("-A requires a number of lines");
                    config.after_lines = Some(lines_after.parse().expect("invalid number"));
                }

                "-B" => {
                    let lines_before = args.next().expect("-B requires a number of lines");
                    config.before_lines = Some(lines_before.parse().expect("invalid number"));
                }
                "-C" => {
                    let lines = args.next().expect("-C requires a number of lines");
                    let num = Some(lines.parse().expect("invalid number"));
                    config.before_lines = num;
                    config.after_lines = num;
                }

                "-F" => config.fixed_string = true,
                "-w" => config.whole_word = true,
                "-x" => config.whole_line = true,
                "-n" => config.show_line_numbers = true,
                "-c" => config.show_line_numbers_only = true,
                "-l" => config.show_filenames_only = true,
                "-i" => config.invert_match = true,
                "-q" => config.quiet_mode = true,
                "-b" => config.byte_offset = true,
                "-o" => config.only_matched_portion = true,
                "-r" => config.recursive = true,
                "-R" => config.recursive = true,
                "-L" => {config.show_filenames_only = true;  config.invert_match = true;},
                
                _ => {
                    if config.queries.is_empty() { config.queries.push(arg); }
                    else { config.file_paths.push(arg); }
                }
            }
        } 
        Ok(config)
    }
}
