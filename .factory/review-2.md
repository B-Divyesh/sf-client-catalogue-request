# Adversarial first-read review 2 — Client Catalogue Request

Reviewed 2026-08-29 UTC against <https://client-catalogue-request.sociobot.in> and repository commit `05addb810f189315a67b46659db3c9b10f646be5`. The live `/health` endpoint reported product build `b7679964d3dcabde7c08a9102f1b128e2a6ee1b5`; the commits after that build change verification documents only.

## Verdict: FAIL

The first screen, demo, routing, privacy boundary, accessibility checks, and all 28 declared claim commands pass. The product is not ready because the advertised ₹1,499 price contradicts the live $15.71 checkout, and two public capabilities are absent from the claims manifest. Two copy defects also remain. Findings are ordered by severity.

### F-2-1 — BLOCKING: the advertised price does not match checkout, and its claim test cannot detect the mismatch

**Exact locations and quotes**

- Landing: “₹1,499 one-time”.
- README: “The ₹1,499 one-time license supports more rows and links, with up to 5,000 rows per import.”
- Live checkout reached from **Buy the full workspace**: “Pay in USD”, “Client Catalogue Request $15.71”, and “Total $15.71”.

**Why this fails:** a buyer is promised a specific INR price, then is shown a different amount and currency at the payment step. The `paid-license` manifest entry includes the ₹1,499 claim, but its test only finds “₹1,499” in the product page and exercises entitlement limits with a recorded response. It never verifies the checkout amount. `billing-handoff` only verifies the checkout URL, legal wording, and revoked-license behavior. Both declared commands pass while the live price is wrong, so this is not observable claim coverage.

**Concrete fix:** configure the Sociobot product so checkout charges ₹1,499 INR one-time, or change both public prices to the exact checkout currency and amount. Add a `billing-price` claim test that follows the Sociobot handoff and asserts product name, amount, currency, and one-time cadence from a recorded checkout fixture. Include a separate safe live check of those four fields.

### F-2-2 — BLOCKING: public sign-in and named-link capabilities are unlisted claims

**Exact locations and quotes**

- README, Seller workflow: “Open `/manage` and sign in with Sociobot.”
- Landing: “Name each link for a client or group.”
- README, Seller workflow: “Create a named client link and share it with that client.”

**Why this fails:** `.factory/claims.json` has no `sociobot-sign-in` or `named-client-links` entry. `seller-tenancy` tests database isolation, not a completed sign-in. `protected-links` tests only that two generated tokens are 28 characters and differ. The untagged Entra test reaches the hosted sign-in page, while the handoff still says a credentialed return was not tested. No declared test verifies that a saved link retains and shows its client/group name. This reopens the incomplete-claims issue from `F-1-1`.

**Concrete fix:** add a `sociobot-sign-in` manifest entry whose sandbox test covers the redirect, product scope, callback, and resulting seller workspace with a recorded identity fixture. Add `named-client-links` with a test that creates “Juniper Corner”, reloads the workspace, and asserts the same label beside the generated client link. If either behavior cannot be tested, remove the corresponding public sentence.

### F-2-3 — Minor: the hero-art caption ends in a slogan with no usable information

**Location and exact quote:** landing figure caption: “One private catalogue. One request you can use.”

**Why this fails:** “a request you can use” does not say what is preserved or what the seller can do with it. It is mood copy beneath an explanatory image.

**Concrete rewrite:** “Catalogue lines become one quote request with SKUs and quantities.”

### F-2-4 — Minor: the README uses an untestable marketing adjective

**Location and exact quote:** README, Try the sandbox: “The isolated demo includes six realistic products, two POA prices, stock notes, a request basket, and a seeded seller inbox.”

**Why this fails:** “realistic” is subjective and adds no instruction. The test proves the count and contents, not realism.

**Concrete rewrite:** “The isolated demo includes six sample products, two POA prices, stock notes, a request basket, and a seeded seller inbox.”

## Mandatory cold-read gate

**Result: PASS.** Fresh Chromium contexts opened the live root at 390 × 844 and 1440 × 900 without scrolling.

- What it does: turns repeat B2B orders into structured requests.
- For whom: small B2B sellers who do not want to run an online store.
- First action: **Try it with sample data**; adjacent copy says it opens a private sample catalogue with no setup.

The decisive first-screen text is “Turn repeat orders into clear requests”, “For small B2B sellers who need client orders without running an online store.”, and “Try it with sample data”. The 390 px document did not overflow, and both cold loads produced no console or page errors.

## Copy audit

Counts treat hyphenated terms, acronyms, amounts, code tokens, and URLs as one word. Separators such as `·` and `/` are not words. Controls, labels, headings, alt text, and footer copy are included. Repeated navigation text is listed once. Shell commands and the CSV fixture are excluded because they are executable/reference syntax. No sentence exceeds 22 words. No landing button fails the result-naming-verb check.

