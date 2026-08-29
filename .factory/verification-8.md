# Independent verification 8 — Client Catalogue Request

Completed 2026-08-29 UTC against candidate `6744f2a7ff39804100fe60f5cf3a24bd13decb4c` at <https://client-catalogue-request.sociobot.in>.

## Result: PASS

Fresh deployment evidence matches the candidate exactly. Live `/health` returned `{"build_sha":"6744f2a7ff39804100fe60f5cf3a24bd13decb4c","ok":true}`. A fresh candidate-SHA frontend build produced `index-BUOHNHNV.js`; its SHA-256 matched the live bundle byte-for-byte. No release-blocking defect was found.

## First read and demo gate

A cold visit displayed the plain H1 “Turn repeat orders into clear requests”, then “For small B2B sellers who need client orders without running an online store.” Its first action is **Try it with sample data**, with the explicit outcome “Opens a private sample catalogue. No setup.” This answers what it does, for whom, and what to click first.

One click opened `/?demo=1`. The direct `/demo` route also worked and showed the persistent “Demo — sample data, nothing is saved” banner, reset/start-real controls, six realistic products, two POA items, stock notes, a basket, and a seeded seller inbox.

## Clean-checkout quality gates

- `npm ci`: PASS — locked dependencies installed; audit reported 0 vulnerabilities.
- All **31** exact commands declared in `.factory/claims.json`: PASS, run sequentially from the demo-capable clean checkout. The unfiltered suite below also covers the claim behaviours.
- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, and 54 Playwright desktop/mobile tests.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- Candidate production build: PASS — `VITE_BUILD_SHA=6744f2a7ff39804100fe60f5cf3a24bd13decb4c npm run build`.
- Locked release build: PASS — `BUILD_SHA=6744f2a7ff39804100fe60f5cf3a24bd13decb4c cargo build --release --locked`.
- The release binary started with an otherwise empty environment and only `PORT=4180`, created its default `data/catalogue.db`, and returned the supplied SHA from `/health`.
- Docker is not installed in this verifier container, so the image itself could not be built here. Its Dockerfile contract is covered by the passing project test and source review: multi-stage build, `rust:1-bookworm`, non-root runtime, `/app/data`, `PORT`, and `BUILD_SHA` are present.

The first-load application chunk is 37.81 KB raw (the 318.10 KB MSAL chunk is lazy loaded); CSS is 20.30 KB raw. The live application asset is immutable-cached and stays well within the 200 KB initial-JS budget.

## End-to-end, mobile, and accessibility

- On live demo, an item could be added, reviewed, and submitted; the completion message was `Request RQ-DEMO-35C4 received`.
- Invalid quantities `0` and `10000` each showed “Check each quantity. Enter a whole number from 1 to 9,999.” Recovery with `25` succeeded. CSV export downloaded `sample-quote-requests.csv`.
- Keyboard check: the first Tab stop was the skip link; Enter operated Add/Review; the request path completed by keyboard.
- Desktop and 390 px live demo checks had `scrollWidth === clientWidth`; no reduced-motion animations ran.
- Fresh live Axe scans (WCAG 2 A/AA) on desktop and 390 px mobile demo: **0 serious or critical findings**. Both had one H1, one main landmark, `lang=en`, and no console or page errors. Visible focus starts at the skip link.

## Privacy, backend, and deployment

- Fresh Playwright request logs for the landing and demo contained only the product origin. No analytics, advertising, remote fonts, or runtime CDN requests were observed.
- Live headers include CSP with response-header `frame-ancestors 'none'`, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`. Missing routes return HTTP 404. Hashed assets have `Cache-Control: public, max-age=31536000, immutable`.
- The real sign-in redirect went only to `sociobotcustomers.ciamlogin.com` and requested `api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user` (plus standard OIDC scopes).
- Local API exercise: seller catalogue save, named client link, client catalogue read, invalid request (400), valid request (201), seller inbox, delete (204), cross-seller empty inbox, and restart persistence all passed. Client response contained no stock-count field.
- Rate limiting is enforced. Local write allowance was 12 requests in one second; requests 13–14 returned 429 with `Retry-After: 1`. A 50-way live single-forwarded-client burst returned 36×201 and 14×429, with `Retry-After: 1` on each rejection (the observed aggregate reflects replicated workers; source sets 12 writes/s per worker).
- The real Sociobot checkout handoff loaded `Sociobot | Checkout`, product `Client Catalogue Request`, USD $15.71, and the one-time license description. No payment data is collected by the product request form.

PWA/package-consumer checks do not apply: this is a backend web product with no service worker, no offline claim, and no published library/CLI API.

## Defects by severity

Critical: none.  High: none.  Medium: none.  Low: none.

Only this report and `.factory/handoff.md` were changed by verification.
