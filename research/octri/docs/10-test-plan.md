# 10 - Test Plan

Acceptance tests derived from VERIFIED and INFERRED claims in `02` and `03`. None of these were executed against Octri (no trial account). They serve two purposes: (a) if someone later runs a trial, these are the probes; (b) a clean-room rebuild should satisfy the equivalents. Marketing-only timing claims (e.g., "under 90 seconds") are excluded here and tracked in `13-open-questions.md`.

## Spec ingestion

1. Given an OpenAPI 3.1 YAML document with 24 operations, when the spec is uploaded, then a docs draft exists for every operation and the audit score is computed on the 14 published rules [E-16].
2. Given a Swagger 2.0 document, when passed to the Spec Audit directly, then it is refused or directed through conversion; after conversion the audit accepts it [E-16].
3. Given a Postman collection, when imported via API Studio, then endpoints appear as OpenAPI-normalized operations (normalization fidelity is the assertion under test) [E-02].
4. Given a 6 MB spec, when audited, then it is rejected with a size error (published limit: 5 MB) [E-16].
5. Given 11 audit requests from one IP within a minute, when the 11th is sent, then it is rate-limited (published limit: 10/min) [E-16].

## Audit scoring (negative + positive)

6. Given an operation with a 200 response lacking a schema, when audited, then the `response-schema` rule deducts proportionally to surface coverage and the finding names the operation [E-16].
7. Given two operationIds that collapse to the same public SDK name, when audited, then the `operation-id-unique` rule fires with a proposed rename [E-16].
8. Given a 204-only operation, when audited, then no response-schema deduction applies (204 exempt per rule text) [E-16].
9. Given a clean spec scoring 10.0, when one heavyweight failure is introduced, then the score drops by the published weight scaled by coverage [E-16].

## Docs generation and editing

10. Given a generated endpoint page, when a field description is edited in place and the spec is then changed elsewhere, when regeneration runs, then the edited field retains the human text and untouched fields refresh [E-02].
11. Given a synced docs folder, when a Markdown file is deleted from the repo, then the published page remains (no destructive delete on push) [E-02].
12. Given a password-protected docs site, when an anonymous reader opens a page, then access is gated; drafts are never served to readers or to MCP documentation tools [E-02] [E-05].
13. Given Ask-the-docs disabled at project level, when any docs page loads, then no chat affordance is reachable [E-02].
14. Given the reader try-it console on a customer API that disallows the docs origin in CORS, when a request is attempted from the browser, then the browser enforces the refusal (console is browser-side) [E-02].

## SDK generation

15. Given a spec with cursor pagination and a `data` wrapper field, when a Python SDK is generated with pagination configured, then the iterator walks multiple pages without user-written loops [E-03] [E-09].
16. Given a write operation with idempotency enabled, when the client retries after a 503 carrying Retry-After, then the same Idempotency-Key is reused and the wait honors the server hint [E-03].
17. Given one of ten languages fails to compile during a build, when the build finishes, then the other nine publish and the failed language can be retried alone [E-03].
18. Given an endpoint excluded in SDK Studio, when SDKs and the MCP snapshot regenerate, then no method and no MCP operation tool exist for it [E-03] [E-05].
19. Given a custom before-request hook importing a hashing library, when the package is generated, then the dependency appears in the manifest and survives regeneration [E-03].
20. Given a spec-transform rename, when generation runs, then the source spec file on GitHub is unchanged [E-03].

## Registry publishing

21. Given connected npm credentials and auto-publish on, when a release build completes, then a new version appears on npm under the customer-controlled package name [E-03].
22. Given Go distribution, when a release is published, then a git tag lands on the customer repository and `go get` resolves it [E-03] [E-09].
23. Given an attempt to rename a package after first publish, then the platform blocks or documents that registries refuse renames (expected refusal path) [E-03].

## MCP server

