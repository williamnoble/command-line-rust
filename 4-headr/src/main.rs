use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(default_value = "-", value_name = "FILE")]
    files: Vec<String>,

    #[arg(
    short('n'),
    long("lines"),
    default_value = "10",
    value_name = "NUM",
    value_parser = clap::value_parser!(u64).range(1..)
    )]
    lines: u64,

    #[arg(
    short('c'),
    long("bytes"),
    value_name = "NUM",
    conflicts_with("lines"),
    value_parser = clap::value_parser!(u64).range(1..)
    )]
    bytes: Option<u64>,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

pub fn run(args: Args) -> MyResult<()> {
    let num_files = args.files.len();

    for (file_num, filename) in args.files.iter().enumerate() {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut file) => {

                // print a heading when there's more than one file otherwise omit header e.g.
                // ==> ./tests/inputs/ten.txt <==
                if num_files > 1 {
                    println!(
                        "{}==> {} <==",
                        if file_num > 0 { "\n" } else { "" },
                        &filename
                    );
                }

                // there's a better way of doing this but i'm not sure how
                // I had to use clone twice otherwise we try to use a moved value,
                // I also tried to use a reference to &num_bytes but file.take expects
                // u64 not &u64. TODO: suck less at rust :)
                if let Some(num_bytes) = args.bytes.clone() {
                    // once we define file.take we still need to read those bytes
                    // into a buffer. hence: handle -> read [buf]
                    let mut handle = file.take(num_bytes.clone());
                    // create a buffer of fixed size i.e. our num_bytes
                    let mut buffer = vec![0; num_bytes as usize];
                    let bytes_read = handle.read(&mut buffer)?;
                    print!(
                        "{}",
                        // note we're expecting &[u8] not u8.
                        String::from_utf8_lossy(&buffer[..bytes_read])
                    );
                } else {
                    // standard behaviour to print lines (we set a default of 10 lines)
                    let mut line = String::new();
                    for _ in 0..args.lines {
                        let bytes = file.read_line(&mut line)?;
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

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}