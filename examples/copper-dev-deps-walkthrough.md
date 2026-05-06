# Copper Dev Deps Deck Walkthrough

I use this file as a small checklist before changing the Rust implementation.

| Case | Focus | Score | Lane |
| --- | --- | ---: | --- |
| baseline | change width | 143 | ship |
| stress | diagnostic quality | 94 | hold |
| edge | review cost | 176 | ship |
| recovery | safe rewrite | 183 | ship |
| stale | change width | 190 | ship |

Start with `stale` and `stress`. They create the widest contrast in this repository's fixture set, which makes them better review anchors than the middle cases.

The useful comparison is `change width` against `diagnostic quality`, not the raw score alone.
