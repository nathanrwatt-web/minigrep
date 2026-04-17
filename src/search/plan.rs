// =============================================================================
// search/plan.rs — Matcher selection
// =============================================================================
// `Matcher` details the kinds of search the runner can dispatch to, and
// `MatcherPlan` bundles that choice with the pre-resolved pattern and the
// case-insensitive flag so the runner doesn't have to re-inspect Config
// =============================================================================

use super::super::Config;
// ======== Todo ========
// implenent a build_plan(&Config -> MatcherPLan)


pub enum Matcher {
    Fixed,
    Line,
    Word,
    Pattern,
}

pub struct MatcherPlan {
    pub kind: Matcher,
    pub case_insensitive: bool,
    pub pattern: String,
}

impl MatcherPlan {

    pub fn new(config: &Config) -> Self {
        use Matcher::*;
        
        MatcherPlan {
            kind :  if config.fixed_string      { Fixed }
                    else if config.whole_line   { Line }
                    else if config.whole_word   { Word }
                    else                        { Pattern },

            case_insensitive : config.case_insensitive,

            pattern : match config.queries.len() {
                0 => String::new(),
                1 => config.queries[0].clone(),
                _ => config.queries.iter()
                        .map(|p| format!("({p})"))
                        .collect::<Vec<_>>()
                        .join("|"),
            }
        }
    }
}


pub enum OutputMatcher {
    LineNumbers,       // -c
    Filenames,         // -l
    Matched,           // -o
    Quiet,             // -q
    ByteOffset,        // -b
    Line,              // default, print the line that matches
}

pub struct OutputMatcherPlan {
    kind: OutputMatcher,
    show_inverted: bool,
    max_count: u32,
}

impl OutputMatcherPlan {

    pub fn new(config: &Config) -> Self {
        use OutputMatcher::*;

        OutputMatcherPlan {
            kind : if config.quiet_mode                     { Quiet }
                   else if config.show_filenames_only       { Filenames }
                   else if config.show_line_numbers_only    { Filenames }
                   else if config.binary_skip               { ByteOffset }
                   else if config.only_matched_portion      { Matched }
                   else                                     { Line },
             
            show_inverted : config.invert_match,

            max_count : config.max_count.unwrap(),
        }
    }

}
