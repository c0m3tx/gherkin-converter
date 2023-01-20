use gherkin_to_markdown::default_cli_parse;
use gherkin_to_markdown::Feature;
use gherkin_to_markdown::Scenario;
use gherkin_to_markdown::Step;
use std::io::Write;

trait ToYoutrack {
    fn to_yt(&self) -> String;
}

impl ToYoutrack for Feature {
    fn to_yt(&self) -> String {
        let scenarios = self
            .scenarios
            .iter()
            .map(Scenario::to_yt)
            .collect::<Vec<_>>()
            .join("\n\n");

        match &self.name {
            Some(name) => format!("## {}\n{}", name, scenarios),
            None => scenarios,
        }
    }
}

impl ToYoutrack for Scenario {
    fn to_yt(&self) -> String {
        let steps = self
            .steps
            .iter()
            .map(Step::to_yt)
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"- [ ] {}
<pre style="padding-top: 10px; padding-bottom: 10px; margin-bottom: 20px">{}</pre>"#,
            self.name, steps
        )
    }
}

impl ToYoutrack for Step {
    fn to_yt(&self) -> String {
        format!(
            r#"<span style="color: darkorange">{}</span> {}"#,
            self.keyword, self.description
        )
    }
}

fn write_output(features: Vec<Feature>, mut writer: impl Write) -> Result<(), std::io::Error> {
    let html = features
        .iter()
        .map(Feature::to_yt)
        .collect::<Vec<_>>()
        .join("\n\n");
    writer.write_all(html.as_bytes())
}

fn main() {
    let features = default_cli_parse();
    write_output(features, std::io::stdout()).expect("Unable to write");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output() {
        let feature = Feature {
            name: Some("Some feature".to_string()),
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
            feature.to_yt(),
            r#"## Some feature
- [ ] Some scenario
<pre style="padding-top: 10px; padding-bottom: 10px; margin-bottom: 20px"><span style="color: darkorange">Given</span> A
<span style="color: darkorange">When</span> B
<span style="color: darkorange">Then</span> C</pre>

- [ ] Some other scenario
<pre style="padding-top: 10px; padding-bottom: 10px; margin-bottom: 20px"><span style="color: darkorange">Given</span> A
<span style="color: darkorange">When</span> B
<span style="color: darkorange">Then</span> C</pre>"#
        );
    }
}
