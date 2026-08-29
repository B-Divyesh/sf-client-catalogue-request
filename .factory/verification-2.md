# Independent product verification — FAIL

Verified: 2026-08-29 UTC

Work order: `client-catalogue-request-verify-2`

Candidate: `5f9d6cc53981ec923480a2f23bb3b2edbd4b87e9`

Live URL: <https://client-catalogue-request.sociobot.in>
Acceptance contract: `.factory/brief.json`, the supplied researched brief, and the attached factory skills.

## Verdict

**FAIL — do not release this candidate.**

The repaired stateless demo works, every listed claim command exits successfully after the locked install, the live frontend is byte-for-byte the candidate, and rate limiting is enforced. The real seller workflow is nevertheless inaccessible because the deployed CSP blocks Microsoft Entra discovery. The advertised paid checkout also returns 404, and an out-of-range quantity can be silently changed before submission. These are release-blocking failures of the real job-to-be-done.

## Release-blocking findings

### Critical — the real seller workspace cannot sign in

At `/manage`, the only real-workspace action is **Sign in with Sociobot**. Clicking it in a fresh live Chromium context did not navigate and produced:

- `Connecting to 'https://sociobotcustomers.ciamlogin.com/.../.well-known/openid-configuration' violates ... connect-src 'self' https://api.sociobot.in`.
- `Fetch API cannot load ... Refused to connect because it violates the document's Content Security Policy.`
- Page error: `endpoints_resolution_error: Endpoints cannot be resolved`.

The live CSP and both repository CSP definitions omit `sociobotcustomers.ciamlogin.com` from `connect-src`. The page remained at `/manage`, so a seller cannot import a CSV, create a client link, or receive a real request.

With CSP bypassed only for diagnosis, the flow did reach the correct Entra tenant and client:

- Authority host: `sociobotcustomers.ciamlogin.com`
- Tenant: `35c6fe40-0ec0-46b6-98c6-213ad4de6650`
- Client: `25c704f4-465a-47af-80ab-2c489466b697`
- Redirect: `https://client-catalogue-request.sociobot.in/auth/callback`

That authorization request contained only `openid profile email offline_access`. The frontend later reads `AuthenticationResult.accessToken`, while the backend requires a bearer token with this application's audience. No product API scope is requested. After repairing CSP, exercise a real account and prove the returned bearer is accepted by `/api/admin/catalogue`; the current automated suite uses debug-only `x-test-seller` tokens and cannot prove this.

### High — the advertised paid purchase is dead

The landing page and seller workspace advertise a **₹1,499 one-time** full workspace. On 2026-08-29, a fresh request to the exact production link returned:

```text
GET https://api.sociobot.in/api/v1/products/client-catalogue-request/checkout
404
{"error":"enabled factory product","status":404}
```

The link crawl found this as the only dead public product link. The browser cannot buy the feature that raises the 12-row and one-link limits. This remains a release blocker even if factory registration, rather than repository code, owns the fix.

The `paid-license` claim test is also insufficient for its claim. It mocks a valid browser verification response and checks stored state, copy, and the checkout `href`; it never proves that a valid license permits a 13th row or second link at the backend. The release claim therefore lacks the observable test required by `.factory/claims.json`.

### High — an invalid quantity is silently changed and sent

In the live demo request dialog:

1. Entered `10000` in the quantity field, whose stated maximum is 9,999.
2. Moved focus away; the field still visibly showed `10000` and no error appeared.
3. Completed the contact fields and selected **Send quote request**.
4. The success screen appeared, while the stored/sent request line contained `9999`.

The same implementation changes `0` to `1`. The quantity input is outside `#request-form`, so its native validity does not stop submission. The change handler clamps the internal basket but does not update the visible value or explain the correction. A structured quote-request tool must not silently alter requested quantities.

## Other findings

### Medium — mobile touch targets and SPA focus handling miss the accessibility contract

- At 390 px, the landing link **Read how request data is handled** measured 273 × 25 CSS px.
- In the demo inbox, the client email link measured 165 × 19 CSS px.
- After keyboard-activating the header **Demo** link, `/demo` loaded and its live region announced `Northline Supply Co.`, but focus fell to `<body>` rather than moving to the new `<h1>` as required by the routing contract.

Positive evidence: the first Tab reaches the skip link; focus styling is a visible 3 px amber outline; the native dialog focuses its close button, closes with Escape, and restores focus; all selected controls in the repaired regression test meet 44 px; 320 px at 200% text has no horizontal overflow; reduced motion is active; and axe found no WCAG A/AA violations on `/`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, and `/manage` at 390 px.

