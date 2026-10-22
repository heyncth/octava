# 11 - Risks

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Legal/trademark: confusingly similar branding or copied marketing/UI expression from Octri | Medium (agents default to copying) | High (cease-and-desist, app-store/registry takedowns) | Clean-room barriers in `09-clean-room-barriers.md`; original wording and visuals; trademark search before launch; counsel review [E-15] [E-17] |
| Legal/ToS: reusing Apache-2.0 Octri repos without preserving license/notice text | Low | Medium | Preserve LICENSE and NOTICE; attribute; do not imply affiliation [E-15] |
| Thin-evidence: ~all runtime behavior is NOT TESTED; a rebuild may encode marketing fiction | High | High | Budget a paid trial explicitly to convert NOT TESTED rows; treat 13-open-questions as trial script; prefer docs pages (behavioral) over marketing copy |
| Contradiction risk: MCP hosting story differs across pages ("server we host" vs "not today") | Certain (observed) | Medium (wrong agent-facing architecture) | Resolve via trial or vendor contact; design rebuild with pluggable hosted-remote option regardless [E-02] [E-05] |
| Dependency/license: generated SDK runtimes claimed MIT but not package-audited | Low | High if false (customer legal exposure) | Spot-check published showcase packages' LICENSE files before relying on the claim; require license audit in any commercial deal [E-13] [E-15] |
| Dependency risk: heavy reliance on Anthropic/OpenAI for core docs UX (prose, chat, embeddings) | Medium | Medium (cost, outage, policy change) | Abstract LLM provider; offline template fallback for prose; self-hosted embeddings option in rebuild |
| Dependency risk: Paddle as sole merchant of record; Stripe fallback inactive | Low | Medium (payments outage) | Rebuild should use its own billing provider; not a clone requirement |
| Platform risk: no SOC 2/ISO means enterprise sales cycles stall (for Octri); for a rebuild, same gap if copying posture | High (stated fact) | Medium | Plan compliance roadmap independently; do not copy "we have no cert" as a feature |
| Competitive risk: Octri ships fast on one-spec-pipeline; feature parity chase is endless | Medium | Medium | Rebuild wins on a wedge (e.g., open-source core, self-host, audit tool only) rather than full parity [E-14] [E-15] |
| Data-portability risk: customers locked via published packages + docs URLs; migration cost is real | Medium (for them, high for their churn) | Medium | Rebuilds that import configs (Fern/Stainless/Mintlify) lower switching cost [E-03] [E-07] |
| Security risk: MCP operation tools can perform destructive writes in production if scoping is sloppy | Medium (documented concern) | High (customer incidents) | Default-deny writes, staging base-URL defaults, scoped credentials, exclusion list as permission list [E-05] |
| Monitoring privacy risk: customer-controlled payloads can still leak end-user PII | Medium | High (GDPR exposure for rebuild operators) | Default-deny writes, staging base-URL defaults, scoped credentials, exclusion list as permission list [E-04] [E-10] [E-17] |
| Rate/abuse risk: free audit and free tier attract scraping | Medium | Low-Medium | Published limits already exist (10 audits/min, 5 MB, plan caps); copy the concept, not the exact numbers blindly [E-16] |
| Evidence staleness: pricing/features change frequently in SaaS; this package freezes 2026-10-22 state | Certain | Medium | Re-fetch pricing/compare pages before any competitive decision; evidence log is append-only for corrections [E-06] |
