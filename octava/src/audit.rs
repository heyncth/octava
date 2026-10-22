//! OpenAPI SDK-readiness audit: 14 weighted rules, score /10.
//! Rule weights and bands follow the publicly documented audit algorithm
//! described in research/octri (E-16). Implementation is original.

use serde_json::Value;

pub const SIZE_LIMIT_BYTES: usize = 5 * 1024 * 1024;

#[derive(Debug, Clone)]
pub struct Finding {
    pub path: String,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RuleResult {
    pub id: &'static str,
    pub title: &'static str,
    pub weight: f64,
    pub earned: f64,
    pub findings: Vec<Finding>,
}

#[derive(Debug, Clone)]
pub struct AuditReport {
    pub score: f64,
    pub band: &'static str,
    pub rules: Vec<RuleResult>,
}

#[derive(Debug)]
pub enum AuditError {
    TooLarge(usize),
    Parse(String),
    NotOpenApi3,
    Swagger2,
}

impl std::fmt::Display for AuditError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuditError::TooLarge(n) => write!(f, "spec is {n} bytes, limit is {SIZE_LIMIT_BYTES} bytes"),
            AuditError::Parse(m) => write!(f, "cannot parse document: {m}"),
            AuditError::NotOpenApi3 => write!(f, "document has no openapi 3.x version field"),
            AuditError::Swagger2 => write!(f, "swagger 2.0 detected, convert to openapi 3.x first"),
        }
    }
}

struct Op<'a> {
    method: &'a str,
    path: &'a str,
    op: &'a Value,
}

const METHODS: [&str; 5] = ["get", "post", "put", "patch", "delete"];

fn collect_ops(doc: &Value) -> Vec<Op<'_>> {
    let mut ops = Vec::new();
    if let Some(paths) = doc.get("paths").and_then(Value::as_object) {
        for (p, item) in paths {
            if let Some(item) = item.as_object() {
                for m in METHODS {
                    if let Some(op) = item.get(m) {
                        if op.is_object() {
                            ops.push(Op { method: m, path: p, op });
                        }
                    }
                }
            }
        }
    }
    ops
}

fn op_label(op: &Op) -> String {
    format!("{} {}", op.method.to_uppercase(), op.path)
}

fn is_success_response(code: &str) -> bool {
    match code.parse::<u16>() {
        Ok(c) if (200..300).contains(&c) => c != 204 && c != 205,
        _ => false,
    }
}

fn is_error_response(code: &str) -> bool {
    matches!(code.parse::<u16>(), Ok(c) if (400..600).contains(&c))
}

fn has_schema(v: &Value) -> bool {
    v.get("schema").is_some() || v.get("$ref").is_some()
}

fn path_level_parameters<'a>(doc: &'a Value, path: &str) -> Vec<&'a Value> {
    doc.get("paths")
        .and_then(|p| p.get(path))
        .and_then(|i| i.get("parameters"))
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default()
}

fn all_params<'a>(op: &'a Op<'a>, doc: &'a Value) -> Vec<&'a Value> {
    let mut out: Vec<&'a Value> = op
        .op
        .get("parameters")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    out.extend(path_level_parameters(doc, op.path));
    out
}

fn path_template_params(path: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = path;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        if let Some(end) = after.find('}') {
            out.push(after[..end].to_string());
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    out
}

fn normalize_path_template(path: &str) -> String {
    let mut out = String::new();
    let mut rest = path;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        out.push_str("{}");
        if let Some(end) = rest[start..].find('}') {
            rest = &rest[start + end + 1..];
        } else {
            break;
        }
    }
    out.push_str(rest);
    out
}

fn is_camel_alnum(id: &str) -> bool {
    let mut chars = id.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric())
}

fn visit_refs_with_path(
    node: &Value,
    path: &str,
    counts: &mut std::collections::HashMap<String, std::collections::HashSet<String>>,
    _touched: &mut bool,
) {
    match node {
        Value::Object(map) => {
            if let Some(Value::String(r)) = map.get("$ref") {
                if let Some(name) = r.strip_prefix("#/components/schemas/") {
                    counts.entry(name.to_string()).or_default().insert(path.to_string());
                }
            }
            for v in map.values() {
                visit_refs_with_path(v, path, counts, _touched);
            }
        }
        Value::Array(arr) => arr.iter().for_each(|v| visit_refs_with_path(v, path, counts, _touched)),
        _ => {}
    }
}

