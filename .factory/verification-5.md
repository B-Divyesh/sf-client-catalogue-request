# Independent verification 5 — Client Catalogue Request

Verified 2026-08-29 UTC for work order `client-catalogue-request-verify-5`.

## Verdict: FAIL — release blocked

The supplied candidate commit, `5e43476098d8bdf816d8c8525a5a8d7d8dcc3f5f`, cannot be verified or released.  After `git fetch --prune origin`, `git cat-file -t` reports that object does not exist; `origin/main` and the clean checkout are instead `5e43474ca4be3b4a7876ae3abd960cd7a4b3a157`.

Fresh deployment evidence independently confirms the mismatch. `https://client-catalogue-request.sociobot.in/health` returned HTTP 200 with:

```json
{"build_sha":"5e43474ca4be3b4a7876ae3abd960cd7a4b3a157","ok":true}
```

That is not the requested candidate. Passing tests on the available revision cannot establish correctness of an unavailable SHA. This is a **release-blocking (P0) provenance/deployment defect**.

## Mandatory claims check

`.factory/claims.json` exists and contains 28 claims. From a clean dependency install (`npm ci`, 64 packages, 0 vulnerabilities), I ran every exact command in its `test` field, sequentially, against the available checkout's shipped demo entry point. All 28 passed. This includes all Playwright demo claims plus the direct Rust and Vitest commands. The result is evidence for `5e43474…` only, not for the requested SHA.

| Claim ID | Result on available checkout |
|---|---|
| `demo-isolation` | PASS |
| `demo-entry` | PASS |
| `demo-sample-content` | PASS |
| `poa-price` | PASS |
| `csv-export` | PASS |
| `free-export` | PASS |
| `structured-request` | PASS |
| `no-card-data` | PASS |
| `service-boundaries` | PASS |
| `protected-links` | PASS |
| `csv-import` | PASS |
| `csv-header-normalization` | PASS |
| `print-request` | PASS |
| `paid-license` | PASS |
| `billing-handoff` | PASS |
| `stock-privacy` | PASS |
| `privacy-runtime` | PASS |
| `demo-local` | PASS |
| `demo-reset` | PASS |
| `csv-import-cap` | PASS |
| `port-runtime` | PASS |
| `runtime-storage` | PASS |
| `container-runtime` | PASS |
| `health-build` | PASS |
| `browser-storage` | PASS |
| `client-data-control` | PASS |
| `seller-tenancy` | PASS |
| `paid-license-invalid` | PASS |

## Available-checkout quality gates

These results are contextual only, because the source identity is wrong:

- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, and the 50-test Playwright run completed with `test-results/.last-run.json` status `passed`.
- `npm run lint`: PASS (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`).
- `VITE_BUILD_SHA=verification-5 npm run build`: PASS; `dist/` produced. Main initial JS was 12.27 KB gzip and CSS 5.21 KB gzip. The 78.66 KB gzip MSAL chunk is lazy.
- `BUILD_SHA=verification-5 cargo build --release --locked`: PASS; release binary produced.
- `git diff --check`: PASS before documentation edits.
- Container build: not run because neither `docker` nor `podman` is installed in this verifier container.

## Fresh live checks (deployed `5e43474…`, not candidate)

First read, cold desktop browser: “Client Catalogue Request” collects clear quote requests for small B2B sellers who need client orders without an online store. The first action is **Try it with sample data**, and adjacent copy says it opens a private sample catalogue with no setup. It therefore passes the plain-words and one-click-demo gate for the deployed revision.

- Cold-page outgoing requests were limited to the product origin: HTML, two self-hosted fonts, JS, CSS, and the self-hosted hero WebP. There were no console or page errors.
- `/?demo=1` at 390 × 844 had `scrollWidth === innerWidth === 390`, showed the persistent “Demo — sample data, nothing is saved” banner, and made only same-origin requests. Tab order exposed the skip link and demo controls, each with a visible 3px focus outline.
- Playwright axe-core WCAG 2 A/AA scan of `/?demo=1` found **0 serious/critical findings** at desktop and 390px mobile.
- Root response headers included `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`, and CSP with `frame-ancestors 'none'`. The hashed deployed JS had `Cache-Control: public, max-age=31536000, immutable`.
- Rate-limit probe of the safe stateless `POST /api/demo` route: a 120-request concurrent burst received 102 × 201 and 18 × 429. A sampled 429 included `Retry-After: 1`. This demonstrates enforcement on the deployed revision.

## Defects

| Severity | Finding | Evidence / required resolution |
|---|---|---|
| P0 release blocker | Candidate object is absent and live deployment serves a different SHA. | Publish/fetch the exact requested commit and deploy it. Re-run verification against `/health` returning `5e43476098d8bdf816d8c8525a5a8d7d8dcc3f5f`. |

No additional product defect is asserted for the currently deployed `5e43474…`; it was not the requested candidate.
