# 04 - Data Model

Entities reconstructed from marketing nouns, docs sidebar structure, pricing meters, privacy-policy tables, and CLI command groups. Field-level storage schemas are not public; anything beyond named attributes is INFERRED or UNKNOWN. Unknown internals are listed in `13-open-questions.md` rather than guessed.

## Organization

- Attributes named in public sources: name, plan, limits, active SDK languages, member list with roles/permissions and per-project access, pending invitations, credit balances, usage counters, AI token totals, subscription and Paddle customer references, optional SAML configuration (entity ID, sign-on URL, signing certificate, allowed email domains, attribute mapping) [E-17].
- Relationships: owns Projects; scopes API keys, audit log, billing [E-17] [E-18].
- Plans are per-organization, not per-seat; team members are unlimited on every published plan [E-06].

## User (account)

- Name, email, profile picture URL, password hash (bcrypt) or Google/GitHub user id, email-verification flag, optional TOTP secret (AES-256-GCM encrypted) and recovery-code hashes, signup attribution (UTM/referrer/landing) [E-17] [E-18].
- Sessions: listable and revocable; refresh tokens expire (30-day issue window per privacy table) [E-17] [E-18].
- Relationship: member of one or more Organizations with a role [E-17].

## Project

- The unit for specs, docs sites, SDK builds, MCP tool lists, monitoring, and webhooks. Plan caps: 1/1/5/20/unlimited projects [E-06].
- Attributes named publicly: monitoring enabled flag, GitHub repository connection (repo + file path + branch), custom domain configuration, docs access mode (public / password / SAML), theme and branding assets, webhook URLs (encrypted at rest) [E-02] [E-04] [E-17] [E-18].
- Relationships: belongs to Organization; contains Specs, Pages, Guides, SDK configs, Builds, MCP tool snapshots, Monitoring data.

## Spec (API document)

- Source: uploaded file or GitHub-synced path; formats OpenAPI 3.0/3.1, Swagger 2.0, Postman, AsyncAPI, normalized toward OpenAPI 3.1 (normalization claim NOT TESTED) [E-02] [E-01].
- Derived artifacts named publicly: SDK-readiness score (14 rules), changelog entries from diffs, docs pages per operation, MCP operation tools, uptime probe set [E-04] [E-16].
- The free Spec Audit path stores only `info` block (title, version, contact), the score, hashed IP, page, user agent - not the document [E-16] [E-17].

## Docs page / guide

- Generated reference pages keyed to operations; hand-written MDX guides and custom pages; navigation tree; version set (Growth+) [E-02] [E-06] [E-08].
- Edits: stored per field on top of generated content; rebuild refreshes untouched fields [E-02].
- Site config file named `octri.json` [E-08].
- Reader-side telemetry: page views (page + timestamp only, 365d), helpful/not-helpful votes (365d), Ask-the-docs conversations with random browser visitor id (7d) [E-17].

## SDK configuration

- Scopes: project-wide, per-language, per-endpoint with endpoint > language > project precedence [E-03] [E-09].
- Project-level knobs named: auth, client style, namespacing, package names, base URL, reliability (retries/timeouts), idempotency [E-09].
- Language-level knobs named: client style, namespacing, package name, repository, HTTP engine, data-model style [E-09].
- Endpoint-level knobs named: method name, doc comment, inclusion, deprecation, pagination, streaming, hooks [E-03] [E-09].
- Adjacent objects: hooks (per language, before/after request), spec transforms (reason-tagged), registry connections per language [E-03].

## Build / release

- Per-language builds under one shared build; independent success/failure; retained artifacts with plan-based version history (1/1/5/20/unlimited) [E-03] [E-06].
- One version number sourced from the spec, written into every package manifest; git tags for Go/Swift (and tagged repos generally) [E-03].
- Downstream: publish targets (npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev, git tags) and optional per-language GitHub repositories receiving regeneration PRs [E-03] [E-09].

## MCP tool snapshot

- Documentation tools (7) + operation tools (1 per included endpoint), built from the published SDK configuration snapshot [E-05].
- Named attributes: tool name (SDK method name, MCP-charset-coerced, max 64 chars, deduplicated), description (API Studio prose), input schema (JSON Schema from path/query/body with enums/formats/examples), documented failures (up to 8), deprecation note, pinned values, required auth scheme [E-05].
- Serving: public read-only tool-list endpoint with format selector (mcp / openai / anthropic / full), 300 requests/minute per client, 2-minute cache [E-05].

## Monitoring event

- Event kinds named: errors/exceptions, structured logs, custom events, traces/spans, release markers, uptime check results, performance samples [E-04] [E-10] [E-17].
- Attributes named in product descriptions: level, route/path, status code, release, duration/latency, SDK version, language, method, user tags, stack frames (symbolicated via uploaded maps/sources), fingerprint/grouping key, event id (for dedup), project-scoped ingest token identity [E-04] [E-10].
- Redaction pipeline: credential-shaped keys and free-text PII patterns removed client-side, re-applied at ingest; values templated out of messages/paths for fingerprinting without changing stored values [E-04] [E-10].
- Retention: raw events 7/30/90/180/365 days by plan; hourly/daily rollups persist beyond raw expiry so long-range charts keep working [E-06] [E-10] [E-17].

## Issue (grouped errors)

- Fingerprint grouping; normalized message (ids/numbers/quoted values templated out); event and affected-user counts; level; status (unresolved / resolved / regressed); assignee; comments; snooze window; suspect commits from the connected repository; tag-based breakdown [E-04].

## Alert rule

- Kinds: count threshold over rolling window on any query; first sighting of new issue; regression of resolved issue [E-04].
- Attributes: name, condition expression, channel (Slack or webhook URL), cooldown, fired count, last-fired timestamp, paused flag [E-04].

## Uptime check / status page

- Check: probe per safe-read endpoint, schedule interval, pass rule (max status, max latency), last run, run history [E-04].
- Status page: custom domain, per-component status, incident lifecycle (investigating -> resolved) with timestamped updates, maintenance notices, overall banner; documentation banner integration [E-04].

## Audit log (org security)

- Who, what, when, IP, user agent, success flag; covers sign-ins, 2FA changes, session revocations, member/permission changes, SAML config, project/spec changes, API key lifecycle, billing events, SDK publishing; staff actions logged under staff email; retained 365 days [E-17] [E-18].

## API key

- Shown once, stored hashed, optional expiry, revocable, organization-scoped; rate limits applied per organization [E-18].

## Billing

- Paddle is merchant of record; card numbers never reach Octri servers; Stripe integration exists in code as an inactive fallback [E-17] [E-19].
- Meters: SDK languages active, endpoints included, AI credits consumed, monitoring ingest bytes, build version history depth [E-06].

## Relationships (summary)

Organization 1-n User (membership), Organization 1-n Project, Project 1-n Spec, Spec 1-n DocsPage, Spec 1-n SDKBuild (per language), Project 1-n MCPSnapshot, Project 1-n MonitoringEvent -> n Issue, Project 1-n AlertRule, Project 1-n UptimeCheck -> StatusPage. [INFERRED] Cardinalities follow the product's plan meters and UI groupings; exact join tables and indexes are UNKNOWN.
