# 06 - Interfaces

## Public API

[UNKNOWN] Octri's own HTTP API is claimed to exist on every plan ("Webhooks, public API and the octri CLI") [E-06], but no public REST reference (endpoint list, auth scheme, rate limits) was found on any fetched page. The CLI docs confirm an API exists behind `--api-url` (default implied `api.octri.dev`) with token auth [E-12]. Treat full API surface as UNKNOWN until an authenticated docs crawl or OpenAPI document for Octri's own API is obtained.

[VERIFIED] Related public machine endpoints that were documented:

- MCP tool-list endpoint: read-only, public, format query parameter selects MCP / OpenAI function / Anthropic tool / full execution-mapping JSON; 300 requests/minute per client; 2-minute cache; no credentials [E-05].
- Monitoring ingest endpoint: accepts events with a project-scoped ingest token, responds 202, fire-and-forget from SDKs; excess beyond 10x allowance returns 429 with Retry-After [E-04].
- Spec Audit intake: URL or file upload; OpenAPI 3.0/3.1/3.2; 5 MB max; 10 audits/minute per address [E-16].
- Hosted docs sites: customer custom domains; generated subdomains are random ids (not branded) [E-06].

## Import/export formats

[VERIFIED] Imports:

- OpenAPI 3.0 and 3.1, Swagger 2.0, Postman, AsyncAPI; JSON or YAML [E-02] [E-06].
- OpenAPI 3.0/3.1/3.2 for the audit tool; Swagger 2.0 requires prior conversion [E-16].
- Generator configs: Speakeasy, Stainless (`stainless.yml`), Fern (`generators.yml`, `docs.yml`), liblab, Mintlify (`docs.json`, `mint.json`) [E-03] [E-07].
- Existing docs: Markdown/MDX files imported as pages with frontmatter titles; URL slugs preserved/settable [E-02].
- Home/onboarding copy also claims Postman and AsyncAPI translation in the pipeline [E-01].

[VERIFIED] Exports:

- Generated SDK source: zip download of any build on every tier [E-03] [E-06].
- Published packages on npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev; Go and Swift via git tag on customer repos [E-03] [E-09].
- Per-language repositories receiving regeneration pull requests [E-03].
- MCP tool lists as JSON files in MCP, OpenAI, or Anthropic formats [E-05].
- Docs: per-page PDF save; copy-as-markdown; llms.txt [E-02].
- Spec itself: customer keeps the OpenAPI file as the source of truth; Octri claims never to require maintaining a copy [E-07].

## Integrations

[VERIFIED] Named integrations:

- GitHub: sign-in, spec sync from a connected repo path/branch, git-tag publishing for Go/Swift, suspect-commit links in issues [E-04] [E-17] [E-19].
- Google sign-in [E-17].
- Package registries: npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev (customer-connected accounts; customer-owned packages) [E-03] [E-19].
- Slack (alert delivery) and arbitrary webhooks (alerts, inbound project webhooks with encrypted URLs) [E-04] [E-06] [E-17].
- MCP clients: Claude Desktop, Cursor, VS Code configs documented; any MCP-compatible client implied [E-05].
- SAML identity providers (Enterprise) for dashboard and docs gating [E-06] [E-18].
- Paddle for billing [E-17] [E-19].
- AI providers Anthropic and OpenAI (docs prose, chat, embeddings) [E-17] [E-19].

[NOT TESTED] Fidelity of any integration was not exercised.

## CLI surface

[VERIFIED] `@octri/cli` on npm; requires Node 20+; install `npm install -g @octri/cli` [E-12].

Command groups: `auth`, `config`, `projects`, `specs`, `sdk`, `docs`, `monitoring`, `orgs`, `keys`, `github`, `jobs`, `mcp` [E-12].

Notable behaviors:

- `octri auth login` (email/password + TOTP); session file `~/.octri/config.json` mode 0600; multiple profiles; `--api-url` override [E-12].
- Env vars: `OCTRI_PROFILE`, `OCTRI_API_URL`, `OCTRI_TOKEN`, `OCTRI_API_KEY`, `OCTRI_PROJECT_ID`, `OCTRI_CONFIG_DIR` [E-12].
- Output modes: interactive (colors/spinners), plain (piped/CI), `--json` single document on stdout, errors on stderr; `NO_COLOR` honored [E-12].
- Global flags: `--project`, `--profile`, `--api-url`, `--json`, `--quiet`, `--plain`, `--no-color` [E-12].
- Example pipeline: `octri specs push ./openapi.yaml` (waits through full pipeline), `octri sdk build --lang go,typescript --download`, `octri docs versions publish <specId>` [E-12].
- Monitoring reads use the login session; symbol/source uploads use an ingest token directly for CI (`octri monitoring sourcemaps upload ./dist`, `octri monitoring sources upload ./src`) [E-12].
- `octri mcp serve` speaks MCP over stdio over the CLI's own surface; registry publish and spec delete refused unless `--allow-publish` / `--allow-delete` [E-12].

## SDK consumer surface (generated clients)

[VERIFIED] Documented consumer-facing behaviors of generated clients (all NOT TESTED for output quality):

- Configure-with-base-URL + auth pattern; method-per-operation surface; typed params/results [E-01] [E-03].
- Retries honoring `Retry-After` over exponential backoff (claimed cap 8s in homepage animation copy) - treat the specific cap as NOT TESTED marketing detail [E-01] [E-03].
- Idempotency keys minted once per logical write and reused across retries [E-03].
- Pagination iterators walking all pages [E-03].
- SSE/chunked streaming as typed event loops [E-03].
- Optional CLI shipped inside the SDK package (subcommand per endpoint) - distinct from Octri's own `@octri/cli` [E-03] [E-12].
- Monitoring hooks inside the client: `logging.enabled` default false; browser `setLoggingConsent` [E-04] [E-10].

## MCP consumer surface

[VERIFIED] Documentation tools (no credentials, no side effects): `search_docs`, `get_endpoint`, `list_endpoints`, `get_changelog`, `list_sdks`, `get_guide`, `get_sdk_methods` [E-05].

[VERIFIED] Operation tools: one per included endpoint; name = SDK method name coerced to MCP charset, max 64 chars, deduplicated; input schema from path/query/body; up to 8 documented failure responses; deprecation notes; pinned values not exposed as inputs; auth schemes bearer / API key / OAuth2 / basic [E-05].

[VERIFIED] Config env/flags: project id via environment variable or flag (flag wins); configurable port for SSE mode; configurable Octri API URL for local development [E-05].