### Landing page

| Copy | Words | Result |
|---|---:|---|
| Skip to main content | 4 | Pass |
| Client Catalogue Request | 3 | Pass |
| Demo | 1 | Pass |
| Seller workspace | 2 | Pass |
| Privacy | 1 | Pass |
| Private catalogue · structured requests | 4 | Pass |
| Turn repeat orders into clear requests | 6 | Pass |
| For small B2B sellers who need client orders without running an online store. | 13 | Pass |
| Try it with sample data | 5 | Pass |
| Opens a private sample catalogue. | 5 | Pass |
| No setup. | 2 | Pass |
| No checkout or card data | 5 | Pass |
| Prices can stay POA | 4 | Pass |
| Client links are hard to guess | 6 | Pass |
| Glass product sheets flowing into one organized request document. | 9 | Pass |
| One private catalogue. | 3 | F-2-3 |
| One request you can use. | 5 | F-2-3 |
| 01 / CLIENT REQUESTS | 3 | Pass |
| How client requests work | 4 | Pass |
| Import your catalogue CSV. | 4 | Pass |
| Share a protected link. | 4 | Pass |
| Each client request arrives with SKUs, quantities, contact details, and notes. | 11 | Pass |
| Ready for CSV | 3 | Pass |
| 02 / HOW IT WORKS | 4 | Pass |
| How to create a request from a CSV | 8 | Pass |
| Import your CSV | 3 | Pass |
| Keep SKU, name, price, category, and stock note fields. | 9 | Pass |
| Leave price blank for POA. | 5 | Pass |
| Share one client link | 4 | Pass |
| Name each link for a client or group. | 8 | F-2-2 |
| The catalogue does not expose stock counts. | 7 | Pass |
| Export each request | 3 | Pass |
| Download the lines as CSV or print a clean request sheet to PDF. | 13 | Pass |
| 03 / SERVICE BOUNDARIES | 3 | Pass |
| What this service does not do | 6 | Pass |
| Use it to collect quote requests. | 6 | Pass |
| Confirm prices, stock, shipping, and fulfilment outside this service. | 9 | Pass |
| Read how request data is handled | 6 | Pass |
| 04 / WORKSPACE LIMITS | 3 | Pass |
| Free and paid workspace limits | 5 | Pass |
| The free workspace supports 12 catalogue rows and one client link. | 11 | Pass |
| Request export stays free. | 4 | Pass |
| ₹1,499 one-time | 2 | F-2-1 |
| Use more than 12 catalogue rows and create more than one client link. | 13 | Pass |
| Buy the full workspace | 4 | Pass |
| Have a license? Paste it | 5 | Pass |
| Restore license | 2 | Pass |
| Checkout opens Sociobot. | 3 | Pass |
| See the terms for refund details. | 6 | Pass |
| Client Catalogue Request turns private catalogues into clear quote requests. | 10 | Pass |
| Terms | 1 | Pass |
| Built by Param Factory (external site) | 6 | Pass |
| Version 1.0 · Build b7679964d3dc · Original generated artwork. | 7 | Pass |

### README

