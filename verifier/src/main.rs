mod issue_parser;

use clap::{Parser, Subcommand};
use issue_parser::parse_issue_body;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs,
    path::PathBuf,
    process,
};

#[derive(Parser)]
#[command(name = "verifier")]
#[command(about = "Software Archaeology investigation verifier")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Verify {
        #[arg(long)]
        answer: PathBuf,

        #[arg(long)]
        submission: PathBuf,
    },

    ParseIssue {
        #[arg(long)]
        input: PathBuf,

        #[arg(long)]
        output: PathBuf,
    },
}

#[derive(Debug, Deserialize)]
struct CaseAnswer {
    case: String,
    title: String,

    required_findings: Vec<FindingRule>,

    fault_change: IdentifierRule,

    #[serde(default)]
    evidence: EvidenceRule,

    corrective_action: FindingGroup,

    #[serde(default = "default_required_explanation")]
    require_explanation: bool,
}

#[derive(Debug, Deserialize)]
struct FindingRule {
    id: String,
    category: String,

    #[serde(default)]
    aliases: Vec<String>,

    #[serde(default = "default_required")]
    required: bool,
}

#[derive(Debug, Deserialize)]
struct FindingGroup {
    required_findings: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct IdentifierRule {
    accepted: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
struct EvidenceRule {
    #[serde(default)]
    accepted: Vec<EvidenceItem>,

    #[serde(default)]
    minimum: usize,
}

#[derive(Debug, Deserialize)]
struct EvidenceItem {
    id: String,

    #[serde(default)]
    aliases: Vec<String>,

    #[serde(default)]
    required: bool,
}

#[derive(Debug, Deserialize)]
struct Submission {
    case: String,

    #[serde(default)]
    findings: Vec<String>,

    fault_change: String,

    #[serde(default)]
    evidence: Vec<String>,

    #[serde(default)]
    corrective_findings: Vec<String>,

    #[serde(default)]
    explanation: String,
}

#[derive(Debug, Serialize)]
struct VerificationResult {
    case: String,
    title: String,
    status: String,
    checks: Checks,
}

#[derive(Debug, Serialize)]
struct Checks {
    findings: FindingCheckResult,
    fault_change: IdentifierCheckResult,
    evidence: EvidenceCheckResult,
    corrective_action: FindingCheckResult,
    explanation: ExplanationCheckResult,
}

#[derive(Debug, Serialize)]
struct FindingCheckResult {
    passed: bool,
    matched: Vec<String>,
    missing_required: Vec<String>,
    categories: Vec<CategoryResult>,
}

#[derive(Debug, Serialize)]
struct CategoryResult {
    category: String,
    passed: bool,
}

#[derive(Debug, Serialize)]
struct IdentifierCheckResult {
    passed: bool,
}

#[derive(Debug, Serialize)]
struct EvidenceCheckResult {
    passed: bool,
    matched: Vec<String>,
    missing_required: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ExplanationCheckResult {
    passed: bool,
}

fn default_required() -> bool {
    true
}

fn default_required_explanation() -> bool {
    true
}

fn normalize(value: &str) -> String {
    value
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '-' || c.is_whitespace() {
                c
            } else {
                ' '
            }
        })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn matches_identifier(value: &str, expected: &str) -> bool {
    normalize(value) == normalize(expected)
}

fn matches_any(value: &str, candidates: &[String]) -> bool {
    candidates
        .iter()
        .any(|candidate| matches_identifier(value, candidate))
}

fn evaluate_identifier(
    value: &str,
    rule: &IdentifierRule,
) -> IdentifierCheckResult {
    IdentifierCheckResult {
        passed: matches_any(value, &rule.accepted),
    }
}

fn evaluate_findings(
    submitted: &[String],
    rules: &[FindingRule],
) -> FindingCheckResult {
    let submitted_normalized: Vec<String> =
        submitted.iter().map(|value| normalize(value)).collect();

    let mut matched = Vec::new();
    let mut missing_required = Vec::new();

    let mut categories: HashSet<String> = HashSet::new();

    for rule in rules {
        categories.insert(rule.category.clone());

        let mut candidates = vec![rule.id.clone()];
        candidates.extend(rule.aliases.clone());

        let found = submitted_normalized.iter().any(|submitted_value| {
            candidates
                .iter()
                .any(|candidate| {
                    submitted_value == &normalize(candidate)
                })
        });

        if found {
            matched.push(rule.id.clone());
        } else if rule.required {
            missing_required.push(rule.id.clone());
        }
    }

    matched.sort();
    missing_required.sort();

    let category_results = categories
        .into_iter()
        .map(|category| {
            let rules_in_category: Vec<&FindingRule> = rules
                .iter()
                .filter(|rule| rule.category == category)
                .collect();

            let required_in_category: Vec<&FindingRule> = rules_in_category
                .iter()
                .copied()
                .filter(|rule| rule.required)
                .collect();

            let passed = required_in_category.iter().all(|rule| {
                matched.contains(&rule.id)
            });

            CategoryResult {
                category,
                passed,
            }
        })
        .collect::<Vec<_>>();

    FindingCheckResult {
        passed: missing_required.is_empty(),
        matched,
        missing_required,
        categories: category_results,
    }
}

fn evaluate_evidence(
    submitted: &[String],
    rule: &EvidenceRule,
) -> EvidenceCheckResult {
    let submitted_normalized: Vec<String> =
        submitted.iter().map(|value| normalize(value)).collect();

    let mut matched = Vec::new();
    let mut missing_required = Vec::new();

    for expected in &rule.accepted {
        let mut candidates = vec![expected.id.clone()];
        candidates.extend(expected.aliases.clone());

        let found = submitted_normalized.iter().any(|submitted_value| {
            candidates
                .iter()
                .any(|candidate| {
                    let candidate = normalize(candidate);

                    submitted_value == &candidate
                        || submitted_value.contains(&candidate)
                })
        });

        if found {
            matched.push(expected.id.clone());
        } else if expected.required {
            missing_required.push(expected.id.clone());
        }
    }

    matched.sort();
    missing_required.sort();

    let enough_evidence = matched.len() >= rule.minimum;

    EvidenceCheckResult {
        passed: enough_evidence && missing_required.is_empty(),
        matched,
        missing_required,
    }
}

fn evaluate_corrective_action(
    submitted: &[String],
    rule: &FindingGroup,
) -> FindingCheckResult {
    let rules = rule
        .required_findings
        .iter()
        .map(|id| FindingRule {
            id: id.clone(),
            category: "corrective_action".to_string(),
            aliases: Vec::new(),
            required: true,
        })
        .collect::<Vec<_>>();

    evaluate_findings(submitted, &rules)
}

fn evaluate_explanation(
    explanation: &str,
    required: bool,
) -> ExplanationCheckResult {
    if !required {
        return ExplanationCheckResult { passed: true };
    }

    ExplanationCheckResult {
        passed: !explanation.trim().is_empty(),
    }
}

fn read_json<T>(path: &PathBuf) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path)
        .map_err(|err| {
            format!("failed to read {}: {err}", path.display())
        })?;