fn coverage_rule(id: &'static str, title: &'static str, weight: f64, ops: &[Op], compliant: impl Fn(&Op) -> Option<String>) -> RuleResult {
    let total = ops.len();
    let mut findings = Vec::new();
    let mut good = 0usize;
    for op in ops {
        match compliant(op) {
            None => good += 1,
            Some(msg) => findings.push(Finding { path: op_label(op), message: msg }),
        }
    }
    let earned = if total == 0 {
        weight
    } else {
        weight * (good as f64) / (total as f64)
    };
    RuleResult { id, title, weight, earned, findings }
}

pub fn parse_spec(bytes: &[u8]) -> Result<Value, AuditError> {
    if bytes.len() > SIZE_LIMIT_BYTES {
        return Err(AuditError::TooLarge(bytes.len()));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| AuditError::Parse("not utf-8".into()))?;
    let doc: Value = if let Ok(v) = serde_json::from_str::<Value>(text) {
        v
    } else {
        serde_yaml::from_str::<Value>(text).map_err(|e| AuditError::Parse(e.to_string()))?
    };
    if !doc.is_object() {
        return Err(AuditError::Parse("root is not an object".into()));
    }
    if doc.get("swagger").and_then(Value::as_str).is_some_and(|s| s.starts_with('2')) {
        return Err(AuditError::Swagger2);
    }
    let ver = doc.get("openapi").and_then(Value::as_str).unwrap_or("");
    if !(ver.starts_with("3.0") || ver.starts_with("3.1") || ver.starts_with("3.2")) {
        return Err(AuditError::NotOpenApi3);
    }
    Ok(doc)
}

