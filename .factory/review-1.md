# Adversarial first-read review 1 — Client Catalogue Request

Reviewed 2026-08-29 UTC against <https://client-catalogue-request.sociobot.in> at commit `efd95c15d981abef896b592ce7fcb7d97487a1d9`.

## Verdict: FAIL

The core product is clear and tryable, and the declared test suite passes. It does not meet the supplied acceptance contract because relied-upon live and README statements are not all represented by testable entries in `.factory/claims.json`. There are also copy-rule and sitemap defects. There are four findings, ordered by severity.

### F-1-1 — BLOCKING: public claims have no declared observable test

**Locations and exact quotes**

- Landing paid tier: “Sociobot is the merchant of record. Refunds are handled there.”
- README, introduction: “It does not take payment or manage inventory.”
- README, Try the sandbox: “It includes six realistic products, POA pricing, stock caveats, a request basket, and a sample seller inbox.”
- README, Seller workflow: “Request export stays free.”
- README, Seller workflow: “Checkout and license checks use Sociobot; no payment provider runs in this app.”
- README, CSV format: “The header names are case-insensitive.”
- README, Run locally: “The server creates `DATA_DIR` and its SQLite database on first boot.”
- README, Deployment and data: “Seller tenant data, catalogue rows, client tokens, and quote requests stay in SQLite.”
- README, Deployment and data: “Browser storage contains an Entra sign-in token and any pasted Sociobot license.”

**Why this fails:** each statement is a factual promise a visitor or operator can rely on. None has a matching claim entry and sandbox test. Existing entries cover POA display, structured requests, no card fields, runtime port defaults, seller isolation, and no remote runtime assets; they do not prove the quoted scope, sample count, pricing/export policy, header normalization, database location/creation, or browser-storage behaviour. The claims contract requires every claim-like sentence to be listed and tested, and makes an unlisted claim a review failure.

**Concrete fix:** either remove/narrow each unsupported promise, or add one `claims.json` entry and an observable clean-sandbox test per promise. For example, add `demo-sample-content` (assert six products, two POA items, stock notes, basket, and seeded inbox); `csv-header-normalization` (import mixed-case headers); `free-export` (export with an unlicensed seller); `storage-boundaries` (inspect the created SQLite path and browser keys); and separate billing/refund/provider statements only if a Sociobot fixture can prove them. Do not count a test that only asserts a button exists.

### F-1-2 — Minor: several headings are slogans or metaphors, rather than section names

**Locations and exact quotes**

- Landing request-layer section: “Keep your catalogue. Lose the retyping.”
- Landing workflow section: “From price list to usable request”
- Landing boundaries section: “A request desk, not a storefront”
- Landing paid section: “Keep the free desk, or raise its limits”
- Designed 404 page: “This request sheet went missing”

**Why this fails:** a first-time visitor and a screen-reader heading list should be able to name each section without translating a metaphor. “Desk” and “sheet” are product lore, and “lose the retyping” does not identify the information in the section. This conflicts with the plain-words requirement that headings name their section and avoid mood/metaphor copy.

**Concrete fix:** replace them respectively with “How client requests work”, “How to create a request from a CSV”, “What this service does not do”, “Free and paid workspace limits”, and “Page not found”.

### F-1-3 — Minor: one concept has competing names

**Locations and exact quotes**

- Landing: “Private catalogue · structured requests”; “Keep your catalogue.”
- README: “small B2B sellers who already maintain a product list”; “Share a private product list”.

**Why this fails:** the product calls the same item a “catalogue” in the interface and a “product list” in its README. A cold visitor has to infer whether these are distinct things. The copy contract requires one term for one concept.

**Concrete fix:** use **catalogue** throughout, for example: “small B2B sellers who already maintain a catalogue” and “Share a private catalogue and receive quote requests.”

### F-1-4 — Minor: a linked public route is absent from the sitemap

**Location:** [public/sitemap.xml](../public/sitemap.xml) omits `/demo/inbox`, while the persistent demo banner links to “Seller sample” at `/demo/inbox`.

**Why this fails:** the site-structure contract requires the sitemap to list real routes. This public, addressable route shows the seller-side half of the sample workflow, yet crawlers are not told it exists.

**Concrete fix:** add `https://client-catalogue-request.sociobot.in/demo/inbox` to `public/sitemap.xml` (or make it intentionally non-public and remove the public link). Keep private token routes out of the sitemap.

## Mandatory cold-read gate

