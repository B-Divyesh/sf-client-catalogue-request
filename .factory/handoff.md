# Polish round 2 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for work order `client-catalogue-request-polish-2`.

## Result

**PASS — every finding in `.factory/review-1.md` and `.factory/review-2.md` is closed.**

The product repair is commit `78e6cf00b4571e6e77c71259ab55c1027b8ee882`, pushed to `origin/main`. ACR run `ch187` built the multi-stage container and deployed it to <https://client-catalogue-request.sociobot.in>. Live `/health` returns that exact full SHA.

The paid copy now matches the hosted checkout at **$15.71 USD, one time**. Checkout price, Sociobot sign-in, and named client links now have observable claim entries and tests. The hero caption states the SKU-and-quantity outcome, and the README uses “sample products”. Earlier demo, routing, legal, accessibility, mobile, privacy, and backend repairs remain intact. The complete finding map is in `.factory/polish-2.md`.

## Exact verification evidence

- Fresh GitHub clone: `/tmp/client-catalogue-polish-2-clean.6zcJZ7`; `npm ci --ignore-scripts` installed 64 packages with 0 audit vulnerabilities.
- Claims: all 31 exact commands from `.factory/claims.json` passed independently in that clone, including the three new outcome tests.
- Full local suite: `npm test` passed 12 Vitest tests, 13 Rust tests, and 48 Playwright tests; 6 intentional project-specific skips. `npm run lint` and `git diff --check` passed.
- Production build: initial application JavaScript 37.77 kB raw / 12.28 kB gzip; CSS 20.30 kB raw / 5.22 kB gzip; two loaded WOFF2 files total 30.07 kB. The 318.10 kB MSAL chunk remains lazy-loaded only for seller sign-in.
- Local release runtime: root, direct demo, and demo inbox passed `/opt/fleet/lib/verify-url.sh` with one h1, `lang=en`, `<main>`, complete alt text, labelled buttons, and no console errors. Unknown routes returned 404. Read limiting produced 40 responses then 10 HTTP 429 responses, all with `Retry-After: 1`; 100 concurrent `/health` requests returned 200. CSP, `nosniff`, referrer policy, and immutable asset caching were present.
- Live cold browser: all eight public routes had route-specific titles, metadata, one h1, legal footer links, and zero serious/critical Axe findings. Forward and Back navigation focused the destination h1. No console or page error occurred. Evidence: [live checks](evidence/polish-2-live-checks.json).
- Demo/privacy: one click entered `/?demo=1`; six products rendered immediately; Reset replaced the random workspace; leaving deleted every `demo:` key. Landing and demo made same-origin requests only. [Demo evidence](evidence/polish-2-live-demo/verify.json).
- Mobile: landing and inbox remained viewport-wide at 390 px; the inbox also remained viewport-wide at 320 px with 200% text. [Inbox screenshot](evidence/polish-2-live-inbox/screenshot-mobile.png).
- Billing: the live Sociobot handoff opened Dodo with product “Client Catalogue Request”, `$15.71`, “Pay in USD”, and a one-time license description. [Checkout screenshot](evidence/polish-2-live-checkout/screenshot-desktop.png).
- Seller identity: the live hosted CIAM redirect used tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650`, client `25c704f4-465a-47af-80ab-2c489466b697`, and the product `access_as_user` scope. The recorded callback claim opened its isolated seller workspace. [Sign-in screenshot](evidence/polish-2-live-signin/screenshot-desktop.png).
- Lighthouse 13.0.1 mobile: local and live scores were Performance 100, Accessibility 100, Best Practices 100, SEO 100. Live FCP 0.9 s, LCP 1.2 s, TBT 0 ms, CLS 0, transfer 62 KiB. [Live report](evidence/polish-2-live-lighthouse.json).
- Offline/package checks: no service worker is registered and no offline claim is made, so offline reload is intentionally unavailable. Package/consumer checks do not apply to this `web-with-backend` artifact.

## Run and verify

```sh
npm ci
npm test
npm run lint
VITE_BUILD_SHA=local npm run build
BUILD_SHA=local cargo build --release --locked
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

Open `http://localhost:8080/?demo=1` for the isolated sample. No unresolved product finding or known release gap remains.
