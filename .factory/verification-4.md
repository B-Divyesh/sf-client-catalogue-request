# Independent product verification — FAIL

Verified: 2026-08-29 UTC

Work order: `client-catalogue-request-verify-4`

Candidate: `0505e44271d0530b78b3f601b79fd515c55c4298`

Live URL: <https://client-catalogue-request.sociobot.in>

Acceptance contract: researched brief, `.factory/brief.json`, `AGENTS.md`, and the supplied factory skills.

## Verdict

**FAIL — do not release this candidate.**

The deployment-only failures reported in older verification rounds are not present: the live service is the candidate, the first-read and claim gates pass, checkout works, the demo completes, Entra uses the required tenant, limits return 429, and all local quality gates pass. A fresh mobile audit nevertheless found a release-blocking reflow defect in the seller sample inbox. At the required 390 px viewport, `/demo/inbox` expands to 615 px. At 320 px with text at 200%, the 752 px request card is clipped by `overflow-x: hidden`, hiding request fields and demo controls. This violates the explicit mobile and 200% text-resize acceptance requirements in the core structured-request handoff screen.

## Release-blocking finding

### High — the request inbox does not reflow on mobile and loses content at 200% text size

Fresh live Chromium evidence:

- At a 390 × 844 viewport, `documentElement.clientWidth` was 390 px and `scrollWidth` was 615 px: **225 px of page-level horizontal overflow**.
- The `.request-card` ran from x=16 to x=614.91 and measured 598.91 px wide. Its table wrapper measured 548.91 px, so the wrapper itself did not constrain the grid item's minimum width.
- The visible page consequently has a wide blank strip and requires horizontal page scrolling. This affects the screen that proves the seller received structured SKUs, quantities, PO details, notes, and prices.
- At 320 px with the root text size set to 200%, the request card measured 752.02 px wide and ended at x=764.02. Both `html` and `body` had `overflow-x: hidden`; the page reported only 320 px scroll width. Client details, product names, unit prices, and the **Start for real** demo action were visibly clipped and could not be reached by horizontal scrolling.
- The repository regression named `mobile controls meet touch and text-reflow boundaries` only checks `/`, `/demo`, and individual inbox target height. It never checks inbox width or 200% reflow, so the full suite passes while this defect remains.
- Axe found no serious/critical rule finding because this failure needs an explicit reflow/geometry check.

Evidence:

- [390 px inbox, left edge](evidence/verification-4/mobile-inbox-left.png)
- [390 px inbox after horizontal page scroll](evidence/verification-4/mobile-inbox-right.png)
- [320 px inbox at 200% text](evidence/verification-4/mobile-inbox-320-200pct.png)

Likely cause: `.request-list` is a grid whose child retains the table's minimum-content width. `.table-wrap { overflow: auto }` cannot help because its containing request card has already expanded. Constrain the grid/card/wrapper with `min-width: 0`, keep table overflow inside the wrapper, and add regression assertions for document width and visible content on `/demo/inbox` at 390 px and 320 px/200%.

## Mandatory first-read and demo gate

**PASS.** A cold 1440 × 900 load answered all three questions without scrolling:

- What: **“Turn repeat orders into clear requests.”**
- For whom: **“For small B2B sellers who need client orders without running an online store.”**
- First action: **“Try it with sample data”**, alongside “Opens a private sample catalogue. No setup.”

One click opened `/?demo=1`, immediately showing the six-product Northline Supply Co. catalogue and the persistent **Demo — sample data, nothing is saved** banner with **Reset demo** and **Start for real**. Evidence: [cold first screen](evidence/verification-4/first-read-desktop.png).

## Claims gate

`.factory/claims.json` exists and contains 28 unique claims. After `npm ci`, every declared command was invoked independently from the candidate checkout. Result: **28 passed, 0 failed**. The manifest integrity test also confirms every declared `@claim` tag or Rust function occurs exactly once and that there are no undeclared test tags.

