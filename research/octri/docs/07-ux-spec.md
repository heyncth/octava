# 07 - UX Specification

Grounded in marketing-page descriptions and the live dogfood documentation site (octri.dev/developers). No pixel dimensions, exact breakpoints, or component libraries are asserted - none were measured. Screens below are the view inventory a rebuild must cover; empty/error states are INFERRED unless a public page stated them.

## Marketing site (octri.dev)

- View: product pages (API Studio, SDK Studio, Monitoring, MCP), pricing with monthly/annual toggle and full 79-feature comparison, compare hub, free tools (Spec Audit, parser, converter), blog, open-source grant page [E-01] [E-06] [E-07] [E-16].
- Primary actions: Start for free, Book a call, Audit my spec, Choose plan [E-01] [E-06].
- [VERIFIED] Spec Audit widget: URL input or file upload, score animation from 10.0 down through deductions to final score, per-rule list with pass/fail, publish-to-public-address checkbox [E-16].

## Dashboard shell

- Org switcher, project selector, role-aware navigation. Members unlimited; roles granular; activity log; multiple orgs per login [E-06] [E-17] [E-18].
- Empty state for new users: create project / upload spec (INFERRED from onboarding copy [E-01]).

## API Studio (docs authoring)

- [VERIFIED] Studio edit mode: the draft opens as the finished endpoint page; edits happen in place on the real layout; Publish promotes the page to the reader site [E-02].
- [VERIFIED] Spec-fix actions from inside docs: add a parameter, describe a field, document a missing response, add an endpoint the spec forgot (writes back to spec and commits to repo) [E-02].
- [VERIFIED] Navigation builder: guides and reference in one tree; 21 built-in components (accordion, banner, button, callout, card, code group, fields, media, steps, tabs, etc.); custom components visual or MDX [E-02].
- [VERIFIED] Theme panel: light/dark both designed, colors, typography, corner radius, sidebar style/width, code themes, contents rail toggle, custom CSS escape hatch [E-02].
- [VERIFIED] Version switcher (Growth+); changelog view generated from spec diffs; redirects manager; password/SSO access settings; per-teammate edit vs publish permissions [E-02] [E-06].

## Published docs site (reader)

- [VERIFIED] Layout: left endpoint navigator grouped by tag; three-column reference page (params, response schemas, request example) [E-01] [E-08].
- [VERIFIED] Schema trees expand one level at a time; parameter tables with defaults and enums; response object trees with per-property prose [E-01] [E-08].
- [VERIFIED] Code sample language switcher covering cURL, JavaScript, Python plus every generated SDK language, showing the customer's own SDK method names [E-02] [E-08].
- [VERIFIED] Try-it drawer: schema-built form; snippets regenerate from typed input; calls run from the reader's browser against the real API, subject to the customer's CORS policy; base URL and allowed origins are customer-configurable [E-02].
- [VERIFIED] Search modal (full-text) with sidebar autocomplete; Ask-the-docs chat affordance on every page with a project-level kill switch [E-02].
- [VERIFIED] Status banner appears site-wide while uptime checks fail, linking to the status page [E-02] [E-04].
- [VERIFIED] Per-page PDF export; feedback helpful/not-helpful vote; deprecation banners on deprecated operations [E-02].
- [VERIFIED] Agent affordances: llms.txt index, copy-as-markdown button, indexing control switch [E-02].
- Reader privacy: page views recorded without IP/UA/visitor id; layout preferences kept in-browser only [E-17].

## SDK Studio

- [VERIFIED] Two-pane workspace: settings left, live code preview right, preview updating as each control changes (client style Functions/Class, namespace None/Tags, method naming Keep/Shortened, folder structure Flat/By-tag, package name, base URL) [E-03].
- [VERIFIED] Endpoint table: every operation a row; search across ~1000 operations by path or name; row opens per-endpoint settings (rename, doc comment, include toggle, deprecation message, pagination/streaming selectors, retry/idempotency overrides, custom folders) [E-03].
- [VERIFIED] Hooks editor: fixed declaration the generator owns (not editable), free-form body; dependency declaration flow that writes the import version into the file and manifest [E-03].
- [VERIFIED] Spec-audit panel: score meter with amber threshold below 8; findings arrive with point deductions; per-finding apply-fix or dismiss [E-03] [E-16].
- [VERIFIED] Build view: ten parallel lanes, per-language phase progress, failure isolation with output, per-language retry, artifact retention and zip download [E-03].
- [VERIFIED] Publish screen: shows which languages rebuild and destination registries before confirming [E-03].
- [VERIFIED] Agent tab: previews MCP tools built from the current draft configuration [E-05].

## Monitoring workspace

- [VERIFIED] Seven tabs (logs, issues, traces, performance, releases, uptime, alerts) sharing one time-range control (1 hour to 90 days) and facet filters with row counts; top-line stats (total events, error rate, distinct issues, p95, costliest issues) [E-04].
- [VERIFIED] Logs: SQL-shaped query editor with field/operator/value completion plus equivalent filter-builder dropdowns; quick chips (errors, 5xx, 4xx, >1s); live tail [E-04].
- [VERIFIED] Event detail: failing frame with project source around it (via uploaded maps), user-step timeline, device/browser/runtime, occurrence stepping, linked server trace [E-04].
- [VERIFIED] Issues table: message, endpoint, level, event count, affected users, last seen, status; issue detail with assign, comment, resolve, snooze (hour/week/count), tag breakdown, suspect commits [E-04].
- [VERIFIED] Traces: waterfall colored by span type (client, server, db, cache, outbound, queue), service map with per-edge error rate, N+1 callout, per-route percentile table with latency/error budgets and burn indicators [E-04].
- [VERIFIED] Alerts table: rule name, kind, condition, channel, fired count, last fired, pause; "evaluate now" simulation [E-04].
- [VERIFIED] Uptime: probe list with interval and pass rule, run-now, status page editor (components, incidents with update history, notices), docs-banner linkage [E-04].
- [INFERRED] Empty states: new project shows no events until first ingest; alert list empty until rules created; issues empty until fingerprints appear. Exact copy UNKNOWN.
- [INFERRED] Error states: 429 on ingest ceiling surfaces in dashboards as refused events with Retry-After guidance to SDKs [E-04].

## MCP workspace (dashboard)

- [VERIFIED] Project MCP page lists both halves (7 documentation tools + operation tools) with filter; pick a tool, fill arguments, Run tool (write methods require a second confirmation), transcript retained with status and body [E-05].
- [VERIFIED] Pre-filled client config snippet with project id and auth-scheme variables; placeholder detection flags values still unset before copy [E-05].

## CLI

- [VERIFIED] Interactive mode uses colors, spinners, live build lanes, tables; plain mode for CI; --json for scripting [E-12].

## Auth screens

- [VERIFIED] Sign-up with email/password or Google/GitHub; email verification; TOTP 2FA with recovery codes; active-session list with remote revoke; password reset via transactional email [E-17] [E-18].
- [VERIFIED] Enterprise: SAML SSO config (entity ID, sign-on URL, certificate, email domains, attribute mapping) and org-wide 2FA enforcement with break-glass owner path [E-17] [E-18].
