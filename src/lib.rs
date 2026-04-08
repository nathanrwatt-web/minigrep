// =============================================================================
// minigrep — Library (Search Functions)
// =============================================================================
//
// Core search logic for minigrep. All pattern matching and filtering
// functions live here, separated from the binary for testability.
//
// CURRENT CAPABILITIES:
//   - search(): case-sensitive substring matching, returns matching lines
//   - search_case_insensitive(): lowercased substring matching
//
// FULL GREP SEARCH FUNCTIONS NEEDED (not yet implemented):
//
//   Core Search Variants:
//     - search_regex: match lines using a compiled regex pattern
//     - search_fixed: literal/fixed-string match (no regex interpretation)
//     - search_word: match only when pattern appears at word boundaries
//     - search_line: match only when entire line equals the pattern
//     - search_inverted: return lines that do NOT match the pattern
//     - search_multi_pattern: accept a list of patterns, match any
//
//   Result Enrichment:
//     - Return line numbers alongside matched lines
//     - Return byte offsets of matches within each line
//     - Return context (N lines before/after each match)
//     - Highlight/extract the matched substring within a line
//     - Return only the matched portion (-o behavior)
//     - Group results by filename for multi-file search
//
//   Streaming & Performance:
//     - Streaming search over a BufRead (for stdin and large files)
//     - Iterator-based lazy matching (avoid collecting all results)
//     - Early termination after N matches (max count)
//     - Parallel search across multiple file contents
//
//   Input Handling:
//     - Accept patterns from a file (one pattern per line)
//     - Binary content detection (return a flag or skip)
//     - Handle different line endings (CRLF, LF, CR)
//     - UTF-8 validation and lossy fallback for non-UTF8 files
//
//   Match Counting:
//     - count_matches: return number of matching lines (not the lines)
//     - has_match: return bool, short-circuit on first match
//
// =============================================================================
#![allow(dead_code, unused_variables, unused_imports, unused_mut)]

use regex::Regex;

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines().filter(|line| line.contains(query)).collect()

}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

// fixed string match
pub fn search_fixed<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Todo


    results
}

// match lines using a compiled regex pattern
pub fn search_regex<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let re = Regex::new(query) // create new regex expression
        .expect("Invalid Regex Patter");

    contents.lines() // turn into lines
        .filter(|line| re.is_match(line)).collect() // filter by lines containing regex 
}

// match when pattern apears at word boundaries 
pub fn search_word<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let regex_result = String::from("\\b") + query + "\\b"; // encase query in \b .. \b for word boundries in regex
    search_regex(&regex_result, contents)

}

// match only when entire line equals the pattern
pub fn search_line<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let regex_result = String::from("^") + query + "$"; // anchors mean the line must exactly match query
    search_regex(&regex_result, contents)
}

// returns all lines that do not match the patter
pub fn search_inverted<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Todo


    results
}

// accept a list of patterns, match any of them
pub fn search_multi_pattern<'a> (query: Vec<&str>, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    // Todo


    results
}



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust: 
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
