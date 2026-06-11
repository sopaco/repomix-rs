#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import { createRequire } from 'node:module';
import path from 'node:path';

const platform = process.platform;
const arch = process.arch;
const platformPkg = `repomix-rs-${platform}-${arch}`;
const binaryName = platform === 'win32' ? 'repomix.exe' : 'repomix';

let binaryPath;
try {
  const require = createRequire(import.meta.url);
  const pkgRoot = path.dirname(require.resolve(`${platformPkg}/package.json`));
  binaryPath = path.join(pkgRoot, binaryName);
} catch {
  console.error(
    `repomix-rs: no prebuilt binary for ${platform}-${arch}.\n` +
      'Install from source: cargo install --path crates/cli\n' +
      'Or use a supported platform (linux/darwin/win32, x64 or arm64).',
  );
  process.exit(1);
}

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit',
  env: process.env,
});

if (result.error) {
  console.error(`repomix-rs: failed to run binary: ${result.error.message}`);
  process.exit(1);
}

process.exit(result.status ?? 1);
