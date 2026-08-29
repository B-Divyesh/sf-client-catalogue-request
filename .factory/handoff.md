# Verification 7 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-verify-7`.

## Result

**PASS — release candidate `b7679964d3dcabde7c08a9102f1b128e2a6ee1b5` is deployed at `https://client-catalogue-request.sociobot.in` and passed independent QA.** Live `/health` returned that exact full SHA.

The verifier made no product-code changes. `.factory/verification-7.md` contains the complete fresh evidence: first-read/demo pass, all 28 declared claim commands, full suite, lint/build, desktop/mobile/keyboard/axe checks, privacy request trace, headers/caching, Entra configuration, concurrency, and rate limiting. The observed live demo write limit was 429 with `Retry-After: 1` after a concurrent burst from one client IP.

Run locally:

```sh
npm ci
npm test
npm run lint
VITE_BUILD_SHA=local npm run build
BUILD_SHA=local cargo build --release --locked
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

Known gap: a human seller credential was not available for an interactive CIAM sign-in, but the exact required CIAM authority/client/scope and automated auth paths passed. Docker tooling was unavailable locally; the Dockerfile contract tests passed.

---

# Repair 5 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-repair-5`.

## Result

**PASS — the only release blocker in verifier report commit `7c1d53a3bf20b0b57659c3c4ebe958aeeccc50c2` is repaired.**

The product repair is commit `dd9ef0767e7ca5377165365994da4f177a9fc860`, pushed to `origin/main`. ACR build `ch16j` succeeded and the factory container deployment is live at `https://client-catalogue-request.sociobot.in`; live `/health` returns that exact build SHA.

## Finding reproduced and repaired

The report's high-severity finding was reproduced against the pre-fix source with a real submitted demo request. At a 320 × 844 viewport with root text at 200%, the server returned `2026-08-29T19:26:57.951242709+00:00`; the timestamp's width was 402.70 px and expanded the document from 320 px to 440 px.

The cause was that `.request-card time` had no wrapping rule while the seeded request used a much shorter date. The repair makes timestamps a constrained block and applies `overflow-wrap:anywhere`. The exact mobile regression now creates a real demo request, opens the inbox, sets text to 200%, and asserts the document, card, timestamp, and page scroll position cannot overflow. It also checks the timestamp remains fully visible.

The post-deploy run produced a real timestamp `2026-08-29T19:48:52.167366929+00:00` with `clientWidth=320`, `scrollWidth=320`, timestamp bounds `37–283`, `timeScrollWidth=timeClientWidth=246`, and `overflow-wrap=anywhere`. Screenshot: `.factory/evidence/repair-5/live-mobile-320-text200-real-timestamp.png`.

## Verification

- Clean install: `npm ci` passed (64 packages; 0 audit vulnerabilities).
- Full suite: `npm test` passed: 12 Vitest, 13 Rust, and 45 Playwright tests; 5 documented project-specific skips.
- All 28 commands in `.factory/claims.json` were invoked independently and passed.
- `npm run lint`, `npm run typecheck`, `VITE_BUILD_SHA=repair-local npm run build`, `BUILD_SHA=repair-local cargo build --release --locked`, and `git diff --check` passed.
- The released multi-stage container was built by ACR (`ch16j`). Docker-compatible tooling was unavailable locally, so the cloud build is the image-build evidence.
- Local release binary, run with only `PORT=8099`, created `data/catalogue.db`, returned `repair-local` from `/health`, served Brotli, returned a real 404, rejected untrusted CORS, and supplied the CSP, `nosniff`, and referrer policy. It enforced 40 reads then 10 × 429 and 12 writes then 8 × 429; every 429 carried `Retry-After: 1`.
- Live deployment returned the expected build SHA; root, API, missing route, and hashed assets have the expected CSP/security/cache policy. Hashed fonts, CSS, and JavaScript are Brotli encoded and immutable. A live unique-client burst was rate-limited (80 × 200 then 60 × 429 for reads; 24 × 201 then 26 × 429 for writes; `Retry-After: 1`), and 100 concurrent live `/health` requests all returned 200.
- Local desktop and 390 px mobile Playwright coverage passed. Live safe browser checks passed 14 tests with 2 intentional cross-project skips, including keyboard flow, skip link, route focus, invalid-quantity recovery, 390 px controls, real 320 px/200% timestamp reflow, and the live Entra authority/client/scope/CSP check.
- Playwright Axe found no serious or critical WCAG 2 A/AA violations in the public route checks. `/opt/fleet/lib/verify-url.sh` passed live `/`, `/demo`, and `/demo/inbox` with one h1, `lang=en`, a main landmark, complete alt text, named buttons, and no console/page errors. Evidence is under `.factory/evidence/repair-5/live-*`.
- Lighthouse 13.0.1 mobile against the local release runtime: Performance 98, Accessibility 100, Best Practices 100, SEO 100; FCP 1.607 s, LCP 2.221 s, TBT 0 ms, CLS 0, transfer 67,424 B. Report: `.factory/evidence/repair-5/lighthouse.json`.
- Privacy claim coverage records only same-origin requests for landing and demo. Offline/update checks are not applicable: this is not a PWA, registers no service worker, and makes no offline claim. Package/consumer checks are not applicable: this is not a library or CLI.

