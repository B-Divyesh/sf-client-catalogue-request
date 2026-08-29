# Client Catalogue Request — verification handoff

Verified: 2026-08-29 UTC
Work order: `client-catalogue-request-verify-3`
Candidate and deployed build: `979fd37f967f2380a2eb2a60f6bd92a4de047822`
URL: <https://client-catalogue-request.sociobot.in>

## Result: PASS

The deployed product matches the candidate (`/health` returns the full SHA) and passes the researched-brief workflow: a small B2B seller can import a catalogue, create a protected client link, receive a structured request, export it, and delete it. The one-click sample route is isolated and usable without setup.

## Verification evidence

- Clean install: `npm ci` completed with 0 reported vulnerabilities.
- Claims: every one of the 18 exact commands in `.factory/claims.json` passed independently from the demo entry point.
- Quality gates: `npm test` passed (8 Vitest, 11 Rust, 38 Playwright); `npm run lint` passed; `BUILD_SHA=979fd37f967f2380a2eb2a60f6bd92a4de047822 cargo build --release --locked` passed; `npm run build` produced `dist/`.
- Runtime: the release binary started with only `PORT=8099`, returned the candidate SHA from `/health`, applied Brotli and immutable cache headers to hashed assets, and was then stopped. Its temporary local SQLite directory was removed.
- Live product: normal demo request, client-side validation/recovery, CSV export, print selection, reset, mobile 390 px, keyboard basket use, reduced motion, and response/error checks passed. Serious/critical axe findings: 0 on desktop and mobile demo checks.
- Privacy: cold landing and complete demo flows requested only `https://client-catalogue-request.sociobot.in`; no analytics, advertising, runtime CDN, or remote-font origin was observed.
- Limits: live read burst permitted 40 requests then returned 429 with `Retry-After: 1`; after a reset window, write burst permitted 12 POSTs then returned 429 with `Retry-After: 1`.
- Identity and billing: seller sign-in redirected to `sociobotcustomers.ciamlogin.com` tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650` with the product API scope; checkout returned 303 to Sociobot’s hosted Dodo checkout.

## Defects by severity

None found. The only scope limitation was that no human Sociobot credential was available to complete an actual authenticated seller session; the live redirect, tenant, requested API scope, CSP, and backend scope validation were verified.

## How to run

```sh
npm ci
npm test
npm run lint
npm run build
BUILD_SHA=local cargo build --release --locked
PORT=8080 target/release/client-catalogue-request
```

See `.factory/verification-3.md` for the complete independent evidence and claim-by-claim results.
