# 03 - User Flows

All flows are reconstructed from public marketing and docs pages. No account was created; steps marked VERIFIED reflect documented intent, not executed sessions. Timing and UI details beyond the cited pages are INFERRED.

### Flow: First-time onboarding (new project from a spec)

- Actors: API team member (org owner).
- Trigger: "Start for free" registration, then a first project.
- Steps:
  1. Register at app.octri.dev/register [E-01].
  2. Create a project; upload an OpenAPI/Swagger/Postman/AsyncAPI file (JSON or YAML) or connect a GitHub repository path [E-02] [E-06].
  3. Platform parses the spec and runs the SDK-readiness audit (14 rules) as the first step [E-16].
  4. AI writes reference prose per endpoint (consumes 1 credit per endpoint); user reviews in API Studio [E-02] [E-06].
  5. User edits pages in place, adds MDX guides, sets navigation, theme, and custom domain [E-02].
  6. In SDK Studio the user picks languages (plan-limited), configures client shape, naming, reliability, and builds [E-03] [E-09].
  7. MCP tools become available from the published snapshot with nothing to switch on [E-05].
  8. Optionally enable Monitoring on the project and rebuild SDKs so the reporter is compiled in (still disabled for end-users) [E-04] [E-10].
  9. Publish: docs go live on the chosen domain; registry publish happens if auto-publish is on [E-02] [E-03].
- Outcomes: Live docs site, one or more SDK packages, MCP tool list, optional monitoring.
- Evidence: [E-02] [E-03] [E-05] [E-06] [E-10] [E-16]. End-to-end success and timing NOT TESTED.

### Flow: Spec change propagation (GitHub sync)

- Actors: API developer pushing to the connected repo; Octri project owner.
- Trigger: Merged pull request touching the watched spec file on the watched branch [E-02] [E-03].
- Steps:
  1. Sync rebuilds a draft of affected docs pages with a diff to review; readers see nothing until publish [E-02] [E-05].
  2. SDK builds regenerate per language; changelog entries are written from the spec diff [E-03].
  3. MCP tool list updates on next publish; running agents see it after their client restarts (tool list cached ~2 minutes, loaded at client startup) [E-05].
  4. If registry auto-publish is on, releases push to connected registries after builds finish; otherwise manual publish [E-01] [E-03].
- Outcomes: Docs, SDKs, MCP tools, changelog all move together from one merge.
- Evidence: [E-02] [E-03] [E-05]. "Under 90 seconds" NOT TESTED [E-02] [E-03].

### Flow: Edit-and-regenerate (docs)

- Actors: Documentation author.
- Trigger: Spec regeneration after an edit session.
- Steps:
  1. Author edits generated endpoint pages in place; edits are stored per field on top of generated text [E-02].
  2. Spec changes; regeneration refreshes untouched fields and preserves edited fields; UI reports how many endpoints carry edits before a run [E-02].
  3. Deleted files in a synced docs folder do not delete pages; renames create new pages unless slug/path still matches; pushes land as drafts [E-02].
- Outcomes: Human prose survives regeneration; drafts stay private until publish.
- Evidence: [E-02]. NOT TESTED.

### Flow: SDK shaping and release

- Actors: SDK maintainer.
- Trigger: Need a new client release.
- Steps:
  1. Configure project/language/endpoint settings in SDK Studio with live preview [E-03] [E-09].
  2. Optionally add before-request/after-response hooks per language; declare hook dependencies so they enter the generated manifest [E-03].
  3. Optionally add spec transforms (with reasons) that never mutate the source spec [E-03].
  4. Build: ten languages build independently; a failed language reports its output and can be retried alone while others ship [E-03].
  5. Publish: version taken from the spec flows into every manifest; Go/Swift published as git tags; others to their registries; zip download always available [E-03] [E-09].
  6. Optional: push each language into a customer-owned repository where regenerations arrive as PRs [E-03].
- Outcomes: Versioned packages on registries, retained build artifacts per plan.
- Evidence: [E-03] [E-09]. Code quality and publish success NOT TESTED.

### Flow: Agent integration via MCP

