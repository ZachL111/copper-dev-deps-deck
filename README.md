# copper-dev-deps-deck

`copper-dev-deps-deck` is a compact Rust repository for developer tools, centered on this goal: Build a Rust toolkit that studies deps behavior through safe and unsafe fixtures, with remediation hints and explicit failure cases.

## Problem It Tries To Make Smaller

The point is to make a small domain rule concrete enough that a reader can change it and immediately see what broke.

## Copper Dev Deps Deck Review Notes

The first comparison I would make is `change width` against `diagnostic quality` because it shows where the rule is most opinionated.

## Working Pieces

- `fixtures/domain_review.csv` adds cases for change width and diagnostic quality.
- `metadata/domain-review.json` records the same cases in structured form.
- `config/review-profile.json` captures the read order and the two review questions.
- `examples/copper-dev-deps-walkthrough.md` walks through the case spread.
- The Rust code includes a review path for `change width` and `diagnostic quality`.
- `docs/field-notes.md` explains the strongest and weakest cases.

## Design Notes

The implementation keeps the scoring rule plain: reward signal and confidence, preserve slack, penalize drag, then classify the result into a review lane.

The Rust code keeps the review rule close to the tests.

## Example Run

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File scripts/verify.ps1
```

## Tests

The verifier is intentionally local. It should fail if the fixture score math, lane assignment, or language-specific test drifts.

## Known Limits

This remains a local project with deterministic fixtures. It does not depend on credentials, hosted services, or live data. Future work should add richer malformed inputs before widening the public API.
