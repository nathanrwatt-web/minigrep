// =============================================================================
// main.rs — Binary entry point
// =============================================================================

use std::env;
use std::process;
use std::error::Error;
use minigrep::{Config, run};

fn main() {
    // build the config struct
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    
    let result: Result<bool, Box<dyn Error>> = run(config);
    
    match result {
        Ok(value) => {
            if value {process::exit(0);}
            else {process::exit(1);}
        },
        Err(e) => process::exit(2),
    }
}
