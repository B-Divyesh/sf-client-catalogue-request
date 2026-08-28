# Independent product verification — FAIL

Verified: 2026-08-28 UTC  
Work order: `client-catalogue-request-verify-1`  
Candidate: `2a0ab98740c4d8519a0073a8718953b68d5e23df`  
Live URL: <https://client-catalogue-request.sociobot.in>  
Acceptance contract: `.factory/brief.json`, the supplied researched brief, and the attached factory skills.

## Verdict

**FAIL — do not release this candidate.**

The source builds and its local automated suite passes. The live build is byte-for-byte the candidate frontend and reports the candidate SHA. However, the deployed demo cannot reliably retain its workspace between requests, so its core request flow fails with a 404. The paid checkout is also dead, seller authentication violates the required Entra contract, the public deployment exposes one globally claimable seller workspace, and paid limits are bypassable.

## Release-blocking findings

### Critical — the live demo loses its workspace between backend replicas

The first click renders six sample products, but the first write can hit another replica that does not hold the in-memory workspace.

- Ten independent `POST /api/demo` followed immediately by `GET /api/demo/{id}/requests` pairs returned **10 × `201` then `404`**.
- A separate workspace returned GET statuses `200, 404, 404, 200, 200, 404`, proving replica-local state rather than expiry.
- At 390 × 844, adding a product and sending the sample request displayed: `This sample workspace expired. Reset the demo to start again.`
- The browser logged a failed 404 resource request. Resetting cannot repair a replica-sharing problem.
- Direct `/demo/inbox` navigation also rendered `The sample workspace could not open` during the route audit.

This breaks the promised demo sandbox and the smallest useful end-to-end flow on the deployed product. Use shared/TTL-backed demo storage or route affinity that cannot lose a workspace between requests.

### High — there is only one globally claimable seller workspace, and sign-in is not Sociobot Entra

- Live `GET /api/setup/status` returned `{"claimed":false}`.
- The first visitor to `/manage` is invited to set the single global business name and password. After one local setup, a second `POST /api/setup` returned `409` with `This workspace already has an owner. Sign in instead.`
- The implementation has one `owner` row and one SQLite catalogue, not seller tenants. A first visitor can therefore claim the public deployment and prevent every other seller from starting.
- Seller authentication is an app-local password plus opaque bearer token. No `sociobotcustomers.ciamlogin.com` authority or Entra/OpenID flow exists. This fails the work order's explicit sign-in requirement.

No live setup was performed because claiming the global workspace would deny service to future visitors.

### High — paid purchase and entitlement enforcement do not work

- `GET https://api.sociobot.in/api/v1/products/client-catalogue-request/checkout` returned **404** with `{"error":"enabled factory product","status":404}`. The live “Buy the full workspace” action is a dead link.
- An authenticated direct API call saved **13 rows without a license** (`200`, `{"count":13,"saved":true}`) and created a second client link without a license (`200`). The server does not enforce either paid limit.
- Setting an arbitrary license plus `{valid:true, checked:<now>}` in local storage made the UI show `Full workspace active` and allowed a 13-row import. Entitlement is trusted entirely from forgeable browser state.
- A real invalid verification response was cached as invalid and stripped from the URL correctly, but the page showed no required “license no longer active” notice.
- Restore purchase uses `prompt()` rather than the specified visible license field.

Register/enable the billing product, enforce entitlement and limits at the backend, and render invalid/revoked state accessibly.

### High — public claims are missing from `.factory/claims.json`

The manifest exists and every listed command passes, but the required cross-check finds relied-upon statements with no corresponding claim entry/test:

- Privacy page and demo docs: sample requests remain in memory for **up to 24 hours**.
- Privacy page: **Reset demo deletes both** browser and server demo state.
- README: paid imports support **up to 5,000 rows**.
- Seller sign-in page: the password hash **stays on this server and is not sent to a third party**.
- README/runtime contract: **Only `PORT` is needed in production**.
- Privacy page: clients can ask the seller to **correct or delete a request**.

The claims contract says an unlisted claim fails review. Add observable tagged tests or remove/narrow the statements.

## Other findings

### Medium — accessibility requirements missed by automated scores

