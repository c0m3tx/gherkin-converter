use crate::parser::Feature;
use crate::parser::Scenario;
use std::io::Write;

trait ToMarkdown {
    fn to_markdown(&self) -> String;
}

impl ToMarkdown for Feature {
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

impl ToMarkdown for Scenario {
    fn to_markdown(&self) -> String {
        let empty_row = "".to_string();
        let steps = self
            .steps
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        vec![
            format!("- [ ] {}", self.name),
            "```".to_string(),
            steps,
            "```".to_string(),
            empty_row.clone(),
        ]
        .join("\n")
    }
}

pub fn format(features: Vec<Feature>, mut writer: impl Write) -> Result<(), std::io::Error> {
    let output_content = features
        .iter()
        .map(Feature::to_markdown)
        .collect::<Vec<_>>()
        .join("\n\n");
    writer.write_all(output_content.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let feature = Feature {
            name: Some("Some feature".to_string()),
            description: None,
            scenarios: vec![
                Scenario {
                    name: "Some scenario".to_string(),
                    steps: vec!["Given A".into(), "When B".into(), "Then C".into()],
                },
                Scenario {
                    name: "Some other scenario".to_string(),
                    steps: vec!["Given A".into(), "When B".into(), "Then C".into()],
                },
            ],
        };
        assert_eq!(
            feature.to_markdown(),
            "## Some feature\n\n- [ ] Some scenario\n```\nGiven A\nWhen B\nThen C\n```\n\n- [ ] Some other scenario\n```\nGiven A\nWhen B\nThen C\n```\n"
        );
    }
}