### Medium — deployed transfer and LCP exceed the supplied budgets

Three fresh Lighthouse 13.0.1 mobile runs measured:

| Run | Performance | Accessibility | Best practices | SEO | FCP | LCP | CLS | TBT | Transfer |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92 | 100 | 100 | 100 | 2.40 s | 2.785 s | 0 | 117 ms | 366,095 B |
| 2 | 93 | 100 | 100 | 100 | 2.40 s | 2.704 s | 0 | 38 ms | 366,036 B |
| 3 | 93 | 100 | 100 | 100 | 2.464 s | 2.701 s | 0 | 16 ms | 366,050 B |

The 2.5 s LCP budget failed in all three runs. The production build reports 77.25 KB gzip JavaScript, but the live server sends no `Content-Encoding`; its initial JS response is 299,476 bytes, above the 200 KB transfer budget. Lighthouse identifies about 230 KiB of unused JavaScript, primarily from loading MSAL on the landing page.

## Mandatory first-read gate

**PASS.** In a fresh 1440 × 900 browser, without scrolling:

- What it does: **Turn repeat orders into clear requests**.
- For whom: **For small B2B sellers who need client orders without running an online store.**
- First click: **Try it with sample data**, beside **Opens a private sample catalogue. No setup.**
- Three visible facts cover checkout/card data, POA prices, and protected links.

One click opened `/demo`, immediately showed six realistic products, and displayed the persistent **Demo — sample data, nothing is saved** banner with **Reset demo** and **Start for real**.

## Claim gate

`.factory/claims.json` exists with 18 entries. The first literal invocations from the clean clone occurred before dependencies were installed and the JavaScript commands reported `vitest: not found`; `npm ci` then installed the lockfile and every listed command was rerun independently and verbatim. All 18 post-install commands exited 0.

| Claim | Exact command | Result |
|---|---|---|
| `demo-isolation` | `npm test -- --grep @claim:demo-isolation` | PASS — 2 projects |
| `poa-price` | `npm test -- --grep @claim:poa-price` | PASS — 2 projects |
| `csv-export` | `npm test -- --grep @claim:csv-export` | PASS — 2 projects |
| `structured-request` | `npm test -- --grep @claim:structured-request` | PASS — 2 projects |
| `no-card-data` | `npm test -- --grep @claim:no-card-data` | PASS — 2 projects |
| `protected-links` | `cargo test generated_tokens_are_long_and_distinct` | PASS — 1 Rust test |
| `csv-import` | `npm test -- --grep @claim:csv-import` | PASS — Chromium; intentional mobile duplicate skipped |
| `print-request` | `npm test -- --grep @claim:print-request` | PASS — Chromium; intentional mobile duplicate skipped |
| `paid-license` | `npm test -- --grep @claim:paid-license` | Command PASS; test does not prove the stated raised limits |
| `stock-privacy` | `cargo test stock_counts_are_not_exposed` | PASS — 1 Rust test |
| `privacy-runtime` | `npm test -- --grep @claim:privacy-runtime` | PASS — 2 projects |
| `demo-local` | `cargo test demo_is_stateless_across_backend_instances` | PASS — 1 Rust test |
| `demo-reset` | `npm test -- --grep @claim:demo-reset` | PASS — 2 projects |
| `csv-import-cap` | `cargo test catalogue_import_cap_is_five_thousand_rows` | PASS — 1 Rust test |
| `port-runtime` | `cargo test runtime_defaults_to_port_and_local_paths` | PASS — 1 Rust test |
| `client-data-control` | `npm test -- --grep @claim:client-data-control` | PASS — Chromium; intentional mobile duplicate skipped |
| `seller-tenancy` | `cargo test sellers_are_isolated` | PASS — 1 Rust test |
| `paid-license-invalid` | `npm test -- --grep @claim:paid-license-invalid` | PASS — 2 projects |

The manifest now covers the previous report's missing statements. The paid entitlement claim remains inadequately asserted as described above.

## Local build and automated checks

