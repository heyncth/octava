mod audit;

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let path = match args.next() {
        Some(p) if p != "-h" && p != "--help" && p != "help" => p,
        _ => {
            eprintln!("octava <openapi-file>");
            eprintln!("Self-hosted OpenAPI readiness audit. Score /10, 14 weighted rules.");
            return ExitCode::from(2);
        }
    };

    let bytes = match std::fs::read(&path) {
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
