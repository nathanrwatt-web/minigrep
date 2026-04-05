use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // args[0] is where the command is run ie target/debug/bin or something
    let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

}
