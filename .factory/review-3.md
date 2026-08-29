# Adversarial first-read review 3 — Client Catalogue Request

Reviewed 2026-08-29 UTC against <https://client-catalogue-request.sociobot.in>. Live `/health` reported build `6744f2a7ff39804100fe60f5cf3a24bd13decb4c`; local `0b1fa3d5c55309c6330918c403e9ecd4912a338a` adds verification documents after that product build.

## Verdict: PASS

There are zero findings. The live product is clear before scrolling, opens a genuinely isolated one-click sample, makes only claims covered by the manifest, and passed every declared clean-clone command. No earlier finding was found to be only nominally fixed.

## Cold first read

Fresh Chromium contexts at 390 × 844 and 1440 × 900 loaded the root without scrolling, horizontal overflow, page errors, or console errors.

- **What it does:** turns repeat orders into structured quote requests.
- **For whom:** small B2B sellers who need client orders without an online store.
- **What to click first:** **Try it with sample data**; the adjacent result is “Opens a private sample catalogue. No setup.”

The decisive first-screen text is “Turn repeat orders into clear requests” and “For small B2B sellers who need client orders without running an online store.” The headline names the job, the next sentence names the audience and situation, and the visible primary action names its result. This gate passes.

## Copy audit

Word counts treat hyphenated terms, acronyms, amounts, URLs, and code tokens as one word. Visitor-facing headings, labels, controls, alt text, and footer text are included. Code blocks and CSV examples are excluded as executable/reference material. No item exceeds 22 words; no jargon, banned marketing adjective, inconsistent term, metaphor/mood heading, or non-result button label was found.

### Landing page

| Copy | Words |
|---|---:|
| Skip to main content | 4 |
| Client Catalogue Request | 3 |
| Demo | 1 |
| Seller workspace | 2 |
| Privacy | 1 |
| Private catalogue · structured requests | 4 |
| Turn repeat orders into clear requests | 6 |
| For small B2B sellers who need client orders without running an online store. | 13 |
| Try it with sample data | 5 |
| Opens a private sample catalogue. | 5 |
| No setup. | 2 |
| No checkout or card data | 5 |
| Prices can stay POA | 4 |
| Client links are hard to guess | 6 |
| Glass product sheets flowing into one organized request document. | 9 |
| Catalogue lines become one quote request with SKUs and quantities. | 10 |
| 01 / CLIENT REQUESTS | 3 |
| How client requests work | 4 |
| Import your catalogue CSV. | 4 |
| Share a protected link. | 4 |
| Each client request arrives with SKUs, quantities, contact details, and notes. | 11 |
| Ready for CSV | 3 |
| 02 / HOW IT WORKS | 4 |
| How to create a request from a CSV | 8 |
| Import your CSV | 3 |
| Keep SKU, name, price, category, and stock note fields. | 9 |
| Leave price blank for POA. | 5 |
| Share one client link | 4 |
| Name each link for a client or group. | 8 |
| The catalogue does not expose stock counts. | 7 |
| Export each request | 3 |
| Download the lines as CSV or print a clean request sheet to PDF. | 13 |
| 03 / SERVICE BOUNDARIES | 3 |
| What this service does not do | 6 |
| Use it to collect quote requests. | 6 |
| Confirm prices, stock, shipping, and fulfilment outside this service. | 9 |
| Read how request data is handled | 6 |
| 04 / WORKSPACE LIMITS | 3 |
| Free and paid workspace limits | 5 |
| The free workspace supports 12 catalogue rows and one client link. | 11 |
| Request export stays free. | 4 |
| $15.71 USD one-time | 3 |
| Use more than 12 catalogue rows and create more than one client link. | 13 |
| Buy the full workspace | 4 |
| Have a license? Paste it | 5 |
| Restore license | 2 |
| Checkout opens Sociobot. | 3 |
| See the terms for refund details. | 6 |
| Client Catalogue Request turns private catalogues into clear quote requests. | 10 |
| Terms | 1 |
| Built by Param Factory (external site) | 6 |
| Version 1.0 · Build 6744f2a7ff39 · Original generated artwork. | 7 |

### README

