//! Fail-closed Rust gate for newly added first-party Python scripts and CI invocations.
//! Usage: rustc --edition=2021 scripts/deny_new_python.rs -o deny_new_python
//!        ./deny_new_python BASE_SHA HEAD_SHA
use std::{env, process::{Command, ExitCode}};

fn is_banned_added_path(path: &str) -> bool {
    if !path.ends_with(".py") { return false; }
    let parts: Vec<&str> = path.split('/').collect();
    parts.len() == 1 || parts.iter().any(|part| {
        matches!(*part, "scripts" | "tests" | "test" | "tools" | ".github" | "ci")
    })
}

fn invokes_python(line: &str) -> bool {
    let trimmed = line.trim_start();
    if trimmed.starts_with('#') || trimmed.starts_with("//") { return false; }
    let normalized = format!(" {trimmed}");
    [
        " python ", " python3 ", " python -", " python3 -",
        " pytest", " unittest", " actions/setup-python@",
        " poetry run ", " pipenv run ",
    ].iter().any(|fragment| normalized.contains(fragment))
}

fn sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn git_diff(args: &[&str]) -> Result<String, String> {
    let command = Command::new("git").args(args).output()
        .map_err(|error| format!("failed to execute git: {error}"))?;
    if !command.status.success() {
        return Err(format!("git diff failed with exit {}", command.status));
    }
    String::from_utf8(command.stdout).map_err(|error| format!("git output not UTF-8: {error}"))
}

fn scan(base: &str, head: &str) -> Result<Vec<String>, String> {
    if !sha(base) || !sha(head) {
        return Err("both revisions must be complete immutable Git SHAs".to_owned());
    }
    let added = git_diff(&["diff", "--name-only", "--diff-filter=A", base, head])?;
    let mut problems = added.lines()
        .filter(|path| is_banned_added_path(path))
        .map(|path| format!("new Python script/test: {path}"))
        .collect::<Vec<_>>();

    // New invocations in workflow and shell source are also disallowed, even
    // when no .py file was introduced by the same change.
    let patch = git_diff(&[
        "diff", "--no-ext-diff", "--unified=0", base, head, "--",
        ".github/workflows", "scripts", "tools",
    ])?;
    let mut in_execution_file = false;
    for line in patch.lines() {
        if let Some(path) = line.strip_prefix("+++ b/") {
            in_execution_file = path.ends_with(".yml") || path.ends_with(".yaml")
                || path.ends_with(".sh") || path.ends_with("Makefile")
                || path.ends_with(".toml");
        } else if line.starts_with("+++ /dev/null") {
            in_execution_file = false;
        } else if in_execution_file && line.starts_with('+') && !line.starts_with("+++") {
            if invokes_python(&line[1..]) {
                problems.push(format!("new Python CI invocation: {}", &line[1..]));
            }
        }
    }
    Ok(problems)
}

fn main() -> ExitCode {
    let args = env::args().collect::<Vec<_>>();
    if args.len() != 3 {
        eprintln!("usage: deny_new_python BASE_SHA HEAD_SHA");
        return ExitCode::FAILURE;
    }
    match scan(&args[1], &args[2]) {
        Ok(problems) if problems.is_empty() => {
            println!("Rust-first policy: no new Python automation detected");
            ExitCode::SUCCESS
        }
        Ok(problems) => {
            for problem in problems { eprintln!("{problem}"); }
            ExitCode::FAILURE
        }
        Err(message) => {
            eprintln!("{message}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flags_new_python_scripts_but_not_product_code() {
        assert!(is_banned_added_path("scripts/generate.py"));
        assert!(is_banned_added_path("tests/schema_check.py"));
        assert!(is_banned_added_path("tool.py"));
        assert!(!is_banned_added_path("src/application.py"));
        assert!(!is_banned_added_path("scripts/build.rs"));
    }

    #[test]
    fn rejects_inline_python_runner_and_detects_safe_rust() {
        assert!(invokes_python("run: python3 tests/validate.py"));
        assert!(invokes_python("run: python -m unittest"));
        assert!(invokes_python("uses: actions/setup-python@abcdef"));
        assert!(!invokes_python("run: cargo test --locked"));
        assert!(!invokes_python("# python3 old command"));
        assert!(!invokes_python("run: rustc --test scripts/check.rs"));
    }

    #[test]
    fn rejects_short_or_malformed_revision_identifiers() {
        assert!(!sha("HEAD"));
        assert!(!sha("01234"));
        assert!(!sha(&"z".repeat(40)));
        assert!(sha(&"f".repeat(40)));
    }
}
