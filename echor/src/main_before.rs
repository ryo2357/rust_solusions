use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("ryo2357<test@test.com>")
        .about("Rust echo")
        .arg(
            Arg::new("test")
                .value_name("TEXT")
                .help("Input text")
                .required(true), // .min_values(1),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .required(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.os_present("omit_newline");
}

#[allow(dead_code)]
fn arg_verify() {
    println!("{:?}", std::env::args());
}
