# Evidence Log - Octri

Target URL: https://octri.dev/
Researched: 2026-10-22
Sources attempted: 19 fetched / 19 attempted
Access: public pages only; no account, no trial, no authenticated fetches.

## E-01
- URL: https://octri.dev/
- Fetched: 2026-10-22
- Type: marketing
- Summary: Homepage. One-spec-to-four-products pitch (docs site, SDKs in ten languages, MCP server, monitoring). Free-to-$249 pricing teaser, plan feature summary table, sample generated SDKs from public specs (OpenAI, Anthropic, GitHub, Twilio, Resend, Clerk, Vercel, DigitalOcean, Neon, Intercom). FAQ covering product bundling, monitoring-in-SDK meaning, edit-vs-regenerate model, spec-change flow, no-training claim, no self-hosting.
- Notes: Heavy marketing prose with illustrative UI mockups. Behavior claims from this page are NOT TESTED unless corroborated by docs pages.

## E-02
- URL: https://octri.dev/api-studio
- Fetched: 2026-10-22
- Type: marketing
- Summary: API Studio product page. Import formats (OpenAPI 3.0/3.1, Swagger 2.0, Postman, AsyncAPI; JSON/YAML). Whole-API, gaps-only, or single-page generation. Published site: explanations per endpoint/field/error, try-it console (browser-side, CORS-bound), search + Ask-the-docs chat (switchable off), 21 built-in MDX components, custom domain every tier, light/dark themes, custom CSS, password/SSO-gated private docs, PDF save per page, redirects, changelog from spec changes, per-field edit preservation across regenerations, llms.txt + copy-as-markdown, agent indexing switch. Claims one merged PR rebuilds pages/clients/tools/changelog "in under 90 seconds".
- Notes: Contradiction risk: states an agent can call endpoints "from a server we host" while E-05 FAQ says no Octri-hosted remote MCP endpoint.

## E-03
- URL: https://octri.dev/sdk-studio
- Fetched: 2026-10-22
- Type: marketing
- Summary: SDK Studio product page. Ten languages (TypeScript, Python, Go, Java, Kotlin, Ruby, PHP, Rust, Swift, Dart). Generated client ships typed models/errors, single request layer, auth from security schemes, doc comments, optional generated test suite, same surface across languages. Retries/timeouts, idempotency keys, pagination iterators (cursor/offset/page), SSE/chunked streaming. Optional generated CLI. Config precedence: endpoint > language > project. Per-endpoint rename, doc-comment override, exclusion, deprecation, pagination/streaming rules, custom folders. Custom code hooks (before-request/after-response seams, per-language, dependencies written into manifest). Spec transforms with reasons, spec audit with suggested fixes. Per-language builds with independent failure/retry, build history/zip download. Registry publish: npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev; Go and Swift by git tag. One version across packages; changelog from spec diff. GitHub sync; regenerations as PRs into user-controlled repos. Migration importers: Speakeasy, Stainless, Fern, liblab, Mintlify configs.
- Notes: Claims about generated-code quality and publish behavior are NOT TESTED (no build run).

## E-04
- URL: https://octri.dev/monitoring
- Fetched: 2026-10-22
- Type: marketing
- Summary: Monitoring product page. Two entry paths: reporter compiled into generated SDK (ships disabled; each SDK end-user opts in) and standalone backend packages for ten languages; uptime checks need neither. Workspace tabs: logs, issues, traces, performance, releases, uptime, alerts. SQL-shaped log query + filter builder, live tail. Issue grouping by fingerprint with source-map de-minification, suspect commits, assign/snooze/breakdown. Traces: client+server on one span tree, service map, N+1 detection, per-route percentiles, latency/error budgets. Alerts: threshold/new-issue/regression rules to Slack or webhook, cooldowns, evaluate-now. Uptime probes generated from spec, public status page on custom domain, incidents/notices, docs banner. Ingest pipeline: project-scoped token, 202 fire-and-forget, client-side + ingest-side redaction, fingerprint templating, dedup by event id. Monitoring off by default. Personal-context browser reporting requires separate consent flag.
- Notes: All runtime behavior NOT TESTED. Redaction efficacy NOT TESTED.

