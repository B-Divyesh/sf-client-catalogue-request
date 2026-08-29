# Independent product verification — PASS

Verified: 2026-08-29 UTC
Work order: `client-catalogue-request-verify-3`
Candidate: `979fd37f967f2380a2eb2a60f6bd92a4de047822`
Live URL: <https://client-catalogue-request.sociobot.in>
Acceptance contract: researched brief, `.factory/brief.json`, `AGENTS.md`, and supplied factory skills.

## Verdict

**PASS — candidate is suitable for release.** The previous deployment-only failures are not present in fresh evidence. `/health` returns `{"build_sha":"979fd37f967f2380a2eb2a60f6bd92a4de047822","ok":true}`, establishing that the live backend and candidate are the same build.

## Mandatory cold-read and demo gate

PASS. On a fresh 1440 × 900 page load, the first screen says:

- What it does: “Turn repeat orders into clear requests.”
- For whom: “For small B2B sellers who need client orders without running an online store.”
- First action: **Try it with sample data**, with “Opens a private sample catalogue. No setup.”

The one-click action opened the realistic six-product catalogue. Its persistent banner says **Demo — sample data, nothing is saved** and provides **Reset demo** and **Start for real**. POA products, stock caveats, request basket, and sample seller inbox are immediately present.

## Claim gate

`.factory/claims.json` exists and contains 18 claims. After `npm ci`, every listed command was invoked independently and passed. The evidence logs are retained in this verification container under `/tmp/client-catalogue-claim-*.log`.

| Claim | Exact declared test | Result |
|---|---|---|
| `demo-isolation` | `npm test -- --grep @claim:demo-isolation` | PASS |
| `poa-price` | `npm test -- --grep @claim:poa-price` | PASS |
| `csv-export` | `npm test -- --grep @claim:csv-export` | PASS |
| `structured-request` | `npm test -- --grep @claim:structured-request` | PASS |
| `no-card-data` | `npm test -- --grep @claim:no-card-data` | PASS |
| `protected-links` | `cargo test generated_tokens_are_long_and_distinct` | PASS |
| `csv-import` | `npm test -- --grep @claim:csv-import` | PASS |
| `print-request` | `npm test -- --grep @claim:print-request` | PASS |
| `paid-license` | `npm test -- --grep '@claim:paid-license(?!-invalid)'` | PASS |
| `stock-privacy` | `cargo test stock_counts_are_not_exposed` | PASS |
| `privacy-runtime` | `npm test -- --grep @claim:privacy-runtime` | PASS |
| `demo-local` | `cargo test demo_is_stateless_across_backend_instances` | PASS |
| `demo-reset` | `npm test -- --grep @claim:demo-reset` | PASS |
| `csv-import-cap` | `cargo test catalogue_import_cap_is_five_thousand_rows` | PASS |
| `port-runtime` | `cargo test runtime_defaults_to_port_and_local_paths` | PASS |
| `client-data-control` | `npm test -- --grep @claim:client-data-control` | PASS |
| `seller-tenancy` | `cargo test sellers_are_isolated` | PASS |
| `paid-license-invalid` | `npm test -- --grep @claim:paid-license-invalid` | PASS |

## Local build and tests

- Starting checkout was clean at the candidate SHA.
- `npm ci`: PASS; 64 packages installed, 0 audit vulnerabilities reported.
- `npm test`: PASS — 8 Vitest tests, 11 Rust tests, and 38 Playwright tests. `test-results/.last-run.json` records `status: passed` with no failed tests.
- `npm run lint`: PASS — `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings`.
- `BUILD_SHA=979fd37f967f2380a2eb2a60f6bd92a4de047822 cargo build --release --locked`: PASS.
- `npm run build`: PASS; `dist/` produced. Entry JavaScript is 37.17 KB raw / 12.16 KB gzip; lazy seller-auth code is separate. CSS is 19.62 KB raw / 5.12 KB gzip; loaded WOFF2 fonts total 30.07 KB. These satisfy the stated static budgets.
- No Docker/Podman/Buildah executable is installed in this worker, so the container image itself could not be built. The exact Vite and locked release-server stages both passed, and the repository Dockerfile tests pass in the full suite.
- Release runtime: `env -i PORT=8099 target/release/client-catalogue-request` served `/health` with the candidate SHA. A Brotli request for a hashed asset returned `Content-Encoding: br` and `Cache-Control: public, max-age=31536000, immutable`. The runtime created only its expected temporary `data/catalogue.db`; it was stopped and that test directory was removed.

