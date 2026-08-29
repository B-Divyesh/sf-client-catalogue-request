import { readFileSync, statSync } from 'node:fs';
import { resolve } from 'node:path';
import { describe, expect, it } from 'vitest';
import { ENTRA_API_SCOPE, ENTRA_AUTHORITY } from '../src/auth';

describe('seller identity contract', () => {
  it('requests the product API scope from the Sociobot tenant', () => {
    expect(ENTRA_AUTHORITY).toBe('https://sociobotcustomers.ciamlogin.com/35c6fe40-0ec0-46b6-98c6-213ad4de6650/');
    expect(ENTRA_API_SCOPE).toBe('api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user');
  });

  it('keeps the unauthenticated entry bundle below 200 KB', () => {
    const html = readFileSync(resolve(process.cwd(), 'dist/index.html'), 'utf8');
    const entry = html.match(/<script[^>]+src="([^"]+\.js)"/)?.[1];
    expect(entry).toBeTruthy();
    const bytes = statSync(resolve(process.cwd(), 'dist', entry!.replace(/^\//, ''))).size;
    expect(bytes).toBeLessThanOrEqual(200_000);
  });
});
