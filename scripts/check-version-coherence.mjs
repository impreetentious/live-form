#!/usr/bin/env node
import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const read = (file) => readFileSync(path.join(root, file), "utf8");
const errors = [];
const semver = "([0-9]+\\.[0-9]+\\.[0-9]+)";

const readmeVersion = read("README.md").match(
  new RegExp(`\\*\\*Version:\\*\\*\\s+v${semver}(?:\\s|$)`),
)?.[1];
if (!readmeVersion) errors.push("README.md missing **Version:** vX.Y.Z marker");

let version = readmeVersion;
if (existsSync(path.join(root, "Cargo.toml"))) {
  const cargoVersion = read("Cargo.toml").match(
    new RegExp(`\\[workspace\\.package\\][\\s\\S]*?^version\\s*=\\s*"${semver}"`, "m"),
  )?.[1];
  if (!cargoVersion) errors.push("Cargo.toml missing workspace package version");
  if (version && cargoVersion !== version) {
    errors.push(`Cargo.toml version ${cargoVersion ?? "missing"} != README.md ${version}`);
  }
  version = cargoVersion ?? version;
}

if (version && existsSync(path.join(root, "Cargo.lock"))) {
  for (const block of read("Cargo.lock").split("[[package]]").slice(1)) {
    const name = block.match(/^\s*name\s*=\s*"([^"]+)"/m)?.[1];
    const packageVersion = block.match(/^\s*version\s*=\s*"([^"]+)"/m)?.[1];
    if (name?.startsWith("st-") && packageVersion !== version) {
      errors.push(`Cargo.lock ${name} version ${packageVersion ?? "missing"} != ${version}`);
    }
  }
}

if (version && existsSync(path.join(root, "web/package.json"))) {
  const pkg = JSON.parse(read("web/package.json"));
  if (pkg.version !== version) {
    errors.push(`web/package.json version ${pkg.version} != ${version}`);
  }
  if (existsSync(path.join(root, "web/package-lock.json"))) {
    const lock = JSON.parse(read("web/package-lock.json"));
    if (lock.version !== version) {
      errors.push(`web/package-lock.json version ${lock.version} != ${version}`);
    }
    if (lock.packages?.[""]?.version !== version) {
      errors.push(`web lock root version ${lock.packages?.[""]?.version} != ${version}`);
    }
  }
}

if (existsSync(path.join(root, "crates/st-cli/src/main.rs"))) {
  if (!read("crates/st-cli/src/main.rs").includes('env!("CARGO_PKG_VERSION")')) {
    errors.push('st-cli must derive its version from CARGO_PKG_VERSION');
  }
}

if (existsSync(path.join(root, ".nvmrc"))) {
  const nvm = read(".nvmrc").trim();
  if (nvm !== "22.23.3") errors.push(`.nvmrc ${nvm} != 22.23.3`);
}

if (existsSync(path.join(root, "rust-toolchain.toml"))) {
  const channel = read("rust-toolchain.toml").match(/channel\s*=\s*"([^"]+)"/)?.[1];
  if (channel !== "1.98.0") errors.push(`rust-toolchain.toml ${channel ?? "missing"} != 1.98.0`);
}

if (errors.length) {
  console.error("version-coherence FAILED:");
  for (const error of errors) console.error(` - ${error}`);
  process.exit(1);
}

console.log(`version-coherence OK — ${version}`);
