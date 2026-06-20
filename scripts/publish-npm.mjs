#!/usr/bin/env bun
/**
 * Assemble and publish repomix-rs npm packages.
 *
 * Usage (node or bun):
 *   bun scripts/publish-npm.mjs platform --npm-suffix linux-x64 --binary ./repomix --version 2.0.1
 *   bun scripts/publish-npm.mjs main --version 2.0.1
 *   bun scripts/publish-npm.mjs main --version 2.0.1 --dry-run
 */
import { execSync, spawnSync } from 'node:child_process';
import {
  chmodSync,
  cpSync,
  mkdirSync,
  readFileSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const NPM_MAIN_DIR = path.join(ROOT, 'npm', 'repomix-rs');
const TEMPLATE_PATH = path.join(ROOT, 'npm', 'templates', 'platform-package.json.tmpl');
const STAGING_ROOT = path.join(ROOT, 'npm', '.staging');

const PLATFORM_SUFFIXES = [
  'linux-x64',
  'linux-arm64',
  'darwin-x64',
  'darwin-arm64',
  'win32-x64',
];

function parseArgs(argv) {
  const args = { _: [] };
  for (let i = 0; i < argv.length; i += 1) {
    const arg = argv[i];
    if (arg === '--dry-run') {
      args.dryRun = true;
    } else if (arg.startsWith('--')) {
      const key = arg.slice(2);
      const value = argv[i + 1];
      if (!value || value.startsWith('--')) {
        throw new Error(`Missing value for ${arg}`);
      }
      args[key] = value;
      i += 1;
    } else {
      args._.push(arg);
    }
  }
  return args;
}

function npmSuffixToOsArch(npmSuffix) {
  const [platform, arch] = npmSuffix.split('-');
  if (!platform || !arch) {
    throw new Error(`Invalid npm suffix: ${npmSuffix}`);
  }
  return { platform, arch };
}

function renderPlatformPackageJson({ platform, arch, version }) {
  const template = readFileSync(TEMPLATE_PATH, 'utf8');
  return template
    .replaceAll('__PLATFORM__', platform)
    .replaceAll('__ARCH__', arch)
    .replaceAll('__VERSION__', version);
}

function hasCommand(command) {
  try {
    execSync(`command -v ${command}`, { stdio: 'ignore' });
    return true;
  } catch {
    return false;
  }
}

function registryPublish(dir, dryRun) {
  const env = {
    ...process.env,
    NODE_AUTH_TOKEN: process.env.NODE_AUTH_TOKEN ?? process.env.NPM_TOKEN,
  };

  if (hasCommand('npm')) {
    const cmd = dryRun ? 'npm publish --dry-run' : 'npm publish --access public';
    execSync(cmd, { cwd: dir, stdio: 'inherit', env });
    return;
  }

  if (hasCommand('bun')) {
    const args = dryRun ? ['publish', '--dry-run'] : ['publish'];
    const result = spawnSync('bun', args, { cwd: dir, stdio: 'inherit', env });
    if (result.status !== 0) {
      throw new Error(`bun publish failed with exit code ${result.status ?? 'unknown'}`);
    }
    return;
  }

  throw new Error('Neither npm nor bun found on PATH');
}

function publishPlatform({ npmSuffix, binary, version, dryRun }) {
  const { platform, arch } = npmSuffixToOsArch(npmSuffix);
  const pkgName = `repomix-rs-${npmSuffix}`;
  const stagingDir = path.join(STAGING_ROOT, pkgName);
  const binaryName = platform === 'win32' ? 'repomix.exe' : 'repomix';

  rmSync(stagingDir, { recursive: true, force: true });
  mkdirSync(stagingDir, { recursive: true });

  writeFileSync(
    path.join(stagingDir, 'package.json'),
    renderPlatformPackageJson({ platform, arch, version }),
  );
  cpSync(binary, path.join(stagingDir, binaryName));
  if (platform !== 'win32') {
    chmodSync(path.join(stagingDir, binaryName), 0o755);
  }

  console.log(`Publishing platform package ${pkgName}@${version}`);
  registryPublish(stagingDir, dryRun);
}

function publishMain({ version, dryRun }) {
  const mainPkgPath = path.join(NPM_MAIN_DIR, 'package.json');
  const mainPkg = JSON.parse(readFileSync(mainPkgPath, 'utf8'));

  mainPkg.version = version;
  mainPkg.optionalDependencies = Object.fromEntries(
    PLATFORM_SUFFIXES.map((suffix) => [`repomix-rs-${suffix}`, version]),
  );

  const stagingDir = path.join(STAGING_ROOT, 'repomix-rs');
  rmSync(stagingDir, { recursive: true, force: true });
  mkdirSync(stagingDir, { recursive: true });

  writeFileSync(path.join(stagingDir, 'package.json'), `${JSON.stringify(mainPkg, null, 2)}\n`);
  cpSync(path.join(NPM_MAIN_DIR, 'bin'), path.join(stagingDir, 'bin'), { recursive: true });
  chmodSync(path.join(stagingDir, 'bin', 'repomix.js'), 0o755);

  console.log(`Publishing main package repomix-rs@${version}`);
  registryPublish(stagingDir, dryRun);
}

function main() {
  const args = parseArgs(process.argv.slice(2));
  const [command] = args._;

  if (!args.version) {
    throw new Error('--version is required');
  }

  if (command === 'platform') {
    if (!args['npm-suffix'] || !args.binary) {
      throw new Error('platform command requires --npm-suffix and --binary');
    }
    publishPlatform({
      npmSuffix: args['npm-suffix'],
      binary: path.resolve(args.binary),
      version: args.version,
      dryRun: args.dryRun,
    });
    return;
  }

  if (command === 'main') {
    publishMain({ version: args.version, dryRun: args.dryRun });
    return;
  }

  throw new Error('Usage: publish-npm.mjs <platform|main> --version <ver> [options]');
}

try {
  main();
} catch (error) {
  console.error(error instanceof Error ? error.message : error);
  process.exit(1);
}
