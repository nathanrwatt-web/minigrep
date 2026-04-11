// =============================================================================
// runner.rs — Orchestration
// =============================================================================
// run(Config) is the single entry point the binary calls. It pulls inputs
// input.rs, builds a MatcherPlan, calls search function, and hands results
// to output.rs for printing.
// =============================================================================

use std::fs;
use std::error::Error;

use crate::config::Config;
use crate::search::{
    Matcher, MatcherPlan,
    search_fixed, search_regex, search_word, search_line,
};

use Matcher::{Fixed, Line, Word, Pattern};

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {

    let contents: Vec<String> = config.file_paths.iter().filter_map(|f| {
        match fs::read_to_string(f) {
            Ok(text) => Some(text),
            Err(e) => {
                eprintln!("{f}: {e}");
                None
            }
        }
        }).collect();



    // determine search type
    let info = MatcherPlan {
        kind :  if config.fixed_string { Fixed }
                else if config.whole_line { Line }
                else if config.whole_word { Word }
                else { Pattern },
        case_insensitive : config.case_insensitive,

        // pattern should be regex modified then passed
        pattern : if config.queries.len() > 1 {
            config.queries.iter().map(|p| format!("({p})")).collect::<Vec<_>>().join("|")
        } else {
            config.queries[0].clone() // need to check if exists
        }
    };

    // modify pattern for case_insensitivity
    let pattern = if info.case_insensitive {
        format!("(?i){}", info.pattern)
    } else {
        info.pattern.clone()
    };

    for content in &contents {
        let results = match info.kind {
            Fixed => search_fixed(&info.pattern, content, info.case_insensitive),
            Line  => search_line(&pattern, content),
            Word  => search_word(&pattern, content),
            Pattern => search_regex(&pattern, content),
        };

        if config.show_line_numbers {
            for (number, line) in results {
                println!("{number}  {line}");
            }
        }
    }
    Ok(())
}