pub fn audit(doc: &Value) -> AuditReport {
    let ops = collect_ops(doc);
    let mut rules = Vec::new();

    // 1. response-schema (2.0): every 2xx (not 204/205) response carries a schema.
    rules.push(coverage_rule(
        "response-schema",
        "Success responses define schemas",
        2.0,
        &ops,
        |op| {
            let responses = op.op.get("responses").and_then(Value::as_object);
            let Some(responses) = responses else {
                return Some("no responses object".into());
            };
            let needing: Vec<(&String, &Value)> = responses
                .iter()
                .filter(|(code, _)| is_success_response(code))
                .collect();
            if needing.is_empty() {
                return None;
            }
            for (code, r) in needing {
                let ok = r
                    .get("content")
                    .and_then(Value::as_object)
                    .map(|c| c.values().any(has_schema))
                    .unwrap_or(false);
                if !ok {
                    return Some(format!("response {code} lacks content schema"));
                }
            }
            None
        },
    ));

    // 2. shared-ref-models (2.0): at least one #/components/schemas model referenced from >=2 paths.
    {
        let mut counts: std::collections::HashMap<String, std::collections::HashSet<String>> =
            std::collections::HashMap::new();
        if let Some(paths) = doc.get("paths").and_then(Value::as_object) {
            for (p, item) in paths {
                visit_refs_with_path(item, p, &mut counts, &mut false);
            }
        }
        let shared = counts.values().filter(|s| s.len() >= 2).count();
        let earned = if counts.is_empty() { 0.0 } else if shared > 0 { 2.0 } else { 2.0 * 0.5 };
        let mut findings = Vec::new();
        if counts.is_empty() {
            findings.push(Finding {
                path: "components.schemas".into(),
                message: "no component schemas referenced from operations".into(),
            });
        } else if shared == 0 {
            findings.push(Finding {
                path: "components.schemas".into(),
                message: "component schemas exist but none are shared across 2+ paths".into(),
            });
        }
        rules.push(RuleResult {
            id: "shared-ref-models",
            title: "Shared $ref models used",
            weight: 2.0,
            earned,
            findings,
        });
    }

    // 3. operation-id-unique (2.0)
    {
        let mut seen: std::collections::HashMap<String, usize> = std::collections::HashMap::new();
        let mut dups: std::collections::HashSet<String> = std::collections::HashSet::new();
        for op in &ops {
            if let Some(id) = op.op.get("operationId").and_then(Value::as_str) {
                *seen.entry(id.to_string()).or_insert(0) += 1;
            }
        }
        for (id, n) in &seen {
            if *n > 1 {
                dups.insert(id.clone());
            }
        }
        let with_id = ops.iter().filter(|o| o.op.get("operationId").is_some()).count();
        let good = ops
            .iter()
            .filter(|o| {
                o.op
                    .get("operationId")
                    .and_then(Value::as_str)
                    .is_some_and(|id| !dups.contains(id))
            })
            .count();
        let earned = if with_id == 0 {
            0.0
        } else {
            2.0 * (good as f64) / (ops.len() as f64)
        };
        let findings = dups
            .iter()
            .map(|d| Finding {
                path: "operationId".into(),
                message: format!("duplicate operationId {d}"),
            })
            .collect();
        rules.push(RuleResult {
            id: "operation-id-unique",
            title: "operationId is unique",
            weight: 2.0,
            earned,
            findings,
        });
    }

    // 4. path-params (1.5): template params declared with in=path.
    rules.push(coverage_rule("path-params", "Path template params declared", 1.5, &ops, |op| {
        let declared: Vec<String> = all_params(op, doc)
            .iter()
            .filter(|p| p.get("in").and_then(Value::as_str) == Some("path"))
            .filter_map(|p| p.get("name").and_then(Value::as_str).map(str::to_string))
            .collect();
        let needed = path_template_params(op.path);
        if let Some(miss) = needed.iter().find(|n| !declared.contains(n)) {
            return Some(format!("template param {{{miss}}} not declared"));
        }
        None
    }));

    // 5. operation-id-present (1.0)
    rules.push(coverage_rule(
        "operation-id-present",
        "operationId present",
        1.0,
        &ops,
        |op| {
            if op.op.get("operationId").and_then(Value::as_str).is_some() {
                None
            } else {
                Some("missing operationId".into())
            }
        },
    ));

    // 6. operation-id-naming (1.0): alphanumeric camelCase among ops that have one.
    {
        let mut findings = Vec::new();
        let with_id: Vec<&Op> = ops.iter().filter(|o| o.op.get("operationId").is_some()).collect();
        let good = with_id
            .iter()
            .filter(|o| {
                let id = o.op.get("operationId").and_then(Value::as_str).unwrap();
                let ok = is_camel_alnum(id);
                if !ok {
                    findings.push(Finding {
                        path: op_label(o),
                        message: format!("operationId {id} is not alphanumeric camelCase"),
                    });
                }
                ok
            })
            .count();
        let earned = if with_id.is_empty() {
            0.0
        } else {
            1.0 * (good as f64) / (ops.len() as f64)
        };
        rules.push(RuleResult {
            id: "operation-id-naming",
            title: "operationId alphanumeric camelCase",
            weight: 1.0,
            earned,
            findings,
        });
    }

    // 7. operation-documented (1.0)
    rules.push(coverage_rule(
        "operation-documented",
        "Operation has summary or description",
        1.0,
        &ops,
        |op| {
            let has = ["summary", "description"].iter().any(|k| {
                op.op.get(*k).and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty())
            });
            if has {
                None
            } else {
                Some("no summary or description".into())
            }
        },
    ));

    // 8. request-body-documented (1.0): ops with requestBody need a description somewhere on it.
    rules.push(coverage_rule(
        "request-body-documented",
        "Request bodies described",
        1.0,
        &ops,
        |op| {
            let Some(body) = op.op.get("requestBody") else {
                return None;
            };
            let described = body.get("description").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty())
                || body
                    .get("content")
                    .and_then(Value::as_object)
                    .map(|c| {
                        c.values().any(|m| {
                            m.get("description").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty())
                        })
                    })
                    .unwrap_or(false);
            if described {
                None
            } else {
                Some("requestBody has no description".into())
            }
        },
    ));

    // 9. error-responses (1.0)
    rules.push(coverage_rule(
        "error-responses",
        "Operations declare 4xx/5xx responses",
        1.0,
        &ops,
        |op| {
            let responses = op.op.get("responses").and_then(Value::as_object).map(Vec::from_iter);
            let Some(responses) = responses else {
                return Some("no responses object".into());
            };
            if responses.iter().any(|(c, _)| is_error_response(c)) {
                None
            } else {
                Some("no 4xx/5xx response declared".into())
            }
        },
    ));

    // 10. tags (1.0)
    rules.push(coverage_rule("tags", "Operations tagged", 1.0, &ops, |op| {
        let tagged = op
            .op
            .get("tags")
            .and_then(Value::as_array)
            .is_some_and(|t| !t.is_empty());
        if tagged {
            None
        } else {
            Some("no tags".into())
        }
    }));

    // 11. duplicate-paths (1.0): distinct path keys that collapse to the same template.
    {
        let mut groups: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();
        if let Some(paths) = doc.get("paths").and_then(Value::as_object) {
            for p in paths.keys() {
                groups.entry(normalize_path_template(p)).or_default().push(p.clone());
            }
        }
        let collisions: Vec<&Vec<String>> = groups.values().filter(|g| g.len() > 1).collect();
        let earned = if collisions.is_empty() { 1.0 } else { 0.0 };
        let findings = collisions
            .iter()
            .map(|g| Finding {
                path: g.join(", "),
                message: "path templates collide after parameter normalization".into(),
            })
            .collect();
        rules.push(RuleResult {
            id: "duplicate-paths",
            title: "No duplicate path templates",
            weight: 1.0,
            earned,
            findings,
        });
    }

    // 12. security-declared (1.0): global security, or every op has security (including []).
    {
        let global = doc.get("security").map(|s| !matches!(s, Value::Null)).unwrap_or(false);
        let op_missing: Vec<String> = ops
            .iter()
            .filter(|o| o.op.get("security").is_none())
            .map(op_label)
            .collect();
        let (earned, findings) = if global {
            (1.0, vec![])
        } else if ops.is_empty() {
            (0.0, vec![Finding { path: "root".into(), message: "no security".into() }])
        } else if op_missing.is_empty() {
            (1.0, vec![])
        } else {
            (
                0.0,
                op_missing
                    .into_iter()
                    .map(|p| Finding { path: p, message: "no security and no global security".into() })
                    .collect(),
            )
        };
        rules.push(RuleResult {
            id: "security-declared",
            title: "Security schemes declared",
            weight: 1.0,
            earned,
            findings,
        });
    }

    // 13. params-documented (0.75): every declared parameter has a non-empty description.
    {
        let mut findings = Vec::new();
        let mut total = 0usize;
        let mut good = 0usize;
        for op in &ops {
            for p in all_params(op, doc) {
                total += 1;
                let desc = p.get("description").and_then(Value::as_str).is_some_and(|s| !s.trim().is_empty());
                let name = p.get("name").and_then(Value::as_str).unwrap_or("?");
                if desc {
                    good += 1;
                } else {
                    findings.push(Finding {
                        path: op_label(op),
                        message: format!("parameter {name} has no description"),
                    });
                }
            }
        }
        let earned = if total == 0 { 0.75 } else { 0.75 * (good as f64) / (total as f64) };
        rules.push(RuleResult {
            id: "params-documented",
            title: "Parameters described",
            weight: 0.75,
            earned,
            findings,
        });
    }

    // 14. server-url (0.5)
    {
        let servers = doc.get("servers").and_then(Value::as_array);
        let ok = servers.is_some_and(|s| {
            !s.is_empty()
                && s.iter().any(|u| {
                    u.get("url")
                        .and_then(Value::as_str)
                        .is_some_and(|u| !u.trim().is_empty())
                })
        });
        let findings = if ok {
            vec![]
        } else {
            vec![Finding {
                path: "servers".into(),
                message: "no server url declared at root".into(),
            }]
        };
        rules.push(RuleResult {
            id: "server-url",
            title: "Root server URL declared",
            weight: 0.5,
            earned: if ok { 0.5 } else { 0.0 },
            findings,
        });
    }

    let total_weight: f64 = rules.iter().map(|r| r.weight).sum();
    let total_earned: f64 = rules.iter().map(|r| r.earned).sum();
    let score = (total_earned / total_weight * 10.0 * 10.0).round() / 10.0;
    let band = if score >= 8.0 {
        "Strong"
    } else if score >= 6.0 {
        "Mixed"
    } else {
        "Weak"
    };
    AuditReport { score, band, rules }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLEAN: &str = r##"
