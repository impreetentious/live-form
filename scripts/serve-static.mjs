#!/usr/bin/env node
import http from "node:http";
import fs from "node:fs";
import path from "node:path";

const args = process.argv.slice(2);
function flag(name, fallback) {
  const i = args.indexOf(name);
  return i >= 0 ? args[i + 1] : fallback;
}

const dir = path.resolve(flag("--dir", "dist"));
const port = Number(flag("--port", "8080"));
const base = flag("--base", "") || "";

const MIME = {
  ".html": "text/html; charset=utf-8",
  ".js": "text/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".wasm": "application/wasm",
  ".png": "image/png",
  ".txt": "text/plain; charset=utf-8",
};

function safeJoin(root, urlPath) {
  const decoded = decodeURIComponent(urlPath.split("?")[0] ?? "");
  const stripped = base && decoded.startsWith(base) ? decoded.slice(base.length) : decoded;
  const rel = path.normalize(stripped).replace(/^[/\\]+/, "");
  if (rel.startsWith("..")) return null;
  return path.join(root, rel);
}

const server = http.createServer((req, res) => {
  const urlPath = req.url ?? "/";
  let file = safeJoin(dir, urlPath);
  if (!file) {
    res.writeHead(400);
    res.end("bad path");
    return;
  }
  let stat;
  try {
    stat = fs.statSync(file);
  } catch {
    res.writeHead(404);
    res.end("not found");
    return;
  }
  if (stat.isDirectory()) file = path.join(file, "index.html");
  try {
    const body = fs.readFileSync(file);
    const type = MIME[path.extname(file)] ?? "application/octet-stream";
    res.writeHead(200, { "content-type": type });
    res.end(body);
  } catch {
    res.writeHead(404);
    res.end("not found");
  }
});

server.listen(port, "127.0.0.1", () => {
  console.log(`http://127.0.0.1:${port}${base || "/"}`);
});
