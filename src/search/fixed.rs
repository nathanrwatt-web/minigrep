// =============================================================================
// search/fixed.rs — Literal (non-regex) matching
// =============================================================================
// Fixed-string search: treats the query as a plain substring, no regex
// metacharacters interpreted. Backs the -F flag.
// =============================================================================

pub fn search_fixed<'a> (query: &str, contents: &'a str, case_insensitive: bool) -> Vec<(usize, &'a str)> {
    if case_insensitive {
        contents.lines().enumerate()
            .filter(|(_, line)| 
                line.to_lowercase()
                .contains(&query.to_lowercase()))
            .collect()
    }
    else {
        contents.lines().enumerate()
            .filter( |(_, line)| line.contains(query))
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_string() {
        let query = "ru.t";
        let contents = "rust and what not\n runt of the litter\n    RuS. has been ever-made to\n ru.t should be returned";
        assert_eq!(vec![(3usize," ru.t should be returned")], search_fixed(query, contents, false));
    }
}
