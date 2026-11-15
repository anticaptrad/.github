//! Offline Rust-only test for the organization's first-party scripting policy.

const POLICY: &str = include_str!("../policies/RUST_FIRST_TESTING.md");
const CONTRIBUTING: &str = include_str!("../CONTRIBUTING.md");
const README: &str = include_str!("../README.md");

fn policy_errors(policy: &str) -> Vec<&'static str> {
    let required = [
        ("Write new test tools and automation in Rust, not Python", "Rust default"),
        ("cargo test --locked", "Cargo test"),
        ("rustc --test", "Rust harness"),
        ("Do not introduce Python .py scripts", "no new Python"),
        ("negative cases", "negative test coverage"),
        ("do not delete blindly", "safe migration"),
        ("Run both implementations over the same fixtures", "dual-run parity"),
        ("matching failures", "failure parity"),
        ("exact-head", "exact head evidence"),
        ("source string check as executable compiler evidence", "compiler evidence"),
        ("verify every blob SHA", "carrier exact blobs"),
        ("Do not transfer secrets", "no secrets"),
        ("not automatically inherited as a workflow policy", "adoption required"),
    ];
    required.iter().filter_map(|(term, reason)| {
        if policy.contains(term) { None } else { Some(*reason) }
    }).collect()
}

#[test]
fn rust_first_guidance_is_present_and_linked() {
    assert!(policy_errors(POLICY).is_empty(), "{:?}", policy_errors(POLICY));
    assert!(CONTRIBUTING.contains("Use Rust instead of Python for all new first-party scripts."));
    assert!(CONTRIBUTING.contains("Existing Python scripts should be migrated with behavioral parity"));
    assert!(CONTRIBUTING.contains("policies/RUST_FIRST_TESTING.md"));
    assert!(README.contains("policies/RUST_FIRST_TESTING.md"));
}

#[test]
fn missing_migration_and_compiler_gates_fail_closed() {
    for (old, new) in [
        ("Write new test tools and automation in Rust, not Python", "Use anything"),
        ("Run both implementations over the same fixtures", "Run the new one only"),
        ("Do not transfer secrets", "Transfer credentials"),
        ("source string check as executable compiler evidence", "text is proof"),
    ] {
        let mutant = POLICY.replace(old, new);
        assert!(!policy_errors(&mutant).is_empty(), "undetected: {old}");
    }
}
