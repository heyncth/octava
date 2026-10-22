# 15 - Product Strategy

Status: PROPOSAL, revised 2026-10-22 after sponsor decision record. Everything here is a strategic recommendation built on this package's evidence, not an observed fact about Octri. Decisions marked DECISION POINT need a human sponsor. Labels follow the package convention; evidence rows resolve in `../research/evidence-log.md`.

## Decision record (sponsor, 2026-10-22)

| Decision | Status | Handling |
|---|---|---|
| Direction | **B locked** - self-host/VPC | Architecture anchor for all phases |
| Business model | **Fully free, no paid tier** | No hosted freemium; users run their own infra |
| License | Proposed Apache-2.0, not final | Pending dependency audit |
| Customer segment | Open | Research below; interviews still required |
| SDK languages v1 | Open | Evidence below; shortlist not locked |
| Octri trial | **Not purchased** | Public sources only; revisit only if evidence-critical |

Architecture constraints implied by "fully free" (sponsor-stated): self-host capable; no paid account required; no dependency on commercial APIs that customers must buy (LLM features optional, BYO-key or off). Operating cost lands on the user's infrastructure, not ours [INFERRED alignment with sponsor note].

## Target segment

Primary (recommended): **API-producing engineering teams (5-50 devs) that cannot put specs, docs generation, or telemetry on a third-party SaaS** - self-host requirements, air-gapped/VPC deploys, EU/UK data-residency procurement, or security review that fails vendors without SOC 2 [E-18] [E-19].

Why this segment over "everyone": Octri is hosted-only on every plan and states self-hosted generation and docs rendering are not available [E-01] [E-06]. Its own security page admits no SOC 2 / no ISO 27001 [E-18]. Those two VERIFIED facts are structural doors a self-hosted competitor walks through without matching four-studio breadth.