| Claim | Declared command | Result |
|---|---|---|
| `demo-isolation` | `npm test -- --grep @claim:demo-isolation` | PASS |
| `demo-entry` | `npm test -- --grep @claim:demo-entry` | PASS |
| `demo-sample-content` | `npm test -- --grep @claim:demo-sample-content` | PASS |
| `poa-price` | `npm test -- --grep @claim:poa-price` | PASS |
| `csv-export` | `npm test -- --grep @claim:csv-export` | PASS |
| `free-export` | `npm test -- --grep @claim:free-export` | PASS |
| `structured-request` | `npm test -- --grep @claim:structured-request` | PASS |
| `no-card-data` | `npm test -- --grep @claim:no-card-data` | PASS |
| `service-boundaries` | `npm test -- --grep @claim:service-boundaries` | PASS |
| `protected-links` | `cargo test generated_tokens_are_long_and_distinct` | PASS |
| `csv-import` | `npm test -- --grep @claim:csv-import` | PASS; one intentional duplicate-project skip |
| `csv-header-normalization` | `npx vitest run --testNamePattern @claim:csv-header-normalization` | PASS |
| `print-request` | `npm test -- --grep @claim:print-request` | PASS; one intentional duplicate-project skip |
| `paid-license` | `npm test -- --grep '@claim:paid-license(?!-invalid)'` | PASS; one intentional duplicate-project skip |
| `billing-handoff` | `npm test -- --grep @claim:billing-handoff` | PASS |
| `stock-privacy` | `cargo test stock_counts_are_not_exposed` | PASS |
| `privacy-runtime` | `npm test -- --grep @claim:privacy-runtime` | PASS |
| `demo-local` | `cargo test demo_is_stateless_across_backend_instances` | PASS |
| `demo-reset` | `npm test -- --grep @claim:demo-reset` | PASS |
| `csv-import-cap` | `cargo test catalogue_import_cap_is_five_thousand_rows` | PASS |
| `port-runtime` | `cargo test runtime_defaults_to_port_and_local_paths` | PASS |
| `runtime-storage` | `cargo test runtime_creates_and_reopens_sqlite_storage` | PASS |
| `container-runtime` | `npx vitest run --testNamePattern @claim:container-runtime` | PASS |
| `health-build` | `cargo test health_returns_supplied_build_sha` | PASS |
| `browser-storage` | `npm test -- --grep @claim:browser-storage` | PASS |
| `client-data-control` | `npm test -- --grep @claim:client-data-control` | PASS; one intentional duplicate-project skip |
| `seller-tenancy` | `cargo test sellers_are_isolated` | PASS |
| `paid-license-invalid` | `npm test -- --grep '@claim:paid-license-invalid'` | PASS |

Landing, legal-page, demo, seller-page, README, runtime, and deployment statements were cross-checked against the manifest. No unlisted relied-upon product claim was found.

## Clean install, tests, lint, and builds

- Starting checkout: clean `main` at the exact candidate SHA.
- `npm ci`: PASS — 64 packages installed, 0 audit vulnerabilities.
- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, and 45 Playwright tests; 5 intentional project-specific skips.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- `VITE_BUILD_SHA=0505e44271d0530b78b3f601b79fd515c55c4298 npm run build`: PASS; `dist/` produced.
- `BUILD_SHA=0505e44271d0530b78b3f601b79fd515c55c4298 cargo build --release --locked`: PASS.
- `git diff --check`: PASS before verification-document changes.
- Candidate build sizes: entry JS 37.66 KB raw / 12.28 KB gzip; CSS 19.69 KB raw / 5.12 KB gzip; two loaded WOFF2 fonts total 30.07 KB. The 318.10 KB raw / 78.66 KB gzip MSAL chunk is lazy and was not requested on landing/demo.
- No Docker, Podman, or Buildah executable exists in this worker. The exact Vite and locked release-server stages passed independently, and all four Docker contract tests passed. The container image itself could not be rebuilt here.

## Runtime and live build identity

- `/health` returns `{"build_sha":"0505e44271d0530b78b3f601b79fd515c55c4298","ok":true}`. Evidence: [health body](evidence/verification-4/health.json) and [headers](evidence/verification-4/health-headers.txt).
- Candidate-stamped local and live `index.html` matched byte-for-byte, SHA-256 `59ba9c93d6652025776f15ff1fd8b10fba084f72049b75ea45edf73d19d7f4a2`.
- Candidate-stamped local and live entry JS matched byte-for-byte, SHA-256 `a1f5d44b7e31caf7bf953fa3d1b8f7491b1f4163fd82afcf2572d68d40fba2ef`.
- The release binary started in a fresh directory with only `PORT=8099`. It created `data/catalogue.db`, served `/` and `/health`, returned a real 404 for an unknown route, compressed assets, and returned the candidate SHA. Startup emitted structured JSON without a secret value.
- The persistence claim created every seller table, wrote representative seller/catalogue/link/request rows, closed SQLite, reopened it, and read the rows back.

## End-to-end and validation evidence

The complete local real-workspace browser claim passed: CSV import, catalogue save, protected client-link creation, client request, seller inbox receipt, and seller deletion. The paid claim used a recorded valid Sociobot verdict and proved 13 rows plus two active links through the real backend. Seller-separation, free limits, revocation, and persistence tests also passed.

Fresh live demo flow at 390 px:

