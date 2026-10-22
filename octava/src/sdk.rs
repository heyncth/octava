//! SDK generator: TypeScript and Python clients from OpenAPI 3.x.
//! Generated code is stdlib-only (fetch / urllib), retry on 429/5xx.

use crate::audit::{collect_ops, Op};
use serde_json::Value;
use std::fmt::Write as _;
use std::path::Path;

pub fn generate(doc: &Value, lang: &str, out_dir: &Path) -> std::io::Result<String> {
    std::fs::create_dir_all(out_dir)?;
    let (name, code) = match lang {
        "ts" | "typescript" => ("client.ts", typescript::generate(doc)),
        "py" | "python" => ("client.py", python::generate(doc)),
        other => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("unknown language {other}, use ts or python"),
            ))
        }
    };
    let path = out_dir.join(name);
    std::fs::write(&path, &code)?;
    Ok(path.display().to_string())
}

fn op_name(op: &Op) -> String {
    op.op
        .get("operationId")
        .and_then(Value::as_str)
        .map(str::to_string)
        .unwrap_or_else(|| format!("{}{}", op.method, op.path.replace(['{', '}', '/'], "_")))
}

fn snake(s: &str) -> String {
    let mut out = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                out.push('_');
            }
            out.extend(c.to_lowercase());
        } else {
            out.push(c);
        }
    }
    out
}

fn schema_ref(schema: &Value) -> Option<String> {
    schema
        .get("$ref")
        .and_then(Value::as_str)?
        .strip_prefix("#/components/schemas/")
        .map(str::to_string)
}

fn op_params<'a>(doc: &'a Value, op: &'a Op<'a>) -> Vec<&'a Value> {
    let mut out: Vec<&'a Value> = op
        .op
        .get("parameters")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    out.extend(crate::audit::path_level_parameters(doc, op.path));
    out
}

/// First2xx (non-204) response schema name, or primitive type.
fn success_type(op: &Op) -> (String, bool) {
    let Some(responses) = op.op.get("responses").and_then(Value::as_object) else {
        return ("void".into(), false);
    };
    for (code, r) in responses {
        let Ok(c) = code.parse::<u16>() else { continue };
        if !(200..300).contains(&c) || c == 204 || c == 205 {
            continue;
        }
        if let Some(schema) = r
            .get("content")
            .and_then(Value::as_object)
            .and_then(|c| c.values().next())
            .and_then(|m| m.get("schema"))
        {
            if let Some(name) = schema_ref(schema) {
                return (name, true);
            }
            if let Some(t) = schema.get("type").and_then(Value::as_str) {
                let mapped = match t {
                    "string" => "string",
                    "integer" | "number" => "number",
                    "boolean" => "boolean",
                    "array" => "unknown[]",
                    _ => "unknown",
                };
                return (mapped.into(), true);
            }
        }
        return ("unknown".into(), true);
    }
    ("void".into(), false)
}

fn has_request_body(op: &Op) -> bool {
    op.op.get("requestBody").is_some()
}

/// Detect auth wiring: returns (api_key_header, uses_bearer).
fn auth_shape(doc: &Value) -> (Option<String>, bool) {
    let schemes = doc
        .get("components")
        .and_then(|c| c.get("securitySchemes"))
        .and_then(Value::as_object);
    let Some(schemes) = schemes else {
        return (None, false);
    };
    let mut header_key = None;
    let mut bearer = false;
    for (_, s) in schemes {
        match s.get("type").and_then(Value::as_str) {
            Some("apiKey") if s.get("in").and_then(Value::as_str) == Some("header") => {
                if header_key.is_none() {
                    header_key = s.get("name").and_then(Value::as_str).map(str::to_string);
                }
            }
            Some("http") if s.get("scheme").and_then(Value::as_str) == Some("bearer") => bearer = true,
            _ => {}
        }
    }
    (header_key, bearer)
}

mod typescript {
    use super::*;

    fn ts_type(schema: &Value) -> String {
        if let Some(name) = schema_ref(schema) {
            return name;
        }
        match schema.get("type").and_then(Value::as_str) {
            Some("string") => "string".into(),
            Some("integer") | Some("number") => "number".into(),
            Some("boolean") => "boolean".into(),
            Some("array") => {
                let inner = schema.get("items").map(ts_type).unwrap_or_else(|| "unknown".into());
                format!("{inner}[]")
            }
            Some("object") => "Record<string, unknown>".into(),
            _ => "unknown".into(),
        }
    }

