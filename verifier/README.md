# Software Archaeology Verifier

The verifier is the automated validation engine used by Software Archaeology investigation submissions.

It does not attempt to determine whether an explanation is well written.

Instead, it checks whether an investigation identified the required findings, fault-introducing change, supporting evidence, and corrective actions for a case.

## Responsibilities

The verifier is responsible for:

* parsing investigation submissions
* loading case verification data
* validating required findings
* validating the fault-introducing change
* validating supporting evidence
* validating corrective findings
* producing a structured verification result

GitHub Actions handles the integration with GitHub issues.

The verifier itself does not communicate directly with GitHub.

## Verification Flow

```text
GitHub Issue
    ↓
Issue body
    ↓
parse-issue
    ↓
submission.json
    ↓
verify
    ↓
answer definition
    ↓
result.json
    ↓
GitHub Actions
    ↓
Investigation report
```

## Commands

### Parse an Issue

The `parse-issue` command converts a GitHub Issue body into a structured submission.

```bash
cargo run -p verifier -- parse-issue \
  --input issue-body.txt \
  --output submission.json
```

Example output:

```json
{
  "case": "000",
  "findings": [
    "configuration-mismatch",
    "test-environment-different"
  ],
  "fault_change": "abc123",
  "evidence": [
    "config.txt",
    "commit abc123"
  ],
  "corrective_findings": [
    "restore-previous-configuration"
  ],
  "explanation": "The testing environment used a different configuration."
}
```

### Verify an Investigation

The `verify` command compares a submission against the verification definition for a case.

```bash
cargo run -p verifier -- verify \
  --answer answers/000-test.json \
  --submission submission.json
```

Example result:

```json
{
  "case": "000",
  "title": "Verifier Test",
  "status": "solved",
  "checks": {
    "findings": {
      "passed": true
    },
    "fault_change": {
      "passed": true
    },
    "evidence": {
      "passed": true
    },
    "corrective_action": {
      "passed": true
    },
    "explanation": {
      "passed": true
    }
  }
}
```

## Case Verification Data

Verification definitions are stored in the repository's `answers/` directory.

Each case defines:

* required findings
* accepted aliases
* the accepted fault-introducing change
* valid evidence
* minimum evidence requirements
* corrective findings
* explanation requirements

Example:

```json
{
  "case": "000",
  "title": "Verifier Test",

  "required_findings": [
    {
      "id": "configuration-mismatch",
      "category": "root_cause",
      "aliases": [
        "config mismatch"
      ]
    }
  ],

  "fault_change": {
    "accepted": [
      "abc123"
    ]
  },

  "evidence": {
    "minimum": 1,
    "accepted": [
      {
        "id": "config.txt",
        "aliases": [
          "configuration file"
        ],
        "required": true
      }
    ]
  },

  "corrective_action": {
    "required_findings": [
      "restore-previous-configuration"
    ]
  }
}
```

## Findings

Findings represent conclusions recovered during an investigation.

They use stable identifiers so the verifier does not need to judge arbitrary natural-language explanations.

For example:

```text
configuration-mismatch
protocol-revision-mismatch
incorrect-frequency-conversion
```

A case may also define aliases when multiple representations should be accepted.

## Evidence

A correct conclusion may still be considered incomplete when it is not supported by sufficient evidence.

Evidence can represent artifacts such as:

* documents
* commits
* logs
* source files
* configuration files
* test results
* engineering records

Cases may require specific evidence or a minimum number of valid artifacts.

## Explanations

Investigation explanations are preserved as part of the submitted report.

The current verifier checks that an explanation is present when required, but it does not attempt to semantically judge the quality of the prose.

The objective verification is based primarily on findings and evidence.

## Status

The verifier currently returns two investigation states.

### `solved`

All required verification checks passed.

### `incomplete`

At least one required part of the investigation is missing or incorrect.

Processing errors are handled by the GitHub Actions workflow and may result in an `invalid-submission` label.

## Tests

Run the verifier test suite with:

```bash
cargo test -p verifier
```

New verification behavior should include tests covering both valid and invalid submissions.

## Design Principle

The verifier should validate whether an investigator reconstructed the system correctly without revealing the expected solution.

It should answer:

> Did the investigator prove what happened?

not:

> Did the investigator write the exact sentence stored by the author?
