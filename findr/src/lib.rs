use crate::EntryType::*;
use clap::{Arg, ArgAction, Command};
use common::MyResult;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[derive(Debug, Eq, PartialEq)]
enum EntryType {
    Dir,
    File,
    Link,
}

#[derive(Debug)]
pub struct Config {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("findr")
        .version("0.1.0")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust find")
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .default_value(".")
                .num_args(1..)
                .action(ArgAction::Append)
                .help("Search paths"),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .num_args(0..)
                .required(false)
                .action(ArgAction::Append)
                .help("Name"),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .num_args(0..)
                .action(ArgAction::Append)
                .value_parser(["f", "d", "l"])
                .help("Entry type"),
        )
        .get_matches();

    let paths = matches
        .remove_many("paths")
        .expect("expected paths")
        .collect::<Vec<String>>();

    let names = matches
        .remove_many("names")
        .expect("expected names")
        .collect::<Vec<String>>()
        .iter()
        .map(|name| Regex::new(name).map_err(|_| format!("Invalid --name \"{}\"", name)))
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    let entry_types = matches
        .remove_many("types")
        .expect("expected types")
        .collect::<Vec<String>>()
        .iter()
        .map(|typ| match typ.as_str() {
            "d" => Dir,
            "f" => File,
            "l" => Link,
            _ => unreachable!("Invalid type"),
        })
        .collect::<Vec<EntryType>>();

    Ok(Config {
        paths,
        names,
        entry_types,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let type_filter = |entry: &DirEntry| {
        config.entry_types.is_empty()
            || config
                .entry_types
                .iter()
                .any(|entry_type| match entry_type {
                    Link => entry.path_is_symlink(),
                    Dir => entry.file_type().is_dir(),
                    File => entry.file_type().is_file(),
                })
    };

    let name_filter = |entry: &DirEntry| {
        config.names.is_empty()
            || config
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in config.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{}", e);
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();
        println!("{}", entries.join("\n"));
    }
    Ok(())
}
