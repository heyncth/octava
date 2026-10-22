# 05 - Architecture

Observed facts are labeled VERIFIED with evidence. Everything else is a plausible clean-room reconstruction for a rebuild: labeled INFERRED. Octri's internal architecture is not public; the core platform source is closed [E-15].

## Components

[VERIFIED] Public surface decomposes into four studios plus shared substrate:

1. **Spec ingestion and audit** - parse OpenAPI 3.0/3.1/3.2 (JSON/YAML); convert Swagger 2.0 / Postman / AsyncAPI; run 14-rule SDK-readiness scoring [E-02] [E-16]. Open-source relatives exist publicly: `octri-openapi-parser`, `octri-spec-auditor`, `octri-spec-converter` (Rust, Apache-2.0, WASM bindings) - likely related to but not proven identical with the hosted pipeline [E-15].
2. **API Studio** - docs site generator, in-place editor, MDX renderer with 21 built-in components, search index, Ask-the-docs chat, llms.txt, custom domains, versioning, password/SSO gating [E-02] [E-08].
3. **SDK Studio** - per-language generators for 10 languages, config precedence engine, hooks, transforms, per-language build lanes, registry publishers (7 registries + git-tag publishing) [E-03] [E-09].
4. **MCP server** - npm package `@octri/mcp` running on the developer's machine (stdio) or self-hosted SSE; reads a published tool list by project id; executes operation tools as real HTTP calls with the local credential [E-05] [E-11].
5. **Monitoring** - ingest API (token auth, 202 fire-and-forget), redaction pipeline, fingerprinting/dedup, log/issue/trace/performance/release/synthetics/alerts backends, status-page publisher, source-map service [E-04] [E-10].
6. **Control plane** - orgs, users, roles, projects, API keys, audit log, billing meters (Paddle), webhooks, public API (undocumented publicly), CLI backend [E-06] [E-12] [E-17] [E-18].
7. **CLI** - `@octri/cli` (TypeScript, Node 20+), talks to the public API with session or token auth; also serves an MCP facade over the CLI surface (`octri mcp serve`) [E-12].

[VERIFIED] Infrastructure claims: app servers, MongoDB, Redis, and message broker are self-hosted on OVH (UK); Cloudflare fronts the API/monitoring endpoints and stores generated SDK artifacts and uploaded assets; Vercel hosts only the marketing site [E-18] [E-19].

[INFERRED] A rebuild would likely structure the same way: a spec-processing worker pool, a docs rendering pipeline (SSG-ish per publish with edge caching), isolated per-language generator workers, a job queue for builds, a high-volume event ingest path separate from the control-plane API, and a static-ish tool-list publisher for MCP with a short TTL cache.

## Interfaces between components

[VERIFIED] Documented couplings:

- One merged spec change regenerates docs drafts, SDK builds, MCP tool list, and changelog together [E-02] [E-03] [E-05].
- SDK Studio configuration (names, exclusions, deprecations, descriptions) is the source for MCP operation tools and for docs code snippets [E-03] [E-05].
- Docs prose (API Studio) supplies MCP tool descriptions and get_sdk_methods content [E-05].
- Monitoring knows the spec: operations are named from it; uptime probes are generated from safe-read endpoints; deprecation notes surface who still calls a deprecated operation [E-04].
- Backend monitoring packages and SDK reporters share one ingest path; client and server spans join into one trace [E-04].
- CLI drives the same operations as the dashboard (specs push, sdk build, docs publish, monitoring reads, symbol uploads, MCP catalogue) [E-12].

[INFERRED] Rebuild equivalents: a single "release snapshot" object (docs + sdk-config + tool-list) produced per publish, consumed by the site renderer, registry publisher, and MCP tool-list endpoint; an event pipeline with a redaction stage before persistence; a job orchestrator with per-language lanes.

## Data flow

### Publish path (spec -> four surfaces)

[VERIFIED as behavior claims, NOT TESTED end-to-end]

1. Spec arrives (upload or GitHub webhook on watched path/branch) [E-02] [E-03].
2. Parse + audit score recompute [E-16].
3. Docs draft rebuild (diff visible; unpublished) [E-02].
4. Per-language SDK builds run independently; failures isolated [E-03].
5. On publish: site goes live on domain; changelog entries commit; MCP tool list refreshes (2-minute cache; agents see it after client restart) [E-02] [E-05].
6. If auto-publish enabled: packages push to registries; Go/Swift receive git tags [E-03].

### Monitoring path (production -> dashboard)

[VERIFIED as behavior claims, NOT TESTED]

1. Application (SDK opt-in or backend package) redacts locally, then POSTs events with project-scoped ingest token; server answers 202 immediately [E-04] [E-10].
2. Ingest re-redacts, fingerprints (templates ids/numbers/strings out of paths and messages), dedups by event id, attributes to project and release [E-04].
3. Events land in the log stream; grouping creates/updates issues; traces assemble spans (client + server) [E-04].
4. Uptime probes run server-side on schedule against customer endpoints; results feed status page and docs banners [E-04].
5. Alerts evaluate rules (threshold / new issue / regression) and deliver to Slack or webhook with cooldown [E-04].

### Agent path (MCP)

[VERIFIED as behavior claims, NOT TESTED]

1. Developer machine runs `npx @octri/mcp` with project id; process speaks stdio to the MCP client [E-05] [E-11].
2. Server fetches the public tool list (published snapshot only; 2-minute cache; 300 req/min per client) [E-05].
3. Documentation tools serve published pages/guides/markdown with no credentials and no side effects [E-05].
4. Operation tools build an HTTP request from tool arguments plus local credential and optional base-URL override, then execute against the customer's API (not against Octri) [E-05].

[INFERRED] Security boundary: Octri never sees the downstream API credential in the stdio model; the hosted-dashboard side only ever sees the published tool list and customer-configured base URLs. The SSE self-hosted mode means the team's own server holds the credential instead.

## Rebuild-relevant notes

- [INFERRED] Hardest technical risks for a clone: per-language idiomatic generation quality (10 languages), edit-preserving regeneration (per-field provenance), symbolication pipeline, and dual-sided monitoring opt-in UX.
- [VERIFIED] A rebuild can study the public Rust parser/auditor/converter repositories under Apache-2.0 as legitimate prior art for the spec-processing layer [E-15].
- [VERIFIED] No SOC 2 / ISO controls narrative exists to copy; the vendor documents its actual controls on a public security page [E-18].
