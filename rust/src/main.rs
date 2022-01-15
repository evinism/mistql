use clap::Parser;
use std::{
    fs::File,
    io::{self, BufRead, Read},
    process,
};

/// Command-line interface for the Rust MistQL implementation
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// The query to run
    query: String,

    /// The file to run the query on (default: stdin)
    #[clap(short, long)]
    file: Option<String>,
    /// The data to run the query on
    #[clap(short, long, conflicts_with = "file")]
    data: Option<String>,

    /// The output file (default: stdout)
    #[clap(short, long)]
    output: Option<String>,
}

fn main() {
    let args = Args::parse();
    let data = if let Some(d) = args.data {
        d
    } else if let Some(f) = args.file {
        read_file(f).unwrap_or_else(|err| {
            eprintln!("Failed to read file: {}", err);
            process::exit(1);
        })
    } else {
        read_stdin().unwrap_or_else(|err| {
            eprintln!("Failed to read stdin: {}", err);
            process::exit(1);
        })
    };

    let result = mistql::query(args.query, data);
    dbg!(result);

    // TODO: Write out result to `args.output`.
}

fn read_file(path: String) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

fn read_stdin() -> io::Result<String> {
    let mut buf = String::new();
    for line in io::stdin().lock().lines() {
        if !buf.is_empty() {
            buf += "\n";
        }
        match line {
            Ok(s) => buf += &s,
            Err(err) => return Err(err),
        }
    }
    Ok(buf)
}
