// =============================================================================
// search/regex.rs — Regex-backed matchers
// =============================================================================
//   search_regex              — base pattern match
//   search_case_insensitive   — (?i) prefix
//   search_word               — \b..\b word boundaries (-w)
//   search_line               — ^..$ full-line anchor (-x)
//   search_inverted           — negative lookahead (-v)
//   search_multi_pattern      — alternation of many patterns (-e)
// =============================================================================

use ::regex::Regex;

// match lines using a compiled regex pattern
pub fn search_regex<'a> (query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let re = Regex::new(query) // create new regex expression
        .expect("Invalid Regex Patter");

    contents.lines().enumerate() // turn into lines
        .filter(|(_, line)| re.is_match(line)).collect() // filter by lines containing regex
}


// in regex (?i)abc... ignores case of what follows
pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let regex_result = String::from("(?i)") + query;
    search_regex(&regex_result, contents)
}


// match when pattern apears at word boundaries
pub fn search_word<'a> (query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let regex_result = String::from("\\b") + query + "\\b"; // encase query in \b .. \b for word boundries in regex
    search_regex(&regex_result, contents)

}

// match only when entire line equals the pattern
pub fn search_line<'a> (query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let regex_result = String::from("^") + query + "$"; // anchors mean the line must exactly match query
    search_regex(&regex_result, contents)
}

// returns all lines that do not match the patter
pub fn search_inverted<'a> (query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let regex_result = String::from("^(?!.*")  + query + ").*$";
    search_regex(&regex_result, contents)
}

// accept a list of patterns, match any of them
// consider manually joining together to avoid the useless collect() heap allocation
pub fn search_multi_pattern<'a> (query: Vec<&str>, contents: &'a str) -> Vec<(usize, &'a str)> {
    let regex_result = query.iter().map(|p| format!("({})",p)).collect::<Vec<_>>().join("|");
    search_regex(&regex_result, contents)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "hello";
        let contents = "Hello, world!\nhEllo world, and then\n there was hell0";
        assert_eq!(vec![(0usize, "Hello, world!"), (1usize, "hEllo world, and then")], search_case_insensitive(query, contents));
    }
}
