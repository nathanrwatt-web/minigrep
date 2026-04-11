// =============================================================================
// search/plan.rs — Matcher selection
// =============================================================================
// `Matcher` details the kinds of search the runner can dispatch to, and
// `MatcherPlan` bundles that choice with the pre-resolved pattern and the
// case-insensitive flag so the runner doesn't have to re-inspect Config
// =============================================================================


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