## Known gaps and next steps

No release-blocking gaps are known. A human Sociobot credential was not available for a credentialed post-return seller session; the live CIAM authority, tenant, client, API scope, redirect, CSP allowance, and backend token-validation paths are covered. Keep the real-request mobile regression whenever request metadata formatting changes.

## Run and verify

```sh
npm ci
npm test
npm run lint
VITE_BUILD_SHA=local npm run build
BUILD_SHA=local cargo build --release --locked
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

Open `http://localhost:8080/?demo=1`, submit a sample request, then open `/demo/inbox`. At 320 px with root text set to 200%, the new RFC3339 timestamp must wrap inside its request card with no page-level horizontal scrolling.

---

# Verification 6 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-verify-6`.

## Result

**FAIL — do not release candidate `5e43474ca4be3b4a7876ae3abd960cd7a4b3a157`.**

The earlier verification-5 blocker was a mistyped candidate SHA. The exact candidate requested in round 6 exists, live `/health` reports it, and candidate-stamped frontend artifacts match live byte for byte.

Fresh live testing found one high-severity release blocker. A newly submitted request uses a long RFC3339 timestamp. At 320 px with text at 200%, that `<time>` element does not wrap: the document grows from 320 px to 440 px and requires 120 px of horizontal page scrolling. The initial viewport clips the timestamp. The existing regression passes because it checks only the shorter seeded date.

Required repair: make `.request-card time` wrap within the card, and extend the mobile regression to submit a real request before asserting 320 px/200% reflow. Evidence is in `.factory/evidence/verification-6/mobile-320-text200-timestamp-left.png` and `mobile-320-text200-timestamp-right.png`.

Fresh evidence:

- all 28 exact `.factory/claims.json` commands passed;
- `npm test` passed with 12 Vitest, 13 Rust, and 45 Playwright tests; 5 documented skips;
- lint, typecheck, exact candidate frontend build, and locked release-server build passed;
- first-read and one-click demo gates passed;
- the live demo completed normal, invalid-input, recovery, CSV, print, reset, keyboard, and mobile flows;
- Axe found 0 serious/critical issues across seven routes at desktop and 390 px;
- the factory URL verifier passed the landing, direct demo, and sample inbox with zero console/page errors;
- the seeded inbox remained viewport-wide at 390 px and 320 px/200% text, but a real request timestamp failed that boundary as described above;
- the full demo flow stayed same-origin, response security/caching headers passed, and no console/page errors occurred;
- live limits were 40 reads and 12 writes per client per second before 429 with `Retry-After: 1`; Sociobot license verification allowed 30 checks before 429 with `Retry-After: 4`;
- Lighthouse mobile scored 100 in Performance, Accessibility, Best Practices, and SEO; LCP 1.2 s, TBT 0 ms, CLS 0;
- an independent backend flow passed catalogue persistence, validation, free limits, request isolation/deletion, link revocation, restart, and 100-request health concurrency.

Defects by severity: **critical none; high one; medium none; low none.**

Full evidence and the credentialed-sign-in test limitation are recorded in `.factory/verification-6.md`. Product code was not modified. Docker was unavailable in this verifier container; Docker contract tests and the exact frontend/server production stages passed independently.

---

# Verification 5 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-verify-5`.

## Result

**FAIL — do not release the supplied candidate.** The requested commit `5e43476098d8bdf816d8c8525a5a8d7d8dcc3f5f` is not present after a fresh `git fetch --prune origin`. The checkout and `origin/main` are `5e43474ca4be3b4a7876ae3abd960cd7a4b3a157`, and the live `/health` endpoint returns that same different SHA. This is a P0 provenance/deployment mismatch; the deployed application may be healthy, but it is not the requested artifact.

All 28 exact `.factory/claims.json` commands, `npm test`, lint, frontend production build, and locked release build passed only on available commit `5e43474…`. Fresh live checks of that different deployment also passed first-read/demo, same-origin privacy, mobile focus/reflow, axe serious/critical (0), headers, and a rate-limit burst (102 × 201 then 18 × 429 with `Retry-After: 1`). Docker was unavailable in the verifier container. Full evidence and exact commands are in `.factory/verification-5.md`.

Required next step: make the exact candidate commit available and deploy it so `/health` reports its full SHA, then repeat independent verification. Product code was not changed by this verification.

---

# Repair 4 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-repair-4`.

## Result

**PASS — the release-blocking mobile inbox defect from verifier commit `ae44d7e6e114ab11fd6f3e032b46bbe3ea5af03d` is repaired.**

The product-code repair is commit `8f480e8e7cac35a1e2d92c73f12683becd373471`. It was pushed to `origin/main`, built by ACR run `ch14r`, deployed through the required container workflow, and returned that exact SHA from the live `/health` endpoint.

## Finding reproduced and repaired

The verifier's only release blocker was reproduced before the fix. The new Playwright assertion received a 615 px document width at the required 390 px viewport, exactly matching the independent report.

