use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParsedSubmission {
    pub case: String,
    pub findings: Vec<String>,
    pub fault_change: String,
    pub evidence: Vec<String>,
    pub corrective_findings: Vec<String>,
    pub explanation: String,
}

fn normalize_heading(value: &str) -> String {
    value
        .trim()
        .to_lowercase()
        .replace('-', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .lines()
        .map(|line| {
            line.trim()
                .trim_start_matches('-')
                .trim_start_matches('*')
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn parse_issue_body(body: &str) -> Result<ParsedSubmission, String> {
    let mut sections = HashMap::new();

    let mut current_heading: Option<String> = None;
    let mut current_content = Vec::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("### ") {
            if let Some(previous) = current_heading.take() {
                sections.insert(
                    previous,
                    current_content.join("\n").trim().to_string(),
                );

                current_content.clear();
            }

            current_heading = Some(normalize_heading(heading));
        } else if current_heading.is_some() {
            current_content.push(line.to_string());
        }
    }

    if let Some(last_heading) = current_heading {
        sections.insert(
            last_heading,
            current_content.join("\n").trim().to_string(),
        );
    }

    let get = |name: &str| {
        sections
            .get(name)
            .cloned()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| format!("missing issue field: {name}"))
    };

    Ok(ParsedSubmission {
        case: get("case")?,
        findings: parse_list(&get("findings")?),
        fault_change: get("fault introducing change")?,
        evidence: parse_list(&get("supporting evidence")?),
        corrective_findings: parse_list(
            &get("corrective findings")?
        ),
        explanation: get("investigation explanation")?,
    })
}