use std::{env, fs::File, io::{self, BufRead, BufReader, Write}};
use regex::Regex;

/// Entry point for the search functionality.
///
/// This function parses command-line arguments, performs a search in the specified file,
/// and writes the results to either the console or an output file.
///
/// # Arguments
/// - `mode`: The search mode (`search` or `regex`).
/// - `search_string`: The string or regex pattern to search for in the file.
/// - `file_path`: The path to the file to search.
/// - `[output_file]`: (Optional) The path to the file where results will be written.
/// - `[--ignore-case]`: (Optional) If provided, the search will ignore case.
///
/// # Usage Examples
/// Perform a case-sensitive substring search:
/// ```bash
/// cargo run -- search "Error" "./large_log_file.txt"
/// ```
///
/// Perform a case-insensitive regex search and write results to an output file:
/// ```bash
/// cargo run -- regex "error\d+" "./large_log_file.txt" "./output.txt" --ignore-case
/// ```
pub fn start_search() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("Usage: cargo run -- <mode> <search_string> <file_path> [output_file] [--ignore-case]");
        return;
    }

    // Parse arguments
    let mode = &args[1];
    let query = &args[2];
    let file_path = &args[3];
    let mut output_file = None;
    let mut ignore_case = false;

    // Determine optional arguments
    for arg in &args[4..] {
        if arg == "--ignore-case" {
            ignore_case = true;
        } else if output_file.is_none() {
            output_file = Some(arg.as_str());
        }
    }

    println!("Performing '{}' mode search for '{}' in file '{}'", mode, query, file_path);

    // Notify the user if results will be printed to the console
    if output_file.is_none() {
        println!("No output file specified. Results will be printed to the console.");
    }

    if let Err(e) = match mode.as_str() {
        "search" => search_in_file(file_path, query, output_file, ignore_case, false),
        "regex" => search_in_file(file_path, query, output_file, ignore_case, true),
        _ => {
            eprintln!("Invalid mode '{}'. Use 'search' or 'regex'.", mode);
            return;
        }
    } {
        eprintln!("Error: {}", e);
    }
}

/// Searches for a query string in a file and writes results to an output file or stdout.
///
/// # Arguments
/// - `file_path`: The path to the file to search.
/// - `query`: The string or regex pattern to search for in the file.
/// - `output_file`: (Optional) The path to the file where results will be written.
///                  If `None`, results are printed to the console.
/// - `ignore_case`: If `true`, the search will ignore case.
/// - `is_regex`: If `true`, the query is treated as a regex pattern.
///
/// # Behavior
/// - Reads the file line by line using a buffered reader for efficiency.
/// - Writes matching lines to the specified output file or to the console.
/// - Prints a reminder if no matches are found.
///
/// # Example
/// Perform a case-insensitive regex search and print results to the console:
/// ```rust
/// use query::search_in_file;
///
/// fn main() -> std::io::Result<()> {
///     search_in_file("./large_log_file.txt", "error\\d+", None, true, true)
/// }
/// ```
fn search_in_file(
    file_path: &str,
    query: &str,
    output_file: Option<&str>,
    ignore_case: bool,
    is_regex: bool,
) -> io::Result<()> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    // Open the output file if provided; otherwise, write to stdout
    let mut writer: Box<dyn Write> = match output_file {
        Some(output_path) => Box::new(File::create(output_path)?),
        None => Box::new(io::stdout()),
    };

    let mut found_match = false; // Track if any matches are found

    for (line_number, line) in reader.lines().enumerate() {
        let line = line?;
        let match_found = if is_regex {
            // Perform regex search
            let regex = if ignore_case {
                Regex::new(&format!("(?i){}", query))
            } else {
                Regex::new(query)
            };
            match regex {
                Ok(re) => re.is_match(&line),
                Err(_) => {
                    eprintln!("Invalid regex pattern '{}'", query);
                    return Ok(());
                }
            }
        } else {
            // Perform substring search
            if ignore_case {
                line.to_lowercase().contains(&query.to_lowercase())
            } else {
                line.contains(query)
            }
        };

        if match_found {
            writeln!(writer, "Line {}: {}", line_number + 1, line)?;
            found_match = true; // Set flag to true if a match is found
        }
    }

    // If no matches were found, print a reminder
    if !found_match {
        eprintln!("No matches found for '{}' in file '{}'", query, file_path);
    }

    Ok(())
}