- At 390 px, several visible interactive targets are below 44 × 44 CSS px: footer links are about 21 px high, the wordmark is 28 px high, and catalogue filter chips are 40 px high. The mobile `Demo` link is about 37 px wide.
- Initial rendering programmatically focuses the `<h1>`. On a cold keyboard session, the first Tab went directly to `Try it with sample data`, skipping the skip link, wordmark, and header navigation until focus wraps.
- At a 320 px viewport with text increased to 200%, document width became 380 px, producing horizontal overflow.

Positive evidence: the focus indicator is a visible 3 px amber outline; the native request dialog initially focuses its close button, traps focus, closes with Escape, and returns focus to its opener. Axe found no serious/critical issues across `/`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, `/missing-page`, and `/manage` on desktop and 390 px mobile. Lighthouse accessibility scored 100.

### Medium — unknown routes return HTTP 200

`/missing-page` renders the designed 404 view but responds with HTTP **200**, not 404. This violates the real-404 routing contract and misleads clients and crawlers.

### Medium — hashed assets have no caching policy

The hashed JS, CSS, font, and WebP responses have no `Cache-Control` header. They expose `Last-Modified`, but not the required long-lived immutable caching for fingerprinted assets.

### Medium — protected links and stored requests have no lifecycle controls

- The seller UI and API can create client links but cannot deactivate, rotate, or delete them, even though the data model exposes an `active` state. A leaked “protected” link remains usable indefinitely.
- The privacy page tells clients to contact the seller to correct or delete a request, but the seller workspace and API provide no correction or deletion action.

Add seller-visible revocation and request correction/deletion controls with specific confirmation and audit behavior.

### Low — strict Rust lint is not clean

`cargo clippy --all-targets -- -D warnings` fails at `src/api.rs:620` with `clippy::useless-borrows-in-formatting`. `cargo fmt --check` passes. No repository lint script is defined, so this is not the reason for the release failure.

## Mandatory first-read gate

**Pass for the cold first screen itself.**

- What it does: “Turn repeat orders into clear requests.”
- For whom: “For small B2B sellers who need client orders without running an online store.”
- First action: “Try it with sample data,” followed by “Opens a private sample catalogue. No setup.”
- One click opens `/demo`, shows six realistic products immediately, and displays `Demo — sample data, nothing is saved`, `Reset demo`, and `Start for real`.

The first screen meets the plain-words gate, but the live demo then fails its core submit/inbox flow as described above.

## Claim command results

All commands were rerun exactly after `npm ci` from the clean candidate. The initial pre-install invocation could not find `vitest`; the locked install then succeeded and all claims passed.

| Claim | Exact command | Result |
|---|---|---|
| `demo-isolation` | `npm test -- --grep @claim:demo-isolation` | PASS — 2 Playwright projects |
| `poa-price` | `npm test -- --grep @claim:poa-price` | PASS — 2 projects |
| `csv-export` | `npm test -- --grep @claim:csv-export` | PASS — 2 projects |
| `structured-request` | `npm test -- --grep @claim:structured-request` | PASS — 2 projects |
| `no-card-data` | `npm test -- --grep @claim:no-card-data` | PASS — 2 projects |
| `protected-links` | `cargo test generated_tokens_are_long_and_distinct` | PASS — 1 Rust test |
| `csv-import` | `npm test -- --grep @claim:csv-import` | PASS — Chromium; intentional mobile duplicate skipped |
| `print-request` | `npm test -- --grep @claim:print-request` | PASS — Chromium; intentional mobile duplicate skipped |
| `paid-license` | `npm test -- --grep @claim:paid-license` | PASS — 2 projects |
| `stock-privacy` | `cargo test stock_counts_are_not_exposed` | PASS — 1 Rust test |
| `privacy-runtime` | `npm test -- --grep @claim:privacy-runtime` | PASS — 2 projects |

These are local single-process tests. In particular, the local demo claim does not reproduce the live multi-replica failure.

## Build and automated checks

