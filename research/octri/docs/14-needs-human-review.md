# 14 - Needs Human Review

Short list; each item is expensive to get wrong.

1. **MCP hosting contradiction** - `/api-studio` marketing says agents call endpoints "from a server we host"; `/mcp` FAQ says no hosted remote endpoint today. Decide which is true before designing any agent-facing architecture on it (open question 2) [E-02] [E-05].
2. **Wedge choice** - this package specs near-parity across four studios. A rebuild almost certainly should not attempt all of it at once. Human sponsor must pick the wedge before Phase 2 of `12-implementation-roadmap.md`.
3. **Trial budget decision** - nearly every runtime claim is NOT TESTED. Authorize or explicitly decline a paid/Free-tier observation trial; without it the spec stays marketing-bounded (open questions 16-30).
4. **Trademark and branding clearance** - "Octri" and its visual identity are off-limits; run a real search before choosing a product name [E-17].
5. **Apache-2.0 reuse strategy** - counsel/product should confirm whether to build on `octri-openapi-parser` / `octri-spec-auditor` / `octri-spec-converter` or rewrite; license compliance obligations attach either way [E-15].
6. **MIT-runtime claim reliance** - if the rebuild plans to resell or embed Octri-generated packages (unusual, but possible in migrations), spot-audit licenses across all ten ecosystems first [E-13].
7. **Monitoring privacy posture** - customer payloads can contain end-users' personal data; a rebuild needs its own DPIA-style review, not a copy of Octri's text [E-17] [E-18].
8. **MCP write-tool safety defaults** - operation tools can delete production data; default configuration choices (staging base URL, write-approval) are product-ethics decisions, not just features [E-05].
9. **No-SOC2 posture** - Octri states it has no SOC 2/ISO. If the rebuild targets enterprise buyers, plan compliance independently rather than copying this posture [E-18].
10. **Evidence staleness** - pricing and feature gates will drift; re-fetch `/pricing` and `/compare` before any board-level competitive claim (evidence log append-only corrections) [E-06] [E-07].