| Copy | Words |
|---|---:|
| Client Catalogue Request | 3 |
| Turn repeat orders into structured quote requests. | 7 |
| Client Catalogue Request is for small B2B sellers who already maintain a catalogue. | 13 |
| A seller imports CSV rows, creates hard-to-guess client links, and receives quote requests with SKUs and quantities. | 17 |
| The request form never collects payment-card data. | 7 |
| Live product: https://client-catalogue-request.sociobot.in | 3 |
| Try the sandbox | 3 |
| Choose Try it with sample data, open `/?demo=1`, or visit https://client-catalogue-request.sociobot.in/?demo=1. | 11 |
| The isolated demo includes six sample products, two POA prices, stock notes, a request basket, and a seeded seller inbox. | 20 |
| The browser stores the sample workspace and requests under `demo:` keys. | 11 |
| The server does not retain demo requests. | 7 |
| Use Reset demo to remove that sample data and create a clean workspace. | 13 |
| Seller workflow | 2 |
| Open `/manage` and sign in with Sociobot. | 7 |
| Your Sociobot account receives its own workspace. | 7 |
| Download the CSV template. | 4 |
| Import a file with `sku` and `name` columns. | 8 |
| Price, description, category, and stock note are optional. | 8 |
| Create a named client link and share it with that client. | 11 |
| Review incoming requests in the workspace. | 6 |
| Export all lines to CSV or print a request to PDF. | 11 |
| The free workspace supports 12 catalogue rows and one client link. | 11 |
| Request export stays free. | 4 |
| The $15.71 USD one-time license supports more rows and links, with up to 5,000 rows per import. | 16 |
| Checkout and license checks use the Sociobot billing API. | 9 |
| The app has no payment-card fields. | 6 |
| Run locally | 2 |
| Requirements: Node.js 22+, npm, current stable Rust, and SQLite build support. | 11 |
| Open http://localhost:8080. | 2 |
| The server creates `DATA_DIR` and its SQLite database on first boot. | 11 |
| Only `PORT` is needed in production; it defaults to `8080`. | 10 |
| For frontend work, run the API with `cargo run`, then run `npm run dev` in another terminal. | 17 |
| Test and verify | 3 |
| Every public product claim and its sandbox command is listed in `.factory/claims.json`. | 12 |
| CSV format | 2 |
| The header names are case-insensitive. | 5 |
| A blank price becomes POA. | 5 |
| Deployment and data | 3 |
| The root Dockerfile builds the Vite frontend and Rust server. | 10 |
| The container runs as a non-root user and keeps SQLite under `/app/data`. | 12 |
| Mount that path as a volume. | 6 |
| `/health` returns the build SHA passed as `BUILD_SHA`. | 8 |
| Seller workspaces, catalogue rows, client links, and quote requests stay in SQLite. | 12 |
| Session storage holds the current sign-in token. | 7 |
| Local storage holds a pasted Sociobot license and its last verification result. | 12 |
| There are no analytics, advertising scripts, remote fonts, or runtime CDNs. | 11 |
| See privacy, terms, demo notes, and visual system. | 8 |
| License | 1 |
| MIT. See LICENSE. | 3 |

Terminology is consistent: **catalogue**, **client link**, **request**, **seller workspace**, **SKU**, **POA**, **demo**, and **license** each retain one meaning.

## Demo and sandbox

From a new 390 px context, one click on **Try it with sample data** opened `/?demo=1`. Its first rendered screen already contained the Northline Supply Co. catalogue, six products, two POA prices, stock notes, categories, and the request basket. The persistent banner was exactly “Demo — sample data, nothing is saved” and exposed **Seller sample**, **Reset demo**, and **Start for real**.

The sample seller inbox at `/demo/inbox` contained the seeded Juniper Corner request. **Reset demo** removed the `demo:client-catalogue-request:requests` key and opened a new sample workspace. A fresh-context request log for landing, demo entry, and reset contained only the product origin. While the banner was shown, demo state used only the `demo:` namespace; a pre-existing license/cache/session sentinel was not read or altered by demo entry. Leaving demo removed the `demo:` keys.

The product makes no offline claim. No analytics, advertising, remote-font, CDN, or third-party product request was observed in the demo flow.

## Claims and clean-clone tests