## E-05
- URL: https://octri.dev/mcp
- Fetched: 2026-10-22
- Type: marketing
- Summary: MCP server product page. Package `@octri/mcp` via npx; project id is the only required setup (env var or flag). Seven credential-free, side-effect-free documentation tools: search_docs, get_endpoint, list_endpoints, get_changelog, list_sdks, get_guide, get_sdk_methods. One executable tool per included endpoint, named as SDK names it, JSON Schema input from path/query/body, up to eight documented failures per tool, deprecated markers, pinned values, auth schemes (bearer, API key, OAuth2, basic). Tool list curated by SDK Studio exclusions/renames. Credential stays in local MCP client config; server holds no Octri session; published content only. Three scoping levers: base URL env, scoped credential, endpoint exclusion. Configs: claude_desktop_config.json, .cursor/mcp.json, .vscode/mcp.json. Transports: stdio (default) and self-hosted SSE. Tool-list cache two minutes; clients read list at startup. Tool definitions exportable as MCP, OpenAI function, or Anthropic tool JSON from a public endpoint (300 req/min per client, two-minute cache). FAQ: no Octri-hosted remote MCP endpoint today.
- Notes: Contradicts E-02 phrase "a server we host". Behavior of operation tools NOT TESTED.

## E-06
- URL: https://octri.dev/pricing
- Fetched: 2026-10-22
- Type: marketing
- Summary: Full pricing. Plans: Free $0, Starter $49/mo, Growth $79/mo (recommended), Business $249/mo, Enterprise custom; annual = 15% off. One plan covers all four studios. SDK languages: 1/2/4/all 10/unlimited; extra language +$50/mo on Starter/Growth. Endpoints: 50/100/300/600/unlimited. Projects: 1/1/5/20/unlimited. AI credits: 100 one-time / 1,000 / 2,500 / 5,000 / custom; 1 credit per endpoint written or chat answer; credit packs on paid plans, never expire. Team members unlimited on every plan (no per-seat). API Studio: versions from Growth, daily spec-URL watch Growth+, custom components/CSS Growth+, white-label Business+, SAML docs gate Enterprise only, AI changelog summaries Starter+. SDK Studio: version history 1/1/5/20/unlimited, CDN artifacts Business+, no Octri header in generated files Enterprise only. Monitoring: ingest 100 MB / 1 GB / 5 GB / 20 GB / custom per month; retention 7/30/90/180/365 days; overage accepted up to 10x allowance then 429 with Retry-After; overage recorded not charged today. MCP: operation tools capped at plan endpoint count. Team/security: roles, activity log, multi-org, TOTP 2FA, remote sign-out, webhooks, public API, CLI on every plan; org-wide 2FA require and SAML SSO Enterprise only. Support: community/email/priority/dedicated; contractual SLA Enterprise only. Free caps: 1 project, 1 SDK language, 50 endpoints, 100 MB monitoring. Self-hosting unavailable on any plan.
- Notes: Prices and feature gates VERIFIED as published on this date; changes over time are expected.

