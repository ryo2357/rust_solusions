use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    version="0.1.0",
    about="Rust echo",
    long_about = None,
    arg_required_else_help(true)
)]
struct Cli {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,

    /// Do not print newline
    #[arg(short = 'n')]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    let text = cli.text.join(" ");
    if cli.omit_newline {
        print!("{}", text);
    } else {
        println!("{}", text);
    }
}