A fresh clone at `/tmp/client-catalogue-review-3.W1NeQ0` received `npm ci` with zero audit vulnerabilities. Every exact command named in `.factory/claims.json` was run independently. All 31 passed:

| Claim ids | Result |
|---|---|
| demo-isolation, demo-entry, demo-sample-content, poa-price | Pass |
| csv-export, free-export, structured-request, no-card-data | Pass |
| service-boundaries, protected-links, csv-import, csv-header-normalization | Pass |
| print-request, paid-license, billing-price, billing-handoff | Pass |
| stock-privacy, privacy-runtime, demo-local, demo-reset | Pass |
| csv-import-cap, port-runtime, runtime-storage, container-runtime | Pass |
| health-build, browser-storage, client-data-control, seller-tenancy | Pass |
| sociobot-sign-in, named-client-links, paid-license-invalid | Pass |

This includes outcome-based checkout-price coverage for `$15.71 USD` one-time, the recorded CIAM return for seller sign-in, named-link persistence after reload, demo request isolation/reset, and outgoing-request privacy coverage. A scan of landing and README claim-like copy found a matching manifest entry for every factual product promise.

## Earlier findings and regression check

Each earlier report and handoff was read, then its issue was checked on the current live site and in code rather than accepted from its status label.

| Earlier finding | Current confirmation |
|---|---|
| F-1-1: incomplete claims contract | Fixed. The manifest has 31 observable entries; every exact command passed. |
| F-1-2: metaphorical headings | Fixed. Landing headings and the 404 use literal section/page names. |
| F-1-3: product-list/catalogue drift | Fixed. Public copy consistently uses catalogue. |
| F-1-4: demo inbox absent from sitemap | Fixed. `/demo/inbox` is in the live sitemap. |
| F-2-1: INR price conflicted with checkout | Fixed. Landing, README, workspace, fixture, and hosted checkout agree on `$15.71 USD` one-time. |
| F-2-2: sign-in/named-link claims missing | Fixed. Both have isolated manifest entries and passing observable tests. |
| F-2-3: slogan caption | Fixed. The caption now names the SKU/quantity result. |
| F-2-4: subjective “realistic” sample claim | Fixed. Copy now says sample products. |
| Demo persistence, routing, mobile/reflow, inaccessible 404, cache, revocation/deletion, lint, identity, and quantity regressions recorded in verification/polish reports | Fixed. Current live route, mobile, demo, headers, Axe, link, and clean-clone checks did not reproduce them. |

## Structure, accessibility, links, and visual identity

`/`, `/?demo=1`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, and `/manage` returned 200; `/missing-page` returned an intentional HTTP 404 with “Page not found” and a working return action. Every checked route had a route-specific title in the required pattern, one h1, one main landmark, a description, canonical URL, shared header/footer, Privacy/Terms links, favicon/apple icon, and no 390 px overflow. Axe WCAG 2 A/AA reported zero violations on all eight routes.

All discovered product links resolved with 200, explicit `mailto:`, or the intended 303 checkout handoff. The checkout link points to Sociobot, which redirects to the hosted Dodo checkout. The live response includes `nosniff`, referrer policy, CSP, and response-header `frame-ancestors`. The original illuminated-order-desk identity remains specific to this product: ink-navy glass panels, mint selection, amber caution, clipped sheets, self-hosted Sora, and original generated order-desk art; it is not a generic feature-card SaaS template.

Client-side navigation and Back returned focus to the new h1. The first keyboard stop is the skip link. No console/page errors occurred on normal routes; the browser's expected failed-resource message on the intentional HTTP-404 response was not treated as an application error.

## Missed leverage / AI review

No finding. The brief implies CSV import, protected client links, a request basket, seller-side review, CSV/PDF handoff, POA handling, stock notes, and client-data control. All are present and covered. An AI step would not remove an obvious bottleneck in this deterministic request workflow, and no decorative AI feature or provider key is present.

## What would make this perfect

The product meets the stated standard in this review. Preserve the same discipline on future changes: retain the direct demo URL and `demo:` isolation, update the manifest and its outcome test before adding factual copy, and rerun the full cold-read/clean-clone review after changes to checkout, sign-in, or client-data handling.
