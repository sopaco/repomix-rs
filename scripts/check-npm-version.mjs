#!/usr/bin/env bun
/**
 * Verify Cargo workspace crate versions match npm/repomix-rs/package.json.
 *
 * Usage (node or bun):
 *   bun scripts/check-npm-version.mjs
 *   bun scripts/check-npm-version.mjs --expected 2.0.1
 */
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const CRATE_MANIFESTS = [
  'crates/cli/Cargo.toml',
  'crates/core/Cargo.toml',
  'crates/config/Cargo.toml',
  'crates/shared/Cargo.toml',
  'crates/mcp/Cargo.toml',
];

function parseExpected(argv) {
  const idx = argv.indexOf('--expected');
  if (idx === -1) {
    return null;
  }
  const value = argv[idx + 1];
  if (!value) {
    throw new Error('Missing value for --expected');
  }
  return value;
}

function readCargoVersion(manifestPath) {
  const content = readFileSync(path.join(ROOT, manifestPath), 'utf8');
  const match = content.match(/^version\s*=\s*"([^"]+)"/m);
  if (!match) {
    throw new Error(`No version in ${manifestPath}`);
  }
  return match[1];
}

function main() {
  const expectedOverride = parseExpected(process.argv.slice(2));
  const npmPkg = JSON.parse(
    readFileSync(path.join(ROOT, 'npm', 'repomix-rs', 'package.json'), 'utf8'),
  );
  const expected = expectedOverride ?? npmPkg.version;

  const mismatches = [];
  for (const manifest of CRATE_MANIFESTS) {
    const cargoVersion = readCargoVersion(manifest);
    if (cargoVersion !== expected) {
      mismatches.push(`${manifest}: ${cargoVersion} (expected ${expected})`);
    }
  }

  if (npmPkg.version !== expected) {
    mismatches.push(`npm/repomix-rs/package.json: ${npmPkg.version} (expected ${expected})`);
  }

  for (const version of Object.values(npmPkg.optionalDependencies ?? {})) {
    if (version !== expected) {
      mismatches.push(`optionalDependency version ${version} (expected ${expected})`);
    }
  }

  if (mismatches.length > 0) {
    console.error('Version mismatch:');
    for (const line of mismatches) {
      console.error(`  - ${line}`);
    }
    process.exit(1);
  }

  console.log(`All versions match: ${expected}`);
}

main();
