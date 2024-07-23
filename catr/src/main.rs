use std::{
    fs::File,
    io::{stdin, BufRead, BufReader},
};

use anyhow::Result;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, about, version)]
struct Args {
    /// Input files
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number lines
    #[arg(short('n'), long("number"), conflicts_with("number_nonblank_lines"))]
    number_lines: bool,

    /// Number non-blank lines
    #[arg(short('b'), long("number-nonblank"))]
    number_nonblank_lines: bool,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {filename}: {e}"),
            Ok(content) => {
                if args.number_lines {
                    print_lines_with_number(content)?;
                } else if args.number_nonblank_lines {
                    print_lines_with_number_nonblank(content)?;
                } else {
                    print_lines_wo_number(content)?;
                }
            }
        }
    }

    Ok({})
}

fn print_lines_with_number(content: Box<dyn BufRead>) -> Result<()> {
    for (i, line) in content.lines().enumerate() {
        println!("{:6}\t{}", i + 1, line?);
    }

    Ok({})
}

fn print_lines_with_number_nonblank(content: Box<dyn BufRead>) -> Result<()> {
    let mut line_number: i32 = 1;
    for line in content.lines() {
        let line = line?;
        if line.is_empty() {
            println!();
        } else {
            println!("{:6}\t{}", line_number, line);
            line_number += 1;
        }
    }

    Ok({})
}

fn print_lines_wo_number(content: Box<dyn BufRead>) -> Result<()> {
    for line in content.lines() {
        println!("{}", line?);
    }

    Ok({})
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