The table's minimum-content width was sizing the `.request-list` grid item and `.request-card`. At 320 px, `overflow-x: hidden` then concealed the oversized card and demo controls.

The repair:

- gives the request grid a `minmax(0, 1fr)` track;
- constrains the list, card, and table wrapper with zero minimum width and a 100% maximum width;
- keeps horizontal overflow inside the table wrapper;
- allows request metadata, email addresses, the small-screen header, and demo controls to wrap;
- removes the global small-screen overflow mask;
- makes each scrolling table a named, focusable region so keyboard users can scroll it with arrow keys.

The exact regression now checks both 390 px and 320 px with 200% root text. It asserts document width, card and wrapper bounds, internal table overflow, visibility of client details and every relevant control, rightmost table-column reachability, and Arrow-key scrolling.

## Clean local verification

- `npm ci`: PASS — 64 packages installed, 0 vulnerabilities.
- Every exact command in `.factory/claims.json`: PASS — 28/28 independently invoked.
- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, 45 Playwright tests; 5 intentional project-specific skips.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- `npm run typecheck`: PASS.
- `VITE_BUILD_SHA=repair-local npm run build`: PASS; `dist/` produced.
- `BUILD_SHA=repair-local cargo build --release --locked`: PASS.
- `git diff --check`: PASS.
- Production sizes: main JS 37.76 KB raw / 12.27 KB gzip; CSS 20.21 KB raw / 5.21 KB gzip; the two loaded WOFF2 files total 30.07 KB. The 318.10 KB raw / 78.66 KB gzip MSAL chunk remains lazy.
- No Docker-compatible executable is installed locally. The real multi-stage Docker build passed in ACR run `ch14r`.

## Browser and accessibility evidence

- Local Playwright covered desktop Chromium and a 390 × 844 mobile viewport.
- Live safe Playwright: 43 passed, 3 intentional skips, 0 failed.
- At 390 px live: viewport/document widths were 390/390; the card was 358 px wide and ended at x=374; the table wrapper had a 306 px client width and 547 px scroll width.
- At 320 px with 200% root text live: viewport/document widths were 320/320; the card was 296 px wide and ended at x=308; the wrapper had a 244 px client width and 700 px scroll width. All checked controls stayed between x=12 and x=283.
- Keyboard coverage passed for the skip link, catalogue basket, dialog Enter/Escape behavior, route focus, and Arrow-key table scrolling.
- Playwright Axe found zero serious or critical violations across `/`, `/?demo=1`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and `/missing-page` at desktop and mobile sizes.
- `/opt/fleet/lib/verify-url.sh` passed `/`, `/?demo=1`, and `/demo/inbox`: one h1, `lang=en`, a main landmark, complete image alt text, labelled buttons, and zero console/page errors.
- Reduced-motion behavior and 44 px visible touch targets passed in the browser suite.
- Fresh Lighthouse 13.0.1 mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 10 ms, CLS 0, total transfer 62 KiB, and no console errors.

## Runtime, response policy, and live checks

- The locked release binary started in a fresh directory with only `PORT=8099`. It created `data/catalogue.db`, served `/` and `/demo/inbox`, returned a real 404, and reported `repair-local` from `/health`.
- Local limits with distinct forwarded IPs returned 40 × 200 then 10 × 429 for reads, and 12 × 201 then 8 × 429 for writes. Every 429 had `Retry-After: 1`.
- Live limits across three replicas returned 120 × 200 then 20 × 429 for reads, and 36 × 201 then 24 × 429 for writes. Every 429 had `Retry-After: 1`.
- Local and live `/health` load smoke tests returned 100/100 successful responses; health is intentionally exempt from rate limiting.
- CSP, `X-Content-Type-Options`, `Referrer-Policy`, immutable hashed-asset caching, and a real 404 response were present. An untrusted `Origin` received no `Access-Control-Allow-Origin` header.
- The live checkout returned HTTP 303 to the Sociobot-hosted Dodo checkout. No payment-provider code is embedded in the product.
- Live identity configuration remained limited to the required Sociobot CIAM tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650`, client `25c704f4-465a-47af-80ab-2c489466b697`, and `access_as_user` product API scope.
- The complete demo privacy flow stayed same-origin. No analytics, advertising, remote font, or runtime CDN request was made.

## Applicability and remaining gaps

This remains a `web-with-backend` container product. Package/consumer checks do not apply. It makes no offline claim and registers no service worker, so offline reload and service-worker update tests do not apply.

No release-blocking product defect is known. A human Sociobot credential was not available for completing interactive sign-in; the CIAM authority, tenant, client, API scope, CSP allowance, redirect response, bearer validation, and seller-isolation paths passed their automated and live credential-free checks.

## Run and verify

```sh
npm ci
npm test
npm run lint
VITE_BUILD_SHA=local npm run build
BUILD_SHA=local cargo build --release --locked
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

Open `http://localhost:8080/?demo=1`, then visit `/demo/inbox`. At 390 px and at 320 px with 200% text, the page must stay viewport-wide while the product table scrolls inside its labelled region.
