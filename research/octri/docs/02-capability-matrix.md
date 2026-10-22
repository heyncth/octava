# 02 - Capability Matrix

Confidence column values are exactly one of: VERIFIED, INFERRED, UNKNOWN, NOT TESTED. VERIFIED rows cite evidence-log rows. "Tested/Observed" reflects that no trial account was used: nearly all runtime behavior is untested; VERIFIED here means "the public source states this, and the source was fetched."

| Capability | Claimed (source) | Tested/Observed | Confidence | Evidence |
|---|---|---|---|---|
| Import OpenAPI 3.0/3.1, Swagger 2.0, Postman, AsyncAPI (JSON/YAML) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| Normalize imported specs to OpenAPI 3.1 | Stated on homepage pipeline diagram | Not executed | NOT TESTED | [E-01] |
| Accept OpenAPI 3.0/3.1/3.2 for the free Spec Audit | Stated on audit page | Not executed | VERIFIED | [E-16] |
| Whole-API, gaps-only, or single-page docs generation | Stated on API Studio page | Not executed | VERIFIED | [E-02] |
| AI-written reference prose per endpoint (1 credit each) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| Edit generated docs in place on the published page; edits preserved per field across regenerations | Stated on API Studio page and FAQ | Not executed | VERIFIED | [E-02] |
| Add/fix endpoints and parameters from inside the docs editor, writing back into the spec | Stated on API Studio page | Not executed | VERIFIED | [E-02] |
| MDX guides with 21 built-in components; custom components (Growth+) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| Three-column endpoint pages with expandable schema trees | Stated on API Studio page and docs overview; visible on dogfood site | Observed on octri.dev/developers (dogfood) | VERIFIED | [E-02] [E-08] |
| Try-it console calling the real API from the reader's browser (CORS-bound) | Stated on API Studio page | Not executed against a customer API | VERIFIED | [E-02] |
| Full-text search plus Ask-the-docs chat on every page (credit-metered, switchable off) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| llms.txt index and copy-as-markdown per page; agent indexing switch | Stated on API Studio page; llms.txt link present on dogfood docs | Observed link presence only | VERIFIED | [E-02] [E-08] |
| Custom domain on every tier including Free; white-label (Business+) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| Private docs via password or SAML SSO; drafts unpublished to readers and agents | Stated on API Studio and MCP pages | Not executed | VERIFIED | [E-02] [E-05] [E-06] |
| Docs versioning (Growth+), redirects, PDF export per page, SEO metadata | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| Changelog auto-written from spec changes; AI summaries (Starter+) | Stated on API Studio and pricing pages | Not executed | VERIFIED | [E-02] [E-06] |
| GitHub sync: merged spec PR rebuilds drafts across docs/SDKs/MCP; every tier | Stated on multiple pages | Not executed | VERIFIED | [E-02] [E-03] [E-05] [E-06] |
| Full pipeline (pages + clients + tools + changelog) completes in under 90 seconds | Stated on API Studio and SDK Studio pages | Not measured | NOT TESTED | [E-02] [E-03] |
| Generate SDKs in 10 languages: TypeScript, Python, Go, Java, Kotlin, Ruby, PHP, Rust, Swift, Dart | Stated on SDK Studio, docs, pricing | Not executed | VERIFIED | [E-03] [E-09] [E-06] |
| Typed models and typed errors; doc comments on methods/models | Stated on SDK Studio page and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Auth wiring from spec security schemes | Stated on SDK Studio page and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Retries with backoff, timeouts (project/language/endpoint scoped) | Stated on SDK Studio and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Idempotency-Key generation for writes (configurable header/methods) | Stated on SDK Studio and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Pagination helpers with iterators (cursor, offset, page) | Stated on SDK Studio and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Streaming via SSE and chunked responses as typed events | Stated on SDK Studio and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Per-language HTTP engines (e.g. fetch, net/http, java.net.http, OkHttp/Ktor, ureq, URLSession, cURL) | Stated per-language on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Client shape config: free functions vs class/client object; namespace by tag or path; file layout | Stated on SDK Studio and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Config precedence: endpoint override > language override > project default | Stated on SDK Studio page and SDK docs overview | Not executed | VERIFIED | [E-03] [E-09] |
| Per-endpoint: rename, doc-comment override, exclusion, deprecation, pagination/streaming rules, custom grouping | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Custom code hooks (before-request / after-response) compiled into the client; hook dependencies enter the package manifest | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Spec transforms applied at generation time with recorded reasons; never mutate the source spec | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Optional generated test suite and optional generated CLI (subcommand per endpoint) | Stated on SDK Studio and pricing pages | Not executed | VERIFIED | [E-03] [E-06] |
| Independent per-language builds; one language failure does not block others; scoped retry | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Build history with artifacts retained per plan (1/1/5/20/unlimited versions) | Stated on SDK Studio and pricing pages | Not executed | VERIFIED | [E-03] [E-06] |
| Publish to npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev; Go and Swift via git tag | Stated on SDK Studio page and SDK docs overview | Not executed; showcase repos exist on GitHub | VERIFIED | [E-03] [E-09] [E-15] |
| One version number across all published packages, taken from the spec | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Regenerations land as pull requests into customer-owned per-language repositories | Stated on SDK Studio page | Not executed | VERIFIED | [E-03] |
| Zip download of any build on every tier | Stated on SDK Studio and pricing pages | Not executed | VERIFIED | [E-03] [E-06] |
| Import existing generator configs: Speakeasy, Stainless, Fern, liblab, Mintlify | Stated on SDK Studio and compare pages | Not executed | VERIFIED | [E-03] [E-07] |
| MCP server package `@octri/mcp` runnable via npx with only a project id | Stated on MCP page, docs overview, GitHub README | Not executed; npm name and registry listing visible publicly | VERIFIED | [E-05] [E-11] [E-15] |
| Seven credential-free documentation tools (search_docs, get_endpoint, list_endpoints, get_changelog, list_sdks, get_guide, get_sdk_methods) | Stated on MCP page | Not executed | VERIFIED | [E-05] |
| One executable tool per included endpoint, named as the SDK names it, JSON Schema inputs, up to 8 documented failures | Stated on MCP page | Not executed | VERIFIED | [E-05] |
| SDK exclusions and renames curate the agent tool list; no separate agent config surface | Stated on MCP page | Not executed | VERIFIED | [E-05] |
| Transports: local stdio (default) and self-hosted SSE for teams | Stated on MCP page | Not executed | VERIFIED | [E-05] |
| Octri-hosted remote MCP endpoint (connect by URL) | FAQ says "Not today"; API Studio page says agent calls endpoints "from a server we host" | Contradictory sources; unresolved | UNKNOWN | [E-02] [E-05] |
| Tool definitions exportable as MCP, OpenAI function, or Anthropic tool JSON (public endpoint, 300 req/min per client, 2-minute cache) | Stated on MCP page | Not executed | VERIFIED | [E-05] |
| MCP tool list cached 2 minutes; clients typically load tools at startup | Stated on MCP page FAQ | Not executed | VERIFIED | [E-05] |
| Monitoring: reporter compiled into generated SDK, ships disabled, end-user must set logging.enabled | Stated on monitoring page and monitoring docs overview | Not executed | VERIFIED | [E-04] [E-10] |
| Standalone backend monitoring packages in all ten languages; no agent/sidecar required | Stated on monitoring page and pricing page | Not executed | VERIFIED | [E-04] [E-06] |
| Logs, issues, traces, performance, releases, synthetics, alerts tabs on one dataset | Stated on monitoring page and monitoring docs overview | Not executed | VERIFIED | [E-04] [E-10] |
| SQL-shaped log query + filter builder, facets, live tail | Stated on monitoring page | Not executed | VERIFIED | [E-04] |
| Error grouping by fingerprint with normalized messages; source-map de-minification; suspect commits; assign/snooze/breakdown | Stated on monitoring page | Not executed | VERIFIED | [E-04] |
| Distributed traces: client SDK + backend package on one span tree; service map; N+1 query detection; per-route percentiles; latency/error budgets | Stated on monitoring page | Not executed | VERIFIED | [E-04] |
| Alerts: threshold, new-issue, regression rules to Slack or webhook; cooldown; evaluate-now | Stated on monitoring page and pricing page | Not executed | VERIFIED | [E-04] [E-06] |
| Uptime probes generated from spec safe-read endpoints; public status page on custom domain; incidents/notices; docs banner | Stated on monitoring page and pricing page | Not executed | VERIFIED | [E-04] [E-06] |
| Redaction: client-side before send + ingest-side; on by default; browser personal context needs setLoggingConsent | Stated on monitoring page and monitoring docs overview | Not executed | VERIFIED | [E-04] [E-10] |
| Monitoring ingest allowances 100 MB / 1 GB / 5 GB / 20 GB; overage accepted up to 10x then 429 + Retry-After; overage recorded not charged (today) | Stated on monitoring FAQ and pricing page | Not executed | VERIFIED | [E-04] [E-06] |
| Event retention 7/30/90/180/365 days by plan; hourly rollups outlive raw events | Stated on pricing page and monitoring docs overview | Not executed | VERIFIED | [E-06] [E-10] |
| Source maps / source files uploaded via CLI with ingest token (CI-friendly) | Stated on CLI docs overview | Not executed | VERIFIED | [E-12] |
| Free Spec Audit: 14 weighted rules, score /10, bands Strong/Mixed/Weak, 5 MB limit, 10 audits/min, spec not retained | Stated on audit page | Not executed | VERIFIED | [E-16] |
| In-product audit can auto-apply fixes for 8 of 14 rules and rescore | Stated on audit page | Not executed | VERIFIED | [E-16] |
| Public scored pages for third-party specs (Box, Adyen, Petstore, SendGrid, Asana, Plaid, OpenAI, Twilio, GitHub, Stripe) with dated scores | Present on audit page | Page existence observed | VERIFIED | [E-16] |
| Pricing tiers Free/$49/$79/$249/custom; annual 15% off; no per-seat pricing; unlimited team members | Stated on pricing page | Not executed | VERIFIED | [E-06] |
| SDK language bundling 1/2/4/all-10; +$50/mo extra language on Starter/Growth | Stated on pricing page | Not executed | VERIFIED | [E-06] |
| Endpoint caps 50/100/300/600/unlimited across docs, SDKs, MCP tools | Stated on pricing page and MCP FAQ | Not executed | VERIFIED | [E-06] [E-05] |
| AI credits 100 one-time (Free) / 1,000 / 2,500 / 5,000 per month; packs never expire on paid plans | Stated on pricing page | Not executed | VERIFIED | [E-06] |
| CLI `@octri/cli` (Node 20+): auth, projects, specs, sdk, docs, monitoring, orgs, keys, github, jobs, mcp groups | Stated on CLI docs overview | Not executed | VERIFIED | [E-12] |
| CLI `octri mcp serve` exposes CLI surface as MCP tools; publish/delete gated by flags | Stated on CLI docs overview | Not executed | VERIFIED | [E-12] |
| CLI machine-readable output via --json; profiles; OCTRI_* env vars | Stated on CLI docs overview | Not executed | VERIFIED | [E-12] |
| Public HTTP API + webhooks available on every plan | Stated on pricing page | Endpoint inventory not published in fetched sources | VERIFIED | [E-06] |
| Public API endpoint inventory, auth scheme, and rate limits | Not published in any fetched page | No public API reference located | UNKNOWN | - |
| SAML SSO for dashboard and docs (Enterprise), org-wide 2FA enforcement (Enterprise) | Stated on pricing and security pages | Not executed | VERIFIED | [E-06] [E-18] |
| TOTP 2FA, session revocation, org audit log (365d), hashed API keys, AES-256-GCM secret encryption | Stated on security and privacy pages | Not executed | VERIFIED | [E-17] [E-18] |
| No SOC 2, no ISO 27001 (explicit vendor statement) | Stated on security page | N/A | VERIFIED | [E-18] |
| Pre-signed GDPR Article 28 DPA in force for every customer; subprocessors listed with 30-day change notice | Stated on privacy, subprocessors, DPA-linked pages | Not executed | VERIFIED | [E-17] [E-19] |
| Billing via Paddle as merchant of record; Stripe present in code as inactive fallback | Stated on privacy and subprocessors pages | Not executed | VERIFIED | [E-17] [E-19] |
| Infrastructure: self-hosted MongoDB, Redis, message broker on OVH servers (UK); Cloudflare for artifacts/DNS/edge | Stated on security and subprocessors pages | Not independently verified | VERIFIED | [E-18] [E-19] |
| AI subprocessors: Anthropic (prose/chat), OpenAI (embeddings, optional writer); no training on customer content | Stated on privacy and subprocessors pages | Not executed | VERIFIED | [E-17] [E-19] |
| Docs-site analytics: page views without IP/UA/visitor id; Ask-the-docs sessions deleted after 7 days | Stated on privacy page | Not executed | VERIFIED | [E-17] |
| Open-source grant: 10 languages, 1,000 endpoints, MIT SDK runtimes, attribution terms | Stated on open-source page | Not executed | VERIFIED | [E-13] |
| Core platform source code public | Only tooling repos (parser/auditor/converter/cli/mcp) and showcase SDKs are public; platform is closed | Org repos inspected | VERIFIED | [E-15] |
| Generated SDK runtime libraries are MIT-licensed (all ten languages) | Stated on open-source page; showcase repos on GitHub show MIT | Showcase repos observed MIT; customer-generated packages not inspected | VERIFIED | [E-13] [E-15] |
| Open-source tooling repos Apache-2.0: octri-openapi-parser, octri-spec-auditor, octri-spec-converter (Rust, WASM bindings) | Visible on GitHub org | License badges observed | VERIFIED | [E-15] |
| Public product changelog with release notes | Changelog page renders but says "No changelogs published yet" | Observed empty | UNKNOWN | [E-14] |
| Self-hosted deployment of generation/docs/monitoring | Explicitly not available on any plan | N/A | VERIFIED | [E-01] [E-06] |
| Real-time collaborative editing of docs | No claim found in any fetched source | - | UNKNOWN | - |
| Offline mode / conflict resolution for the studio editors | No claim found | - | UNKNOWN | - |
| Mobile-native apps (iOS/Android) | No claim found; product is browser + CLI + npm packages | - | UNKNOWN | - |
| Exact internal storage schema, queue topology, build farm details | Not published; inferred only at the level in 05 | - | UNKNOWN | - |
| Support response-time SLAs outside Enterprise | Support tiers named; no numeric SLAs published for non-Enterprise | - | UNKNOWN | [E-06] |
