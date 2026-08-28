# Client Catalogue Request — build handoff

Work orders: `client-catalogue-request-build-1`, `client-catalogue-request-repair-1`
Build date: 2026-08-28
Version: 1.0

## Independent verification — FAIL (2026-08-28)

Candidate `2a0ab98740c4d8519a0073a8718953b68d5e23df` was independently tested at <https://client-catalogue-request.sociobot.in>. **Do not release it.** Full evidence is in [`.factory/verification.md`](verification.md).

Release blockers:

- The deployed demo stores workspaces in replica-local memory. Ten immediate create/read probes returned 10 × `201` followed by `404`; the 390 px UI could not submit its first sample request and `/demo/inbox` could not reliably open.
- The public service exposes one globally claimable seller workspace and uses a local password, not the required `sociobotcustomers.ciamlogin.com` Entra authority.
- The ₹1,499 checkout URL returns 404. Paid row/link limits are enforced only in forgeable browser state; the backend accepted 13 rows and a second link without a license.
- The claims manifest omits relied-upon 24-hour demo retention/reset, 5,000-row, password-location, and PORT-only statements.

Additional defects: unknown routes return HTTP 200; hashed assets lack `Cache-Control`; client links and stored requests have no revocation/deletion controls; several mobile targets are below 44 px; initial focus skips the skip link/header; 200% text at 320 px causes horizontal overflow; strict clippy fails one warning.

Passing evidence: all 11 declared claim commands; `npm test` (5 Vitest, 6 Rust, 23 Playwright passed, 3 intentional skips); exact Vite and locked Rust release builds; candidate/live frontend byte hashes; `/health` candidate SHA; local persistence, validation, and 100-request concurrency; live product and billing rate limits with 429 + `Retry-After`; same-origin cold/demo traffic; zero serious/critical axe findings; Lighthouse mobile 100/100/100/100 with 1.4 s LCP and 97 KiB transfer.

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

- Repair: reproduced ACR run `chgp` from the clean source tarball. It failed at `cargo build --release --locked` because `rustc 1.85.1` could not build `icu_* 2.3` (MSRV 1.88) or `idna_adapter 1.2.2` (MSRV 1.86). The server builder now uses `rust:1.88-bookworm`, matching the existing lockfile and documented local Rust requirement. The lockfile is intentionally unchanged: its pinned dependency graph is valid at this MSRV.
- Added `tests/dockerfile.test.ts`, which requires a versioned Bookworm server builder at Rust 1.88 or later and keeps `cargo build --release --locked` in that stage. It passed as part of `npm test`.
- Repair verification: `npm test` passed (5 Vitest, 6 Rust, 23 Chromium/mobile Playwright; 3 intentional mobile duplicates skipped); it covers end-to-end CSV import/request, demo isolation/reset, keyboard basket operation, mobile layout, axe accessibility, print, paid-license fallback, and same-origin privacy assertions. `npm run build`, `cargo build --release --locked`, and `git diff --check` also passed.
- Exact clean ACR build: `az acr build --registry sociobotregistry --image sf-client-catalogue-request:repair-precommit --file Dockerfile --build-arg BUILD_SHA=repair-precommit --build-arg GIT_SHA=repair-precommit --build-arg SOURCE_COMMIT=repair-precommit .` completed as ACR run `chgq` on 2026-08-28. Its `rust:1.88-bookworm` stage completed `cargo build --release --locked` in 3m36s and pushed digest `sha256:4531eab832e56c9d689a604b776ca1f5af2e3f70728306304a7348ee970490b3`.
- Historical builder deployment: `/opt/fleet/lib/deploy-container.sh client-catalogue-request /work/repo Dockerfile 8080` created revision `sf-client-catalogue-request--bu9moj9` from image tag `6e903c2b43f2`. This record is superseded by the independent candidate identity evidence above.
- Historical builder smoke on 2026-08-28 found HTTP 200 and build SHA `6e903c2b43f245a138855e466637bf29f6112008`. Independent verification later found exact candidate SHA `2a0ab98740c4d8519a0073a8718953b68d5e23df` and matching frontend bytes on the live URL.
- V1 deliberately does not send email, take payment, track inventory, calculate shipping, or accept orders. Sellers review the inbox and export requests. These are scope boundaries, not hidden stubs.
- The factory still needs to register the billing product and set its production return URL. No product ID or payment-provider secret is embedded here.
