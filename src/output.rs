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

// -c: print the number of matching lines per file instead of the lines themselves
pub fn print_count(file_name: &str, file_lines: Vec<(usize, &str)>) {
    println!("{} : {}", file_name, file_lines.len());
}

// -l: print only the name of each file that has at least one match
pub fn print_filenames_only(file_name: &str, _file_lines: Vec<(usize, &str)>) {
    println!("{file_name}");
}

// -o: print only the matched portion of each line, one match per line
pub fn print_only_matched(_file_name: &str, _file_lines: Vec<(usize, &str)>, _show_line_numbers: bool) {
    todo!()
}

// -q: produce no output; exit code reflects whether any match was found
pub fn print_quiet() {
    todo!()
}

// -b: prefix each matching line with its byte offset within the file
pub fn print_with_byte_offset(_file_name: &str, _file_lines: Vec<(usize, &str)>) {
    todo!()
}