- First Tab focused **Skip to main content** with a 3 px amber outline.
- Keyboard Enter added a product and opened the request dialog; Escape/focus behavior also passes the live suite.
- Quantity `0` was rejected with the announced message “Check each quantity. Enter a whole number from 1 to 9,999.” Correcting it to `25` produced a receipt and preserved 25.
- The seller sample then displayed two requests. Reset changed the workspace ID and removed the submitted-request key. **Start for real** removed every `demo:` key.
- A direct live demo API matrix returned 400 for one-character name, malformed email, quantities 0 and 10,000, duplicate product IDs, an unknown product, and 101 lines. Quantity 9,999 returned 201 and remained 9,999 in the structured response.

No human Sociobot credential was supplied. Therefore a real user could not complete the last credentialed sign-in step. The browser did prove the sole app identity authority is `sociobotcustomers.ciamlogin.com`, tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650`, client `25c704f4-465a-47af-80ab-2c489466b697`, scope `api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user`, and callback `/auth/callback`. No alternative identity provider is offered by the app.

The public purchase URL returned HTTP 303 to the Sociobot-hosted Dodo checkout. The product contains no card fields or payment-provider script.

## Accessibility and browser behavior

- Factory `verify-url.sh` passed `/` and `/?demo=1`: titles, `lang=en`, one h1, main landmark, image alt text, labelled controls, and zero console/page errors. Evidence: [root](evidence/verification-4/verify-root.json) and [demo](evidence/verification-4/verify-demo.json).
- Safe live Playwright suite: 43 passed, 3 intentional skips across desktop and 390 px mobile.
- Independent Axe WCAG 2 A/AA audit: zero serious/critical findings on `/`, `/?demo=1`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and `/missing-page` at desktop and 390 px.
- Successful routes produced no console/page errors. Direct navigation to the intentional 404 produced only Chromium's expected failed-main-resource console line.
- Visible mobile targets on the audited routes measured at least 44 × 44 CSS px.
- `prefers-reduced-motion: reduce` matched and left no active animations in the exercised flow.
- The release-blocking reflow failure is documented above.

## Privacy, security headers, caching, and links

- A complete fresh demo flow recorded only `https://client-catalogue-request.sociobot.in` requests, including its POST/GET/DELETE demo API calls. No analytics, ads, remote fonts, or runtime CDN request occurred.
- Root, health, API, asset, and 404 responses include CSP, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`. CSP allows only the product, Sociobot API, and required CIAM host for app runtime connections.
- An API request carrying `Origin: https://evil.example` received no `Access-Control-Allow-Origin` header.
- Hashed JS, CSS, fonts, and images return `Cache-Control: public, max-age=31536000, immutable`. Brotli is enabled for text assets.
- Every discovered product link was checked. Internal public routes returned 200, checkout returned 303, Sociobot returned 200, and the designed missing route returned 404 as intended.
- `robots.txt` and `sitemap.xml` return 200; the sitemap includes `/`, `/demo`, `/demo/inbox`, `/privacy`, and `/terms`.

## Rate limiting and concurrency

The live deployment currently has three replicas. Fresh same-client bursts with a unique forwarded IP observed:

- 60 concurrent `POST /api/demo`: 36 × 201, then 24 × 429; every limited response had `Retry-After: 1`.
- 140 concurrent `GET /api/auth/config`: 120 × 200, then 20 × 429; every limited response had `Retry-After: 1`.

This matches the implemented per-replica allowance of 12 writes and 40 reads per second, or an observed deployment-wide burst of 36 writes and 120 reads across three replicas. `/health` is intentionally exempt. The API therefore satisfies the required 429 plus `Retry-After` behavior, though the effective client allowance scales with replica count.

## Performance

Fresh Lighthouse 13.0.1 mobile result: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 30 ms, CLS 0, total transfer 62 KiB, and no console errors. Evidence: [Lighthouse JSON](evidence/verification-4/lighthouse.json).

Direct Brotli transfer sizes were 12,429 B entry JS, 5,311 B CSS, 30,071 B combined WOFF2 fonts, and 14,024 B mobile hero. They satisfy the supplied budgets.

## Applicability

This is not a library or CLI, so consumer package installation does not apply. It is not a PWA and registers no service worker, so offline reload and service-worker update checks do not apply. There is no AI feature; the brief does not imply one that would improve the smallest useful workflow.

## Defects by severity

- **Critical:** none.
- **High:** mobile inbox page-level overflow and unrecoverable clipping at 200% text size.
- **Medium:** none separate from the high finding.
- **Low:** none.

## Required remediation

Constrain the request-list grid and card to the viewport, keep the data table's horizontal scrolling inside `.table-wrap`, and prove no content/control is lost on `/demo/inbox` at both 390 px and 320 px with 200% text. Add those exact geometry and visibility checks to the mobile Playwright regression, then rerun every claim, the complete suite, and live mobile verification.