Secondary (later): open-source maintainers who already want free SDK generation (Octri's grant program proves the demand exists; the ask is attribution, not money) [E-13].

DECISION POINT 1: confirm primary segment; if the sponsor's real goal is a hosted clone for indie devs, this document's recommended wedge does not apply and Phase scope in `12` must be re-cut.

## Three problems not served well today (with evidence)

1. **No deploy-your-own option.** [VERIFIED] Self-hosting unavailable on all plans; product is hosted-only [E-01] [E-06]. Consequence: regulated and VPC teams cannot adopt regardless of feature love.
2. **No independent compliance posture.** [VERIFIED] Vendor states no SOC 2, no ISO 27001; DPA is pre-signed but certifications are absent [E-18] [E-17]. Consequence: enterprise procurement stalls even when the buyer wants the product.
3. **Agent surface safety is opt-in wisdom, not default.** [VERIFIED] MCP operation tools execute real writes against production APIs; the documented mitigations (staging base URL, scoped keys, exclusion list) are configuration the customer must arrange [E-05]. Consequence: teams deploying MCP tooling to agents want policy defaults (deny-write, environment pinning) baked into the product, not a FAQ.

Supporting soft spots (VERIFIED but secondary): public product changelog empty as of research date [E-14]; monitoring overage "recorded and not charged today" - explicitly unfinished policy [E-04] [E-06]; MCP hosting story contradicts itself across pages [E-02] [E-05].

## Three viable product directions

### Direction A - Full hosted clone (four studios, parity with Octri)

- Scope: docs + 10-language SDK gen + MCP + monitoring, hosted SaaS, same pricing shape.
- Evidence fit: package `02` is essentially the spec for this.
- Cost: highest. Ten idiomatic generators, dual-sided monitoring opt-in, registry publish integrations, billing - years of work for a small team. Octri already ships it with a year-plus head start visible in blog/changelog surfaces [E-14].
- Risk: competes on the incumbent's ground; no structural wedge; compliance gap identical.
- Verdict: reject as primary direction.

### Direction B - Self-hosted API toolchain, fully free open source (RECOMMENDED, LOCKED)

- Scope wedge: **spec audit + docs site + SDK generator (start 2-3 languages), deployable via Docker/Helm into the customer's VPC; MCP server included with deny-write defaults; monitoring deferred to OTLP export into the customer's existing stack.**
- Model: every capability listed above ships free under one license; no premium tier; sustainability via sponsorship/donations/contributions (see research update below).
- Evidence fit: attacks the two structural gaps (self-host [E-01] [E-06], compliance-by-deployment [E-18]); reuses publicly licensed prior art (`octri-openapi-parser`, `octri-spec-auditor`, `octri-spec-converter`, Apache-2.0) for the spec layer [E-15]; the 14-rule audit is fully specified in public marketing text and is the cheapest credible entry feature [E-16].
- Cost: medium. Spec core + docs renderer + 2 generators + install story is a focused Phase 0-3 of `12-implementation-roadmap.md`. Language count grows after quality, not before (INFERRED sequencing).
- Risk: OTLP-export monitoring is an INFERRED bet that buyers prefer wiring into Datadog/Grafana over a new telemetry SaaS; needs 5-10 design interviews before Phase 6 of `12`.
- Verdict: recommended primary direction.

### Direction C - Thin niche: audit CLI/SaaS + MCP permission gateway

- Scope: (i) spec-readiness audit as a CLI/CI action (open the algorithm from [E-16]); (ii) an MCP gateway that fronts any existing API docs/tools with policy defaults (write-deny, env pinning, approval hooks).
- Evidence fit: smallest surface; audit rules are public; MCP pain is documented even by Octri's own FAQ [E-05].
- Cost: lowest. Could ship an MVP in weeks.
- Risk: easy to copy; no moat beyond speed; does not by itself capture docs/SDK budget.
- Verdict: viable as **Phase 1 of Direction B** (audit as the entry feature that seeds the parser/auditor core), not as a standalone company unless the sponsor wants a tool, not a platform.

## Verifiable competitive advantage (for Direction B)

| Advantage | How it is verified (falsifiable test) |
|---|---|
| Runs fully in customer VPC with zero outbound calls (LLM features optional, bring-your-own-key or disabled) | Airplane-mode install test; network egress audit in CI |
| Spec engine under Apache-2.0 (proposed), identical CLI and server behavior | Public repo builds the same binary the self-host package ships |
| Audit scores match Octri's published rule weights on public specs (Box, Petstore, GitHub, Stripe as fixtures) | Golden-file tests against the scores listed on [E-16] |
| Compliance path: customer deploys inside their own SOC 2 boundary | Architecture review deck; no Octri-style "we hold your data" problem |
| MCP defaults: write operations denied unless explicitly allow-listed per environment | Integration test: delete tool absent in default policy |

Advantages we are deliberately **not** claiming: ten languages at launch, monitoring parity, sub-90-second pipelines - all NOT TESTED on the incumbent and expensive to chase [E-02] [E-03].

## Cost and risk per direction (relative, INFERRED)

| Direction | Eng. cost | Time to first value | Main risk | Kill condition |
|---|---|---|---|---|
| A parity clone | Very high | 12-24 mo | No wedge; incumbent iteration speed [E-14] | Sponsor cannot staff 4+ workstreams |
| B self-host toolchain | Medium | 3-6 mo to private beta | Interview signal shows VPC buyers also demand 10 languages on day one | 10/10 interviewees reject 2-3 language start |
| C thin tools | Low | 2-6 wk | Commoditized; no platform follow-on | First free alternative ships the same audit |

## Production architecture and scope decision (Direction B)

In scope for v1 (production-ready bar):

1. Spec engine: OpenAPI 3.x parse, convert Swagger 2.0/Postman, 14-rule audit with published weights [E-16]; built on or compatible with the Apache-2.0 Rust crates [E-15].
2. Docs renderer: reference pages, MDX guides, search, llms.txt, private-token access; per-field edit provenance (hard requirement from [E-02], spike first).
3. SDK generator: launch shortlist **proposed** TypeScript + Python (evidence below; not locked); retries, pagination, idempotency, auth wiring [E-03] [E-09]; zip artifacts; registry publish optional.
4. MCP server: stdio, docs tools + operation tools, **deny-write default policy** (improvement over observed incumbent defaults [E-05]).
5. Deploy: Docker Compose + Helm; SSO via customer OIDC; audit log; no phone-home; no telemetry to project authors.
6. Out of scope v1: 8 remaining languages, full monitoring SaaS, status pages, hosted anything, billing of any kind.

Explicitly deferred (phase 2+): remaining languages one at a time behind quality gates; OTLP span export instead of building an observability UI.

## Research update - second pass (2026-10-22, no trial)

Sponsor asked three questions before code. Answers below; evidence rows E-20 through E-27.

### Q1 - Who actually needs self-host/VPC API tooling, and what do they use today?

[VERIFIED via third-party vendor material, not neutral] Enterprise need is stated concretely for **finance and healthcare**: API definitions contain proprietary business logic or patient-adjacent data that cannot transit third-party SaaS; generation must run behind the VPC or air-gapped [E-23].

[VERIFIED] The free/self-host shelf already exists and is mature:

| Need | Free/self-host option today | Gap our product must beat |
|---|---|---|
| Reference renderer | Scalar API Reference (MIT, self-host anywhere) [E-20]; Redoc community edition [E-21]; Swagger UI | Not a wedge alone - commoditized |
| SDK generation | openapi-generator (Apache-2.0, Docker online image for CI) [E-22] | Quality/configurability complaints are the opening; Merge CTO publicly cited customer complaints about openapi-generator [E-23] |
| SDK generation (idiomatic, modern) | Fern generators Apache-2.0 with on-prem option (open-core, paid hosting/publishing) [E-20] [E-23]; Speakeasy generator now AGPL-3.0 [E-20] [E-24] | Fully-free + permissive (Apache/MIT) + docs+SDK+MCP in one install is not covered by any single free tool |
| Docs platform (guides + versions + search) | Docusaurus/Mintlify-open etc. - none observed bundling SDK gen + MCP | Integration gap |
| MCP with policy defaults | None observed in the free self-host shelf (INFERRED from absence in E-20..E-24) | Differentiator candidate |

[VERIFIED market context, third-party] 2026 consolidation: Stainless winding down hosted products; Postman acquired Fern; Speakeasy open-sourced its generator (AGPL) [E-20]. Buyers of SDK tooling are actively re-deciding vendors - timing favors a free self-host entrant, but Fern on-prem is the closest direct rival for the regulated segment [E-23].

[INFERRED] Primary personas: platform/API guild engineers at regulated mid-market (5-50 devs) and open-source maintainers wanting CI-generated SDKs without a SaaS account. Confirm with interviews (still open).

### Q2 - Which SDK languages have the highest demand in that segment?

General-population demand (not VPC-segment-pure; label accordingly):

- [VERIFIED] Stack Overflow 2025 (n=49,123): JavaScript 66%, Python ~57.9% (+7pp YoY, largest mover), TypeScript ~43.6%, Java ~29%, Go ~16.4% [E-25].
- [VERIFIED] Stack Overflow 2026: among AI-coding usage Python 39%, JavaScript 38.4%, TypeScript 30.4%; among 16+ year pros SQL/JS/HTML/Python lead [E-26].
- [VERIFIED] Octri itself ships TS, Python, Go, Java, Kotlin, Ruby, PHP, Rust, Swift, Dart [E-03] [E-09]; Fern ships a similar minus-mobile-heavy languages [E-23 ecosystem].

[INFERRED] Evidence favors **TypeScript + Python** as the v1 pair (largest combined consumer base, strongest AI-tooling gravity toward Python), with **Go** as the first phase-2 language (backend/API platforms, simple distribution via git tag, aligns with regulated infra teams). Java/C# remain enterprise-important but are heavier to make idiomatic; defer. This shortlist is a proposal, not a locked decision.

### Q3 - Fully free model: what is free, and how does it sustain?

Per sponsor decision: **everything free** - audit, docs generation, SDK gen (TS/Python), MCP server with deny-write defaults, self-host monitoring export. No paid tier, no hosted freemium [DECISION RECORD].

[VERIFIED] Sustainability patterns that fit a fully-free project (from public funding landscape):

- GitHub Sponsors crossed $100M community-funded (Jul 2026); 70k+ maintainers, 280k+ sponsors, 103 regions [E-27].
- Corporate structured giving: Open Source Pledge members pay >=$2,000 per employed developer per year; 45 companies, $7.4M+ paid since launch [E-27].
- Published sponsorship tiers work (example: Vite $5-$2,000/mo; logo tier at $500) [E-27].
- Large donation-funded baselines exist (Blender ~EUR 282k/month, Sep 2026) but took decades and a famous product [E-27].
- Fragility warning: Gentoo publicly seeking infrastructure sponsors after long-running sponsors dropped (Oct 2026) - donation-only is not a safe floor [E-27].

[INFERRED] Architecture constraints now mandatory because there is no paid escape hatch:

1. Zero mandatory cloud dependency (no license server, no phone-home activation).
2. LLM features optional and BYO-key or disabled - never a hidden metered service.
3. Build/release infra cheap enough for volunteers (favor Rust/Go static binaries, minimal services).
4. Funding channel: GitHub Sponsors + Open Collective + Open Source Pledge eligibility from day one; README/docs donate links [E-27].
5. Optional future non-code revenue that does not break "fully free software": paid support/SLA contracts or training - sponsor has not approved; flag as open (does not change license or feature access).

## Remaining DECISION POINTS (post-research)

1. Segment: accept regulated mid-market VPC + OSS maintainers as primary, or run 5-10 interviews first (recommended before Phase 2 of `12`).
2. SDK v1 shortlist: confirm TypeScript + Python (evidence-backed default) or substitute.
3. License: confirm Apache-2.0 after dependency audit (watch AGPL contamination if wrapping Speakeasy generator [E-24]).
4. Sustainability: approve funding channels (Sponsors/Open Collective/Pledge) and whether paid support is ever in scope.
5. Positioning claim: "one Docker install = audit + docs + SDKs + MCP, all free, all yours" vs narrower audit-only launch (Direction C as Phase 1 remains available).

## Relationship to other docs

- `12-implementation-roadmap.md` phases map onto Direction B with Phase 6-7 (monitoring) replaced by OTLP export work.
- `09-clean-room-barriers.md` still binds: no copied prose/assets; license notices preserved.
- `13-open-questions.md` items 16-30 remain the trial script if verification budget is approved.
