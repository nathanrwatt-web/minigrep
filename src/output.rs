// =============================================================================
// output.rs — Result formatting
// =============================================================================
// Turns search results into printed output. Owns the formatting side of every
// output-control flag: -n line numbers, -c count-only, -l / -L filenames,
// -o matched-portion, -A/-B/-C context, color highlighting, -q quiet.
// =============================================================================
//
//
//
// ========= Todo ==========


// takes in a filename, a vector of (linenumber: usize, line: string) and prints 
pub fn print_results( file_name: &str, file_lines: Vec<(usize, &str)>, show_line_numbers : bool, ) {
    println!("{file_name}");

    if show_line_numbers {
        for line in &file_lines {
            println!("{}  {}", line.0, line.1);
        }
    } else {
        for line in &file_lines {
            println!("{}", line.1);
        }
    }
}


