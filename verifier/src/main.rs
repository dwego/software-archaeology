use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process};

#[derive(Parser)]
#[command(name = "verifier")]
#[command(about = "Verify Software Archaeology investigation reports")]
struct Cli {
    #[arg(long)]
    answer: PathBuf,

    #[arg(long)]
    submission: PathBuf,
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

fn main() {
    let cli = Cli::parse();

    let answer: CaseAnswer = match read_json(&cli.answer) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    let submission: Submission = match read_json(&cli.submission) {
        Ok(value) => value,
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    };

    if answer.case != submission.case {
        eprintln!(
            "case mismatch: expected {}, received {}",
            answer.case, submission.case
        );
        process::exit(1);
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

    match serde_json::to_string_pretty(&result) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("failed to serialize verification result: {err}");
            process::exit(1);
        }
    }
}