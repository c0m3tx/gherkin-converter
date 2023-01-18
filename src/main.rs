#![allow(dead_code, unused_imports)]

use std::io::Read;
use std::io::Write;

#[derive(Debug)]
struct Scenario {
    name: String,
    steps: Vec<String>,
}

#[derive(Debug)]
struct Feature {
    name: String,
    scenarios: Vec<Scenario>,
}

fn read_input(file: Option<String>, mut reader: impl Read) -> Result<String, std::io::Error> {
    // read the input from the file passed as the first argument, or stdin if the first argument is None

    let result = match file {
        None => {
            let mut buffer = String::new();
            reader.read_to_string(&mut buffer)?;
            buffer
        }
        Some(filename) => {
            let mut file = std::fs::File::open(filename)?;
            let mut buffer = String::new();
            file.read_to_string(&mut buffer)?;
            buffer
        }
    };

    Ok(result)
}

fn write_output(features: Vec<Feature>, mut writer: impl Write) -> Result<(), std::io::Error> {
    let output_content = features
        .iter()
        .map(Feature::to_markdown)
        .collect::<Vec<_>>()
        .join("\n\n");
    writer.write_all(output_content.as_bytes())
}

fn parse_scenario(input: &str) -> Scenario {
    let scenario_name = input.lines().next().unwrap().trim().to_string();
    let steps = input
        .lines()
        .skip(1)
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    Scenario {
        name: scenario_name,
        steps,
    }
}

fn parse_input(input: String) -> Vec<Feature> {
    // parse the input into a list of features
    input
        .split("Feature:")
        .filter(|s| !s.is_empty())
        .map(|fc| {
            let name = fc.lines().next().unwrap().trim().to_string();
            let scenarios = fc.split("Scenario:").skip(1).map(parse_scenario).collect();
            Feature { name, scenarios }
        })
        .collect()
}

impl Feature {
    fn to_markdown(&self) -> String {
        let empty_row = "".to_string();

        let scenarios: Vec<_> = self.scenarios.iter().map(Scenario::to_markdown).collect();
        let scenarios = scenarios.join("\n");

        vec![format!("## {}", self.name), empty_row.clone(), scenarios].join("\n")
    }
}

impl Scenario {
    fn to_markdown(&self) -> String {
        let empty_row = "".to_string();

        vec![
            format!("- [ ] {}", self.name),
            "```".to_string(),
            self.steps.join("\n"),
            "```".to_string(),
            empty_row.clone(),
        ]
        .join("\n")
    }
}

fn main() {
    let input = read_input(std::env::args().nth(1), std::io::stdin()).unwrap();
    let features = parse_input(input);
    write_output(features, std::io::stdout()).expect("Unable to write")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../test_input.txt");

    #[test]
    fn test_reading_from_stdin() {
        let stdin = TEST_INPUT.clone().as_bytes();
        let result = read_input(None, stdin).unwrap();
        assert_eq!(result, TEST_INPUT);
    }

    #[test]
    fn test_reading_from_file() {
        let stdin = "".as_bytes();
        let result = read_input(Some("test_input.txt".to_string()), stdin).unwrap();
        assert_eq!(result, TEST_INPUT);
    }

    #[test]
    fn test_to_markdown() {
        let feature = Feature {
            name: "Some feature".to_string(),
            scenarios: vec![
                Scenario {
                    name: "Some scenario".to_string(),
                    steps: vec![
                        "Given A".to_string(),
                        "When B".to_string(),
                        "Then C".to_string(),
                    ],
                },
                Scenario {
                    name: "Some other scenario".to_string(),
                    steps: vec![
                        "Given A".to_string(),
                        "When B".to_string(),
                        "Then C".to_string(),
                    ],
                },
            ],
        };
        assert_eq!(
            feature.to_markdown(),
            "## Some feature\n\n- [ ] Some scenario\n```\nGiven A\nWhen B\nThen C\n```\n\n- [ ] Some other scenario\n```\nGiven A\nWhen B\nThen C\n```\n"
        );
    }
}