24. Given only a project id and no credential, when `npx @octri/mcp` starts and a client lists tools, then exactly the seven documentation tools plus operation tools from the published snapshot appear; documentation tool calls succeed; operation tool calls fail auth or are absent of executable behavior without credentials [E-05] [E-11].
25. Given a credential in the local config, when an operation tool runs, then a real HTTP request reaches the configured base URL (point at staging in tests) [E-05].
26. Given base URL pointed at staging via environment variable, when the same tool runs, then requests hit the staging host [E-05].
27. Given an API returning 403 insufficient_scope, when the operation tool runs, then the agent-visible error includes status and response body [E-05].
28. Given a tool name exceeding 64 chars or colliding after coercion, when the snapshot is built, then the name is truncated/deduplicated per documented rules [E-05].
29. Given two publishes within two minutes, when a running server fetches the tool list, then it may serve the cached list until TTL expiry [E-05].

## Monitoring

30. Given a project with monitoring disabled, when an SDK is built, then no reporter code path sends events [E-04] [E-10].
31. Given monitoring enabled and SDK rebuilt, when a consumer app does not set `logging.enabled=true`, then zero events arrive at ingest [E-04] [E-10].
32. Given `logging.enabled=true`, when an exception occurs, then an event arrives with release tag, SDK version, language, and method; a 202 is returned; retries by the SDK do not duplicate the event id (dedup) [E-04].
33. Given an event whose payload contains an Authorization header and a card-like string under an innocuous key, when it passes client redaction and ingest redaction, then neither secret is stored [E-04] [E-10].
34. Given two events with messages differing only by numeric ids, when grouping runs, then both land on one issue [E-04].
35. Given client and server instrumentation on one request, when traces are queried, then one trace contains both client and server spans [E-04].
36. Given a resolved issue that throws again, when alert rules evaluate, then the regression alert fires once per cooldown window [E-04].
37. Given ingest 10x plan allowance exceeded, when an event is sent, then the response is 429 with Retry-After [E-04] [E-06].
38. Given raw events older than the plan retention window, when queried, then raw events are gone but rollup charts still render [E-06] [E-10].
39. Given a safe-read GET in the spec, when uptime config generates, then a probe exists with the customer's pass rules; failing probes update the status page and docs banner [E-04].

## Accounts, tenancy, security

40. Given an unauthenticated request to dashboard routes, then access is refused (login redirect or 401) [E-18].
41. Given a member scoped to one project, when they open another project in the same org, then access is denied [E-18].
42. Given TOTP enabled, when a reused code is presented, then it is rejected (single-use enforcement) [E-18].
43. Given an API key, when listed later, then only a hash was stored; revocation takes effect immediately for new requests [E-18].
44. Given org owner revokes a session, when that session's next request arrives, then it fails [E-18].
45. Given project deletion, when it completes, then specs, docs, analytics, chat sessions, webhooks, and build artifacts for that project are gone; org-level erase completes within 30 days [E-17].

## Billing meters

46. Given Free plan, when a second SDK language is requested, then it is refused; when a 51st endpoint is included, then it exceeds the 50-endpoint cap [E-06].
47. Given Starter plan, when a third language is added mid-cycle, then the +$50/mo add-on applies immediately; removal waits until period end [E-06] [E-09].
48. Given AI credits exhausted, when Ask-the-docs or generation runs, then those features stop while hosting, SDK builds, and monitoring ingest continue [E-06].

## CLI

49. Given CI without a browser, when `octri specs push` runs with `--json` and token env vars, then a single JSON document is emitted on stdout and the pipeline completes or fails with errors on stderr [E-12].
50. Given `octri mcp serve` without `--allow-publish`, when an assistant attempts a registry publish tool, then it is refused; with the flag, it succeeds [E-12].

## Negative / abuse cases

51. Given a tool definition consumer hits the public tools endpoint above 300 req/min from one client identity, then further requests are limited [E-05].
52. Given the Spec Audit with a private staging URL, when run, then no copy of the document is retained (only info block + score + hashed IP) [E-16] [E-17].
53. Given a docs site indexed switch set off, when crawlers request llms.txt, then machine indexing affordances are withdrawn [E-02].
