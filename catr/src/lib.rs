use clap::{Arg, ArgAction, Command};
use common::{open, MyResult};
use std::io::BufRead;

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(content) => {
                let mut last_num = 0;
                for (index, line) in content.lines().enumerate() {
                    if let Ok(line) = line {
                        if config.number_lines {
                            println!("{:>6}\t{}", index + 1, line);
                        } else if config.number_nonblank_lines {
                            if !line.is_empty() {
                                last_num += 1;
                                println!("{:>6}\t{}", last_num, line);
                            } else {
                                println!();
                            }
                        } else {
                            println!("{}", line);
                        }
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
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("catr")
        .version("0.1.0")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .action(ArgAction::Append)
                .help("Input file(s)")
                .num_args(1..)
                .default_value("-"),
        )
        .arg(
            Arg::new("number")
                .short('n')
                .long("number")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("number_nonblank")
                .help("Number lines"),
        )
        .arg(
            Arg::new("number_nonblank")
                .short('b')
                .long("number-nonblank")
                .action(clap::ArgAction::SetTrue)
                .help("Number non-blank lines"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .remove_many("files")
        .expect("`files` is required")
        .collect();

    Ok(Config {
        files,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}
