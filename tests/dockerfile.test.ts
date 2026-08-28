import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';

const dockerfile = readFileSync(resolve(process.cwd(), 'Dockerfile'), 'utf8');

describe('container build contract', () => {
  it('uses a Rust builder compatible with the locked ICU graph', () => {
    const match = dockerfile.match(/^FROM rust:(\d+)\.(\d+)-bookworm AS server$/m);
    expect(match, 'the server stage must use a versioned Rust Bookworm image').not.toBeNull();

    const [, major, minor] = match!;
    expect(Number(major)).toBeGreaterThanOrEqual(1);
    expect(Number(minor)).toBeGreaterThanOrEqual(88);
  });

  it('keeps the release build locked inside the server stage', () => {
    expect(dockerfile).toMatch(/RUN cargo build --release --locked/);
  });
});