    fn interface(name: &str, schema: &Value) -> String {
        let props = schema
            .get("properties")
            .and_then(Value::as_object)
            .map(|m| {
                m.iter()
                    .map(|(pn, ps)| {
                        let req = schema
                            .get("required")
                            .and_then(Value::as_array)
                            .is_some_and(|r| r.iter().any(|x| x.as_str() == Some(pn.as_str())));
                        format!("  {}{}: {};", pn, if req { "" } else { "?" }, ts_type(ps))
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            })
            .unwrap_or_default();
        format!("export interface {name} {{\n{props}\n}}")
    }

    pub fn generate(doc: &Value) -> String {
        let title = doc
            .get("info")
            .and_then(|i| i.get("title"))
            .and_then(Value::as_str)
            .unwrap_or("API");
        let mut interfaces = String::new();
        if let Some(schemas) = doc
            .get("components")
            .and_then(|c| c.get("schemas"))
            .and_then(Value::as_object)
        {
            for (name, s) in schemas {
                let _ = writeln!(interfaces, "{}\n", interface(name, s));
            }
        }

        let (api_key_header, bearer) = auth_shape(doc);
        let mut methods = String::new();
        for op in collect_ops(doc) {
            let name = op_name(&op);
            let (ret, returns_value) = success_type(&op);
            let mut params: Vec<String> = Vec::new();
            for p in op_params(doc, &op) {
                let pname = p.get("name").and_then(Value::as_str).unwrap_or("param");
                let ty = p
                    .get("schema")
                    .map(ts_type)
                    .unwrap_or_else(|| "unknown".into());
                let required = p.get("required").and_then(Value::as_bool).unwrap_or(false);
                let qname = if required { pname.to_string() } else { format!("{pname}?") };
                // path params must not be optional
                let qname = if p.get("in").and_then(Value::as_str) == Some("path") {
                    pname.to_string()
                } else {
                    qname
                };
                params.push(format!("{qname}: {ty}"));
            }
            if has_request_body(&op) {
                params.push("body: unknown".into());
            }
            let path_expr = op.path.split('/').map(|seg| {
                if let Some(pname) = seg.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                    format!("encodeURIComponent(String({pname}))")
                } else {
                    format!("\"{seg}\"")
                }
            })
            .collect::<Vec<_>>()
            .join(" + \"/\" + ");
            let query: String = op_params(doc, &op)
                .iter()
                .filter(|p| p.get("in").and_then(Value::as_str) == Some("query"))
                .map(|p| {
                    let pname = p.get("name").and_then(Value::as_str).unwrap();
                    format!(
                        "    if ({pname} !== undefined) url.searchParams.set(\"{pname}\", String({pname}));\n"
                    )
                })
                .collect();
            let auth = {
                let mut a = String::new();
                if let Some(h) = &api_key_header {
                    let _ = writeln!(a, "    if (this.apiKey) headers[\"{h}\"] = this.apiKey;");
                }
                if bearer {
                    let _ = writeln!(a, "    if (this.token) headers[\"Authorization\"] = `Bearer ${{this.token}}`;");
                }
                a
            };
            let body_arg = if has_request_body(&op) { ", body: JSON.stringify(body)" } else { "" };
            let body_headers = if has_request_body(&op) {
                "    headers[\"Content-Type\"] = \"application/json\";\n"
            } else {
                ""
            };
            let return_stmt = if returns_value {
                "    return await res.json() as RET;"
            } else {
                "    return undefined as unknown as RET;"
            };
            let _ = writeln!(
                methods,
                r#"  async {name}({params}): Promise<{ret}> {{
    const url = new URL(this.baseUrl + "/" + {path_expr});
{query}    const headers: Record<string, string> = {{ "Accept": "application/json" }};
{body_headers}{auth}    let res: Response | undefined;
    for (let attempt = 0; attempt < 3; attempt++) {{
      res = await this.fetchImpl(url.toString(), {{ method: "{METHOD}", headers{body_arg} }});
      if (res.status === 429 || res.status >= 500) {{
        await new Promise((r) => setTimeout(r, 250 * 2 ** attempt));
        continue;
      }}
      break;
    }}
    if (!res) throw new Error("request failed");
    if (res.status === 204 || res.status === 205) return undefined as unknown as {ret};
    if (!res.ok) throw new Error(`HTTP ${{res.status}}: ${{await res.text()}}`);
{return_stmt}
  }}
"#,
                params = params.join(", "),
                ret = ret,
                METHOD = op.method.to_uppercase(),
            );
            // fix RET placeholder
            methods = methods.replace("Promise<RET>", &format!("Promise<{ret}>"));
            methods = methods.replace("as RET;", &format!("as {ret};"));
        }

