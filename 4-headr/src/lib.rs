#![allow(dead_code)]

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};
use std::process::Command;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("headr")
        .version("0.1.0")
        .author("William Noble")
        .about("head but written in Rust, yeah i'd never used it either.")
        .arg(
            Arg::with_name("lines")
                .short("n")
                .long("lines")
                .value_name("LINES")
                .help("Number of lines")
                .default_value("10"),
        ).arg(
        Arg::with_name("bytes")
            .short("c")
            .long("bytes")
            .value_name("BYTES")
            .takes_value(true)
            .conflicts_with("lines")
            .help("Number of bytes"),
    ).arg(
        Arg::with_name("files")
            .value_name("FILE")
            .help("Input file(s)")
            .multiple(true)
            .default_value("-"),
    )
        .get_matches();


    let lines = matches
        .value_of("lines")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal line count -- {}", e))?;
    let bytes = matches
        .value_of("bytes")
        .map(parse_positive_int)
        .transpose()
        .map_err(|e| format!("illegal byte count -- {}", e))?;
    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines: lines.unwrap(),
        bytes,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let num_files = config.files.len();

    for (file_index, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            // We returned an error or something which implements the BufRead trait.
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {} <==", if file_index > 0 { "\n" } else { "" }, filename
                    );
                }
                if let Some(num_bytes) = config.bytes {
                    let mut buffer = vec![0; num_bytes];
                    let bytes_read = file
                        .take(num_bytes as u64)
                        .read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 { break; }
                        print!("{}", line);
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}


fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        // _ => Err(From::from(val)),
        _ => Err(val.into()),
    }
}

#[test]
fn test_parse_positive_int() {

    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    let res = parse_positive_int("foo");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());

    let res = parse_positive_int("0");
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
}

// MyResult returns a BufRead Trait, this trait is unsize so we need to Box it via Box<dyn T>
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        // if we pass a hyphen read from std::in otherwise create a buffered reader for a given file
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}