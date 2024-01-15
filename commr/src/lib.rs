use crate::Column::*;
use clap::{Arg, Command};
use common::{open, MyResult};
use std::{cmp::Ordering::*, io::BufRead};

#[derive(Debug)]
enum Column<'a> {
    Col1(&'a str),
    Col2(&'a str),
    Col3(&'a str),
}

#[derive(Debug)]
pub struct Config {
    file1: String,
    file2: String,
    show_col1: bool,
    show_col2: bool,
    show_col3: bool,
    insensitive: bool,
    delimiter: String,
}

// --------------------------------------------------
pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("commr")
        .version("0.1.0")
        .author("ohmycloud <ohmycoudy@gmail.com>")
        .about("Rust comm")
        .arg(
            Arg::new("file1")
                .value_name("FILE1")
                .num_args(1)
                .required(true)
                .help("Input file 1"),
        )
        .arg(
            Arg::new("file2")
                .value_name("FILE2")
                .num_args(1)
                .required(true)
                .help("Input file 2"),
        )
        .arg(
            Arg::new("suppress_col1")
                .short('1')
                .num_args(0)
                .help("Suppress printing of column 1"),
        )
        .arg(
            Arg::new("suppress_col2")
                .short('2')
                .num_args(0)
                .help("Suppress printing of column 2"),
        )
        .arg(
            Arg::new("suppress_col3")
                .short('3')
                .num_args(0)
                .help("Suppress printing of column 3"),
        )
        .arg(
            Arg::new("insensitive")
                .short('i')
                .num_args(0)
                .help("Case-insensitive comparison of lines"),
        )
        .arg(
            Arg::new("delimiter")
                .short('d')
                .long("output-delimiter")
                .value_name("DELIM")
                .default_value("\t")
                .num_args(1)
                .help("Output delimiter"),
        )
        .get_matches();

    Ok(Config {
        file1: matches
            .get_one::<String>("file1")
            .expect("expected file1")
            .to_string(),
        file2: matches
            .get_one::<String>("file2")
            .expect("expected file2")
            .to_string(),
        show_col1: !matches.get_flag("suppress_col1"),
        show_col2: !matches.get_flag("suppress_col2"),
        show_col3: !matches.get_flag("suppress_col3"),
        insensitive: matches.get_flag("insensitive"),
        delimiter: matches.get_one::<String>("delimiter").unwrap().to_string(),
    })
}

// --------------------------------------------------
pub fn run(config: Config) -> MyResult<()> {
    let file1 = &config.file1;
    let file2 = &config.file2;

    if file1 == "-" && file2 == "-" {
        return Err(From::from("Both input files cannot be STDIN (\"-\")"));
    }

    let case = |line: String| {
        if config.insensitive {
            line.to_lowercase()
        } else {
            line
        }
    };

    let mut lines1 = open(file1)?.lines().filter_map(Result::ok).map(case);
    let mut lines2 = open(file2)?.lines().filter_map(Result::ok).map(case);

    let print = |col: Column| {
        let mut columns = vec![];
        match col {
            Col1(val) => {
                if config.show_col1 {
                    columns.push(val);
                }
            }
            Col2(val) => {
                if config.show_col2 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
            Col3(val) => {
                if config.show_col3 {
                    if config.show_col1 {
                        columns.push("");
                    }
                    if config.show_col2 {
                        columns.push("");
                    }
                    columns.push(val);
                }
            }
        };

        if !columns.is_empty() {
            println!("{}", columns.join(&config.delimiter));
        }
    };

    let mut line1 = lines1.next();
    let mut line2 = lines2.next();

    while line1.is_some() || line2.is_some() {
        match (&line1, &line2) {
            (Some(val1), Some(val2)) => match val1.cmp(val2) {
                Equal => {
                    print(Col3(val1));
                    line1 = lines1.next();
                    line2 = lines2.next();
                }
                Less => {
                    print(Col1(val1));
                    line1 = lines1.next();
                }
                Greater => {
                    print(Col2(val2));
                    line2 = lines2.next();
                }
            },
            (Some(val1), None) => {
                print(Col1(val1));
                line1 = lines1.next();
            }
            (None, Some(val2)) => {
                print(Col2(val2));
                line2 = lines2.next();
            }
            _ => (),
        }
    }

    Ok(())
}
