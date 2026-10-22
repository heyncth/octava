# 13 - Open Questions

Every UNKNOWN from `02-capability-matrix.md` and every material NOT TESTED behavioral claim appears here. Trial-account observation or vendor contact can close these; the evidence log is append-only for any upgrade.

## UNKNOWN (not found in public sources)

1. Full inventory, auth scheme, and rate limits of Octri's own public HTTP API - claimed to exist on every plan but undocumented publicly [E-06] [E-12].
2. Whether Octri offers a hosted remote MCP endpoint (connect-by-URL). `/api-studio` says agents call endpoints "from a server we host"; `/mcp` FAQ says "Not today" and no Octri-hosted remote endpoint exists. Unresolved contradiction [E-02] [E-05].
3. Public product changelog / release cadence - page exists but is empty as of 2026-10-22 [E-14].
4. Real-time collaborative editing or conflict resolution in studio editors - no claims found.
5. Mobile-native applications - no claims found.
6. Exact internal storage schemas, queue topology, build-farm design - closed source [E-15].
7. Non-Enterprise support response-time SLAs - tiers named without numbers [E-06].
8. Whether standard OTLP/OpenTelemetry ingest is supported - concepts used, protocol compatibility unstated [E-04].
9. Licenses of the `octri-cli` and `octri-mcp` repositories specifically (Rust tooling repos are Apache-2.0; TypeScript repos' licenses not confirmed on the org page) [E-15].
10. Patent holdings of Octri, LLC - no statements on fetched pages.
11. Whether generated SDK packages' claimed MIT runtimes hold uniformly across all ten language ecosystems (spot-check needed) [E-13] [E-15].
12. Enterprise plan internals (SLA numbers, data-residency options beyond US/EEA subprocessors, dedicated support mechanics) - "Custom" only [E-06] [E-17].
13. Behavior of overage billing beyond "recorded, not charged today" - explicitly future-dependent [E-04] [E-06].
14. Whether Postman/AsyncAPI conversion preserves webhooks/async semantics faithfully - claimed translation, fidelity untested [E-01] [E-02].
15. Details of the AI credit economy beyond "1 per endpoint written or chat answer" (batch pricing, caching rules for chat, embedding refresh costs) [E-06].

## NOT TESTED behavioral claims (marketing/docs only; highest-value trial probes)

16. End-to-end pipeline completes in "under 90 seconds" for a merged spec change across pages, clients, tools, changelog [E-02] [E-03].
17. Idiomatic quality of generated clients in each of the ten languages (type fidelity, naming, error surfaces) [E-03] [E-09].
18. Retry/backoff/Retry-After semantics as described, including the 8s cap mentioned in homepage animation copy [E-01] [E-03].
19. Effectiveness of client-side + ingest-side redaction against real secret shapes (tokens in stack frames, emails in free text) [E-04] [E-10].
20. Fingerprint grouping quality on messy real-world messages (false merges / false splits) [E-04].
21. N+1 detection accuracy and service-map correctness under distributed load [E-04].
22. MCP operation-tool execution success rate against real APIs (auth schemes, OAuth2 flows, binary bodies) [E-05].
23. Importer fidelity for Speakeasy / Stainless / Fern / liblab / Mintlify configs, including method renames and pagination markers [E-03] [E-07].
24. Edit-preserving regeneration behavior under large diffs (thousands of operations) [E-02].
25. Registry publish mechanics: Maven Central signing, Pub.dev review quirks, crates.io ownership verification [E-03].
26. Source-map symbolication quality for minified browser stacks and native-ish frames [E-04] [E-12].
27. Uptime probe geographic distribution and scheduling accuracy (not published) [E-04].
28. SSO/SAML docs-gating UX with real IdPs (Okta, Entra) [E-06] [E-18].
29. Ask-the-docs answer grounding quality and citation accuracy [E-02].
30. Behavior at plan boundaries: endpoint cap enforcement UX, language add/remove timing, build retention pruning [E-03] [E-06].

## Process questions for the rebuild team

31. Which wedge differentiates enough to justify not chasing full parity (self-host? open generator? audit-only?) [this package, 12].
32. Will a paid Octri trial be authorized to close items 16-30, and who logs the evidence rows?
33. Is reusing the Apache-2.0 Rust crates for the spec layer acceptable to product strategy (speed) vs clean-room rewrite (differentiation)?

## Sources attempted that came up empty or thin

- `https://octri.dev/developers/changelog` - fetched, empty content [E-14].
- No public REST API reference for Octri's own API was linked from docs nav, pricing, or footer (nav enumerates guides for api-docs, sdks, languages, mcp, monitoring, cli only) [E-08] [E-12].
