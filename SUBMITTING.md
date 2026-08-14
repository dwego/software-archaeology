# Submitting an Investigation

Once you believe you have reconstructed a case, you can submit your investigation for automated verification.

The submission system is designed to evaluate the conclusions and evidence you recovered without revealing the expected solution.

## 1. Open the Submission Form

Open the repository's **Issues** page and create a new issue using the:

**Submit Investigation**

issue form.

Do not use a blank issue for case submissions.

The form automatically applies the `investigation` label, which starts the verification workflow.

## 2. Case

Enter the identifier of the case you investigated.

Example:

```text
001
```

Use the identifier shown in the case directory and README.

## 3. Findings

List the conclusions you reached during your investigation.

Each finding should represent something you believe is necessary to explain the incident.

Enter one finding per line.

Example format:

```text
finding-one
finding-two
finding-three
```

The exact expected findings depend on the case.

Your goal is to describe the important technical conclusions you recovered from the available evidence.

## 4. Fault-introducing Change

Identify the change that introduced the incident.

Depending on the case, this may be:

* a Git commit
* a software revision
* a configuration change
* a deployment
* a migration
* an operational event
* another identifiable change in the recovered system

Provide the most specific identifier available.

## 5. Supporting Evidence

List the artifacts that support your conclusions.

Evidence may include:

* technical documents
* source files
* commits
* logs
* test results
* configuration files
* engineering notes
* memoranda
* captured data
* other recovered artifacts

Enter one piece of evidence per line.

The verifier evaluates whether your investigation is supported by sufficient evidence.

Correct conclusions without the required supporting evidence may still be considered incomplete.

## 6. Corrective Findings

List the changes you believe are required to correctly restore the system.

These should address the actual causes of the incident rather than only its visible symptoms.

Enter one corrective finding per line.

## 7. Investigation Explanation

Provide a short explanation connecting your conclusions to the evidence you found.

This section is your investigation report.

Explain:

* what happened
* how the evidence led you to that conclusion
* how the failures are connected
* why your proposed correction resolves the incident

The explanation does not need to follow a specific wording.

## Verification

After the issue is submitted, GitHub Actions automatically processes the investigation.

The workflow:

```text
Issue submitted
      ↓
Submission parsed
      ↓
Case verifier executed
      ↓
Findings evaluated
      ↓
Evidence evaluated
      ↓
Investigation report generated
```

The verifier returns a result for each major area of the investigation.

For example:

```text
✅ Findings
✅ Fault-introducing change
❌ Supporting evidence
✅ Corrective action
✅ Investigation explanation

INCOMPLETE
```

The verifier intentionally does not reveal the missing answer.

If an area is marked as incomplete, return to the case and continue investigating that part of the system.

## Revising an Investigation

You do not need to open another issue.

Edit your original investigation issue.

The verification workflow runs again whenever the submission is updated.

Previous verification results may remain visible in the issue history.

## Investigation Status

An investigation can receive one of the following states:

### `solved`

The investigation successfully reconstructs the incident and provides the required evidence.

### `incomplete`

The submission is valid, but one or more parts of the investigation are missing or incorrect.

### `invalid-submission`

The verifier could not process the submission.

This may happen when:

* the case identifier does not exist
* required submission fields are missing
* the submission format is invalid
* the verification data for the case cannot be loaded

## Spoilers

Investigation submissions are currently public GitHub issues.

A submitted investigation may contain major spoilers for a case.

If you intend to investigate a case yourself, avoid reading other investigation issues related to that case.

## Verification Philosophy

Software Archaeology does not use hidden flags.

The objective is not to discover a secret string.

A successful investigation should demonstrate that you understand:

* the failure
* its cause
* the change that introduced it
* the evidence supporting the conclusion
* the correct recovery action

The answer is the reconstruction of the incident itself.