- Clean starting commit/status: exact candidate on `main`, no changes.
- `npm ci`: PASS; 64 packages installed, 0 audit vulnerabilities.
- `npm test`: PASS; 5 Vitest, 8 Rust, 28 Playwright; 4 intentional project-specific skips.
- `VITE_BUILD_SHA=5f9d6cc… npm run build`: PASS; TypeScript check and exact production Vite build completed and produced `dist/`.
- Build sizes: JS 299.48 KB raw / 77.25 KB gzip; CSS 19.46 KB raw / 5.09 KB gzip; two loaded WOFF2 files total 30.07 KB; mobile hero 14.02 KB.
- `BUILD_SHA=5f9d6cc… cargo build --release --locked`: PASS.
- `cargo fmt --check`: PASS.
- `cargo clippy --all-targets -- -D warnings`: PASS.
- `npm audit --omit=dev`: PASS; 0 vulnerabilities.
- `git diff --check`: PASS before documentation edits.
- Docker, Podman, and Buildah are unavailable in this worker, so the complete container image could not be built. Its frontend and locked release-server stages were built independently, and repository Dockerfile contract tests passed.

## End-to-end, boundaries, and persistence

The live demo passed normal and recovery flows on desktop and mobile:

- Six sample products load; POA and stock-note presentation are visible.
- Search, no-results copy, clearing/changing search, basket opening, contact validation, successful request creation, seller sample inbox, CSV export, and reset all work.
- A submitted one-line request appeared beside the seeded two-line request; exported CSV had one row per line and included the submitted PO and quantity.
- Only `demo:client-catalogue-request:requests` and `demo:client-catalogue-request:submitted` were written. Reset changed the workspace ID and removed the submitted key.
- Ten independent live `POST /api/demo` → `GET /api/demo/{id}/requests` pairs returned 201 → 200 with one seeded request. An invalid workspace ID returned the documented 404. The previous multi-replica loss is repaired.
- Direct live API checks rejected short identity, malformed email, quantity 0, quantity 10,000, duplicate lines, unknown products, and 101 lines with specific 400 responses; a subsequent valid request returned 201.

An isolated local real-workspace flow using the debug-only seller test hook verified the backend independently:

- Invalid currency: 400; duplicate SKU: 400.
- 13 rows without a license: 403; exactly 12 rows: 200.
- First client link: 200; second active link without a license: 403.
- Protected catalogue, POA item, and a two-line request with quantity 9,999: PASS.
- After graceful stop/restart against the same SQLite directory, 12 products, one link, one request, and quantity 9,999 remained.
- Link revocation returned 204 and the link then returned 404.
- Request deletion returned 204 and removed it from the inbox.
- A real invalid Sociobot license remained limited at 12 rows (403).

The production binary started and served `/health` with **only `PORT`** in an otherwise cleared environment. It created its default local data path, served the correct build SHA, returned a real 404, and applied immutable asset caching. One hundred concurrent local health requests all returned 200.

PWA offline/update and library/CLI packaging checks are not applicable: this is neither a PWA nor a library/CLI.

## Live identity, privacy, headers, caching, and limits

- `/health`: 200 with exact SHA `5f9d6cc53981ec923480a2f23bb3b2edbd4b87e9`.
- Candidate/live parity: exact byte matches for `index.html`, JS, CSS, fonts, both hero files, social card, favicon, apple-touch icon, CSV template, robots, and sitemap.
- `/opt/fleet/lib/verify-url.sh`: PASS in 678 ms with title, `lang=en`, one `<h1>`, `<main>`, alt text, button names, and no cold-load console errors.
- Unknown route: real HTTP 404 with the designed recovery page.
- Hashed asset: `Cache-Control: public, max-age=31536000, immutable`; conditional request returned 304.
- Root and API responses include CSP, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`. An arbitrary-origin API request received no CORS allowance.
- Landing and the complete demo flow requested only the product origin. No analytics, advertising, remote fonts, or runtime CDN requests occurred.
- A fresh invalid license was removed from the URL, checked only with `api.sociobot.in`, cached invalid, and produced the visible inactive-license notice without console errors.
- Product API read burst: 40 × 200 then 60 × 429, all 429 responses with `Retry-After: 1`.
- Product API write burst: 12 × 201 then 28 × 429, all 429 responses with `Retry-After: 1`.
- Sociobot license verification burst: 30 × 200 then 50 × 429, all 429 responses with `Retry-After: 4`.
- Health remains exempt, as the backend contract allows.

## Required remediation order

1. Make Entra sign-in work under the production CSP, request an actual API scope, and prove a real returned bearer opens the seller workspace.
2. Register/enable the production billing product and replace the paid claim test with a backend-observable 13-row and second-link entitlement test.
3. Reject an out-of-range quantity with an announced error, or visibly normalize it before submission; never send a value different from the value shown.
4. Move focus to the new page heading on client-side navigation and enlarge the remaining mobile inline-link targets.
5. Avoid loading the full MSAL bundle on the landing/demo paths and enable gzip or Brotli so transfer and LCP meet budget.
