mod parser;

mod markdown_formatter;
mod youtrack_formatter;

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long, default_value = "markdown")]
    format: String,

    target: Option<String>,
}

macro_rules! parse_with {
    ($i:ident, $f:expr) => {{
        let features = parser::default_cli_parse($f);
        $i::format(features, std::io::stdout()).expect("Unable to write");
    }};
}

fn main() {
    let args = Args::parse();

    match args.format.as_str() {
        "youtrack" => parse_with!(youtrack_formatter, args.target),
        "markdown" => parse_with!(markdown_formatter, args.target),
        _ => {
            eprintln!("Unknown format: {}", args.format);
            std::process::exit(1);
        }
    }
}