## Live workflow, validation, and persistence boundaries

- Normal demo flow: added a product, reviewed it, supplied contact details, submitted a request, then verified it in the sample inbox. CSV export has one row per requested line. Print marks only the selected request. Reset removes the submitted local key.
- UI recovery: quantities `0` and `10000` are rejected with the announced message “Check each quantity. Enter a whole number from 1 to 9,999.” Correcting to `25` permits submission.
- Direct live demo API boundary checks returned 400 for one-character name, malformed email, quantity 0, quantity 10,000, duplicate product IDs, and unknown products. A valid quantity of 9,999 returned 201 and preserved 9,999 in the structured line.
- The complete real seller flow is covered by `@claim:csv-import` and `@claim:client-data-control`: one-row CSV import, client link creation, client request, seller inbox receipt, and request deletion all passed against the Rust backend. Backend tests additionally cover seller separation, 5,000-row cap, 12-row free limit, license-raised limits, revocation, and stateful persistence paths.
- The product is neither a PWA nor a library/CLI; service-worker/offline-update and package-consumer checks do not apply.

## Accessibility, desktop/mobile, and browser behavior

- `/opt/fleet/lib/verify-url.sh https://client-catalogue-request.sociobot.in …`: PASS in 709 ms; title present, `lang=en`, exactly one h1, main landmark, all images have alt attributes, controls are labelled, and there were no console/page errors.
- Fresh desktop and 390 × 844 mobile Playwright checks: 0 axe serious/critical WCAG 2 A/AA findings; 0 console/page errors; no horizontal overflow at 390 px.
- Keyboard: first Tab reaches the skip link; basket can be opened and closed with keyboard; route navigation focuses the new heading. The inspected focus ring is visible.
- Reduced-motion media emulation converts animation to no animation and transitions to `0.00001s`.

## Privacy, headers, cache, rate limits, identity, and live parity

- Cold landing and complete demo request flows recorded only same-origin requests. No analytics, ads, remote fonts, or runtime CDN requests occurred.
- Root, API, health, and 404 responses include CSP, `X-Content-Type-Options: nosniff`, and `Referrer-Policy: strict-origin-when-cross-origin`. CSP restricts source lists and correctly permits only the Sociobot API and Sociobot CIAM host required by this product. Unknown routes return an HTTP 404.
- The live hashed entry asset is Brotli-compressed at 12,376 B transfer and immutable-cached. This is below the 150/200 KB initial JavaScript budget.
- A same-client live read burst completed 40 requests with 200 then 40 with 429 and `Retry-After: 1`. After waiting for the one-second window, a write burst completed 12 POSTs with 201 then 18 with 429 and `Retry-After: 1`. This confirms the documented server endpoint allowance.
- Seller sign-in goes only to `https://sociobotcustomers.ciamlogin.com/35c6fe40-0ec0-46b6-98c6-213ad4de6650/…`, requests `api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user`, and returns to `/auth/callback`; no other identity provider was observed. No human test credential was supplied, so entering a real account was intentionally not attempted.
- The public paid URL returned HTTP 303 to hosted Sociobot/Dodo checkout. No payment-card handling is embedded in the app.
- All landing links were checked: product routes return 200, the checkout returns 303, and the Param Factory link returns 200.

## Defects by severity

No critical, high, medium, or low defects found.

## Known scope limitation

The verifier had no human Sociobot user credential. The full identity redirect, tenant, requested product API scope, CSP, and server-side scope enforcement were verified; completing a real seller login is the only unexercised credential-dependent step.
