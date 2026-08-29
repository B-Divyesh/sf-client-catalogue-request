# Independent product verification 6 — Client Catalogue Request

Verified 2026-08-29 UTC for work order `client-catalogue-request-verify-6`.

## Verdict: FAIL

**Do not release candidate `5e43474ca4be3b4a7876ae3abd960cd7a4b3a157`.** The live product at <https://client-catalogue-request.sociobot.in> has a high-severity text-reflow defect in the core seller inbox.

The prior deployment-only failure does not reproduce. It named a mistyped, unavailable SHA (`5e434760…`). The candidate requested in this round exists, the live `/health` response reports its full SHA, and the candidate-stamped local `index.html`, entry JavaScript, CSS, and fonts match the deployed files byte for byte.

## Release-blocking finding

### High — a real request timestamp breaks 200% mobile reflow

After submitting a valid request through the live demo, its `created_at` value is rendered as a long RFC3339 timestamp such as `2026-08-29T18:53:30.577820572+00:00`. At a 320 × 844 viewport with root text set to 200%:

- the document `clientWidth` is 320 px but `scrollWidth` is 440 px;
- the `<time>` element runs from x=37 to x=439.70 and is 402.70 px wide;
- its computed `overflow-wrap` is `normal`;
- the initial viewport clips the end of the timestamp, and reaching it requires 120 px of horizontal page scrolling.

This violates the supplied accessibility baseline that text must resize to 200% without loss and the product's explicit mobile reflow requirement. It affects metadata for a real newly submitted quote request on the seller's core handoff screen.

The existing regression did not catch the defect because `/demo/inbox` starts with the shorter seeded date `2026-08-28 09:14`. The product CSS applies `overflow-wrap:anywhere` to request paragraphs, links, and metadata descriptions, but not to `.request-card time`.

Evidence: [left edge with clipped timestamp](evidence/verification-6/mobile-320-text200-timestamp-left.png) and [right edge after 120 px page scroll](evidence/verification-6/mobile-320-text200-timestamp-right.png).

## Mandatory first-read and demo gate

**PASS.** A cold 1440 × 900 visit answers all three required questions before scrolling:

- What it does: **“Turn repeat orders into clear requests.”**
- For whom: **“For small B2B sellers who need client orders without running an online store.”**
- What to do first: **“Try it with sample data.”**

The action is visible above the fold. One click opens `/?demo=1`, immediately shows the six-product Northline Supply Co. catalogue, and displays the persistent **“Demo — sample data, nothing is saved”** banner with **Reset demo** and **Start for real**. The cold load and click produced no console or page errors.

## Claims gate

`.factory/claims.json` exists with 28 unique claims. After `npm ci`, every listed `test` command was run independently and verbatim from the candidate checkout. Result: **28 passed, 0 failed**.

| Claim | Exact command | Result |
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
| `csv-import` | `npm test -- --grep @claim:csv-import` | PASS |
| `csv-header-normalization` | `npx vitest run --testNamePattern @claim:csv-header-normalization` | PASS |
| `print-request` | `npm test -- --grep @claim:print-request` | PASS |
| `paid-license` | `npm test -- --grep '@claim:paid-license(?!-invalid)'` | PASS |
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
| `client-data-control` | `npm test -- --grep @claim:client-data-control` | PASS |
| `seller-tenancy` | `cargo test sellers_are_isolated` | PASS |
| `paid-license-invalid` | `npm test -- --grep '@claim:paid-license-invalid'` | PASS |

The landing, demo, legal pages, README, runtime documentation, and deployment statements were cross-checked against the manifest. No unlisted relied-upon product claim was found. The manifest test also proves each declared claim maps to exactly one tagged browser/unit test or named Rust test.

## Clean checkout and production gates

