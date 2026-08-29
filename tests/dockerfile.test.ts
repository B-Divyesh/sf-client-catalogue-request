import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const dockerfile = readFileSync(resolve(process.cwd(), 'Dockerfile'), 'utf8');
const server = readFileSync(resolve(process.cwd(), 'src/main.rs'), 'utf8');

describe('container build contract', () => {
  it('uses the current stable Rust Bookworm builder', () => {
    expect(dockerfile).toMatch(/^FROM rust:1-bookworm AS server$/m);
  });

  it('keeps the release build locked inside the server stage', () => {
    expect(dockerfile).toMatch(/RUN cargo build --release --locked/);
  });

  it('compresses production responses', () => {
    expect(server).toContain('.layer(CompressionLayer::new())');
  });

  it('@claim:container-runtime builds both tiers and runs non-root with persistent data defaults', () => {
    expect(dockerfile).toMatch(/^FROM node:22-bookworm-slim AS web$/m);
    expect(dockerfile).toMatch(/^FROM rust:1-bookworm AS server$/m);
    expect(dockerfile).toMatch(/^USER app$/m);
    expect(dockerfile).toMatch(/^ENV PORT=8080 DATA_DIR=\/app\/data WEB_DIST=\/app\/dist$/m);
    expect(dockerfile).toMatch(/^VOLUME \["\/app\/data"\]$/m);
  });
});
