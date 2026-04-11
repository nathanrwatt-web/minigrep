// =============================================================================
// input.rs — Input acquisition
// =============================================================================
// Responsible for turning Config's file_paths (and eventually stdin, recursive
// directory walks, --include/--exclude globs, binary detection, CRLF handling)
// into a stream of `(source_name, contents)` that the runner can hand to the
// search layer. Currently empty — file reading still lives inline in runner.rs
// and should be migrated here as the -r / stdin / multi-file work lands.
// =============================================================================
