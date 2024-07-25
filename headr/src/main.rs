use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    usize,
};

use anyhow::Result;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(about, version, author)]
/// Rust version of `head`
struct Args {
    /// Input file(s)
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    /// Number of lines
    #[arg(short('n'),
        long,
        value_name = "LINES",
        default_value = "10",
        value_parser=clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    /// Number of bytes
    #[arg(short('c'),
        long,
        value_name = "BYTES",
        conflicts_with("lines"),
        value_parser=clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn run(args: Args) -> Result<()> {
    let num_files = args.files.len();
    for (iter, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(e) => eprintln!("{}: {}", filename, e),
            Ok(mut content) => {
                if num_files > 1 {
                    if iter > 0 {
                        println!();
                    }
                    println!("==> {} <==", filename);
                }

                if let Some(num_bytes) = args.bytes {
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = content.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]))
                } else {
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = content.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
