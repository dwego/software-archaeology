use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ParsedSubmission {
    pub case: String,
    pub root_cause: String,
    pub fault_change: String,
    pub evidence: String,
    pub detection: String,
    pub corrective_action: String,
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

pub fn parse_issue_body(body: &str) -> Result<ParsedSubmission, String> {
    let mut sections: HashMap<String, String> = HashMap::new();

    let mut current_heading: Option<String> = None;
    let mut current_content = Vec::new();

    for line in body.lines() {
        if let Some(heading) = line.strip_prefix("### ") {
            if let Some(previous) = current_heading.take() {
                sections.insert(previous, current_content.join("\n").trim().to_string());
                current_content.clear();
            }

            current_heading = Some(normalize_heading(heading));
        } else if current_heading.is_some() {
            current_content.push(line);
        }
    }

    if let Some(last_heading) = current_heading {
        sections.insert(last_heading, current_content.join("\n").trim().to_string());
    }

    let get = |name: &str| {
        sections
            .get(name)
            .cloned()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| format!("missing issue field: {name}"))
    };

    Ok(ParsedSubmission {
        case: get("case")?,
        root_cause: get("root cause")?,
        fault_change: get("fault introducing change")?,
        evidence: get("supporting evidence")?,
        detection: get("failure detection")?,
        corrective_action: get("corrective action")?,
    })
}