        let api_key_field = if api_key_header.is_some() { "\n  apiKey?: string;" } else { "" };
        let token_field = if bearer { "\n  token?: string;" } else { "" };
        let api_key_init = if api_key_header.is_some() { "\n    this.apiKey = opts.apiKey;" } else { "" };
        let token_init = if bearer { "\n    this.token = opts.token;" } else { "" };

        format!(
            r#"// Generated by Octava. Your code, your license.
{interfaces}
export interface ApiClientOptions {{
  baseUrl: string;{api_key_field}{token_field}
  fetchImpl?: typeof fetch;
}}

export class ApiClient {{
  private baseUrl: string;{api_key_field}{token_field}
  private fetchImpl: typeof fetch;

  constructor(opts: ApiClientOptions) {{
    this.baseUrl = opts.baseUrl.replace(/\/$/, "");{api_key_init}{token_init}
    this.fetchImpl = opts.fetchImpl ?? fetch;
  }}
{methods}}}
"#,
            api_key_field = if api_key_field.is_empty() { String::new() } else { "  apiKey?: string;".into() },
            token_field = if token_field.is_empty() { String::new() } else { "  token?: string;".into() },
        )
    }
}

mod python {
    use super::*;

    fn py_type(schema: &Value) -> String {
        if let Some(name) = schema_ref(schema) {
            return format!("dict[str, object]  # {name}");
        }
        match schema.get("type").and_then(Value::as_str) {
            Some("string") => "str".into(),
            Some("integer") => "int".into(),
            Some("number") => "float".into(),
            Some("boolean") => "bool".into(),
            Some("array") => {
                let inner = schema.get("items").map(py_type).unwrap_or_else(|| "object".into());
                format!("list[{inner}]")
            }
            _ => "object".into(),
        }
    }

    pub fn generate(doc: &Value) -> String {
        let (api_key_header, bearer) = auth_shape(doc);
        let mut methods = String::new();
        for op in collect_ops(doc) {
            let name = snake(&op_name(&op));
            // shortcut: required params must precede optional in Python signatures
            let mut required_params: Vec<String> = Vec::new();
            let mut optional_params: Vec<String> = Vec::new();
            for p in op_params(doc, &op) {
                let pname = snake(p.get("name").and_then(Value::as_str).unwrap_or("param"));
                let ty = p.get("schema").map(py_type).unwrap_or_else(|| "object".into());
                let required = p.get("required").and_then(Value::as_bool).unwrap_or(false)
                    || p.get("in").and_then(Value::as_str) == Some("path");
                if required {
                    required_params.push(format!("{pname}: {ty}"));
                } else {
                    optional_params.push(format!("{pname}: {ty} | None = None"));
                }
            }
            let mut params: Vec<String> = required_params;
            params.extend(optional_params);
            if has_request_body(&op) {
                if !params.is_empty() {
                    params.push(", ".into());
                }
                params.push("body: object | None = None".into());
            }
            let mut path_f = String::new();
            for seg in op.path.split('/').filter(|s| !s.is_empty()) {
                if let Some(p) = seg.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                    let _ = write!(path_f, "/{{{}}}", snake(p));
                } else {
                    let _ = write!(path_f, "/{seg}");
                }
            }
            let path_build = format!("f\"{path_f}\"");
            let query: String = op_params(doc, &op)
                .iter()
                .filter(|p| p.get("in").and_then(Value::as_str) == Some("query"))
                .map(|p| {
                    let pname = snake(p.get("name").and_then(Value::as_str).unwrap());
                    format!(
                        "        if {pname} is not None:\n            url += (\"&\" if \"?\" in url else \"?\") + \"{pname}=\" + str({pname})\n"
                    )
                })
                .collect();
            let auth = {
                let mut a = String::new();
                if let Some(h) = &api_key_header {
                    let _ = writeln!(a, "        if self.api_key:\n            headers[\"{h}\"] = self.api_key");
                }
                if bearer {
                    let _ = writeln!(a, "        if self.token:\n            headers[\"Authorization\"] = \"Bearer \" + self.token");
                }
                a
            };
            let body_line = if has_request_body(&op) {
                "        data = json.dumps(body).encode() if body is not None else None\n        if data is not None:\n            headers[\"Content-Type\"] = \"application/json\"\n"
            } else {
                ""
            };
            let params_str = if params.is_empty() {
                String::new()
            } else {
                format!(", {}", params.concat())
            };
            let _ = writeln!(
                methods,
                r#"    def {name}(self{params_str}) -> object | None:
        url = self.base_url + {path_build}
{query}        headers = {{"Accept": "application/json"}}
{body_line}{auth}        last_exc: Exception | None = None
        for attempt in range(3):
            req = urllib.request.Request(url, headers=headers, method="{METHOD}", data=locals().get("data"))
            try:
                with urllib.request.urlopen(req, timeout=30) as res:
                    if res.status == 204 or res.status == 205:
                        return None
                    return json.loads(res.read())
            except urllib.error.HTTPError as e:
                if e.code == 429 or e.code >= 500:
                    last_exc = e
                    time.sleep(0.25 * (2 ** attempt))
                    continue
                raise
        raise last_exc if last_exc else RuntimeError("request failed")
"#,
                METHOD = op.method.to_uppercase(),
            );
        }

