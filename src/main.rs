use std::env;
use std::fs;
use std::process;

use regex::Regex;

/// Helper function for invalid arguments or command usage
fn help(msg: Option<&str>) {
    println!(
        "
        {}
        Usage: minigrep SEARCH_STRING PATH_TO_FILE FLAGS
        Will search file for specified search string and output results

        Flag         Meaning\n
        -i           Case Insensitive
        -o FILENAME  Write output to specified file
    ",
        match msg {
            Some(msg) => format!("FATAL: {}\n", msg),
            None => "".to_string(),
        }
    );
    process::exit(1);
}

/// Data stored about a matched result
struct MatchResult {
    pub line: u32,
    pub char: u32,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        help(Some("Not enough arguments supplied"));
    }

    let query = &args[1];
    let filename = &args[2];

    let mut insensitive_flag = false;
    let mut output_flag = (false, "");

    // Iterate over args
    for (idx, arg) in args.iter().enumerate() {
        if arg == "-h" || arg == "--help" || arg == "help" {
            help(None);
        }

        if idx > 2 {
            if arg == "-i" {
                insensitive_flag = true;
            } else if arg == "-o" {
                output_flag.0 = true;
            } else if args[idx - 1] == "-o" {
                output_flag.1 = arg;
            }
        }
    }

    let mut flag = "";
    if insensitive_flag {
        flag = "(?i)";
    }

    let full_query = flag.to_owned() + query;

    let contents = fs::read_to_string(filename).expect("Failed to read input file");
    let query_regex = Regex::new(&full_query).expect("Invalid search string");

    let mut results: Vec<MatchResult> = Vec::new();

    // Run the Regular Expression on each line to isolate line & char number
    for (line_number, line_contents) in contents.split("\n").enumerate() {
        let matched_result = query_regex.find(line_contents);
        if matched_result.is_some() {
            let matched_content = matched_result.unwrap();
            results.push(MatchResult {
                line: line_number as u32,
                char: matched_content.start() as u32,
            })
        }
    }

    if results.len() == 0 {
        println!("No matches found")
    }

    // Create format string from results
    let mut result_str: String = "".to_owned();

    for result in results {
        let new_result =
            &format!("\nMatch at line {}, character {}", result.line, result.char) as &str;
        result_str.push_str(new_result);
    }

    if output_flag.0 {
        if output_flag.1 == "" {
            help(Some("Output flag provided, but missing file"));
        } else {
            println!("Results found, writing to output....");
            let result = fs::write(output_flag.1, result_str);
            if result.is_err() {
                help(Some("Failed to write to output file"));
            } else {
                println!("Operation complete 🚀, see '{}' for results", output_flag.1);
                process::exit(0);
            }
        }
    } else {
        println!("Operation Complete 🚀\n{}", result_str);
        process::exit(0);
    }
}
