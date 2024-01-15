use clap::{Arg, ArgAction, Command};
use common::{open, MyResult};
use std::io::{BufRead, Read};

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(mut file) => {
                println!("==> {} <==", filename);
                if let Some(num_bytes) = config.bytes {
                    let bytes = file.bytes().take(num_bytes).collect::<Result<Vec<_>, _>>();
                    print!("{}", String::from_utf8_lossy(&bytes?));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
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

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        // For any other outcome, return an Err with the given value.
        // The From trait simplifies error handling by allowing a function to return a single error
        // type that encapsulates multiple error types.
        _ => Err(From::from(val)),
    }
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("headr")
        .version("0.1.0")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .action(ArgAction::Append)
                .num_args(1..)
                .default_value("-")
                .help("Input file(s)"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .num_args(0..=1)
                .required(false)
                .default_value("10")
                .value_parser(clap::value_parser!(usize))
                .conflicts_with("bytes")
                .help("Number of lines [default: 10]"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .num_args(0..=1)
                .required(false)
                .value_parser(clap::value_parser!(usize))
                .help("Number of bytes"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .remove_many("files")
        .expect("`files` is required")
        .collect();

    let lines = matches
        .get_one("lines")
        .map(|n| *n)
        .expect("Expect a number");

    let bytes = matches.get_one("bytes").map(|n| *n);

    Ok(Config {
        files,
        lines,
        bytes,
    })
}
