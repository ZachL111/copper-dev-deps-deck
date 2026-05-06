# copper-dev-deps-deck

`copper-dev-deps-deck` is a focused Rust codebase around build a Rust toolkit that studies deps behavior through safe and unsafe fixtures, with remediation hints and explicit failure cases. It is meant to be easy to inspect, run, and extend without a hosted service.

## Copper Dev Deps Deck Walkthrough

I would read the project from the outside in: command, fixture, model, then roadmap. That keeps the developer tools idea grounded in files that can be checked locally.

## Reason For The Project

The goal is to capture the core behavior in code and make the surrounding assumptions obvious. A reader should be able to run the verifier, open the fixtures, and understand why each decision was made.

## Where Things Live

- `src`: primary implementation
- `tests`: verification harness
- `fixtures`: compact golden scenarios
- `examples`: expanded scenario set
- `metadata`: project constants and verification metadata
- `docs`: operations and extension notes
- `scripts`: local verification and audit commands
- `Cargo.toml`: Rust package metadata

## Capabilities

- Includes extended examples for safe defaults, including `surge` and `degraded`.
- Documents repeatable output tradeoffs in `docs/operations.md`.
- Runs locally with a single verification command and no external credentials.
- Stores project constants and verification metadata in `metadata/project.json`.
- Adds a repository audit script that checks structure before running the language verifier.

## How It Is Put Together

The interesting part is the boundary between accepted and reviewed scenarios. Extended examples sit near that boundary so future edits can show whether the model became more permissive or more cautious. The Rust code keeps ownership and data movement plain, which makes the tests useful for checking both behavior and API shape.

## Getting It Running

Clone the repository, enter the directory, and run the verifier. No database server, cloud account, or token is required.

## Data Notes

`boundary` is the first example I would inspect because it lands on the `review` path with a score of 116. The broader file also keeps `degraded` at 18 and `surge` at 258, which gives the model a useful low-to-high spread.

## Command Examples

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

This runs the language-level build or test path against the compact fixture set.

## Check The Work

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/audit.ps1
```

The audit command checks repository structure and README constraints before it delegates to the verifier.

## Possible Extensions

- Split the scoring constants into a typed configuration object and validate it before use.
- Add a comparison mode that shows how decisions change when one signal is adjusted.
- Add a loader for `examples/extended_cases.csv` and promote selected cases into the language test suite.
- Add one more developer tools fixture that focuses on a malformed or borderline input.

## Tradeoffs

This code is local-first. It makes no claim about deployed usage and avoids credentials, hosted state, and environment-specific setup.
