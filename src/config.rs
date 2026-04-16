// =============================================================================
// config.rs — Config data type
// =============================================================================

#[derive(Default)]
pub struct Config {
    pub queries: Vec<String>,
    pub file_paths: Vec<String>,
    pub fixed_string: bool,             // -F
    pub whole_word: bool,               // -w
    pub whole_line: bool,               // -x e
    pub case_insensitive: bool,         //
    pub show_line_numbers: bool,        // -n
    pub show_line_numbers_only: bool,   // -c 
    pub show_filenames_only: bool,      // -l
    pub invert_match: bool,             // -v
    pub quiet_mode: bool,               // -q 
    pub before_lines: Option<u32>,      // -B,     -C will modify surrounding linesat the same time
    pub after_lines: Option<u32>,       // -A
    pub byte_offset: bool,              // -b
    pub only_matched_portion: bool,     // -o
    pub recursive: bool,                // -r -R
    pub count_mode: bool,               // -c 
    pub max_count: Option<u32>,         // -m
    pub multi_pattern: bool,            // -e 
    pub from_file: bool,
    pub binary_skip: bool,
}