- `npm ci`: PASS — 64 packages, 0 audit vulnerabilities.
- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, and 45 Playwright tests; 5 documented project-specific skips.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- `npm run typecheck`: PASS.
- `VITE_BUILD_SHA=5e43474ca4be3b4a7876ae3abd960cd7a4b3a157 npm run build`: PASS; `dist/` produced.
- `BUILD_SHA=5e43474ca4be3b4a7876ae3abd960cd7a4b3a157 cargo build --release --locked`: PASS.
- `git diff --check`: PASS before verification-document changes.
- No Docker, Podman, or Buildah executable is installed in this verifier container. The Docker contract tests passed, and the exact frontend and locked server production stages passed independently; the image wrapper was not rebuilt here.

## Product and backend behavior

The locked release binary started in a fresh directory with only `PORT=8099`. It created `data/catalogue.db`, served the app, returned a real 404 for an unknown route, and returned the exact candidate SHA from `/health`. It restarted against the same unchanged database successfully. A 100-request local health concurrency smoke returned 100 × 200.

An independent debug-backend seller flow exercised the real API without a human credential:

- saved a representative configurable catalogue with a POA item and stock note;
- rejected a two-letter currency and duplicate SKU, then preserved the prior valid catalogue;
- rejected row 13 and client link 2 without a license, and rejected a 5,001-row import;
- created a 28-character protected client link without exposing stock counts or link tokens in the client catalogue;
- rejected malformed email and quantity 10,000, then accepted quantity 9,999;
- stored the structured SKU, quantity, PO number, contact, and note only in the correct seller inbox;
- kept a second seller's inbox empty;
- deleted the request, revoked the link, and returned clear 404 recovery messages on repeated actions;
- retained the catalogue and inactive link, and retained the deleted-request boundary, after process restart.

The release deployment rejects missing auth, `X-Test-Seller`, and fake `test-seller:` bearer values with 401. Interactive sign-in redirects only to `sociobotcustomers.ciamlogin.com`, tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650`, client `25c704f4-465a-47af-80ab-2c489466b697`, scope `access_as_user`, and the product callback. The CIAM page returned 200 without an AADSTS or CSP error.

## Live end-to-end and boundary evidence

In a fresh 390 px browser context, keyboard input added a product and opened the review dialog. The form rejected quantities 0 and 10,000 with an announced whole-number error and did not write sample storage. Changing the quantity to 25 recovered successfully, produced a request receipt, and displayed the request's email, PO number, note, SKU, quantity, and price in the two-request seller inbox.

The seeded request exported as `sample-quote-requests.csv` with one header and two product rows, including `NW-101`, `PK-228`, and POA. Print selected exactly one request and invoked the browser print path. Reset removed the submitted request and provisioned a fresh sample workspace; Start for real removed all `demo:` keys.

Direct API boundaries also accepted quantity 9,999 and rejected one-character names, malformed email, quantities 0 and 10,000, duplicate products, unknown products, and more than 100 lines. A valid request immediately after failures returned 201.

## Accessibility, mobile, and motion

Independent Playwright Axe WCAG 2 A/AA scans covered `/`, `/?demo=1`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and `/missing-page` at 1440 px and 390 px. All 14 standard seeded route/viewport combinations had **0 serious or critical findings**, one `<h1>`, one `<main>`, ordered headings, `lang=en`, no unexpected console/page errors, and no page-level horizontal overflow. Every visible interactive target measured at least 44 × 44 CSS px on mobile. Axe does not detect the real-request timestamp reflow defect described above.

The factory `verify-url.sh` also passed `/`, `/?demo=1`, and `/demo/inbox`: HTTP 200, title, `lang=en`, one h1, main landmark, complete image alt text, labelled buttons, and zero console/page errors. Its measured cold loads were 603 ms, 587 ms, and 595 ms respectively. Evidence: [landing](evidence/verification-6/verify-root/verify.json), [demo](evidence/verification-6/verify-demo/verify.json), and [inbox](evidence/verification-6/verify-inbox/verify.json).

Keyboard checks passed for the visible 3 px amber skip-link focus ring, Space/Enter basket operation, initial dialog focus, Escape close and focus return, history navigation focus, and Arrow-key scrolling of wide request tables. Reduced-motion emulation matched and left no active animations.

The seeded inbox geometry is confirmed live:

- 390 px viewport: document 390 px, request card x=16–374, table viewport 306 px with 547 px scrollable content.
- 320 px viewport at 200% root text with only the short seeded date: document 320 px, request card x=12–308, table viewport 244 px with 547 px scrollable content; demo controls remain within the viewport.

Evidence: [390 px demo](evidence/verification-6/demo-mobile-390.png) and [seeded 320 px inbox at 200% text](evidence/verification-6/inbox-mobile-320-text200.png). A newly submitted request changes this result as documented in the release-blocking finding.

## Privacy, headers, caching, and rate limits

The complete landing → demo request → seller inbox → reset → exit flow made requests only to `https://client-catalogue-request.sociobot.in`. No analytics, advertising, remote-font, CDN, or undisclosed cross-origin request occurred. It produced no console or page error.

