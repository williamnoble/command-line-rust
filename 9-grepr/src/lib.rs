use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use clap::{App, Arg};
use regex::{Regex, RegexBuilder};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    pattern: Regex,
    files: Vec<String>,
    recursive: bool,
    count: bool,
    invert_match: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("9-grepr")
        .version("0.1")
        .author("William")
        .about("RUST Grep")
        .arg(
            Arg::with_name("pattern").value_name("PATTERN").help("Search pattern").required(true),
        ).arg(
        Arg::with_name("files").value_name("FILE").help("Input file(s)").multiple(true).default_value("-"),
    ).arg(
        Arg::with_name("insensitive")
            .short("i")
            .long("insensitive").help("Case-insensitive").takes_value(false),
    ).arg(
        Arg::with_name("recursive")
            .short("r")
            .long("recursive").help("Recursive search").takes_value(false),
    ).arg(
        Arg::with_name("count")
            .short("c")
            .long("count").help("Count occurrences").takes_value(false),
    ).arg(
        Arg::with_name("invert")
            .short("v")
            .long("invert-match").help("Invert match").takes_value(false),
    )
        .get_matches();

    let pattern = matches.value_of("pattern").unwrap();
    let pattern = RegexBuilder::new(pattern)
        .case_insensitive(matches.is_present("insensitive"))
        .build()
        .map_err(|_| format!("Invalid Pattern \"{}\"", pattern))?;

    // We're returning a Result<> so return OK
    Ok(Config {
        pattern,
        files: matches.values_of_lossy("files").unwrap(),
        recursive: matches.is_present("recursive"),
        count: matches.is_present("count"),
        invert_match: matches.is_present("invert"),
    })
}

// void function
pub fn run(config: Config) -> MyResult<()> {
    let entries = find_files(&config.files, config.recursive);
    for entry in entires {
        match entry{
            Err(e) => eprintln!("{}", e);

        }
    }
    println!("{:#?}", config);
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

fn find_lines<T: BufRead>(mut file: T, pattern: &Regex, invert_match: bool) -> MyResult<Vec<String>>{
    unimplemented!();
}

