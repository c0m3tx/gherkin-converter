use std::io::Read;
use std::io::Write;

#[derive(Debug)]
pub struct Step {
    pub keyword: String,
    pub description: String,
}

impl From<&str> for Step {
    fn from(input: &str) -> Self {
        match input.split_once(" ") {
            Some((keyword, description)) => Self {
                keyword: keyword.into(),
                description: description.into(),
            },
            None => Self {
                keyword: "".into(),
                description: input.into(),
            },
        }
    }
}

impl ToString for Step {
    fn to_string(&self) -> String {
        format!("{} {}", self.keyword, self.description)
    }
}

#[derive(Debug)]
pub struct Scenario {
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug)]
pub struct Feature {
    pub name: Option<String>,
    pub description: Option<String>,
    pub scenarios: Vec<Scenario>,
}

pub fn default_cli_parse(file: Option<String>) -> Vec<Feature> {
    let features = match file {
        None => parse_stdin(std::io::stdin()),
        Some(filename) => parse_file(filename),
    };

    match features {
        Ok(features) => features,
        Err(error) => {
            eprintln!("Unable to parse input: {}", error);
            std::process::exit(1);
        }
    }
}

pub fn parse_file(filename: String) -> Result<Vec<Feature>, std::io::Error> {
    let mut file = std::fs::File::open(filename)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(parse_input(&buffer))
}

pub fn parse_stdin(mut stdin: impl Read) -> Result<Vec<Feature>, std::io::Error> {
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer)?;

    Ok(parse_input(&buffer))
}

fn parse_scenario(input: &[&str]) -> Scenario {
    let name = input[0].trim().to_string().replace("Scenario: ", "");
    let steps = input[1..input.len()]
        .iter()
        .filter_map(|&s| (!s.is_empty()).then_some(s.trim().into()))
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

    let description: Vec<String> = input[1..scenario_rows[0]]
        .iter()
        .filter_map(|&s| (!s.is_empty()).then_some(s.trim().into()))
        .collect();

    let description = (description.len() > 0).then_some(description.join("\n"));

    let scenarios = scenario_rows
        .windows(2)
        .map(|window| (window[0], window[1]))
        .map(|(start, end)| parse_scenario(&input[start..end]))
        .collect();

    Feature {
        name,
        description,
        scenarios,
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../test_input.txt");

    #[test]
    fn test_parse_input() {
        let features = parse_input(TEST_INPUT);
        assert_eq!(features.len(), 1);
        let feature = &features[0];
        assert_eq!(feature.name, Some("feature name".to_string()));
        assert_eq!(feature.description, Some("This is the feature description.\nIt may span multiple lines, but it is not required.".into()));
        let scenario = &feature.scenarios[0];
        assert_eq!(scenario.name, "scenario name");
        let steps = &scenario.steps;
        assert_eq!(steps.len(), 3);

        let scenario_2 = &feature.scenarios[1];
        assert_eq!(scenario_2.name, "another scenario");
        let steps = &scenario_2.steps;
        assert_eq!(steps.len(), 4);
    }
}