| Copy | Words | Result |
|---|---:|---|
| Client Catalogue Request | 3 | Pass |
| Turn repeat orders into structured quote requests. | 7 | Pass |
| Client Catalogue Request is for small B2B sellers who already maintain a catalogue. | 13 | Pass |
| A seller imports CSV rows, creates hard-to-guess client links, and receives quote requests with SKUs and quantities. | 17 | Pass |
| The request form never collects payment-card data. | 7 | Pass |
| Live product: https://client-catalogue-request.sociobot.in | 3 | Pass |
| Try the sandbox | 3 | Pass |
| Choose Try it with sample data, open `/?demo=1`, or visit https://client-catalogue-request.sociobot.in/?demo=1. | 11 | Pass |
| The isolated demo includes six realistic products, two POA prices, stock notes, a request basket, and a seeded seller inbox. | 20 | F-2-4 |
| The browser stores the sample workspace and requests under `demo:` keys. | 11 | Pass |
| The server does not retain demo requests. | 7 | Pass |
| Use Reset demo to remove that sample data and create a clean workspace. | 13 | Pass |
| Seller workflow | 2 | Pass |
| Open `/manage` and sign in with Sociobot. | 7 | F-2-2 |
| Your Sociobot account receives its own workspace. | 7 | Pass |
| Download the CSV template. | 4 | Pass |
| Import a file with `sku` and `name` columns. | 8 | Pass |
| Price, description, category, and stock note are optional. | 8 | Pass |
| Create a named client link and share it with that client. | 11 | F-2-2 |
| Review incoming requests in the workspace. | 6 | Pass |
| Export all lines to CSV or print a request to PDF. | 11 | Pass |
| The free workspace supports 12 catalogue rows and one client link. | 11 | Pass |
| Request export stays free. | 4 | Pass |
| The ₹1,499 one-time license supports more rows and links, with up to 5,000 rows per import. | 16 | F-2-1 |
| Checkout and license checks use the Sociobot billing API. | 9 | Pass |
| The app has no payment-card fields. | 6 | Pass |
| Run locally | 2 | Pass |
| Requirements: Node.js 22+, npm, current stable Rust, and SQLite build support. | 11 | Pass |
| Open http://localhost:8080. | 2 | Pass |
| The server creates `DATA_DIR` and its SQLite database on first boot. | 11 | Pass |
| Only `PORT` is needed in production; it defaults to `8080`. | 10 | Pass |
| For frontend work, run the API with `cargo run`, then run `npm run dev` in another terminal. | 17 | Pass |
| Test and verify | 3 | Pass |
| Every public product claim and its sandbox command is listed in `.factory/claims.json`. | 12 | F-2-2 |
| CSV format | 2 | Pass |
| The header names are case-insensitive. | 5 | Pass |
| A blank price becomes POA. | 5 | Pass |
| Deployment and data | 3 | Pass |
| The root Dockerfile builds the Vite frontend and Rust server. | 10 | Pass |
| The container runs as a non-root user and keeps SQLite under `/app/data`. | 12 | Pass |
| Mount that path as a volume. | 6 | Pass |
| `/health` returns the build SHA passed as `BUILD_SHA`. | 8 | Pass |
| Seller workspaces, catalogue rows, client links, and quote requests stay in SQLite. | 12 | Pass |
| Session storage holds the current sign-in token. | 7 | Pass |
| Local storage holds a pasted Sociobot license and its last verification result. | 12 | Pass |
| There are no analytics, advertising scripts, remote fonts, or runtime CDNs. | 11 | Pass |
| See privacy, terms, demo notes, and visual system. | 8 | Pass |
| License | 1 | Pass |
| MIT. | 1 | Pass |
| See LICENSE. | 2 | Pass |

Terminology is otherwise consistent: **catalogue**, **client link**, **request**, **seller workspace**, **SKU**, **POA**, **demo**, and **license** each have one meaning.

## Demo, sandbox, and privacy

**Result: PASS.** A fresh mobile context used the landing action once and arrived at `/?demo=1`. The first rendered screen already showed Northline Supply Co., six product records, product search/categories, and a request basket. The persistent banner read “Demo — sample data, nothing is saved” and exposed **Seller sample**, **Reset demo**, and **Start for real**.

The demo used only `demo:client-catalogue-request:requests` before submission. Reset deleted the submitted state and changed the workspace ID. A context preloaded with sentinel real license/cache/session values retained all three values through demo entry and **Start for real**, while every `demo:` key was removed. The landing, demo entry, and reset generated only same-origin requests. No analytics, advertising, remote font, CDN, or service worker request appeared. The product makes no offline claim.

## Claims and local quality gates

A clean clone was created at `/tmp/client-catalogue-review-2.PFpkTg`, followed by `npm ci` with 0 audit vulnerabilities. Every exact `test` command in `.factory/claims.json` was invoked independently:

| Claim ids | Result |
|---|---|
| demo-isolation, demo-entry, demo-sample-content, poa-price | Pass |
| csv-export, free-export, structured-request, no-card-data | Pass |
| service-boundaries, protected-links, csv-import, csv-header-normalization | Pass |
| print-request, paid-license, billing-handoff, stock-privacy | Pass |
| privacy-runtime, demo-local, demo-reset, csv-import-cap | Pass |
| port-runtime, runtime-storage, container-runtime, health-build | Pass |
| browser-storage, client-data-control, seller-tenancy, paid-license-invalid | Pass |

`npm test` also passed from that clone: 12 Vitest tests, 13 Rust tests, and 45 Playwright tests passed; 5 project-specific tests were skipped. `npm run lint` passed. The production build produced a 37.75 kB raw / 12.27 kB gzip initial application bundle; the 318.10 kB MSAL bundle is lazy-loaded for seller sign-in.

Passing commands do not resolve F-2-1 or F-2-2: the price assertion is not outcome-based, and the sign-in/named-link statements have no manifest entries.

An exploratory full-suite invocation against the live URL produced two expected failures where local-only `test-seller:` credentials were sent to production, plus one parallel demo request that hit the live rate limit. The affected mobile/reflow test passed when rerun alone. The safe live route/keyboard/Axe subset passed 8/8.

## Earlier findings and regression check

