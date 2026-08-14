mod issue_parser;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process};

use issue_parser::parse_issue_body;

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
    root_cause: Vec<String>,
    fault_change: Vec<String>,
    detection: Vec<String>,
    corrective_action: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Submission {
    case: String,
    root_cause: String,
    fault_change: String,
    detection: String,
    corrective_action: String,
}

#[derive(Debug, Serialize)]
struct Checks {
    root_cause: bool,
    fault_change: bool,
    detection: bool,
    corrective_action: bool,
}

#[derive(Debug, Serialize)]
struct VerificationResult {
    case: String,
    title: String,
    status: String,
    checks: Checks,
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

fn matches_answer(value: &str, accepted: &[String]) -> bool {
    let value = normalize(value);

    accepted
        .iter()
        .any(|expected| value == normalize(expected))
}

fn read_json<T>(path: &PathBuf) -> Result<T, String>
where
    T: for<'de> Deserialize<'de>,
{
    let content = fs::read_to_string(path)
        .map_err(|err| format!("failed to read {}: {err}", path.display()))?;

    serde_json::from_str(&content)
        .map_err(|err| format!("failed to parse {}: {err}", path.display()))
}

fn verify(answer_path: PathBuf, submission_path: PathBuf) -> Result<(), String> {
    let answer: CaseAnswer = read_json(&answer_path)?;
    let submission: Submission = read_json(&submission_path)?;

    if answer.case != submission.case {
        return Err(format!(
            "case mismatch: expected {}, received {}",
            answer.case, submission.case
        ));
    }

    let checks = Checks {
        root_cause: matches_answer(&submission.root_cause, &answer.root_cause),
        fault_change: matches_answer(&submission.fault_change, &answer.fault_change),
        detection: matches_answer(&submission.detection, &answer.detection),
        corrective_action: matches_answer(
            &submission.corrective_action,
            &answer.corrective_action,
        ),
    };

    let solved = checks.root_cause
        && checks.fault_change
        && checks.detection
        && checks.corrective_action;

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
        .map_err(|err| format!("failed to serialize result: {err}"))?;

    println!("{json}");

    Ok(())
}

fn parse_issue(input: PathBuf, output: PathBuf) -> Result<(), String> {
    let body = fs::read_to_string(&input)
        .map_err(|err| format!("failed to read {}: {err}", input.display()))?;

    let submission = parse_issue_body(&body)?;

    let json = serde_json::to_string_pretty(&submission)
        .map_err(|err| format!("failed to serialize submission: {err}"))?;

    fs::write(&output, json)
        .map_err(|err| format!("failed to write {}: {err}", output.display()))?;

    Ok(())
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Verify {
            answer,
            submission,
        } => verify(answer, submission),

        Commands::ParseIssue { input, output } => parse_issue(input, output),
    };

    if let Err(err) = result {
        eprintln!("{err}");
        process::exit(1);
    }
}