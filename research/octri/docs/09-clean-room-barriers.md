# 09 - Clean-Room Barriers

## Legal/contractual barriers

Flag for counsel; this file is awareness, not advice.

- [VERIFIED] Octri, LLC is a Delaware company; the marketing site and app are proprietary; no public terms grant a right to clone the product [E-17].
- [VERIFIED] "Open Source" on the site means a grant program for third-party projects, not an open-core platform license. Core generation, docs, monitoring, and dashboard source are closed [E-13] [E-15].
- [VERIFIED] Specific Apache-2.0 repositories (`octri-openapi-parser`, `octri-spec-auditor`, `octri-spec-converter`) may be reused under their license terms with preserved notices; doing so is lawful compliance with those licenses, not cloning [E-15].
- [VERIFIED] Trademark: "Octri", the Octri logo, and product screenshots are protected expression. A competing product must not use them or confusingly similar branding [E-01].
- [VERIFIED] The vendor publishes a security vulnerability policy covering the dashboard, public API, hosted docs, marketing site, generated SDKs, and MCP server, with a good-faith non-prosecution commitment [E-18]. Competitors building similar products are not covered by that policy.
- [UNKNOWN] Whether any patents are held was not researched; no patent statements appeared on fetched pages. Counsel should search USPTO/EPO if this is a launch gate.
- [UNKNOWN] Customer contracts between Octri and its users (e.g., exclusivity, non-compete clauses in enterprise agreements) are private; a rebuild does not inherit them, but market-moving customers may be under contract.

## Technical barriers

- [VERIFIED] No self-hosted deployment exists to fork; the product is hosted-only on every plan [E-01] [E-06].
- [VERIFIED] Octri's own HTTP API is not publicly documented (no endpoint reference found); a rebuild cannot faithfully clone the control-plane API from public sources [E-06] [E-12].
- [VERIFIED] Generator internals are closed. Output idioms per language are only observable by running the product (requires an account) or by reading published showcase SDKs, which show results but not the generator [E-03] [E-15].
- [VERIFIED] Edit-preserving regeneration depends on per-field provenance storage whose schema is not public [E-02].
- [VERIFIED] Monitoring redaction rules and fingerprinting algorithms are described at a behavioral level but rule lists/patterns are not published (beyond categories: credentials, tokens, JWTs, card numbers, emails) [E-04] [E-10].
- [VERIFIED] The 14-rule audit is the most completely published algorithm (weights and fixes on the marketing page), making it the easiest subsystem to reimplement cleanly from public description [E-16].
- [INFERRED] Scale behavior (build farm throughput, ingest volume limits enforcement, cache tiers) cannot be reverse-engineered from outside; rebuild must design its own.

## What must not be copied

- Marketing copy, blog prose, FAQ answers, comparison-page text, or documentation sentences - even short phrases - from octri.dev or the hosted docs site.
- Screenshots, UI mockups, logos, brand SVGs, product-hunt badges, or favicon assets.
- Source code from closed repositories (none are public anyway) or from any leaked/unauthorized channel.
- Customer showcase SDK names or README text beyond what the MIT licenses on those repos permit (check each repo's LICENSE before reuse; attribution required).
- Octri's name, mark, or confusingly similar branding in a competing product.
- Third-party vendor marks (Anthropic, OpenAI, etc.) that appear on Octri pages - those belong to their owners.

Rebuild materials produced from this package should describe behavior in original wording, use generic terminology ("docs studio", "SDK generator", "spec audit score"), and carry their own visual design.

## Clean-room process suggestions (INFERRED process, not Octri policy)

1. Engineering reads this package and public open-source repos only; do not paste Octri marketing text into requirements.
2. If deeper behavioral detail is needed, authorize a trial account and log every observation as new evidence rows (append-only) with explicit NOT TESTED -> VERIFIED upgrades.
3. Keep an authoring log of which public sources informed each requirement, mirroring this evidence log.
4. Legal reviews branding, trademark search, and any reuse of Apache-2.0 crates before shipping.

This is not legal advice; consult counsel before shipping a competing product.
