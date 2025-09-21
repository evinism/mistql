//! MistQL Command Line Interface
//!
//! This binary provides a command-line interface for executing MistQL queries,
//! similar to the JavaScript and Python implementations.

use clap::{Arg, ArgAction, Command};
use mistql::query;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let matches = Command::new("mq")
        .about("MistQL Command Line Interface")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::new("query").help("The MistQL query to execute").required(true).index(1))
        .arg(
            Arg::new("command_line_data")
                .short('c')
                .long("command")
                .help("Read JSON data from command line argument instead of stdin")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Read JSON data from file instead of stdin")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .help("Write output to file instead of stdout")
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("debug")
                .short('d')
                .long("debug")
                .help("Pretty print the RuntimeValue directly (for debugging)")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty print the JSON output")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let query_str = matches.get_one::<String>("query").unwrap();

    // Determine input source
    let json_data = if let Some(data_str) = matches.get_one::<String>("command_line_data") {
        match serde_json::from_str::<Value>(data_str) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing JSON from command line: {}", e);
                process::exit(1);
            }
        }
    } else if !matches.contains_id("file") && !matches.contains_id("command_line_data") {
        // Default to stdin if no other input method specified
        let mut input = String::new();
        if let Err(e) = io::stdin().read_to_string(&mut input) {
            eprintln!("Error reading from stdin: {}", e);
            process::exit(1);
        }
        match serde_json::from_str::<Value>(&input) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("Error parsing JSON from stdin: {}", e);
                process::exit(1);
            }
        }
    } else if let Some(filename) = matches.get_one::<String>("file") {
        match fs::read_to_string(filename) {
            Ok(content) => match serde_json::from_str::<Value>(&content) {
                Ok(data) => data,
                Err(e) => {
                    eprintln!("Error parsing JSON from file '{}': {}", filename, e);
                    process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Error reading file '{}': {}", filename, e);
                process::exit(1);
            }
        }
    } else {
        eprintln!("No input data specified. Use -c for command line data, -f for file, or pipe to stdin.");
        process::exit(1);
    };

    let result = match query(query_str, &json_data) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("MistQL error: {}", e);
            process::exit(1);
        }
    };

    let output = if matches.get_flag("debug") {
        // Debug mode: pretty print RuntimeValue directly
        format!("{:#?}", result)
    } else {
        // Normal mode: convert to serde_json
        match result.to_serde_value(false) {
            Ok(json_value) => {
                if matches.get_flag("pretty") {
                    serde_json::to_string_pretty(&json_value).unwrap()
                } else {
                    serde_json::to_string(&json_value).unwrap()
                }
            }
            Err(e) => {
                eprintln!("Error serializing result: {}", e);
                process::exit(1);
            }
        }
    };

    if let Some(output_file) = matches.get_one::<String>("output") {
        match fs::write(output_file, output) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error writing to output file '{}': {}", output_file, e);
                process::exit(1);
            }
        }
    } else {
        println!("{}", output);
    }
}