openapi: 3.1.0
info:
  title: Mini Petstore
  version: 1.0.0
servers:
  - url: https://api.example.com/v1
security:
  - apiKey: []
components:
  securitySchemes:
    apiKey:
      type: apiKey
      in: header
      name: X-API-Key
  schemas:
    Pet:
      type: object
      properties:
        id:
          type: integer
        name:
          type: string
paths:
  /pets:
    get:
      operationId: listPets
      summary: List pets
      description: Returns a page of pets.
      tags: [pets]
      parameters:
        - name: limit
          in: query
          description: Max items
          schema:
            type: integer
      responses:
        "200":
          description: ok
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "400":
          description: bad request
    post:
      operationId: createPet
      summary: Create pet
      description: Creates a pet.
      tags: [pets]
      requestBody:
        description: pet payload
        content:
          application/json:
            schema:
              $ref: "#/components/schemas/Pet"
      responses:
        "201":
          description: created
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "500":
          description: server error
  /pets/{petId}:
    get:
      operationId: getPet
      summary: Get pet
      description: Fetch one pet.
      tags: [pets]
      parameters:
        - name: petId
          in: path
          description: Pet id
          required: true
          schema:
            type: integer
      responses:
        "200":
          description: ok
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "404":
          description: not found
    delete:
      operationId: deletePet
      summary: Delete pet
      description: Removes a pet.
      tags: [pets]
      parameters:
        - name: petId
          in: path
          description: Pet id
          required: true
          schema:
            type: integer
      responses:
        "204":
          description: deleted
        "404":
          description: not found
