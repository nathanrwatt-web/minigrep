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

pub enum MatchOutcome {
    Matched,
    NotMatched,
}

