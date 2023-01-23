#![allow(dead_code, unused_imports)]

mod markdown_formatter;
mod parser;
mod youtrack_formatter;

fn main() {
    let format = std::env::args().nth(1);
    match format {
        Some(format) => {
            let features = parser::default_cli_parse(std::env::args().nth(2));
            match format.as_str() {
                "youtrack" => youtrack_formatter::format(features, std::io::stdout())
                    .expect("Unable to write"),
                "markdown" => markdown_formatter::format(features, std::io::stdout())
                    .expect("Unable to write"),
                _ => {
                    eprintln!("Unsupported format: {}", format);
                    std::process::exit(1);
                }
            }
        }
        None => {
            eprintln!("Format is required. Supported formatters: markdown, youtrack");
            std::process::exit(1)
        }
    }
}