**Result: PASS.** Fresh Chromium contexts were opened at 390 × 844 and 1440 × 900 without scrolling.

- What it does: it turns repeat B2B orders into quote requests.
- For whom: small B2B sellers who want client orders without an online shop.
- First action: **Try it with sample data**; adjacent copy says it opens a private sample catalogue with no setup.

The exact first-screen copy was: “Turn repeat orders into clear requests”, “For small B2B sellers who need client orders without running an online store.”, and “Try it with sample data”. The hero fits at 390 px without horizontal overflow and had no console errors.

## Copy audit

Method: word counts treat a hyphenated term, number, URL, and acronym as one word. Controls, headings, facts, and footer copy are included because they are visitor-facing copy. Shell snippets and CSV examples are excluded as executable/reference syntax. No audited sentence exceeds 22 words; flags are marked below.

### Landing page

| Copy | Words | Result |
|---|---:|---|
| Skip to main content | 4 | Pass |
| Client Catalogue Request | 3 | Pass |
| Demo | 1 | Pass |
| Seller workspace | 2 | Pass |
| Privacy | 1 | Pass |
| Private catalogue · structured requests | 4 | Pass |
| Turn repeat orders into clear requests | 7 | Pass |
| For small B2B sellers who need client orders without running an online store. | 13 | Pass |
| Try it with sample data | 6 | Pass |
| Opens a private sample catalogue. | 5 | Pass |
| No setup. | 2 | Pass |
| No checkout or card data | 5 | Pass — `no-card-data` |
| Prices can stay POA | 4 | Pass — `poa-price` |
| Client links are hard to guess | 6 | Pass — `protected-links` |
| Glass product sheets flowing into one organized request document. | 9 | Pass (image alt) |
| One private catalogue. | 3 | Pass |
| One request you can use. | 5 | Pass |
| 01 / THE REQUEST LAYER | 4 | Pass |
| Keep your catalogue. | 3 | **F-1-2** |
| Lose the retyping. | 3 | **F-1-2** |
| Import your product CSV. | 4 | Pass |
| Share a protected link. | 4 | Pass |
| Each client request arrives with SKUs, quantities, contact details, and notes. | 11 | Pass — `structured-request` |
| Ready for CSV | 3 | Pass |
| 02 / HOW IT WORKS | 4 | Pass |
| From price list to usable request | 6 | **F-1-2** |
| Import your CSV | 3 | Pass |
| Keep SKU, name, price, category, and stock note fields. | 9 | Pass — `csv-import` |
| Leave price blank for POA. | 5 | Pass — `poa-price` |
| Share one client link | 4 | Pass |
| Name each link for a client or group. | 8 | Pass |
| The catalogue does not expose stock counts. | 7 | Pass — `stock-privacy` |
| Export each request | 3 | Pass |
| Download the lines as CSV or print a clean request sheet to PDF. | 13 | Pass — `csv-export`, `print-request` |
| 03 / CLEAR BOUNDARIES | 4 | Pass |
| A request desk, not a storefront | 6 | **F-1-2** |
| It does not take payments, promise stock, calculate shipping, or manage fulfillment. | 12 | **F-1-1** for inventory/shipping scope |
| You review every request before quoting. | 6 | Pass — request review is shown in demo inbox |
| Read how request data is handled | 6 | Pass |
| 04 / PAID TIER | 4 | Pass |
| Keep the free desk, or raise its limits | 8 | **F-1-2** |
| The free workspace supports 12 catalogue rows and one client link. | 11 | Pass — `paid-license` |
| Request export stays free. | 4 | **F-1-1** |
| ₹1,499 one-time | 2 | Pass — `paid-license` |
| Use more than 12 catalogue rows and create more than one client link. | 12 | Pass — `paid-license` |
| Buy the full workspace | 4 | Pass; link returned HTTP 303 to Sociobot checkout |
| Restore license | 2 | Pass |
| Sociobot is the merchant of record. | 6 | **F-1-1** |
| Refunds are handled there. | 4 | **F-1-1** |
| Client Catalogue Request turns private product lists into clear quote requests. | 11 | **F-1-3** (`product lists`) |
| Terms | 1 | Pass |
| Built by Param Factory (external site) | 5 | Pass; external link returned HTTP 200 |
| Version 1.0 · Build 979fd37f967f · Original generated artwork. | 7 | Pass (build/provenance label) |

### README

