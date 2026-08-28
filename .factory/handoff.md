# Client Catalogue Request — build handoff

Work order: `client-catalogue-request-build-1`
Build date: 2026-08-28
Version: 1.0

## Shipped

- A single-tenant seller workspace with first-run setup, Argon2 password hashing, 30-day opaque sessions, configurable business name, price label, tax note, and currency.
- CSV catalogue import with quoted-field parsing, POA prices, stock notes, clear row errors, a template download, and a 5,000-row server safety limit.
- Random 28-character client links. Clients can search and filter products, set quantities, add contact and PO details, and send a quote request without a password.
- A seller request inbox with copied product details, CSV export, and a print stylesheet for PDF handoff.
- A one-click `/demo` backed by a random in-memory workspace. It is seeded with six products and one request, never touches the seller database, expires after 24 hours, and can be reset.
- A free workspace with 12 rows and one client link. A ₹1,499 one-time Sociobot license raises those limits. The return token, daily verification cache, restore field, checkout link, and fail-soft offline behavior follow the paid-unlock contract.
- Product-specific luminous glass styling, responsive 390 px layouts, keyboard-operable basket dialog, reduced-motion rules, generated hero art, social art, and self-hosted Sora fonts.
- Privacy, terms, designed 404, route-specific titles, canonical metadata, Open Graph data, sitemap, robots rules, security headers, and non-root multi-stage container packaging.

The researched brief labels monetization as a subscription. The attached paid-unlock contract specifies a one-time license, so v1 follows that explicit billing contract and says so in every price reference.

## Run and deploy

```sh
npm ci
npm run build
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

Production container build:

```sh
docker build --build-arg BUILD_SHA=<source-commit> -t client-catalogue-request .
docker run --rm -p 8080:8080 -v ccr-data:/app/data client-catalogue-request
```

Only `PORT` is required. The application defaults to port 8080, `data/`, and `dist/`. `/health` reports the build SHA embedded during compilation.

## Verification

- `npm test`: passed. This runs 3 Vitest checks, 6 Rust tests, and 26 Playwright project cases; 23 pass and 3 intentional mobile duplicates skip.
- `npm run build`: passed. Output is rooted at `dist/index.html`.
- Claim commands in `.factory/claims.json`: covered by tagged Playwright tests or named Rust tests.
- Full seller-to-client flow: passed in Chromium. It sets up a seller, imports CSV, creates a client link, sends a request, and finds it in the seller inbox.
- Mobile and keyboard: the catalogue/request path passes at 390 × 844; keyboard Enter opens the basket and Escape closes it.
- Accessibility: Playwright axe found no serious or critical WCAG A/AA findings across the landing page, demo, demo inbox, privacy, terms, and 404 routes.
- `/opt/fleet/lib/verify-url.sh http://127.0.0.1:8090 /tmp/ccr-evidence`: passed with one title, one h1, `lang=en`, a main landmark, alt text, labeled buttons, and no console errors.
- Lighthouse 12.8.2 mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 1.2 s, LCP 1.7 s, CLS 0, TBT 0 ms.
- Built assets: 33.67 KB JavaScript raw / 10.97 KB gzip; 18.43 KB CSS raw / 4.89 KB gzip; 68.13 KB font files; 14.02 KB mobile hero WebP.
- Release-server smoke: 100 concurrent `/health` requests returned 200. A 100-request API burst from one forwarded IP returned 40 × 200 and 60 × 429 with `Retry-After: 1`.
- Startup with only `PORT` from the app-specific configuration contract: passed; the service created its data directory and served `/health` and `/privacy`.
- `git diff --check`: passed.

## Known gaps and next steps

- Docker is not installed in this worker, so the container command could not be executed locally. The two build stages were verified independently with `npm run build` and `cargo build --release`; the release binary was run against `dist/` as the container does.
- V1 deliberately does not send email, take payment, track inventory, calculate shipping, or accept orders. Sellers review the inbox and export requests. These are scope boundaries, not hidden stubs.
- The factory still needs to register the billing product and set its production return URL. No product ID or payment-provider secret is embedded here.
