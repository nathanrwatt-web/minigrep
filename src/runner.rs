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
use crate::input::load_sources;

use Matcher::{Fixed, Line, Word, Pattern};

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {


    let contents: Vec<(String,String)> = load_sources(&config); // (filename, contents)
    let info = MatcherPlan::new(&config);

    // modify pattern for case_insensitivity
    let pattern = if info.case_insensitive {
        format!("(?i){}", info.pattern)
    } else {
        info.pattern.clone()
    };

    for (_, content) in &contents {
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
