// =============================================================================
// main.rs — Binary entry point
// =============================================================================

use std::env;
use std::process;

use minigrep::{Config, run};

fn main() {
    // build the config struct
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // catch errors by calling run 
    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