Every finding recorded in `review-1.md`, `polish-1.md`, and the accumulated handoff was checked in both live behavior and current code.

| Earlier issue | Current confirmation |
|---|---|
| `F-1-1` — incomplete public-claims contract | **Not fully fixed / regressed.** All 28 listed commands pass, but F-2-1 and F-2-2 show a non-observable price assertion and omitted public capabilities. |
| `F-1-2` — metaphorical section/404 headings | Fixed. Live headings are “How client requests work”, “How to create a request from a CSV”, “What this service does not do”, “Free and paid workspace limits”, and “Page not found”. The old phrases are absent from code. |
| `F-1-3` — catalogue/product-list terminology | Fixed. `product list` is absent from `src`, `README.md`, and `index.html`; live copy uses **catalogue**. |
| `F-1-4` — demo inbox missing from sitemap | Fixed. The live sitemap returns 200 and contains `/demo/inbox`. |
| Demo data could cross replicas or touch real storage | Fixed. Demo claims pass, live traffic is same-origin, reset changes the demo ID, and sentinel real keys remain untouched. |
| Global password / missing Entra seller identity | Partly verified. The live sign-in redirect, authority, client, scope, CSP, and tenant-isolation tests pass. A completed credentialed return remains untested and is now recorded in F-2-2 rather than treated as silently complete. |
| Checkout dead or paid limits bypassed | Entitlement behavior is fixed and the checkout link returns 303 to the hosted checkout. The newly observed amount mismatch is F-2-1. |
| Touch targets, initial keyboard stop, and 200% text reflow | Fixed. The isolated live mobile regression and first-keyboard-stop tests pass. |
| 390 px demo inbox overflow | Fixed. The document remains viewport-wide and the request table scrolls inside its labelled region. |
| Long real RFC3339 timestamp overflow at 320 px / 200% text | Fixed. A newly submitted live request wraps with `overflow-wrap: anywhere`; the exact regression passes. |
| Unknown route returned 200 / generic recovery | Fixed. `/missing-page` returns HTTP 404 with the designed “Page not found” route and working recovery link. |
| Hashed assets lacked cache policy | Fixed in the deployed build; no regression appeared in the current route checks. |
| Missing link revocation or request deletion | Fixed. Backend and `client-data-control` tests pass. |
| Clippy warning | Fixed. `npm run lint` passes. |
| CIAM CSP/API scope and quantity normalization | Fixed. The safe live identity configuration and invalid-quantity tests pass. |
| Candidate/deployment SHA mismatch | Fixed. Live reports `b7679964…`; repository changes after it are verification-document changes only. |

## Structure, links, accessibility, and visual identity

**Result: PASS.** `/`, `/?demo=1`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and `/missing-page` were opened directly at 390 px. Every route has a route-specific title, one h1, one main landmark, `lang="en"`, a 20–155 character description, route canonical, matching Open Graph/Twitter title, social image, SVG favicon, 180 px apple-touch icon, site header, footer, Privacy, and Terms. `/missing-page` returns a real 404. The social image is 1200 × 630.

Client-side navigation focuses the destination h1; Back restores and focuses the landing h1. The skip link is the first keyboard stop. The live mobile reflow test passes at 390 px and at 320 px with 200% root text. Axe WCAG 2 A/AA scans found zero violations on all eight checked routes. The factory URL verifier passed `/`, `/?demo=1`, and `/demo/inbox` with no console/page errors.

Every discovered internal route and asset returned 200, except the intentional 404 route. The checkout endpoint returned 303 to the hosted checkout, `mailto:` was explicit, and the Param Factory link returned 200. `robots.txt`, `sitemap.xml`, favicon, apple-touch icon, and social card returned 200. Security headers include `nosniff`, referrer policy, and CSP with `frame-ancestors` in the response header.

The illuminated-order-desk identity is distinct: deep navy glass fields, mint actions, amber status color, clipped sheet corners, self-hosted Sora, and original order-desk art. It is not a centered generic SaaS hero or a three-card template.

## Missed leverage and AI review

**No additional finding.** The brief implies CSV import, protected client links, a request basket, seller review, CSV/PDF handoff, POA, stock notes, and data control. Those steps exist. Import/export is present, and an AI step would not remove an obvious bottleneck in this narrow deterministic workflow. There is no decorative AI feature or embedded provider key.

## What would make this perfect

1. Make the advertised currency and amount identical to the hosted checkout, then test the checkout outcome rather than the landing copy.
2. Put the Sociobot sign-in and named-link capabilities into the claims manifest with observable sandbox tests.
3. Replace the hero slogan with the concrete SKU/quantity outcome.
4. Remove “realistic” from the sample-data description.
5. Re-run the entire review from a fresh browser context and clean clone. PASS requires zero findings and no untested claim.