"##;

    #[test]
    fn clean_spec_scores_ten() {
        let doc = parse_spec(CLEAN.as_bytes()).unwrap();
        let report = audit(&doc);
        assert_eq!(report.score, 10.0, "rules: {:?}", report.rules);
        assert_eq!(report.band, "Strong");
    }

    #[test]
    fn missing_success_schema_deducts() {
        let bad = CLEAN.replace(
            r##""200":
          description: ok
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "404":
          description: not found"##,
            r##""200":
          description: ok
        "404":
          description: not found"##,
        );
        let doc = parse_spec(bad.as_bytes()).unwrap();
        let report = audit(&doc);
        assert!(report.score < 10.0);
        let rule = report.rules.iter().find(|r| r.id == "response-schema").unwrap();
        assert!(rule.earned < rule.weight);
    }

    #[test]
    fn response_204_exempt_from_response_schema() {
        let doc = parse_spec(CLEAN.as_bytes()).unwrap();
        let report = audit(&doc);
        let rule = report.rules.iter().find(|r| r.id == "response-schema").unwrap();
        assert_eq!(rule.earned, rule.weight);
        assert!(rule.findings.is_empty());
    }

    #[test]
    fn swagger2_refused() {
        let err = parse_spec(b"swagger: \"2.0\"\npaths: {}").unwrap_err();
        assert!(matches!(err, AuditError::Swagger2));
    }

    #[test]
    fn oversize_rejected() {
        let bytes = vec![b' '; SIZE_LIMIT_BYTES + 1];
        let err = parse_spec(&bytes).unwrap_err();
        assert!(matches!(err, AuditError::TooLarge(_)));
    }

    #[test]
    fn duplicate_operation_id_fails_uniqueness() {
        let bad = CLEAN.replace("operationId: createPet", "operationId: listPets");
        let doc = parse_spec(bad.as_bytes()).unwrap();
        let report = audit(&doc);
        let rule = report.rules.iter().find(|r| r.id == "operation-id-unique").unwrap();
        assert!(rule.earned < rule.weight);
        assert!(report.score < 10.0);
    }

    #[test]
    fn duplicate_path_templates_detected() {
        let spec = r##"
openapi: 3.1.0
info:
  title: t
  version: "1"
paths:
  /a/{x}:
    get:
      operationId: getA
      summary: s
      description: d
      tags: [t]
      responses:
        "204":
          description: ok
  /a/{y}:
    get:
      operationId: getB
      summary: s
      description: d
      tags: [t]
      responses:
        "204":
          description: ok
"##;
        let doc = parse_spec(spec.as_bytes()).unwrap();
        let report = audit(&doc);
        let rule = report.rules.iter().find(|r| r.id == "duplicate-paths").unwrap();
        assert_eq!(rule.earned, 0.0);
    }
}

