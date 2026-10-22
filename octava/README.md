# Octava

Self-hosted OpenAPI readiness audit. Free, Apache-2.0, no account, no phone-home.

## Usage

```
octava <openapi-file>
```

Parses OpenAPI 3.0/3.1/3.2 (JSON or YAML), runs 14 weighted readiness rules, prints score /10 and band (Strong >= 8.0, Mixed >= 6.0, Weak < 6.0). Swagger 2.0 must be converted first. Size limit 5 MB.

Exit codes: 0 = ran, 2 = usage or parse error.

## Build

```
cargo build --release
cargo test
```

## Rules (weights sum 15.75, normalized to /10)

| Rule | Weight |
|---|---|
| response-schema | 2.0 |
| shared-ref-models | 2.0 |
| operation-id-unique | 2.0 |
| path-params | 1.5 |
| operation-id-present | 1.0 |
| operation-id-naming | 1.0 |
| operation-documented | 1.0 |
| request-body-documented | 1.0 |
| error-responses | 1.0 |
| tags | 1.0 |
| duplicate-paths | 1.0 |
| security-declared | 1.0 |
| params-documented | 0.75 |
| server-url | 0.5 |

Rule behavior is an original implementation of the publicly described audit algorithm (see research/octri, E-16). No Octri code or assets copied.

## License

Apache-2.0. See LICENSE.
