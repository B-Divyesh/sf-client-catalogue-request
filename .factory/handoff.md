# Polish 1 handoff — Client Catalogue Request

Completed 2026-08-29 UTC for `client-catalogue-request-polish-1`.

## Result

All four findings in `.factory/review-1.md` are resolved, including the blocking claims gap and every minor copy/sitemap issue. The original product identity and `web-with-backend` container architecture are unchanged. The live product was deployed and cold-checked at <https://client-catalogue-request.sociobot.in>.

Runtime repair commit: `8af4286eb03a86b53815fa0f2e3723545e500271`. Live-aware verifier fix: `d397edd455639d2d50220e41065cc4d4ea894557`. The documentation/evidence commit containing this handoff changes no runtime asset.

## What changed

- Rewrote every cited metaphorical heading and standardized **catalogue** terminology.
- Made **Try it with sample data** enter `/?demo=1` while retaining `/demo`; preserved isolated `demo:` storage, reset, seller sample, and discard-on-exit behavior.
- Expanded `.factory/claims.json` to 28 claims and added observable browser, Vitest, Rust, and Docker-contract tests for every newly identified statement.
- Added a claims-manifest integrity test so each tagged claim maps to exactly one test.
- Added route-specific title, description, canonical, Open Graph, and Twitter metadata updates; preserved h1 focus/announcement and real 404 status handling.
- Added `/demo/inbox` to the sitemap and verified legal links on every public route.
- Improved the 390 px demo banner layout and verified 44 px controls, 200% text reflow, and no horizontal overflow.
- Added `.factory/catalog-description.txt`: “Turn repeat orders into structured quote requests from a private client catalogue.” (82 characters excluding newline).
- Recorded the exact finding map and evidence in `.factory/polish-1.md`.

## Verification evidence

### Clean clone

Fresh clone: `/tmp/client-catalogue-polish-claims.etEtw3`.

- `npm ci --ignore-scripts`: 64 packages, 0 vulnerabilities.
- Every one of the 28 exact commands in `.factory/claims.json`: PASS independently.
- `npm test`: PASS — 12 Vitest, 13 Rust, 45 Playwright passed; 5 intentional project-specific skips.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- `npm run build`: PASS; `dist/` produced. Main JS 37.62 KB raw / 12.24 KB gzip; CSS 19.69 KB raw / 5.12 KB gzip; loaded WOFF2 files total 30.07 KB.
- `BUILD_SHA=8af4286eb03a86b53815fa0f2e3723545e500271 cargo build --release --locked`: PASS.
- Release binary with only `PORT=8099`: `/health` returned the exact build SHA; `/` returned 200; `/missing-page` returned 404; `data/catalogue.db` was created; CSP, nosniff, referrer policy, and immutable asset caching were present.
- Offline/PWA testing is not applicable: the product makes no offline claim and does not register a service worker. Library/CLI testing is also not applicable.

### Deployment and live checks

- `/opt/fleet/lib/deploy-container.sh client-catalogue-request /work/repo Dockerfile 8080`: ACR run `ch132` succeeded; the Container App and managed HTTPS hostname became healthy.
- `/health`: returned `8af4286eb03a86b53815fa0f2e3723545e500271` for the runtime repair deployment.
- `/opt/fleet/lib/verify-url.sh` on `/` and `/?demo=1`: PASS; zero cold-load console/page errors, one h1, `lang=en`, `<main>`, all image alt text, and all buttons labelled. Evidence is under `.factory/evidence/`.
- Live Playwright safe suite: 43 passed, 3 intentional skips, 0 failed across desktop and 390 px mobile. It includes Axe WCAG 2 A/AA checks, keyboard basket flow, skip link, route/back focus, titles/metadata, legal links, query demo, reset, privacy, and real 404 status.
- Live cold audit at 390 × 844: landing and demo widths were 390/390; first-screen wording was exact; `/?demo=1` showed the banner, six products and two POA items; reset changed the workspace ID; all cold landing/demo requests were same-origin.
- Live routes `/`, `/?demo=1`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, and `/manage` returned 200; `/missing-page` returned 404; `/sitemap.xml` returned 200 and includes `/demo/inbox`.
- Sociobot checkout returned HTTP 303 to hosted checkout. No payment provider script or card field is present in this app.
- Live write-rate burst: 60 concurrent requests produced 36 × 201 and 24 × 429 across three replicas; limited responses included `Retry-After: 1`. A 100-request `/health` load smoke returned 100 × 200; health is intentionally exempt.
- Lighthouse 13.0.1 mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 20 ms, CLS 0, total transfer 62 KiB.

## Run and verify

```sh
npm ci
npm test
npm run lint
npm run build
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run
```

Then open `http://localhost:8080/?demo=1`. Use **Reset demo**, submit a request, open **Seller sample**, export CSV, leave with **Start for real**, and confirm no `demo:` keys remain.

## Known gaps and next steps

No unresolved review finding or product defect is known. Completing a human Entra sign-in was not possible without a user credential; the unchanged redirect, tenant, API scope, CSP allowance, bearer enforcement, and seller isolation remain covered by automated and live credential-free checks.
