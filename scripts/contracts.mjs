#!/usr/bin/env node
import { readFileSync, existsSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const throughArg = process.argv.indexOf("--through");
const through = throughArg >= 0 ? Number(process.argv[throughArg + 1]) : 0;
if (!Number.isInteger(through) || through < 0) {
  console.error("usage: contracts.mjs --check [--through N]");
  process.exit(2);
}

const file = path.join(root, "tests/contracts.json");
const data = JSON.parse(readFileSync(file, "utf8"));
const rows = data.contracts;
if (!Array.isArray(rows) || rows.length === 0) {
  console.error("contracts.json: empty selection");
  process.exit(1);
}

const ids = new Set();
for (const row of rows) {
  if (ids.has(row.id)) {
    console.error(`duplicate contract id ${row.id}`);
    process.exit(1);
  }
  ids.add(row.id);
  if (row.phase > through && row.kind !== "pending") continue;
  if (row.phase <= through && row.kind !== "pending") {
    const testPath = path.join(root, row.testPath);
    if (!existsSync(testPath)) {
      console.error(`missing test ${row.testPath} for ${row.id}`);
      process.exit(1);
    }
  }
}

const selected = rows.filter((row) => row.phase <= through && row.kind !== "pending");
if (selected.length === 0) {
  console.error("zero selected tests");
  process.exit(1);
}

console.log(`contracts OK — through ${through}, ${selected.length} active`);
