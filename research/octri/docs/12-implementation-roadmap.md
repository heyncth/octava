# 12 - Implementation Roadmap

Phased for a clean-room rebuild inspired by this package. Scope assumptions marked INFERRED. Goal is a viable competing wedge, not feature-for-feature cloning (see 09).

## Phase 0 - Evidence and legal grounding (exit: go/no-go)

- Read this package end-to-end; list every NOT TESTED claim the business plan depends on [this doc].
- Optional: buy one month of Octri (or use Free tier) strictly as an observation harness; log all findings as new append-only evidence rows; do not copy artifacts into the repo.
- Counsel pass: trademark class search, Apache-2.0 compliance for any reused crates, branding clearance.
- Exit criteria: shortlist of differentiating wedge (e.g., self-host-first, open generator, audit-only tool) approved by human sponsor.

## Phase 1 - Spec core (exit: parse + audit + convert)

- Build or adopt an OpenAPI 3.x parser (public Apache-2.0 prior art: `octri-openapi-parser` lineage [E-15]).
- Implement the 14-rule SDK-readiness audit with published weights and scaled deductions (fully specified publicly) [E-16].
- Spec converter for Swagger 2.0 / Postman / AsyncAPI with explicit loss review (concept from public converter repo) [E-15] [E-02].
- Exit criteria: audit matches Octri's published scoring on the same public specs they score (Box, Petstore, GitHub, Stripe lists are public targets) [E-16]; negative tests 4-9 in `10-test-plan.md` pass.

## Phase 2 - Docs renderer (exit: publishable reference site)

- Endpoint pages with schema trees, parameter tables, multi-language code samples (start with cURL + 1-2 SDK languages) [E-02] [E-08].
- MDX guide support, navigation tree, search index, llms.txt, markdown export [E-02].
- Custom domain + TLS; public/private access modes (password first; SAML later) [E-02] [E-06].
- In-place editing with per-field provenance so regenerations preserve human edits [E-02] - this is the hardest docs-side INFERRED requirement; spike it before committing.
- Exit criteria: dogfood your own API docs on the renderer (as Octri does) [E-08]; tests 10-14 pass on your stack.

## Phase 3 - SDK generator (exit: one language end-to-end, then expand)

- Start with TypeScript or Python only (INFERRED: prove quality before multiplying languages) [E-03].
- Config precedence (endpoint > language > project), method naming, namespacing, pagination, retries, idempotency, streaming [E-03] [E-09].
- Zip artifact output; then one registry publish path (npm or PyPI) under customer accounts [E-03].
- Add per-language build lanes with failure isolation once >= 3 languages exist [E-03].
- Exit criteria: tests 15-23 pass; external developer can install and call the generated client without reading your docs (usability check).

## Phase 4 - Sync and changelog (exit: git-driven regeneration)

- GitHub app: watch branch + path; open draft diffs; PR-based regeneration into customer repos [E-02] [E-03].
- Changelog generation from spec diffs; deprecation propagation across surfaces [E-02] [E-03].
- Exit criteria: test Flow "Spec change propagation" equivalents; no draft leaks to public.

## Phase 5 - MCP surface (exit: docs tools + operation tools)

- npm package, stdio transport, project-id-only docs tools (search, get_endpoint, list_endpoints, get_changelog, list_sdks, get_guide, get_sdk_methods equivalents) [E-05] [E-11].
- Operation tools from SDK config snapshot; export as MCP/OpenAI/Anthropic JSON [E-05].
- Scoping levers: base URL env, credential locality, exclusion list [E-05].
- Exit criteria: tests 24-29 pass against Claude Desktop and Cursor; write-tool safety review done.

## Phase 6 - Monitoring MVP (exit: logs + issues + opt-in reporter)

- Ingest API with token auth, 202 fire-and-forget, client+ingest redaction, fingerprinting, dedup [E-04] [E-10].
- Backend package for one language + reporter compiled into the generated SDK with `logging.enabled` default false [E-04] [E-10].
- Logs and Issues views; source-map upload via CLI [E-04] [E-12].
- Exit criteria: tests 30-34 and 45 pass; privacy review of stored payloads.

## Phase 7 - Traces, alerts, uptime (exit: full observability parity core)

- Trace assembly (client+server spans), service map, N+1 detection, percentiles, budgets [E-04].
- Alert rules (threshold/new/regression) to Slack/webhook with cooldown [E-04].
- Uptime probes from spec, status page on custom domain, docs banner [E-04].
- Exit criteria: tests 35-39 pass.

## Phase 8 - Control plane polish (exit: sellable)

- Orgs/roles/audit log/2FA (TOTP); API keys hashed with expiry; webhooks [E-06] [E-17] [E-18].
- CLI covering the same groups with --json CI mode; optional `serve` MCP facade with publish/delete gated [E-12].
- Billing meters per plan caps; credit system for AI features (if AI shipped) [E-06].
- Importers for at least one competitor config format to capture switchers [E-03] [E-07].
- Exit criteria: tests 40-53 pass; Free tier abuse-tested.

## Phase 9 - Differentiation backlog (post-parity)

- Self-host distribution (Octri's biggest stated gap) [E-01] [E-06].
- OTLP-standard ingest (UNKNOWN if Octri supports; a clean differentiator) [E-04].
- Compliance certifications ahead of Octri's stated posture [E-18].
- Open-source the generator core under a real license, keeping hosting paid (mirror of the public Rust crates strategy, extended) [E-15].

## Cross-cutting exit criteria

- Every VERIFIED claim either reproduced on the rebuild or explicitly dropped from marketing.
- Evidence log maintained; claims in rebuild docs carry labels until trial-verified.
- Clean-room audit: no copied prose/assets; license notices preserved for reused OSS.
