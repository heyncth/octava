# 00 - Handoff Readme

## Reading order

For a coding agent consuming this package:

1. `01-overview.md` - what the product is and who it serves.
2. `02-capability-matrix.md` - the claim-by-claim spine; every other doc expands a row.
3. `04-data-model.md` and `05-architecture.md` - core objects and a plausible rebuild architecture.
4. `03-user-flows.md` and `06-interfaces.md` - journeys and external surfaces.
5. `07-ux-spec.md` through `12-implementation-roadmap.md` - build details.
6. `13-open-questions.md` and `14-needs-human-review.md` - read before any estimation or scoping decision.

Evidence rows live in `../research/evidence-log.md`. Every `[VERIFIED]` claim cites an `[E-xx]` row from that log.

## What this package is

An evidence-bounded behavioral spec of Octri (https://octri.dev/), reconstructed from public web pages, public docs, public legal pages, and the public GitHub organization. Research date: 2026-10-22.

Labels used throughout:

| Label | Meaning |
|---|---|
| VERIFIED | Observed on a fetched public source; cited `[E-xx]` |
| INFERRED | Synthesis from sources; not stated verbatim |
| UNKNOWN | Not found in public sources |
| NOT TESTED | Claimed by marketing/docs but never executed (no trial account was used) |

The package is not a legal opinion, not a pixel-perfect clone plan, and not proof that any claimed behavior works as described. Runtime claims (SDK build output quality, publish pipeline, redaction efficacy, latency of rebuilds) sit at NOT TESTED.

## What not to copy

Do not copy Octri's marketing copy, UI screenshots, icons, logos, trademarks, blog prose, or any code from its public repositories beyond what their open-source licenses (Apache-2.0 / MIT on specific repos) permit for reuse with attribution. Specs describe behavior; they do not repackage expression. Full constraints in `09-clean-room-barriers.md`.

## Contract

| File | Purpose |
|---|---|
| `docs/00-readme-handoff.md` | This file |
| `docs/01-overview.md` | Product overview |
| `docs/02-capability-matrix.md` | Claim-by-claim matrix |
| `docs/03-user-flows.md` | Major user journeys |
| `docs/04-data-model.md` | Entities and relationships |
| `docs/05-architecture.md` | Components and data flow (rebuild-oriented) |
| `docs/06-interfaces.md` | Public API, CLI, import/export, integrations |
| `docs/07-ux-spec.md` | Key screens and states |
| `docs/08-dependencies.md` | Third-party services and licenses |
| `docs/09-clean-room-barriers.md` | Legal/technical barriers |
| `docs/10-test-plan.md` | Acceptance tests derived from claims |
| `docs/11-risks.md` | Risk register |
| `docs/12-implementation-roadmap.md` | Phased rebuild roadmap |
| `docs/13-open-questions.md` | UNKNOWNs and open NOT TESTED claims |
| `docs/14-needs-human-review.md` | Human checkpoint list |
| `docs/15-product-strategy.md` | Proposal: segment, wedge, direction comparison (sponsor decisions) |
| `research/evidence-log.md` | 19 source rows, E-01 through E-19 |

## Known limits

- Core platform source is closed; architecture in `05` is a reconstruction, not observed internals [E-15].
- No trial account was used; every behavioral claim about generated output is NOT TESTED.
- The public product changelog is empty as of research date, so release cadence is UNKNOWN [E-14].
- One documented contradiction on MCP hosting (`/api-studio` vs `/mcp` FAQ) - see `13-open-questions.md` [E-02] [E-05].
- No SOC 2 or ISO 27001 certification is claimed; the vendor says so explicitly [E-18].
