# Client Catalogue Request — repair handoff

Repair work order: `client-catalogue-request-repair-2`
Source commit: `28d771d423e1e3237a90542df60b96cf1cf8065f`

## What changed

- Replaced replica-local demo storage with a stateless demo API. The browser keeps only `demo:` sample keys; the server never retains demo requests, so any replica can serve the next request.
- Replaced the single global password owner with Sociobot Entra External ID sign-in and seller-subject tenant tables. Catalogue rows, links, and requests are tenant-scoped.
- Enforced the 12-row and one-active-link free limits on the backend. A browser cache cannot bypass them. Added a visible license restore field and an accessible inactive-license notice.
- Added seller actions to revoke client links and soft-delete stored requests.
- Returned real HTTP 404 responses, added immutable caching for `/assets/`, removed initial heading focus, fixed 44 px mobile targets, and added 320 px/200% text reflow coverage.
- Updated demo/privacy/README copy and claims to remove the false server-memory retention statement and cover all retained product claims.

## How to run

```sh
npm ci
npm test
npm run build
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run --release
```

The container starts with `PORT` only. `DATA_DIR` and `WEB_DIST` are optional local overrides.

## Verification evidence

- `npm ci`: passed (0 audit vulnerabilities).
- `npm test`: passed: 5 Vitest, 8 Rust, and 28 Playwright cases; 4 intentional mobile duplicate/desktop-only skips.
- `npm run build`: passed; initial JavaScript is 77.23 KB gzip and CSS is 5.09 KB gzip.
- `cargo build --release --locked`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `git diff --check`: passed.
- Playwright axe checks found no serious or critical issues on the landing, demo, legal, 404, keyboard, desktop, and 390 px paths.
- Local release server: `/missing-page` returned HTTP 404; a fingerprinted asset returned `Cache-Control: public, max-age=31536000, immutable`; `/opt/fleet/lib/verify-url.sh` passed (no console errors, title/lang/main/alt checks).
- Entra discovery and the in-product authority endpoint both resolve to `sociobotcustomers.ciamlogin.com` for tenant `35c6fe40-0ec0-46b6-98c6-213ad4de6650`.

## Deployment evidence

- Pushed to `origin/main` and deployed with `/opt/fleet/lib/deploy-container.sh`.
- Azure Container App revision: `sf-client-catalogue-request--0000002` using image `sociobotregistry.azurecr.io/sf-client-catalogue-request:28d771d423e1`.
- Live `/health` returned the exact source SHA `28d771d423e1e3237a90542df60b96cf1cf8065f`.
- Live `/missing-page` returns HTTP 404. The fingerprinted live CSS response has `Cache-Control: public, max-age=31536000, immutable`.
- Ten independent live demo create/read pairs, each with a separate forwarded client IP, returned 201 then 200 with the seeded request. This exercises the prior replica-loss boundary without retained server workspace state.
- Live `verify-url.sh` passed in 627 ms with no console errors and valid title/lang/main/alt/button checks.

## Known external dependency

On 2026-08-29, the factory billing endpoint still returned `404 {"error":"enabled factory product"}` for `client-catalogue-request/checkout`. The repository keeps the required Sociobot checkout URL and now enforces entitlement server-side, but billing-product registration is factory-owned and was not available in this container. Do not represent checkout as verified until the factory enables that product.
