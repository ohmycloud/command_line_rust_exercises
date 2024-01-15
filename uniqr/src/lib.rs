use clap::{Arg, Command};
use common::{open, MyResult};
use std::fs::File;
use std::io::{self, BufRead, Write};

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut file = open(&config.in_file).map_err(|e| format!("{}: {}", config.in_file, e))?;

    let mut out_file: Box<dyn Write> = match &config.out_file {
        Some(out_file) => Box::new(File::create(out_file)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |count: u64, text: &str| -> MyResult<()> {
        if count > 0 {
            if config.count {
                write!(out_file, "{:>4} {}", count, text)?;
            } else {
                write!(out_file, "{}", text)?;
            }
        };
        Ok(())
    };

    let mut line = String::new();
    let mut previous = String::new();
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous = line.clone();
            count = 0;
        }

        count += 1;
        line.clear();
    }
    print(count, &previous)?;

    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("uniqr")
        .version("0.1.1")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust uniq")
        .arg(
            Arg::new("in_file")
                .value_name("IN_FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(
            Arg::new("out_file")
                .value_name("OUT_FILE")
                .required(false)
                .help("Output file"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .num_args(0)
                .help("Show counts"),
        )
        .get_matches();

    let in_file: String = matches
        .get_one::<String>("in_file")
        .expect("Please give a file path")
        .to_string();
    let out_file = matches.get_one::<String>("out_file").map(|x| x.to_string());
    let count = matches.get_flag("count");

    Ok(Config {
        in_file,
        out_file,
        count,
    })
}
