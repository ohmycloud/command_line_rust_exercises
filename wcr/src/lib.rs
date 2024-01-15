use clap::{Arg, ArgAction, Command};
use common::{open, MyResult};
use std::io::BufRead;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>, // 一个或多个文件
    lines: bool,        // 是否显示行数
    words: bool,        // 是否显示单词数
    bytes: bool,        // 是否显示字节数
    chars: bool,        // 是否显示字符数
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize, // 行数
    num_words: usize, // 单词数
    num_bytes: usize, // 字节数
    num_chars: usize, // 字符数
}

pub fn get_args() -> MyResult<Config> {
    let mut matches = Command::new("wcr")
        .version("0.1.0")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust wc")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .action(ArgAction::Append)
                .num_args(1..)
                .default_value("-")
                .help("Input file(s)"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .num_args(0)
                .help("Show word count"),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .num_args(0)
                .help("Show byte count"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .num_args(0)
                .help("Show character count"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .num_args(0)
                .help("Show line count"),
        )
        .get_matches();

    let files = matches
        .remove_many("files")
        .expect("`files` is required")
        .collect::<Vec<String>>();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let chars = matches.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files,
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-" {
                            "".to_string()
                        } else {
                            format!(" {}", &filename)
                        },
                    );

                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_field() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
