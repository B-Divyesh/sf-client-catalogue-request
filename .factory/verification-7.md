# Independent verification 7 — Client Catalogue Request

Completed 2026-08-29 UTC against candidate `b7679964d3dcabde7c08a9102f1b128e2a6ee1b5` and `https://client-catalogue-request.sociobot.in`.

## Result: PASS

The deployed `/health` response was `{"build_sha":"b7679964d3dcabde7c08a9102f1b128e2a6ee1b5","ok":true}`. This is an exact match for the requested candidate. No release-blocking defect was found.

## First read and demo gate

A cold desktop visit rendered the title **Client Catalogue Request — collect quote requests** and one H1: “Turn repeat orders into clear requests.” The first screen says it is “For small B2B sellers who need client orders without running an online store” and places **Try it with sample data** beside “Opens a private sample catalogue. No setup.” This answers what it does, who it is for, and what to click first in plain words.

That first-screen link opens `/?demo=1` in one click. The direct demo showed the persistent “Demo — sample data, nothing is saved” banner, reset/start-real controls, six realistic products, two POA items, stock caveats, basket, and seller sample inbox.

## Clean-checkout quality gates

- `npm ci`: PASS — 64 packages installed; audit reported 0 vulnerabilities.
- Every one of the 28 exact commands in `.factory/claims.json`: PASS. The sequential command run was stopped on any non-zero exit; it reached the end without stopping.
- `npm test`: PASS — 12 Vitest tests, 13 Rust tests, and 50 Playwright tests. The full test run exercised desktop and 390 px mobile projects.
- `npm run lint`: PASS (`cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`).
- Exact frontend production build: PASS with `VITE_BUILD_SHA=b7679964d3dcabde7c08a9102f1b128e2a6ee1b5 npm run build`.
- Locked release build: PASS with `BUILD_SHA=b7679964d3dcabde7c08a9102f1b128e2a6ee1b5 cargo build --release --locked`.
- Docker-compatible tooling is not installed in this verifier container, so a local image build was not possible. The Dockerfile contract is covered by its passing project tests; it is multi-stage, uses `rust:1-bookworm`, runs non-root, and accepts `BUILD_SHA`.

The production output is within the stated static budget: initial main JS is 37.75 KB raw / 12.27 KB gzip, CSS is 20.30 KB raw / 5.22 KB gzip, and loaded WOFF2 fonts total 30.07 KB. The 318.10 KB raw / 78.66 KB gzip MSAL code is lazy.

## End-to-end and accessibility evidence

- Normal demo flow: added catalogue items, reviewed the basket, completed a quote request, and observed `Request RQ-DEMO-… received`.
- Boundary/recovery: quantity `0` produced “Check each quantity. Enter a whole number from 1 to 9,999.” and did not store a request; replacing it with `25` submitted normally.
- Keyboard smoke: the first Tab stop is the skip link; Add/Review works with Enter; Escape closes the request dialog. Focus moves to the H1 on client-side route changes (also covered by the full suite).
- Mobile: live `390 × 844` demo had `scrollWidth === clientWidth === 390`; controls remained usable and no reduced-motion animations were active. Visual inspection was recorded in `/tmp/live-demo-form-mobile.png` during this verification.
- Axe, live mobile demo: 0 serious or critical WCAG 2 A/AA findings. Browser console and page errors: none in landing, demo, basket, and completed request checks.
- The production pages expose `lang=en`, one H1, a `<main>`, labelled inputs, dialog semantics, visible 3 px amber `:focus-visible` outline, and self-hosted fonts.

## Privacy, headers, identity, and backend checks

- A cold landing trace and a direct demo trace made only same-origin requests: the product document, self-hosted assets/fonts, and `/api/demo`. No analytics, advertising, remote fonts, or runtime CDN calls were observed.
- Live response headers include `X-Content-Type-Options: nosniff`, `Referrer-Policy: strict-origin-when-cross-origin`, and a CSP with `frame-ancestors 'none'`. Hashed assets use `Cache-Control: public, max-age=31536000, immutable`. A missing page returns HTTP 404.
- `/api/auth/config` specifies only `https://sociobotcustomers.ciamlogin.com/35c6fe40-0ec0-46b6-98c6-213ad4de6650/`, client `25c704f4-465a-47af-80ab-2c489466b697`, and the product API scope. No other sign-in provider was found.
- 100 concurrent live `/health` requests: 100 × HTTP 200.
- Rate-limit verification used an isolated demo endpoint and a single synthetic forwarded client IP. A 160-way concurrent `POST /api/demo` burst received 125 × 201 and 35 × 429; the first observed 429 was request 27 and every 429 carried `Retry-After: 1`. This deployment is replicated, so the per-worker write allowance is observable as 12 requests/second in source rather than as a single global count. The mandatory outcome — 429 plus Retry-After after a client exceeds the allowance — is enforced.
- The release binary, run from a fresh temporary directory with **only** `PORT=8099` in its environment and a `dist` sibling, created its default `data/catalogue.db` and returned the supplied candidate SHA from `/health`.

PWA/service-worker and package-consumer checks do not apply: this is a backend web product, it makes no offline claim, and it does not register a service worker. Interactive sign-in was not completed because no human Sociobot account was provided; the required tenant configuration, CSP, client ID, scope, and backend automated auth/isolation tests passed.

## Defects by severity

Critical: none.  High: none.  Medium: none.  Low: none.

Product code was not modified. This report and the handoff are the only changes made by verification.
