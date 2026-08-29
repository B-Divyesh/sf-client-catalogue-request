# Verification 4 handoff — Client Catalogue Request

Verified 2026-08-29 UTC for work order `client-catalogue-request-verify-4`.

## Result

**FAIL — do not release candidate `0505e44271d0530b78b3f601b79fd515c55c4298`.**

The live deployment at <https://client-catalogue-request.sociobot.in> reports the exact candidate SHA and matches the candidate-stamped HTML and entry JavaScript byte-for-byte. The earlier deployment-only failures are not present. The remaining release blocker is a newly reproduced mobile reflow defect in the seller sample inbox.

At 390 px, `/demo/inbox` has a 615 px document width. At 320 px with 200% text, its request card is 752 px wide while page overflow is hidden, clipping request data and demo controls. Exact evidence, screenshots, commands, and all QA results are in [`.factory/verification-4.md`](verification-4.md).

## Verification summary

- All 28 exact commands in `.factory/claims.json`: PASS.
- `npm test`: PASS — 12 Vitest, 13 Rust, 45 Playwright; 5 intentional skips.
- `npm run lint`: PASS.
- Candidate-stamped Vite production build: PASS; `dist/` produced.
- Candidate-stamped locked Rust release build: PASS.
- Safe live Playwright suite: 43 passed, 3 intentional skips.
- Factory URL verifier: PASS on landing and demo.
- Axe serious/critical: 0 on seven routes at desktop and 390 px.
- Lighthouse mobile: 100 performance, 100 accessibility, 100 best practices, 100 SEO; LCP 1.2 s, CLS 0.
- Privacy: complete demo flow remained same-origin.
- Live rate limit: observed 36 write successes then 429 across three replicas, and 120 read successes then 429; `Retry-After: 1` present.
- Live checkout: 303 through the Sociobot billing endpoint.
- Entra: required Sociobot CIAM tenant and product API scope verified; no human credential was available for completing sign-in.

## Required next step

Repair `/demo/inbox` so the request card remains within the viewport and only the data table scrolls internally. Add regression checks for document width and content/control visibility at 390 px and 320 px with 200% text. Then rerun the full verification contract.

No product code was changed during this verification. Only verification documentation and evidence were added.
