# 08 - Dependencies

All entries from public pages (subprocessors, security, privacy, GitHub org). Licenses marked only where observed.

## Third-party processors (subprocessors) [E-19]

| Service | Role | Data reaching it | Location |
|---|---|---|---|
| Anthropic PBC | Documentation prose generation; Ask-the-docs answers | Relevant spec portions, matching doc passages, reader questions | United States |
| OpenAI, L.L.C. | Vector embeddings for docs search; optional writing model | Documentation text; spec content when selected as writer | United States |
| Paddle.com Market Ltd | Merchant of record; payments, invoices, tax | Name, email, billing address, payment details, transaction history | United Kingdom |
| OVH Ltd | Rented servers for API, databases, message broker, monitoring (self-operated) | All customer content at rest; no application-level access for OVH | United Kingdom |
| Cloudflare, Inc. | Object storage for SDK artifacts/assets; DNS + certificates for custom domains; edge proxy/firewall | Generated packages, uploaded logos/images, DNS records, in-transit API/monitoring traffic | United States + global edge |
| Resend (Plus Five Five, Inc.) | Transactional email | Recipient name/email + message content | United States |
| GitHub, Inc. | Sign-in, spec sync, git-tag SDK publishing | GitHub account id, repo metadata for authorized repos | United States |
| Google Ireland Limited | GTM + GA4 on marketing site only, consent-gated | Truncated IP, UA, pages, referrer, random cookie | Ireland (processing in US) |
| Vercel Inc. | Marketing site and blog hosting | Published marketing content only | United States |
| Slack Technologies, LLC | Internal operational alerts to Octri staff | Alert metadata only | United States |

[VERIFIED] 30-day advance notice before adding a subprocessor that handles customer content or personal data; objection window in DPA section 8 [E-19].

## Deliberately absent [E-19]

- No managed database vendor: MongoDB, Redis, and the message broker are self-hosted [E-18] [E-19].
- No advertising pixels or cross-context ad tracking anywhere [E-17] [E-19].
- No third-party support/chat widget; support is email [E-19].
- Stripe exists in the codebase as an inactive fallback; Paddle processes every payment today [E-19].

## Customer-connected services (not subprocessors) [E-19]

- Package registries: npm, PyPI, Maven Central, crates.io, RubyGems, Packagist, pub.dev.
- Customer DNS provider for custom doc domains.
- Customer identity provider for SAML.
- Customer application itself (decides monitoring payload).

## Open-source components and licenses

[VERIFIED] Public repositories under github.com/OctriDev (observed 2026-10-22) [E-15]:

- `octri-openapi-parser` - Rust OpenAPI parser with source-aware resolution, language bindings, WASM; Apache-2.0.
- `octri-spec-auditor` - Rust OpenAPI auditor with source-aware findings and reviewed fixes; Apache-2.0.
- `octri-spec-converter` - Rust specification converter with loss review; Apache-2.0.
- `octri-cli` - TypeScript CLI; license not individually confirmed on the org page (treat as UNKNOWN pending repo LICENSE read).
- `octri-mcp` - TypeScript MCP server; license not individually confirmed (UNKNOWN pending LICENSE read).
- Numerous showcase SDK repos (unofficial clients for Twilio, Resend, Vercel, GitHub, Neon, Intercom, Anthropic, etc.) under MIT; explicitly unaffiliated with those vendors [E-15].

[VERIFIED] Claim: every SDK runtime Octri ships is MIT, including CLI and MCP layer; generated code belongs to the customer under the customer's license [E-13]. This claim applies to packages Octri generates/publishes for customers; it was not audited package-by-package (NOT TESTED).

[UNKNOWN] Full dependency manifests of the hosted platform, internal libraries, and font/CSS assets are not public.

## Formats and protocols

- OpenAPI 3.0 / 3.1 / 3.2 (JSON, YAML); Swagger 2.0 via conversion; Postman collections; AsyncAPI [E-02] [E-16].
- MDX for guides and custom pages [E-02].
- MCP protocol (stdio, SSE transports) [E-05].
- OpenTelemetry-like concepts (spans, traces, service maps) are described but no OTLP compatibility claim was found - UNKNOWN whether standard OTLP ingest is supported [E-04].
- Source maps and source files for symbolication [E-04] [E-12].
- JSON Schema for MCP tool inputs [E-05].

## Runtime requirements (consumer side)

- Node.js 20+ for `@octri/cli` [E-12].
- npx/Node available for `@octri/mcp` [E-05].
- Browsers for dashboard, studio, docs readers; no OS-native desktop app claimed (UNKNOWN if any exists).
- Generated SDK language toolchains per ecosystem (npm, pip, maven, cargo, gem, composer, pub, swift package manager, go modules) [E-03] [E-09].

## Compliance posture [E-18]

- No SOC 2, no ISO 27001 (vendor's own statement).
- Pre-signed GDPR Article 28 DPA with SCCs + UK Addendum; subprocessor list is Annex III [E-17] [E-19].
- PCI scope reduced via Paddle (cards never reach Octri) [E-17].
