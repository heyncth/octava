mod audit;
mod docs;

use std::process::ExitCode;

fn usage() -> ExitCode {
    eprintln!("octava audit <openapi-file>");
    eprintln!("octava docs <openapi-file> -o <out-dir>");
    eprintln!("Self-hosted OpenAPI toolchain. Free, no account, no phone-home.");
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("audit") | None => run_audit(args.get(1)),
        Some("docs") => {
            let path = args.get(1);
            let out = args.iter().position(|a| a == "-o").and_then(|i| args.get(i + 1));
            match (path, out) {
                (Some(p), Some(o)) => run_docs(p, o),
                _ => usage(),
            }
        }
        Some("-h") | Some("--help") | Some("help") => usage(),
        Some(other) => {
            eprintln!("error: unknown command {other}");
            usage()
        }
    }
}

fn run_audit(path: Option<&String>) -> ExitCode {
    let Some(path) = path else {
        return usage();
    };
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::from(2);
        }
    };
    let doc = match audit::parse_spec(&bytes) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    let report = audit::audit(&doc);
    println!("{path}");
    println!("score: {:.1}/10  band: {}", report.score, report.band);
    println!();
    for rule in &report.rules {
        let status = if rule.findings.is_empty() { "PASS" } else { "FAIL" };
        println!(
            "[{status}] {:<24} {:>4.2}/{:.2}  {}",
            rule.id, rule.earned, rule.weight, rule.title
        );
        for f in &rule.findings {
            println!("       - {}: {}", f.path, f.message);
        }
    }
    ExitCode::SUCCESS
}

fn run_docs(path: &str, out: &str) -> ExitCode {
    let bytes = match std::fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("error: cannot read {path}: {e}");
            return ExitCode::from(2);
        }
    };
    let doc = match audit::parse_spec(&bytes) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {e}");
            return ExitCode::from(2);
        }
    };
    match docs::render(&doc, std::path::Path::new(out)) {
        Ok(()) => {
            println!("wrote {out}/index.html and {out}/llms.txt");
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("error: cannot write {out}: {e}");
            ExitCode::from(2)
        }
    }
}
