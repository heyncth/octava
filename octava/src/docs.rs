//! Static docs site generator: one index.html + llms.txt, no outbound calls.

use crate::audit::{collect_ops, path_level_parameters};
use serde_json::Value;
use std::fmt::Write as _;
use std::path::Path;

pub fn render(doc: &Value, out_dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(out_dir)?;
    std::fs::write(out_dir.join("index.html"), render_index(doc))?;
    std::fs::write(out_dir.join("llms.txt"), render_llms(doc))?;
    Ok(())
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

fn info(doc: &Value, key: &str) -> String {
    doc.get("info")
        .and_then(|i| i.get(key))
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string()
}

fn schema_ref_name(schema: &Value) -> Option<String> {
    schema
        .get("$ref")
        .and_then(Value::as_str)?
        .strip_prefix("#/components/schemas/")
        .map(str::to_string)
}

fn op_params<'a>(doc: &'a Value, path: &str, op: &'a Value) -> Vec<&'a Value> {
    let mut out: Vec<&'a Value> = op
        .get("parameters")
        .and_then(Value::as_array)
        .map(|a| a.iter().collect())
        .unwrap_or_default();
    out.extend(path_level_parameters(doc, path));
    out
}

fn param_row(p: &Value) -> String {
    let name = p.get("name").and_then(Value::as_str).unwrap_or("?");
    let loc = p.get("in").and_then(Value::as_str).unwrap_or("");
    let req = p.get("required").and_then(Value::as_bool).unwrap_or(false);
    let desc = p.get("description").and_then(Value::as_str).unwrap_or("");
    let ty = p
        .get("schema")
        .and_then(|s| s.get("type"))
        .and_then(Value::as_str)
        .unwrap_or("");
    format!(
        "<tr><td><code>{}</code></td><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
        esc(name),
        esc(loc),
        if req { "required" } else { "optional" },
        esc(ty),
        esc(desc)
    )
}

fn response_rows(op: &Value) -> String {
    let Some(responses) = op.get("responses").and_then(Value::as_object) else {
        return "<tr><td colspan=3>none</td></tr>".into();
    };
    responses
        .iter()
        .map(|(code, r)| {
            let desc = r.get("description").and_then(Value::as_str).unwrap_or("");
            let schema = r
                .get("content")
                .and_then(Value::as_object)
                .and_then(|c| c.values().next())
                .and_then(|m| m.get("schema"))
                .and_then(|s| {
                    schema_ref_name(s)
                        .map(|n| format!("<code>{n}</code>"))
                        .or_else(|| s.get("type").and_then(Value::as_str).map(|t| format!("<code>{t}</code>")))
                })
                .unwrap_or_default();
            format!("<tr><td><code>{}</code></td><td>{}</td><td>{}</td></tr>", esc(code), schema, esc(desc))
        })
        .collect()
}

fn render_index(doc: &Value) -> String {
    let title = info(doc, "title");
    let version = info(doc, "version");
    let description = info(doc, "description");
    let ops = collect_ops(doc);

    let servers = doc
        .get("servers")
        .and_then(Value::as_array)
        .map(|s| {
            s.iter()
                .filter_map(|u| u.get("url").and_then(Value::as_str))
                .map(|u| format!("<li><code>{}</code></li>", esc(u)))
                .collect::<String>()
        })
        .unwrap_or_default();

    let schemas = doc
        .get("components")
        .and_then(|c| c.get("schemas"))
        .and_then(Value::as_object)
        .map(|m| {
            m.iter()
                .map(|(name, s)| {
                    let props = s
                        .get("properties")
                        .and_then(Value::as_object)
                        .map(|p| {
                            p.iter()
                                .map(|(pn, ps)| {
                                    let ty = ps
                                        .get("type")
                                        .and_then(Value::as_str)
                                        .or_else(|| ps.get("$ref").and_then(Value::as_str))
                                        .unwrap_or("any");
                                    format!("<li><code>{}</code>: <code>{}</code></li>", esc(pn), esc(ty))
                                })
                                .collect::<String>()
                        })
                        .unwrap_or_default();
                    format!("<section class=schema><h3><code>{}</code></h3><ul>{}</ul></section>", esc(name), props)
                })
                .collect::<String>()
        })
        .unwrap_or_default();

    let mut ops_html = String::new();
    let mut last_path = String::new();
    for op in &ops {
        if op.path != last_path {
            let _ = writeln!(ops_html, "<h3 class=path><code>{}</code></h3>", esc(op.path));
            last_path = op.path.to_string();
        }
        let summary = op.op.get("summary").and_then(Value::as_str).unwrap_or("");
        let description = op.op.get("description").and_then(Value::as_str).unwrap_or("");
        let params: String = op_params(doc, op.path, op.op).iter().map(|p| param_row(p)).collect();
        let params_block = if params.is_empty() {
            String::new()
        } else {
            format!("<table><tr><th>Name</th><th>In</th><th>Required</th><th>Type</th><th>Description</th></tr>{params}</table>")
        };
        let body = op
            .op
            .get("requestBody")
            .and_then(|b| b.get("description").and_then(Value::as_str))
            .map(|d| format!("<p><strong>Request body:</strong> {}</p>", esc(d)))
            .unwrap_or_default();
        let _ = writeln!(
            ops_html,
            r#"<section class=op id="{id}">
<h4><span class=method>{method}</span> {summary}</h4>
<p>{description}</p>
{params_block}{body}
<table><tr><th>Status</th><th>Schema</th><th>Description</th></tr>{responses}</table>
</section>"#,
            id = esc(&format!("{}-{}", op.method, op.path.replace(['{', '}'], ""))),
            method = esc(&op.method.to_uppercase()),
            summary = esc(summary),
            description = esc(description),
            responses = response_rows(op.op),
        );
    }

    format!(
        r##"<!doctype html>
<html lang=en>
<head>
<meta charset=utf-8>
<meta name=viewport content="width=device-width,initial-scale=1">
<title>{title} API reference</title>
<style>
:root {{ color-scheme: light dark; font-family: ui-sans-serif, system-ui, sans-serif; }}
body {{ margin: 0 auto; max-width: 56rem; padding: 1.5rem; line-height: 1.5; }}
h1 {{ margin-bottom: .25rem; }}
.meta {{ opacity: .7; margin-top: 0; }}
.method {{ display: inline-block; padding: .1rem .45rem; border-radius: .3rem; background: #4a5568; color: #fff; font-size: .75rem; }}
.op {{ border: 1px solid rgba(128,128,128,.35); border-radius: .5rem; padding: .75rem 1rem; margin: .75rem 0; }}
table {{ border-collapse: collapse; width: 100%; font-size: .9rem; }}
th, td {{ text-align: left; padding: .3rem .5rem; border-bottom: 1px solid rgba(128,128,128,.25); }}
.path {{ margin-top: 1.5rem; }}
nav a {{ margin-right: .75rem; }}
</style>
</head>
<body>
<nav><a href="#operations">Operations</a><a href="#schemas">Schemas</a></nav>
<h1>{title}</h1>
<p class=meta>version {version}</p>
<p>{description}</p>
{servers_block}
<h2 id=operations>Operations</h2>
{ops_html}
<h2 id=schemas>Schemas</h2>
{schemas}
</body>
</html>
"##,
        title = esc(&title),
        version = esc(&version),
        description = esc(&description),
        servers_block = if servers.is_empty() { String::new() } else { format!("<h2>Servers</h2><ul>{servers}</ul>") },
    )
}

fn render_llms(doc: &Value) -> String {
    let title = info(doc, "title");
    let version = info(doc, "version");
    let mut out = format!("# {title} API (version {version})\n\n## Operations\n\n");
    for op in collect_ops(doc) {
        let summary = op.op.get("summary").and_then(Value::as_str).unwrap_or("");
        let id = op.op.get("operationId").and_then(Value::as_str).unwrap_or("");
        let _ = writeln!(out, "- `{} {}` ({}) - {}", op.method.to_uppercase(), op.path, id, summary);
    }
    out
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
paths:
  /pets:
    get:
      operationId: listPets
      summary: List pets
      description: Returns pets.
      tags: [pets]
      parameters:
        - name: limit
          in: query
          description: Max
          schema: {type: integer}
      responses:
        "200":
          description: ok
          content:
            application/json:
              schema:
                $ref: "#/components/schemas/Pet"
components:
  schemas:
    Pet:
      type: object
      properties:
        id: {type: integer}
        name: {type: string}
"##;

    #[test]
    fn renders_index_and_llms() {
        let doc = crate::audit::parse_spec(SPEC.as_bytes()).unwrap();
        let dir = std::env::temp_dir().join(format!("octava-docs-{}", std::process::id()));
        render(&doc, &dir).unwrap();
        let html = std::fs::read_to_string(dir.join("index.html")).unwrap();
        assert!(html.contains("Mini"));
        assert!(html.contains("GET"));
        assert!(html.contains("/pets"));
        assert!(html.contains("Pet"));
        let llms = std::fs::read_to_string(dir.join("llms.txt")).unwrap();
        assert!(llms.contains("listPets"));
        let _ = std::fs::remove_dir_all(&dir);
    }
}