## E-07
- URL: https://octri.dev/compare
- Fetched: 2026-10-22
- Type: marketing
- Summary: Comparison hub (written by Octri, dated 16 Oct 2026) against Mintlify, ReadMe, Redocly, Docusaurus, Scalar, Fern, Stainless, APIMatic. Claims importers read Fern generators.yml/docs.yml, Mintlify docs.json/mint.json, Stainless stainless.yml. States Stainless stopped taking new projects in May 2026 (third-party claim, not verified on Stainless's own site).
- Notes: Vendor-authored comparisons; treat competitor facts as UNVERIFIED third-party claims.

## E-08
- URL: https://octri.dev/developers/guides/api-docs/overview
- Fetched: 2026-10-22
- Type: docs
- Summary: API docs guide overview (the docs site itself is generated by Octri - dogfood). Endpoint pages with expandable schema trees, code samples in every generated SDK language, try-it drawer, full-text search, MDX guides. Sidebar enumerates docs features: components, writing guides, custom components, navigation, custom domains, versioning, SEO, search and AI chat, AI credits, access control, files, octri.json config.
- Notes: Dogfooding is itself VERIFIED evidence the product can render its own docs.

## E-09
- URL: https://octri.dev/developers/guides/sdks/overview
- Fetched: 2026-10-22
- Type: docs
- Summary: SDK guide overview. Confirms generated client includes typed models, auth wiring, retries with backoff, idempotency keys, pagination helpers, streaming where supported. Language/registry table: TypeScript/npm, Python/PyPI, Java & Kotlin/Maven Central, Rust/crates.io, Ruby/RubyGems, PHP/Packagist, Dart/pub.dev, Go and Swift/git tag (no registry). SDK Studio settings scopes: project-wide (auth, client style, namespacing, package names, base URL, reliability, idempotency), per language (client style, namespacing, package name, repository, HTTP engine, data-model style), per endpoint (method name, doc comment, inclusion, deprecation, pagination, streaming, hooks). Precedence: endpoint > language > project. Billing note: base plan includes one language; add/remove timing rules.
- Notes: Docs site URL host differs from marketing claims (octri.dev/developers). GitHub profile README mentions docs.octri.dev as alternate docs host.

## E-10
- URL: https://octri.dev/developers/guides/monitoring/overview
- Fetched: 2026-10-22
- Type: docs
- Summary: Monitoring guide overview. Setup sequence: enable monitoring on project, rebuild SDKs (reporter compiled at build time; already-published versions report nothing), end-user sets logging.enabled=true (defaults false), upload source maps/symbols. Browser personal context requires separate setLoggingConsent(true). Tabs list: logs, issues, traces, performance, releases, synthetics, alerts. Events stream live. Runtime redacts credential-shaped keys and sweeps free text for tokens, JWTs, card numbers, emails before send; on by default. Raw-event retention: 7/30/90/180/365 days by plan; hourly rollups outlive raw events.
- Notes: Detailed opt-in mechanics are the strongest public behavioral source for monitoring.

## E-11
- URL: https://octri.dev/developers/guides/mcp-server/overview
- Fetched: 2026-10-22
- Type: docs
- Summary: MCP guide overview. Server is @octri/mcp via npx; project id required. Docs tools work without credentials; operation tools need configured credentials and make real HTTP calls (warnings about write endpoints, point at staging). Sidebar: install, doc tools, operation tools, configuration, transports, troubleshooting.
- Notes: Reinforces E-05; no hosted-endpoint claim here.

## E-12
- URL: https://octri.dev/developers/guides/cli/overview
- Fetched: 2026-10-22
- Type: docs
- Summary: Octri CLI (`@octri/cli`, npm global, Node 20+). Commands groups: auth, config, projects, specs, sdk, docs, monitoring, orgs, keys, github, jobs, mcp. Auth via email/password + TOTP; session in ~/.octri/config.json mode 0600; profiles; env vars OCTRI_PROFILE, OCTRI_API_URL, OCTRI_TOKEN, OCTRI_API_KEY, OCTRI_PROJECT_ID, OCTRI_CONFIG_DIR. Output modes: interactive, plain, --json. First-run: specs push, sdk build --lang, docs versions publish. Monitoring: summary/issues/issue/resolve reads via session; sourcemaps/sources uploads use ingest token directly (CI-friendly). `octri mcp serve` exposes CLI surface as MCP tools over stdio; publish/delete refused unless --allow-publish/--allow-delete.
- Notes: Public CLI surface is unusually well documented for a closed-source SaaS; good grounding for 06.

## E-13
- URL: https://octri.dev/open-source
- Fetched: 2026-10-22
- Type: marketing
- Summary: Open-source grant program (not an open-core platform). Eligible projects get 10 SDK languages, 1,000 endpoints, docs site, MCP server, monitoring capped at free-tier 100 MB and 100 AI credits. Conditions: OSI license on the project, published OpenAPI/Swagger, active (release in last six months), no paid hosted tier of the project. Ask: Powered-by badge in docs footer, one-line credit in SDK READMEs, permission to mention collaboration. Claims every SDK runtime shipped is MIT (all ten), plus CLI and MCP layer; generated code belongs to the customer under customer's license. Decision ~3 working days; annual review.
- Notes: "MIT runtime" claim is about generated client libraries, not the Octri platform. Platform remains proprietary.

## E-14
- URL: https://octri.dev/developers/changelog
- Fetched: 2026-10-22
- Type: changelog
- Summary: Product changelog page exists but contains "No changelogs published yet."
- Notes: Public release cadence is therefore UNKNOWN from this source. Blog posts exist (marketing) but are not a structured changelog.

## E-15
- URL: https://github.com/OctriDev
- Fetched: 2026-10-22
- Type: repo
- Summary: GitHub org OctriDev (3 followers, no public members). README: CLI npm i -g @octri/cli; MCP @octri/mcp listed in official MCP Registry as dev.octri/mcp; SDK runtimes for all ten languages. ~119 public repositories, largely showcase SDKs (unofficial generated clients for Twilio, Resend, Vercel, GitHub, Neon, Intercom, Anthropic etc., MIT). Notable open-source tooling: octri-spec-converter (Rust, Apache-2.0), octri-spec-auditor (Rust, Apache-2.0), octri-openapi-parser (Rust, Apache-2.0, WASM bindings), octri-cli (TypeScript), octri-mcp (TypeScript). Core generation/docs/monitoring platform source is NOT public.
- Notes: Star counts near zero; young org. Open-source surface is tooling + examples, not the SaaS.

## E-16
- URL: https://octri.dev/openapi-audit
- Fetched: 2026-10-22
- Type: marketing
- Summary: Free Spec Audit tool. Score /10 from 14 weighted rules (response schema 2pts, shared $ref models 2pts, operationId uniqueness 2pts, path params 1.5, operationId present 1, operationId naming 1, operation documented 1, request body documented 1, error responses 1, tags 1, duplicate paths 1, security declared 1, params documented 0.75, server URL 0.5). Deductions scale with surface coverage. Bands: 8.0+ Strong, 6.0-7.9 Mixed, <6.0 Weak. Accepts OpenAPI 3.0/3.1/3.2 JSON/YAML; Swagger 2.0 via converter first. Limit 5 MB, 10 audits/minute per address. Does not retain the spec (keeps info block + score + hashed IP). Sample scored public specs (Box 9.1, Adyen 8.7, Petstore 8.7, SendGrid 8.6, Asana 8.3, Plaid 7.8, OpenAI 7.6, Twilio 7.3, GitHub 6.9, Stripe 6.6) with dated score pages.
- Notes: The 14 rules and weights are the most concretely specified public algorithm in the product.

## E-17
- URL: https://octri.dev/privacy
- Fetched: 2026-10-22
- Type: terms
- Summary: Privacy policy (updated 2026-08-01). Octri, LLC (Delaware). Controller for account data; processor for customer content (specs, docs, reader questions, monitoring payloads). Collects: account profile, org plan/limits/languages, members/roles, credit balances, Paddle subscription refs (Paddle = merchant of record), SAML config if set, specs/docs/embeddings, GitHub and registry connections, audit log (365d), server logs. Docs sites: page views without IP/UA/visitor-id (365d), feedback votes (365d), Ask-the-docs conversations with random browser visitor id (7d). Monitoring payload under customer control. Spec Audit keeps info block + score + hashed IP 730d, not the spec. AI providers: LLM provider for doc prose and chat answers; embedding provider for search; commercial terms prohibit training; account/billing/monitoring data never sent to model providers. Retention table with plan-based monitoring windows. Deletion: project delete immediate; org close/erase within 30 days. GDPR/CCPA rights; SCCs + UK Addendum for transfers; US + EEA subprocessors.
- Notes: Legal text, not tested. Names concrete infra choices (Paddle, no Stripe active per E-19).

## E-18
- URL: https://octri.dev/security
- Fetched: 2026-10-22
- Type: terms
- Summary: Security page (updated 2026-08-01). TOTP 2FA every plan; org-wide enforcement Enterprise; SAML SSO Enterprise (dashboard + docs gating). Sessions revocable; bcrypt passwords; AES-256-GCM for TOTP seeds and webhook URLs; API keys hashed, expirable, org-scoped. Org-scoped permissions and per-project member scoping. Audit log readable by org, staff actions logged under staff email, 365d retention. Databases, cache, message broker self-hosted (no managed DB vendor). App hardening claims: CORS allowlist, security headers, HTTP-only signed cookies, schema validation, compiled/scoped custom code and components, pinned dependencies. Vulnerability disclosure to founders@octri.dev; no paid bounty. Explicit compliance status: no SOC 2, no ISO 27001; early-stage product.
- Notes: Self-hosted Mongo/Redis/broker claims are unusual and verifiable only partially (subprocessors page corroborates absence of managed DB vendor).

## E-19
- URL: https://octri.dev/subprocessors
- Fetched: 2026-10-22
- Type: terms
- Summary: Subprocessors (Annex III to pre-signed DPA). Anthropic (doc prose, Ask-the-docs), OpenAI (embeddings, optional writing model), Paddle (payments/merchant of record), OVH Ltd UK (servers for API/DB/broker/monitoring; no app-level access), Cloudflare (artifact object storage, DNS/TLS for custom domains, edge proxy/firewall), Resend (transactional email), GitHub (sign-in, spec sync, git-tag publish), Google Ireland (GTM/GA4 marketing only, consent-gated), Vercel (marketing site hosting), Slack (internal alerts). Deliberately absent: managed databases (self-hosted), advertising, third-party chat widget, Stripe (code present as inactive fallback). Customer-connected services (registries, DNS, IdP, own app) are not subprocessors. 30-day notice before additions.
- Notes: Strong grounding for 08-dependencies and data-flow claims in 04/05.

## E-20
- URL: https://scalar.com/products/api-references (+ https://scalar.com/alternatives)
- Fetched: 2026-10-22
- Type: third-party-review
- Summary: Competitor Scalar. API Reference renderer MIT-licensed, free to self-host; paid side is hosted Docs/Registry/SDKs. Claims framework default integrations (FastAPI, Spring Boot, NestJS, etc.). Alternatives page (checked Sept 2026) states market events: Stainless winding down hosted products; Postman acquired Fern; Speakeasy open-sourced its generator. Notes open-source self-host costs "nothing in licence fees but costs someone's time."
- Notes: Vendor-authored competitor page; market-event claims are third-party and not independently verified here.

## E-21
- URL: https://github.com/redocly/redoc
- Fetched: 2026-10-22
- Type: repo
- Summary: Redoc open-source API reference (OpenAPI 3.1/3.0, Swagger 2.0), three-panel layout, CLI/Docker/React. Community edition free; hosted Redoc/Realm is Redocly's commercial product.
- Notes: Confirms mature free self-host reference renderer segment exists.

## E-22
- URL: https://github.com/OpenAPITools/openapi-generator
- Fetched: 2026-10-22
- Type: repo
- Summary: openapi-generator, Apache-2.0, generates clients/stubs/docs for many languages from OpenAPI 2.0/3.0. Self-host via Docker image `openapi-generator-online` for CI. JVM-based; community core team + technical committees per language. Not affiliated with OpenAPI Initiative.
- Notes: Dominant free self-host SDK generator; quality complaints documented by competitors (see E-23 ecosystem).

## E-23
- URL: https://www.buildwithfern.com/post/openapi-code-generation-enterprise
- Fetched: 2026-10-22
- Type: third-party-review
- Summary: Fern (vendor) enterprise article (Nov 2025). States enterprises in finance/healthcare need on-prem/VPC generation because API definitions contain proprietary logic/patient data. Comparison table: Fern open-core with on-prem; openapi-generator self-host OSS; Speakeasy and Stainless no on-prem (as of article date); APIMatic vendor-hosted only. Merge CTO quote: used openapi-generator for years, customers complained.
- Notes: Competitor marketing; useful for segment-need evidence, not neutral.

## E-24
- URL: https://pkg.go.dev/github.com/speakeasy-api/openapi-generation/v2
- Fetched: 2026-10-22
- Type: repo
- Summary: Speakeasy's Go generator repository published under AGPL-3.0; 12 targets including C#, Go, Java, MCP, TypeScript, PHP, Python, Ruby, Unity; parse -> SDK AST -> templates -> render flow documented.
- Notes: Corroborates E-20 claim that Speakeasy open-sourced its generator; AGPL is a different license posture than Apache-2.0/MIT.

## E-25
- URL: https://survey.stackoverflow.co/2025/technology (+ press release)
- Fetched: 2026-10-22
- Type: third-party-review
- Summary: Stack Overflow Developer Survey 2025 (49,123 respondents). JavaScript 66%, HTML/CSS 62%, SQL 59%; Python ~57.9% with +7pp YoY growth (largest mover); TypeScript ~43.6%; Java ~29%; Go ~16.4%; Rust admired #1 (72%). Docker +17pp to near-universal. FastAPI +5pp.
- Notes: General developer population, not segmented to VPC/API-platform buyers.

## E-26
- URL: https://survey.stackoverflow.co/2026
- Fetched: 2026-10-22
- Type: third-party-review
- Summary: Stack Overflow Developer Survey 2026. Top used: JavaScript, SQL, HTML/CSS, Python, Bash. Among AI-coding usage: Python 39%, JavaScript 38.4%, HTML/CSS 33.3%, TypeScript 30.4%. 16+ yrs pros: SQL 63%, JS 62%, HTML/CSS 56.4%, Python 53.2%.
- Notes: Reinforces Python/TypeScript as primary SDK targets for broadest consumer coverage.

## E-27
- URL: https://github.blog/open-source/maintainers/100-million-for-open-source-a-milestone-built-by-the-community/ (+ https://openresource.dev/guide/financing/effective-fundraising-strategies)
- Fetched: 2026-10-22
- Type: third-party-review
- Summary: GitHub Sponsors passed $100M community-funded (Jul 2026), 70,000+ maintainers, 280,000+ sponsors, 103 regions. Fundraising guide (Oct 2026): donate links in README/docs/package metadata; published sponsorship tiers (example Vite $5-$2,000/mo); Blender Development Fund ~EUR 282,000/month from ~7,700 individuals + 44 corporates (Sep 2026); Open Source Pledge 45 companies paying >=$2,000 per employed developer per year ($7.4M paid since launch); ASF/Django corporate tier examples. Warning example: Gentoo seeking infrastructure sponsors after losses (Oct 2026).
- Notes: Grounds sustainability options for a fully-free model; donation dependence is fragile (Gentoo counterexample).
