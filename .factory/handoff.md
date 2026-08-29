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
