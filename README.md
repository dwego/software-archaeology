# Software Archaeology

Software Archaeology is a collection of fictional legacy software systems designed to be investigated through source code, technical documentation, logs, internal records, and Git history.

Each case represents an independent recovered system.

Your goal is to reconstruct what happened, identify the failure, determine when it was introduced, and support your conclusion with evidence found throughout the repository.

## How to investigate

Each case contains its own documentation and investigation context.

Start by reading the case README.

Git history is part of the evidence.

Useful commands include:

```bash
git log -- <case-path>
git show <commit>
git blame <file>
git diff <commit-a> <commit-b> -- <path>
```

You may also compile recovered code, inspect logs, compare specifications, analyze captured data, or write your own investigation tools.

Not every document should be assumed to be correct.

A newer specification may not have been approved.

A successful internal test may not represent the real operating environment.

A comment may describe behavior that no longer exists.

## Solving a case

There are no hidden flags.

A case is considered solved when you can determine:

* what failed
* why it failed
* when the failure was introduced
* what evidence supports the conclusion
* what should be changed to correctly restore the system

Solutions can be submitted using the repository's **Case Solution** issue template.

## Cases

### [Case 001 — Silent Orbit](cases/001-silent-orbit/README.md)
