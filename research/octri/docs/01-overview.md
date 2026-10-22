# 01 - Product Overview

## One-sentence description

Octri is a hosted SaaS that turns one OpenAPI (or convertible) API specification into four coupled products: a documentation site, generated client libraries in ten languages, an MCP server exposing docs and endpoints to AI agents, and production monitoring that can ship inside the generated SDKs [E-01] [E-02] [E-03] [E-04] [E-05].

## Who it is for

[VERIFIED - positioning stated on marketing and pricing pages] Primary audience is API-producing teams (indie devs to scaling products) who maintain a spec and want docs, SDKs, agent tooling, and observability without running four vendors [E-01] [E-06].

Secondary audiences observed on public pages:

- Open-source maintainers (grant program: 10 languages, 1,000 endpoints, attribution in exchange) [E-13].
- AI-agent builders needing structured docs and callable endpoint tools [E-05].
- Teams migrating off Fern, Stainless, Mintlify, Speakeasy, liblab, or similar (importers named) [E-03] [E-07].

## Core jobs to be done

1. Publish an API reference that stays in sync with the spec, with editable prose, guides, search, and a try-it console [E-02] [E-08].
2. Generate idiomatic SDKs in up to ten languages from the same spec, shaped by explicit configuration, and publish them to public registries [E-03] [E-09].
3. Expose the published docs and every included endpoint to MCP-compatible agents, with the SDK exclusion list acting as the agent permission list [E-05].
4. Collect production telemetry from backend packages and opt-in SDK reporters: logs, issues, traces, performance, releases, uptime checks, alerts [E-04] [E-10].
5. Score spec quality for SDK-generation readiness with a public, account-free 14-rule audit [E-16].
6. Keep all four surfaces regenerated from one merged spec change (claimed under 90 seconds) [E-02] [E-03].

## Public positioning vs observed behavior

[VERIFIED] The vendor positions Octri as "one input, four products" with no per-product plans; any studio can be left off and enabled later [E-01] [E-06].

[VERIFIED] The vendor states self-hosted generation and docs rendering are not available on any plan; the product is hosted-only [E-01] [E-06].

[VERIFIED] The vendor states neither customer content nor telemetry is used to train models; AI subprocessors operate under commercial terms prohibiting training [E-01] [E-17] [E-19].

[NOT TESTED] Marketing claims not verified by running the product include: rebuild "in under 90 seconds", generated-SDK idiomaticity per language, retry/backoff behavior in generated clients, redaction efficacy, N+1 detection accuracy, MCP operation-tool execution, and the try-it console against real APIs [E-02] [E-03] [E-04] [E-05].

[VERIFIED] The product's own documentation site is generated and hosted by the product (dogfooding), which is public, browsable evidence the docs renderer works at least for Octri's own content [E-08].

[VERIFIED] The public product changelog page exists but lists no releases as of 2026-10-22, so external observers cannot track feature cadence from that page [E-14].
