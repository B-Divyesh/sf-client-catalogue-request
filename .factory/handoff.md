# Client Catalogue Request — verification handoff

Verification work order: `client-catalogue-request-verify-2`

Tested candidate: `5f9d6cc53981ec923480a2f23bb3b2edbd4b87e9`

Tested live URL: <https://client-catalogue-request.sociobot.in>
Date: 2026-08-29 UTC

## Result

**FAIL — do not release this candidate.**

The live deployment is the exact candidate and the repaired demo, tenant storage, rate limits, build, tests, accessibility automation, caching, and deployment identity all pass. The real product still fails its job because seller sign-in is blocked by the deployed CSP. The paid checkout is also 404, and quantity 10,000 is silently submitted as 9,999.

## Release blockers

1. **Critical — seller sign-in is inaccessible.** Clicking **Sign in with Sociobot** leaves the browser on `/manage`; CSP blocks Entra discovery at `sociobotcustomers.ciamlogin.com` and raises `endpoints_resolution_error`. The diagnostic authorize request also contains only OIDC scopes, not a product API scope.
2. **High — paid purchase is dead.** The advertised ₹1,499 checkout returns `404 {"error":"enabled factory product","status":404}`. The `paid-license` claim test checks a mocked browser verdict and a link, not the promised backend limit increase.
3. **High — quantity is silently altered.** A visible input of 10,000 submits/stores 9,999 without an error or visible normalization.

## Additional defects

- **Medium:** two measured mobile inline links are below 44 px high, and SPA route changes announce but do not focus the new `<h1>`.
- **Medium:** three Lighthouse mobile runs score 92/93/93 but LCP is 2.70–2.78 s. Live JS is sent uncompressed at 299,476 bytes; total initial transfer is about 366 KB.

## Verification summary

- All 18 commands in `.factory/claims.json`: exit 0 after `npm ci`; see `.factory/verification-2.md` for the claim-quality exception.
- `npm test`: 5 Vitest + 8 Rust + 28 Playwright passed; 4 intentional skips.
- Exact TypeScript/Vite build, locked Rust release build, format, strict clippy, audit, and diff checks: PASS.
- Live `/health`: exact candidate SHA.
- Candidate/live frontend and public assets: byte-for-byte match.
- First-read and one-click sample-data gates: PASS.
- Live demo, CSV export, reset, invalid-input recovery, 390 px mobile, 320 px at 200% text, keyboard dialog behavior, reduced motion, and axe: PASS apart from the defects above.
- Ten demo replica-boundary pairs: 10 × 201 → 200.
- Local SQLite restart retained 12 products, one link, and one request; revocation and deletion passed.
- Rate limits observed: product reads 40/s, writes 12/s, Sociobot license verification 30 per observed window; all excess responses were 429 with `Retry-After`.
- Lighthouse runs: Performance 92/93/93, Accessibility 100, Best Practices 100, SEO 100.
- Full Docker image rebuild was not possible because no container engine is installed; both build stages and Dockerfile contract tests passed independently.

Full evidence and remediation order: [`.factory/verification-2.md`](verification-2.md).

No product code was modified during verification.
