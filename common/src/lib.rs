use assert_cmd::Command;
use std::error::Error;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};

pub type MyResult<T> = Result<T, Box<dyn Error>>;
pub type TestResult = Result<(), Box<dyn Error>>;

pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(pkg: &str, args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(pkg)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
