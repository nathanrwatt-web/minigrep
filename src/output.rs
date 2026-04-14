// =============================================================================
// output.rs — Result formatting
// =============================================================================



// takes in a filename, a vector of (linenumber: usize, line: string) and prints wi/wo file lines
// for -o functionality, we will assume that the slice in the vec is only the matched portion 
// similarly -A/-B/-C will be handeld in input
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

// -c count only 
pub fn print_line_numbers(file_name: &str, file_lines: Vec<(usize, &str)>) {
    println!("{file_name}");
    
    for (num, _) in file_lines {
        println!("{num}");
    }
}


