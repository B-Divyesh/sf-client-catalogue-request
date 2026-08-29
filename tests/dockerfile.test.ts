import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const dockerfile = readFileSync(resolve(process.cwd(), 'Dockerfile'), 'utf8');

describe('container build contract', () => {
  it('uses the current stable Rust Bookworm builder', () => {
    expect(dockerfile).toMatch(/^FROM rust:1-bookworm AS server$/m);
  });

  it('keeps the release build locked inside the server stage', () => {
    expect(dockerfile).toMatch(/RUN cargo build --release --locked/);
  });
});
