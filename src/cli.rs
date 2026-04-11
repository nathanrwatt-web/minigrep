// =============================================================================
// cli.rs — Argument parsing
// =============================================================================
// Translates env::args into a populated Confi`. All flag/option dispatch lives here.
// Adding a new CLI flag means adding a field in config.rs and a match arm here.
// =============================================================================

use crate::config::Config;

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>, ) -> Result<Config, &'static str> {
        args.next(); // skip first item since it is filepath 
        let mut config = Config::default();

        while let Some(arg) = args.next() {
            match arg.as_str() {

                // check for things that requre something after them first
                "-e" => {
                    let pattern = args.next().expect("-e requires a pattern");
                    config.queries.push(pattern);
                }

                // match count
                "-m" => {
                    let num_matches = args.next().expect("-m requires a number of matches");
                    config.max_count = Some(num_matches.parse().expect("invalid number"));
                }

                // -A, -B, -C augment the surrounding lines shown
                "-A" => {
                    let lines_after = args.next().expect("-A requires a number of lines");
                    config.after_lines = Some(lines_after.parse().expect("invalid number"));
                }

                "-B" => {
                    let lines_before = args.next().expect("-B requires a number of lines");
                    config.before_lines = Some(lines_before.parse().expect("invalid number"));
                }
                "-C" => {
                    let lines = args.next().expect("-C requires a number of lines");
                    let num = Some(lines.parse().expect("invalid number"));
                    config.before_lines = num;
                    config.after_lines = num;
                }

                "-F" => config.fixed_string = true,
                "-w" => config.whole_word = true,
                "-x" => config.whole_line = true,
                "-n" => config.show_line_numbers = true,
                "-c" => config.show_line_numbers_only = true,
                "-l" => config.show_filenames_only = true,
                "-i" => config.case_insensitive = true,
                "-v" => config.invert_match = true,
                "-q" => config.quiet_mode = true,
                "-b" => config.byte_offset = true,
                "-o" => config.only_matched_portion = true,
                "-r" => config.recursive = true,
                "-R" => config.recursive = true,
                "-L" => {config.show_filenames_only = true;  config.invert_match = true;},

                _ => {
                    if config.queries.is_empty() { config.queries.push(arg); }
                    else { config.file_paths.push(arg); }
                }
            }
        }
        Ok(config)
    }
}