Browser response headers on the root, hashed assets, API, and real 404 include `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`, and the documented CSP with `frame-ancestors 'none'`. An untrusted Origin received no `Access-Control-Allow-Origin`. Hashed JS, CSS, fonts, and imagery return `Cache-Control: public, max-age=31536000, immutable`; browser responses use Brotli. `robots.txt`, `sitemap.xml`, icons, and the CSV template return 200. All discovered product links resolve as intended. Checkout returns 303 to the Sociobot-hosted payment session.

Fresh same-client bursts observed these enforced allowances:

- local product API: 40 reads then 10 × 429; 12 writes then 8 × 429;
- live product API: 40 reads then 100 × 429; 12 writes then 48 × 429;
- Sociobot license verifier: 30 checks then 10 × 429.

Every 429 included `Retry-After` (`1` second for the product API and `4` seconds for license verification). Live `/health` is intentionally exempt and returned 100 × 200 concurrently.

## Build identity and performance

Live `/health` returned:

```json
{"build_sha":"5e43474ca4be3b4a7876ae3abd960cd7a4b3a157","ok":true}
```

Candidate-stamped local and live `index.html`, entry JavaScript, CSS, and both loaded WOFF2 fonts matched byte for byte. Candidate build sizes are 37.78 KB raw / 12.31 KB gzip for initial JavaScript, 20.21 KB raw / 5.21 KB gzip CSS, 30.07 KB for the two loaded fonts, and 14.02 KB for the mobile hero. The 318.10 KB raw / 78.66 KB gzip MSAL chunk remains lazy and was not requested by the landing or demo flow.

Fresh Lighthouse 13.0.1 mobile scores are Performance 100, Accessibility 100, Best Practices 100, and SEO 100. FCP and LCP are 1.2 s, TBT is 0 ms, CLS is 0, and total transfer is 62 KiB. Evidence: [Lighthouse JSON](evidence/verification-6/lighthouse.json).

## Defects by severity

- Critical: none.
- High: one — a real request's unbroken RFC3339 timestamp expands the 320 px/200% text layout to 440 px and requires horizontal page scrolling.
- Medium: none.
- Low: none.

## Required remediation

Allow `.request-card time` to wrap within its card (for example with `overflow-wrap:anywhere` and the necessary `min-width:0` constraint). Extend the mobile regression to create a real demo request first, then assert `document.documentElement.scrollWidth === clientWidth` at 320 px/200% text and verify the full timestamp remains visible without horizontal page scrolling. Rerun all claims, the complete suite, and live verification after deployment.

## Applicability and verification limits

This is a web product with a backend, not a library or CLI, so clean-consumer package testing does not apply. It makes no offline/PWA claim and registers no service worker, so offline reload and service-worker update testing do not apply. The brief does not need an AI step; no missed AI leverage finding applies.

A human Sociobot account was not supplied, so the final credentialed return from CIAM was not completed. Authority, tenant, client, scope, redirect, release-token rejection, tenant isolation, and seller behavior were verified without credentials. This is a test limitation, not a product defect.