    serde_json::from_str(&content)
        .map_err(|err| {
            format!("failed to parse {}: {err}", path.display())
        })
}

fn verify(
    answer_path: PathBuf,
    submission_path: PathBuf,
) -> Result<(), String> {
    let answer: CaseAnswer = read_json(&answer_path)?;
    let submission: Submission = read_json(&submission_path)?;

    if answer.case != submission.case {
        return Err(format!(
            "case mismatch: expected {}, received {}",
            answer.case, submission.case
        ));
    }

    let checks = Checks {
        findings: evaluate_findings(
            &submission.findings,
            &answer.required_findings,
        ),

        fault_change: evaluate_identifier(
            &submission.fault_change,
            &answer.fault_change,
        ),

        evidence: evaluate_evidence(
            &submission.evidence,
            &answer.evidence,
        ),

        corrective_action: evaluate_corrective_action(
            &submission.corrective_findings,
            &answer.corrective_action,
        ),

        explanation: evaluate_explanation(
            &submission.explanation,
            answer.require_explanation,
        ),
    };

    let solved = checks.findings.passed
        && checks.fault_change.passed
        && checks.evidence.passed
        && checks.corrective_action.passed
        && checks.explanation.passed;

    let result = VerificationResult {
        case: answer.case,
        title: answer.title,
        status: if solved {
            "solved".to_string()
        } else {
            "incomplete".to_string()
        },
        checks,
    };

    let json = serde_json::to_string_pretty(&result)
        .map_err(|err| {
            format!("failed to serialize result: {err}")
        })?;

    println!("{json}");

    Ok(())
}

fn parse_issue(
    input: PathBuf,
    output: PathBuf,
) -> Result<(), String> {
    let body = fs::read_to_string(&input)
        .map_err(|err| {
            format!("failed to read {}: {err}", input.display())
        })?;

    let submission = parse_issue_body(&body)?;

    let json = serde_json::to_string_pretty(&submission)
        .map_err(|err| {
            format!("failed to serialize submission: {err}")
        })?;

    fs::write(&output, json)
        .map_err(|err| {
            format!("failed to write {}: {err}", output.display())
        })?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Verify {
            answer,
            submission,
        } => verify(answer, submission),

        Commands::ParseIssue {
            input,
            output,
        } => parse_issue(input, output),
    };

    if let Err(err) = result {
        eprintln!("{err}");
        process::exit(1);
    }
}