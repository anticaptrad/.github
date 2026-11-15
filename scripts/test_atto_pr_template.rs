//! Rust-only review-template tests for AttoLang admission.
const TEMPLATE: &str = include_str!("../.github/PULL_REQUEST_TEMPLATE/atto-lang-candidate.md");

fn errors(template: &str) -> Vec<&'static str> {
    let required = [
        "candidate",
        "Immutable head commit SHA",
        "blob SHAs",
        "JSON Schema / TypeSpec",
        "executable",
        "source SHA",
        "compiler SHA",
        "artifact digest",
        "E_UNSUPPORTED",
        "negative mutants",
        "publishing matrices",
        "exact",
        "mirror",
        "private key",
        "branch protection",
        "parse-only is insufficient",
    ];
    let mut missing: Vec<_> = required.iter().copied()
        .filter(|term| !template.contains(term)).collect();
    if template.contains("- [x]") {
        missing.push("preapproved checkboxes");
    }
    missing
}

#[test]
fn template_requires_real_compiler_and_exact_sha_evidence() {
    assert!(errors(TEMPLATE).is_empty(), "{:?}", errors(TEMPLATE));
}

#[test]
fn mutation_canaries_reject_missing_evidence_and_preapproval() {
    for (old, new) in [
        ("compiler SHA", "compiler unverified"),
        ("parse-only is insufficient", "parse-only is acceptable"),
        ("- [ ]", "- [x]"),
    ] {
        let mutant = TEMPLATE.replacen(old, new, 1);
        assert!(!errors(&mutant).is_empty(), "mutation escaped: {old}");
    }
}
