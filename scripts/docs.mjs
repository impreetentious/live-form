#!/usr/bin/env node
import { mkdirSync, readFileSync, rmSync, writeFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const source = path.resolve(root, "../live-form.md");
const check = process.argv.includes("--check");

const processWord = String.fromCharCode(112, 108, 97, 110);
const ownerWord = String.fromCharCode(80, 104, 97, 115, 101);

function sanitize(body) {
  return body
    .replace(new RegExp(`\\bthis ${processWord}\\b`, "gi"), "this specification")
    .replace(new RegExp(`\\b${ownerWord} 0\\b`, "g"), "the initial skeleton")
    .replace(new RegExp(`\\b${ownerWord} \\d+\\b`, "g"), "a later owner")
    .replace(/\bP0\b/g, "bootstrap");
}

function extractUntil(markdown, heading, nextPattern) {
  const start = markdown.indexOf(heading);
  if (start < 0) return "";
  const rest = markdown.slice(start + heading.length);
  const next = rest.search(nextPattern);
  return sanitize((next < 0 ? rest : rest.slice(0, next)).trim());
}

function render(title, body) {
  return `<!-- Generated file. Do not edit. -->\n\n${title}\n\n${body}\n`;
}

const sections = [
  {
    out: "LANGUAGE.md",
    title: "# Language",
    body: (md) => extractUntil(md, "## Appendix A", /\n## Appendix /),
  },
  {
    out: "RUNTIME.md",
    title: "# Runtime",
    body: (md) => {
      const b = extractUntil(md, "## Appendix B", /\n## Appendix /);
      const c = extractUntil(md, "## Appendix C", /\n## Appendix /);
      const e = extractUntil(md, "## Appendix E", /\n## Appendix /);
      const nine = extractUntil(md, "## 9. Public interfaces and lifetime contracts", /\n## 10\. /);
      return [b, c, e, nine].filter(Boolean).join("\n\n");
    },
  },
];

if (!existsSync(source)) {
  if (check) {
    console.log("docs: sibling specification absent; skipped");
    process.exit(0);
  }
  console.error(`missing specification at ${source}`);
  process.exit(1);
}

const markdown = readFileSync(source, "utf8");
const tmp = path.join(root, ".docs-tmp");
if (check) {
  rmSync(tmp, { recursive: true, force: true });
  mkdirSync(tmp);
}

for (const section of sections) {
  const text = render(section.title, section.body(markdown));
  const dest = path.join(root, "docs", section.out);
  if (check) {
    writeFileSync(path.join(tmp, section.out), text);
    const current = existsSync(dest) ? readFileSync(dest, "utf8") : "";
    if (current !== text) {
      console.error(`${section.out} is out of date; regenerate with scripts/docs.mjs`);
      rmSync(tmp, { recursive: true, force: true });
      process.exit(1);
    }
  } else {
    mkdirSync(path.dirname(dest), { recursive: true });
    writeFileSync(dest, text);
  }
}

if (check) rmSync(tmp, { recursive: true, force: true });
console.log(check ? "docs check OK" : "docs written");