- Actors: End developer using Claude Desktop, Cursor, or VS Code; API project owner.
- Trigger: Desire to let an assistant read docs and call the API.
- Steps:
  1. Project owner notes project id; no credentials required for documentation tools [E-05] [E-11].
  2. End developer adds `npx @octri/mcp` with the project id to their client config (claude_desktop_config.json, .cursor/mcp.json, or .vscode/mcp.json) [E-05].
  3. Client restarts; seven documentation tools appear immediately; operation tools appear from the published SDK snapshot [E-05] [E-11].
  4. To execute real calls, the end developer adds their own credential to the local config; the credential never reaches Octri [E-05].
  5. Project owner scopes risk via base-URL variable (e.g. staging), narrow credentials, and SDK exclusions (excluded endpoint = no tool) [E-05].
- Outcomes: Agent reads published docs and optionally performs real HTTP calls, including writes.
- Evidence: [E-05] [E-11]. Tool execution NOT TESTED.

### Flow: Monitoring enablement (two-sided opt-in)

- Actors: API publisher; downstream SDK consumer.
- Trigger: Publisher wants production visibility into SDK usage.
- Steps:
  1. Publisher enables Monitoring on the project and rebuilds/publishes SDKs so the reporter is compiled in [E-04] [E-10].
  2. Copies already installed report nothing; new versions still default `logging.enabled=false` [E-04] [E-10].
  3. Downstream developer sets `logging.enabled=true` in their application; browser personal context additionally requires `setLoggingConsent(true)` [E-04] [E-10].
  4. Events post fire-and-forget with a project-scoped ingest token; 202 accepted; redaction runs client-side then again at ingest [E-04].
  5. Publisher triages in Logs/Issues/Traces; can upload source maps via CLI for readable stacks [E-04] [E-12].
- Outcomes: Opt-in telemetry tagged with release, SDK version, language, method.
- Evidence: [E-04] [E-10] [E-12]. Ingest behavior NOT TESTED.

### Flow: Uptime and status page (no SDK required)

- Actors: API publisher; publisher's public users.
- Trigger: Project has monitoring on; spec lists safe read endpoints.
- Steps:
  1. Probes are generated from spec safe-read endpoints; pass rule = status ceiling and latency ceiling, both configurable [E-04].
  2. Checks run on a schedule from Octri's side; any check can be run on demand [E-04].
  3. Results publish to a status page on the customer's own domain with incident workflows and notices [E-04].
  4. Failing checks raise a banner on the documentation site [E-02] [E-04].
- Outcomes: Public uptime history without deploying any agent.
- Evidence: [E-04]. NOT TESTED.

### Flow: Free Spec Audit (account-free)

- Actors: Any visitor.
- Trigger: Paste a public OpenAPI URL or upload a file at /openapi-audit.
- Steps:
  1. Spec is parsed (OpenAPI 3.0/3.1/3.2; Swagger 2.0 must be converted first) [E-16].
  2. 14 weighted rules run; deductions scale with how much of the surface a rule affects [E-16].
  3. Score /10 with band (Strong >=8.0, Mixed 6.0-7.9, Weak <6.0) plus per-rule findings and fixes [E-16].
  4. Optional: publish the score at a public octri.dev address (public specs only; document still not stored) [E-16].
  5. The service retains only the spec's info block, the score, and a hashed IP for up to 730 days - not the document [E-16] [E-17].
- Outcomes: Readiness score; optional public score page.
- Evidence: [E-16] [E-17]. NOT TESTED.

### Flow: Migration from another docs/SDK vendor

- Actors: Team leaving Fern, Stainless, Mintlify, Speakeasy, liblab, or Redocly-adjacent stacks.
- Trigger: Vendor change, pricing, or Stainless project intake stop (claimed May 2026) [E-07].
- Steps:
  1. Point Octri at the existing config; importers read Fern generators.yml/docs.yml, Mintlify docs.json/mint.json, Stainless stainless.yml; method renames and pagination markers survive via vendor extensions [E-03] [E-07].
  2. Preview lists what imported and what needs attention; nothing publishes until confirmed [E-03].
  3. Docs migration: existing Markdown/MDX files land as pages with frontmatter titles; URL slugs are settable and old addresses can keep working [E-02].
- Outcomes: Imported configuration; dual-running sites until DNS cutover.
- Evidence: [E-02] [E-03] [E-07]. Importer fidelity NOT TESTED.