        let api_key_field = if api_key_header.is_some() {
            "        self.api_key = api_key\n"
        } else {
            ""
        };
        let token_field = if bearer { "        self.token = token\n" } else { "" };
        let api_key_sig = if api_key_header.is_some() { ", api_key: str | None = None" } else { "" };
        let token_sig = if bearer { ", token: str | None = None" } else { "" };

        format!(
            r#"# Generated by Octava. Your code, your license.
from __future__ import annotations

import json
import time
import urllib.error
import urllib.request


class ApiClient:
    def __init__(self, base_url: str{api_key_sig}{token_sig}) -> None:
        self.base_url = base_url.rstrip("/")
{api_key_field}{token_field}
{methods}"#
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SPEC: &str = r##"
openapi: 3.1.0
info:
  title: Mini
  version: 1.0.0
servers:
  - url: https://api.example.com
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
      required: [id, name]
      properties:
        id: {type: integer}
        name: {type: string}
paths:
  /pets:
    get:
      operationId: listPets
      summary: List
      parameters:
        - name: limit
          in: query
          description: max
          schema: {type: integer}
      responses:
        "200":
          description: ok
          content:
            application/json:
              schema:
                type: array
                items:
                  $ref: "#/components/schemas/Pet"
        "400":
          description: bad
    post:
      operationId: createPet
      summary: Create
      requestBody:
        description: pet
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
          description: err
  /pets/{petId}:
    get:
      operationId: getPet
      summary: Get
      parameters:
        - name: petId
          in: path
          required: true
          description: id
          schema: {type: integer}
      responses:
        "200":
          description: ok
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
        "404":
          description: nf
"##;

    #[test]
    fn typescript_client_has_methods_and_retry() {
        let doc = crate::audit::parse_spec(SPEC.as_bytes()).unwrap();
        let ts = typescript::generate(&doc);
        assert!(ts.contains("export interface Pet"));
        assert!(ts.contains("async listPets"));
        assert!(ts.contains("async createPet"));
        assert!(ts.contains("async getPet"));
        assert!(ts.contains("attempt < 3"));
        assert!(ts.contains("res.status === 429"));
        assert!(ts.contains("X-API-Key"));
        assert!(ts.contains("encodeURIComponent(String(petId))"));
    }

    #[test]
    fn python_client_has_methods_and_retry() {
        let doc = crate::audit::parse_spec(SPEC.as_bytes()).unwrap();
        let py = python::generate(&doc);
        assert!(py.contains("def list_pets"));
        assert!(py.contains("def create_pet"));
        assert!(py.contains("def get_pet(self, pet_id: int)"));
        assert!(py.contains("def list_pets(self, limit: int | None = None)"));
        assert!(py.contains("range(3)"));
        assert!(py.contains("429"));
        assert!(py.contains("X-API-Key"));
        assert!(py.contains("f\"/pets/{pet_id}\""));
    }

    #[test]
    fn generate_writes_files() {
        let doc = crate::audit::parse_spec(SPEC.as_bytes()).unwrap();
        let dir = std::env::temp_dir().join(format!("octava-sdk-{}", std::process::id()));
        let p = generate(&doc, "ts", &dir).unwrap();
        assert!(p.ends_with("client.ts"));
        let p = generate(&doc, "python", &dir).unwrap();
        assert!(p.ends_with("client.py"));
        let err = generate(&doc, "rust", &dir).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidInput);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
