use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_non_blank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .author("William")
        .version("0.0.1")
        .about("'cat' but written in Rust")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number of lines")
                .takes_value(false)
                .conflicts_with("number_non_blank"),
        )
        .arg(
            Arg::with_name("number_non_blank")
                .short("b")
                .long("number-non-blank")
                .help("The number of non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_non_blank_lines: matches.is_present("number_non_blank"),
    })
}

// return Box which implements BufRead Trait. dash => stdin, file => file::open
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {

                let mut last_num = 0;

                for (line_num, line) in file.lines().enumerate() {

                    // this shadows line but opens the Result via ?
                    let line = line?;

                    if config.number_lines {
                        // return the number of lines, counting empty_lines,
                        // lines are zero indexed so return n + 1
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_non_blank_lines {
                        // if the line is empty print empty otherwise
                        if !line.is_empty() {
                            // for every line with isn't empty increment by 1
                            last_num += 1;
                            println!("{:>6}\t{}", last_num, line);
                        }
                    } else {
                        // if no args print line
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}