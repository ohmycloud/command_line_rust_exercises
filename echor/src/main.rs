use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .bin_name("echor")
        .version("0.1.0")
        .author("ohmycloud ohmycloudy@gmail.com")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .long("new_line")
                .short('n')
                .action(clap::ArgAction::SetTrue)
                .help("Do not print newline"),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .try_get_raw("text")
        .expect("s")
        .clone()
        .unwrap()
        .map(|x| x.to_str().unwrap())
        .collect();
    let omit_newline = matches.get_flag("omit_newline");

    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
