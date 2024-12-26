# Liveform

A small runtime whose heap objects can move and change shape while a guest program continues. A stable object-table handle joins compaction and live object migration. The product is the Colony demonstration, the Stitch language and runtime and evidence that work is scheduled in bounded increments.

## The problem

A long-running guest that must change its types, move its heap and keep running usually gets a stop-the-world collector, a restart, or a story about “soft real time” that does not name a budget. None of those say what happens to a suspended frame when a field disappears.

## What it does

- **Stitch** — a small language with lexical closures, fibers, records and named yield labels.
- **Heap** — generational handles, incremental tracing, sweep and compaction that copies in bounded chunks.
- **Updates** — a transactional apply of new source. Object recipes and frame recipes run with finite fuel; a failed migration keeps the old object.
- **Colony** — a browser demonstration that applies v2 then v3 without reload: identity and energy persist, positions move into `Pos`, then nests appear. Simulation and renderer fiber IDs persist.
- **CLI** — `stitch check`, `dump`, `test`, `run` and `soak` on the same runtime as the browser artifact.
- **Evidence** — corpus goldens, update scenarios, a two-hour soak report and heap occupancy / work histograms drawn from actual counters.

## What it is not

No JIT, native threads, generational GC, package system, saved heaps, Windows-native host, runtime-vs-runtime benchmark, framework, or bundler. WSL2 is the Windows route. Bounded accounted work is not a proof that an operating system cannot delay a frame.

## Limitations

- The runtime is single-threaded and cooperative. OS preemption, host allocation and browser rendering can delay a frame; those costs are measured, not disguised.
- Failed migration retains the old object rather than installing corrupt partial fields.
- Large objects are not compacted.
- Frame migration applies at named suspension points. Old code retained by other frames or closures stays live and may see ordinary field errors if it touches fields the program removed.
- Timing gates are for the recorded Colony profile on the recorded machine. They are not a portable hard real-time claim.

## Stack

Rust `1.83.0` (edition 2021) · `libm` for portable VM math · wasm-bindgen · Node `22.12.0` for web tooling · Playwright (Chromium, Firefox, WebKit). Native hosts: macOS arm64 and Linux x86_64.

## Project docs

- `docs/LANGUAGE.md` — Stitch syntax, evaluation, bytecode and builtins.
- `docs/RUNTIME.md` — heap, scheduling, reports, updates and the public host surface.
- `docs/THIRD-PARTY.md` — bundled third-party notices when artifacts include their code.

## Run locally

Requires rustup (Rust `1.83.0`) and Node `22.12.0` (see `.nvmrc` and `rust-toolchain.toml`).

```sh
./scripts/setup.sh
./scripts/demo.sh
```

`./scripts/demo.sh` opens Colony v1 by default. `./scripts/pages-preview.sh` serves the same artifact under `/live-form/`. `./scripts/soak.sh 1` is the short terminal demonstration.

Commands abbreviated `stitch` mean `cargo run --locked -p st-cli --`.

## Verify

```sh
./scripts/check.sh
```

CI runs the same command on Ubuntu and macOS after locked Cargo and `npm ci --prefix web`. Playwright Chromium, Firefox and WebKit are gating for the browser suites; shipping Safari observations are recorded separately.

## Build and deploy

```sh
./scripts/build-web.sh
```

This is the only web artifact builder. It writes a portable relative-path `dist/` with `.nojekyll`. GitHub Pages is a manual `workflow_dispatch` of `.github/workflows/pages.yml`; enabling Pages is outside the repository checks. The local artifact is tested under both `/` and `/live-form/`.

## License

[Apache-2.0](LICENSE) © 2024 Sidakpreet Singh

---

**Version:** v0.0.1