| Copy | Words | Result |
|---|---:|---|
| Turn repeat orders into structured quote requests. | 7 | Pass |
| Client Catalogue Request is for small B2B sellers who already maintain a product list. | 14 | **F-1-3** |
| A seller imports CSV rows, creates hard-to-guess client links, and receives quote requests with SKUs and quantities. | 17 | Pass — import/protected/structured claims |
| It does not take payment or manage inventory. | 8 | **F-1-1** (`manage inventory`) |
| Live product: URL | 3 | Pass |
| Open `/demo` or URL. | 4 | Pass |
| It includes six realistic products, POA pricing, stock caveats, a request basket, and a sample seller inbox. | 15 | **F-1-1** |
| The browser stores the isolated sample workspace and sample requests. | 9 | Pass — `demo-isolation` |
| The server does not retain demo requests. | 7 | Pass — `demo-local` |
| Use **Reset demo** to remove the sample data from this browser. | 10 | Pass — `demo-reset` |
| Open `/manage` and sign in with Sociobot. | 7 | Pass |
| Your Sociobot account receives its own workspace. | 7 | Pass — `seller-tenancy` |
| Download the CSV template. | 4 | Pass |
| Import a file with `sku` and `name` columns. | 9 | Pass — `csv-import` |
| Price, description, category, and stock note are optional. | 8 | Pass — `csv-import` |
| Create a named client link and share it with that client. | 11 | Pass |
| Review incoming requests in the workspace. | 6 | Pass |
| Export all lines to CSV or print a request to PDF. | 11 | Pass — export/print claims |
| The free workspace supports 12 catalogue rows and one client link. | 11 | Pass — `paid-license` |
| Request export stays free. | 4 | **F-1-1** |
| The ₹1,499 one-time license supports more rows and links, with up to 5,000 rows per import. | 18 | Pass — `paid-license`, `csv-import-cap` |
| Checkout and license checks use Sociobot; no payment provider runs in this app. | 12 | **F-1-1** |
| Requirements: Node.js 22+, npm, current stable Rust, and SQLite build support. | 10 | Pass |
| Open URL. | 2 | Pass |
| The server creates `DATA_DIR` and its SQLite database on first boot. | 10 | **F-1-1** |
| Only `PORT` is needed in production; it defaults to `8080`. | 8 | Pass — `port-runtime` |
| For frontend work, run the API with `cargo run`, then run `npm run dev` in another terminal. | 17 | Pass |
| Vite proxies `/api` and `/health` to port 8080. | 8 | **F-1-1** |
| `npm test` runs unit tests, the complete Rust API flow, claim tests in Chromium, a 390 px mobile pass, and automated accessibility checks. | 22 | Pass |
| Every public claim and its sandbox evidence is listed in `.factory/claims.json`. | 11 | **F-1-1** until the quoted omissions are fixed |
| The header names are case-insensitive. | 5 | **F-1-1** |
| A blank price becomes POA. | 5 | Pass — `poa-price` |
| Prices use major currency units in CSV and integer minor units in the API. | 14 | **F-1-1** |
| The root `Dockerfile` builds the Vite frontend and Rust server. | 9 | **F-1-1** |
| It runs as a non-root user and persists SQLite under `/app/data`. | 11 | **F-1-1** |
| Mount that path as a volume. | 6 | Pass (instruction) |
| `/health` returns the build SHA passed as `BUILD_SHA`. | 8 | **F-1-1** |
| Seller tenant data, catalogue rows, client tokens, and quote requests stay in SQLite. | 11 | **F-1-1** |
| Browser storage contains an Entra sign-in token and any pasted Sociobot license. | 12 | **F-1-1** |
| There are no analytics, advertising scripts, remote fonts, or runtime CDNs. | 11 | Pass — `privacy-runtime` |
| See privacy, terms, demo notes, and visual system. | 8 | Pass |
| MIT. | 1 | Pass |
| See LICENSE. | 2 | Pass |

No button label failed the result-naming-verb check. The primary label, “Try it with sample data”, names the outcome and has adjacent result copy.

## Demo, sandbox, and privacy checks

**Result: PASS.** From a new browser context, one tap on the first-screen action opened `/demo` with six product cards immediately visible, including POA products and stock notes. The persistent banner read exactly “Demo — sample data, nothing is saved” and included **Seller sample**, **Reset demo**, and **Start for real**.