- `npm ci`: PASS; 62 packages installed, 0 audit vulnerabilities.
- `npm test`: PASS; 5 Vitest tests, 6 Rust tests, and 23 Playwright cases passed; 3 intentional mobile duplicates skipped.
- `VITE_BUILD_SHA=2a0ab987… npm run build`: PASS; TypeScript check and exact production Vite build completed.
- `BUILD_SHA=2a0ab987… cargo build --release --locked`: PASS with Rust 1.98.0.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: FAIL, one low-severity warning described above.
- `git diff --check`: PASS before report edits.
- The environment has no Docker/Podman/Buildah executable, so the complete Dockerfile could not be rebuilt locally. Its exact frontend and release-server stages were built independently, Dockerfile contract tests passed, and the live artifact matched the resulting frontend bytes.
- Startup with only `PORT` in an otherwise cleared environment: PASS; `/health` and `/privacy` returned 200.

## End-to-end and boundary evidence

The isolated local release server exercised the real backend and production `dist/`:

- Valid setup, CSV save, protected link, POA catalogue, maximum quantity 9,999, request creation, and seller inbox: PASS.
- Wrong password: 401 with a recovery message; subsequent correct login passed.
- Missing admin bearer: 401.
- Invalid business name/password, two-letter currency, duplicate SKU, one-character link label, malformed email, quantity 0/10,000, duplicate request line, missing product, and 101 request lines: correct 4xx responses.
- 5,001-row import: 400 with the documented split-file instruction.
- Invalid CSV in the UI reported `The CSV needs a sku column.`; replacing it with a valid 12-row CSV recovered and saved.
- Persistence: after graceful restart against the same data directory, setup remained claimed and 13 products, 2 links, the session, and 1 request remained available.
- 100 concurrent local `/health` calls: 100 × 200.
- PWA/offline and library/CLI checks: not applicable; this product does not register a service worker and is not a library or CLI.

## Live deployment, privacy, and response policy

- `/health`: 200 with exact candidate SHA `2a0ab98740c4d8519a0073a8718953b68d5e23df`.
- Candidate/live byte comparison: exact SHA-256 matches for `index.html`, hashed JS, hashed CSS, and both hero WebPs.
- `/opt/fleet/lib/verify-url.sh`: PASS; 200, 616 ms load, title, `lang=en`, one `<h1>`, `<main>`, all image alt text, labeled buttons, no cold-load console errors.
- Root and API responses include CSP, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`.
- An arbitrary cross-origin API request received no `Access-Control-Allow-Origin` header.
- Cold landing and demo loads requested only the product origin. No analytics, ads, runtime CDN, or remote font requests were observed.
- Supplying a dummy license made exactly one disclosed cross-origin request to `api.sociobot.in`, stripped the token from the URL, and produced no console error.
- Link crawl: every discovered product link returned 200 except the checkout URL, which returned 404.

## Rate limiting

- Live product API, 100 concurrent `GET /api/setup/status` requests from one forwarded IP: **40 × 200, 60 × 429**, `Retry-After: 1`.
- Local write endpoint, 100 concurrent `POST /api/demo`: **12 × 201, 88 × 429**, `Retry-After: 1`.
- Sociobot license verification endpoint, 120 concurrent invalid-token checks: **30 × 200, 90 × 429**, `Retry-After: 4`.
- Health is intentionally exempt, as allowed by the backend contract.

## Performance and responsive evidence

- Lighthouse 13.0.1 mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100.
- FCP 1.2 s; LCP 1.4 s; CLS 0; TBT 20 ms. No interaction occurred, so lab INP was not reported.
- Initial transfer: 97 KiB total.
- Candidate build: JS 33.71 KB raw / 11.00 KB gzip; CSS 18.43 KB raw / 4.89 KB gzip; browser-loaded WOFF2 fonts 30.07 KB; mobile hero 14.02 KB.
- Desktop 1440 × 900 and mobile 390 × 844: no horizontal overflow; the visual hierarchy and product-specific art remain clear.
- Reduced-motion mode applies 0.01 ms one-shot animation/transition durations and removes hover/hero transforms.

## Required remediation order

1. Put demo workspaces in shared TTL storage and add a deployed multi-replica claim test.
2. Replace the global owner/password model with the required Sociobot Entra tenant and real seller tenancy.
3. Register the live billing product and enforce entitlements on the backend, never from mutable browser cache.
4. Add missing claim entries/tests, including deployment-state claims.
5. Fix touch targets, initial keyboard focus, 200% text reflow, 404 status handling, immutable asset caching, and the strict clippy warning.
