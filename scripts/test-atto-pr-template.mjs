import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";

const original = readFileSync(
  new URL("../.github/PULL_REQUEST_TEMPLATE/atto-lang-candidate.md", import.meta.url),
  "utf8",
);

function templateErrors(value) {
  const errors = [];
  const requireTerms = [
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
  ];
  for (const term of requireTerms) {
    if (!value.includes(term)) errors.push(term);
  }
  if (value.includes("- [x]")) errors.push("prechecked admission");
  if (!value.includes("parse-only is insufficient")) errors.push("false compiler admission");
  return errors;
}

test("AttoLang PR template preserves review and compiler admission gates", () => {
  assert.deepEqual(templateErrors(original), []);
});

test("missing receipt and preapproved checkboxes fail closed", () => {
  assert.notDeepEqual(templateErrors(original.replace("compiler SHA", "")), []);
  assert.notDeepEqual(templateErrors(original.replace("- [ ]", "- [x]")), []);
  assert.notDeepEqual(templateErrors(original.replace("parse-only is insufficient", "")), []);
});
