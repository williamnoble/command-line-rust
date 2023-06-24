use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.0.1")
        .author("Will")
        .about("A simple version of echo in Rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        // this is like flag.Parse()
        .get_matches();

    // text is a Vec<String>, safe to unwrap as .required(true)
    let text = matches.values_of_lossy("text").unwrap();

    // simple bool flag
    let omit_newline = matches.is_present("omit_newline");

    // ternary operator in Rust
    let ending = if omit_newline { "" } else { "\n" };

    // text.join because text is a Vec<String>
    // e.g. ["Hello".to_string(), World".to_string()] -> "Hello World"
    print!("{}{}", text.join(" "), ending);
}