A one-line sample request for `Dana Mills` was sent, then appeared in `/demo/inbox` beside the seeded Juniper Corner request. The browser held only `demo:client-catalogue-request:requests` and, after submission, `demo:client-catalogue-request:submitted`; no real tenant key was used. **Reset demo** removed the submitted key and returned a new demo workspace. The complete cold landing and demo flow made same-origin requests only; no analytics, advertising, remote font, CDN, or API origin request occurred. No console or page errors occurred in the tested flow.

## Claims

**Declared-command result: PASS, with F-1-1 exception above.** A fresh clone at `/tmp/client-catalogue-review-1.zt83Oq` received `npm ci --ignore-scripts` (0 vulnerabilities). Every exact command in `.factory/claims.json` passed independently:

| Claim id | Result |
|---|---|
| demo-isolation | Pass |
| poa-price | Pass |
| csv-export | Pass |
| structured-request | Pass |
| no-card-data | Pass |
| protected-links | Pass |
| csv-import | Pass |
| print-request | Pass |
| paid-license | Pass |
| stock-privacy | Pass |
| privacy-runtime | Pass |
| demo-local | Pass |
| demo-reset | Pass |
| csv-import-cap | Pass |
| port-runtime | Pass |
| client-data-control | Pass |
| seller-tenancy | Pass |
| paid-license-invalid | Pass |

`npm test` also passed from that clone (8 Vitest, 11 Rust, 38 Playwright tests), and `npm run lint` passed. These results do not cure an unlisted public statement.

## Earlier-review regression check

There are no earlier `review-*.md` or `polish-*.md` files. I read `.factory/verification.md`, `verification-2.md`, `verification-3.md`, and the prior handoff. Each earlier finding was checked rather than accepted as marked fixed:

| Earlier finding | Current confirmation |
|---|---|
| Demo could be lost across replicas | Fixed: a fresh live request reached the sample inbox; `demo-local` and `demo-isolation` passed. |
| Global password workspace / missing Entra | Fixed as far as a credential-free reviewer can verify: `/manage` shows Sociobot sign-in; live CSP permits the CIAM host; the test verifies the configured product scope and seller-isolation test passes. |
| Checkout was dead / limits bypassed | Fixed: checkout returned HTTP 303 to Sociobot/Dodo; paid-limit claim and backend entitlement tests passed. |
| Missing claim entries | Partly regressed/not complete: F-1-1 identifies remaining unlisted public claims. |
| Touch targets, initial keyboard stop, 200% reflow | Fixed: full Playwright mobile suite passed; first Tab reaches the skip link. |
| Unknown route HTTP 200 | Fixed: `/missing-page` returned HTTP 404 and a designed recovery page. |
| Hashed assets had no cache policy | Fixed by the checked release history; no regression surfaced in the fresh suite. |
| No link revocation/request deletion | Fixed: backend and `client-data-control` test passed. |
| Clippy warning | Fixed: `npm run lint` passed. |
| CIAM CSP/API scope and quantity normalization | Fixed by the full Playwright suite and live credential-free redirect configuration check. |

## Structure, accessibility, and live-route checks

**Pass except F-1-4.** `/`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and `/missing-page` each had a route title, one h1 after load, `<main>`, `lang="en"`, a description, canonical URL, OG image, favicon/apple-touch icon, and the shared header/footer. The landing title is `Client Catalogue Request — collect quote requests`; route titles use the documented pattern. The social card is 1200 × 630. `robots.txt` and a designed HTTP-404 response are present.

Address-bar routes worked; browser Back restored the landing page and placed focus on its h1. A fresh client-side navigation to Demo placed focus on `Northline Supply Co.` h1 after rendering. All discovered product links returned an appropriate result: product routes/assets 200, the checkout 303, mailto explicit, and the Param Factory link 200. The checkout is external but is named by its result.

Direct Axe 4.10.2 scans at 390 px found zero violations, including zero serious/critical violations, on all seven routes above. The cold 390 px and desktop loads had no console errors, and the 390 px page width stayed at 390 px.

## Missed leverage / AI review

**No finding.** The brief calls for CSV import, protected client links, structured quote requests, CSV/PDF handoff, POA, and stock notes. Those workflow steps are present. An AI drafting or classification feature would not be an obvious required step for this narrow ordering workflow, so no decorative AI feature is expected.

## What would make this perfect

1. Resolve F-1-1 by making the claims manifest a complete, observable contract for every relied-upon statement, or removing unsupported statements.
2. Use literal section names throughout and one term, **catalogue**, for the product list.
3. Add the linked demo seller inbox route to the sitemap.
4. Re-run this entire review from a fresh browser context and fresh clone; a PASS requires zero findings and no untested claim.
