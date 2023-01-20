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
    name: Option<String>,
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

fn parse_scenario(input: &[&str]) -> Scenario {
    let name = input[0].trim().to_string().replace("Scenario: ", "");
    let steps = input[1..input.len()]
        .iter()
        .filter_map(|&s| (!s.is_empty()).then_some(s.trim().to_owned()))
        .collect();

    Scenario { name, steps }
}

fn parse_feature(name: Option<String>, input: &[&str]) -> Feature {
    let scenario_rows: Vec<_> = input
        .iter()
        .enumerate()
        .filter(|(_, l)| l.trim().starts_with("Scenario:"))
        .map(|(i, _)| i)
        .chain(std::iter::once(input.len()))
        .collect();

    let scenarios = scenario_rows
        .windows(2)
        .map(|window| (window[0], window[1]))
        .map(|(start, end)| parse_scenario(&input[start..end]))
        .collect();

    Feature { name, scenarios }
}

fn parse_input(input: &str) -> Vec<Feature> {
    let input: Vec<_> = input.lines().collect();

    let feature_rows: Vec<_> = input
        .iter()
        .enumerate()
        .filter(|(_, l)| l.trim().starts_with("Feature:"))
        .map(|(i, _)| i)
        .chain(std::iter::once(input.len()))
        .collect();

    if feature_rows.len() == 1 {
        vec![parse_feature(None, &input)]
    } else {
        feature_rows
            .windows(2)
            .map(|window| (window[0], window[1]))
            .map(|(start, end)| {
                let name = input[start].trim().to_string();
                let name = name
                    .contains("Feature: ")
                    .then_some(name.replace("Feature: ", ""));
                let feature = &input[start..(end - start)];
                parse_feature(name, feature)
            })
            .collect()
    }
}

impl Feature {
    fn to_markdown(&self) -> String {
        let empty_row = "".to_string();

        let scenarios: Vec<_> = self.scenarios.iter().map(Scenario::to_markdown).collect();
        let scenarios = scenarios.join("\n");

        match &self.name {
            Some(name) => vec![format!("## {}", name), empty_row.clone(), scenarios].join("\n"),
            None => scenarios,
        }
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
    let features = parse_input(&input);
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
    fn test_parse_input() {
        let features = parse_input(TEST_INPUT);
        assert_eq!(features.len(), 1);
        let feature = &features[0];
        assert_eq!(feature.name, Some("feature name".to_string()));
        let scenario = &feature.scenarios[0];
        assert_eq!(scenario.name, "scenario name");
        let steps = &scenario.steps;
        assert_eq!(steps.len(), 3);

        let scenario_2 = &feature.scenarios[1];
        assert_eq!(scenario_2.name, "another scenario");
        let steps = &scenario_2.steps;
        assert_eq!(steps.len(), 4);
    }

    #[test]
    fn test_to_markdown() {
        let feature = Feature {
            name: Some("Some feature".to_string()),
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
