# Polish round 1 — cumulative finding closure

Completed 2026-08-29 for work order `client-catalogue-request-polish-1`. Base review commit: `d2c3a2094293ffdddb0d1db8e64a7ae056ce16fe`. Runtime repair commit: `8af4286eb03a86b53815fa0f2e3723545e500271`. Live-aware verifier fix: `d397edd455639d2d50220e41065cc4d4ea894557`.

## Finding map

| Finding | Change made | Automated evidence | Screenshot and live evidence |
|---|---|---|---|
| `F-1-1` — public claims lacked declared observable tests | Expanded `.factory/claims.json` from 18 to 28 entries. Added real tests for one-click/query demo entry, sample contents, free export, service boundaries, mixed-case CSV headings, Sociobot billing handoff/refund lockout, SQLite bootstrap/persistence, container runtime, health build identity, and browser namespaces. Removed the unsupported inventory, Vite-proxy, and API-unit prose. Narrowed payment copy to observable behavior. Added a manifest test that requires one test per tagged claim. | `claims manifest > has unique IDs and a real test for every declared claim`; `@claim:demo-entry`; `@claim:demo-sample-content`; `@claim:free-export`; `@claim:service-boundaries`; `@claim:csv-header-normalization`; `@claim:billing-handoff`; `runtime_creates_and_reopens_sqlite_storage`; `@claim:container-runtime`; `health_returns_supplied_build_sha`; `@claim:browser-storage`. All 28 exact manifest commands passed independently from clean clone `/tmp/client-catalogue-polish-claims.etEtw3`. | [Landing desktop](evidence/polish-1-landing/screenshot-desktop.png), [demo desktop](evidence/polish-1-demo/screenshot-desktop.png). Live cold flow at <https://client-catalogue-request.sociobot.in/?demo=1> showed six products, two POA prices, seeded inbox, same-origin-only traffic, and a changed workspace ID after reset. |
| `F-1-2` — metaphorical section headings | Replaced all five cited headings with literal names: “How client requests work”, “How to create a request from a CSV”, “What this service does not do”, “Free and paid workspace limits”, and “Page not found”. Also removed uncited “request desk”, “protected door”, “list you already keep”, and “use every line” heading metaphors. | `public routes keep the document skeleton and load without console errors`; updated `.factory/copy-audit.md` has zero metaphor or banned-word flags. | [Landing mobile](evidence/polish-1-landing/screenshot-mobile.png). Live DOM returned the four exact landing h2 values above; `/missing-page` returned HTTP 404 with h1 “Page not found”. |
| `F-1-3` — “catalogue” and “product list” competed | Standardized landing metadata, footer, README, and workspace copy on **catalogue**. | `rg -i 'product list' src README.md index.html` returned no matches; `@claim:csv-import` still completed the catalogue-to-request workflow. | [Landing desktop](evidence/polish-1-landing/screenshot-desktop.png). Live description: “Share a private catalogue and receive quote requests without running an online store.” |
| `F-1-4` — `/demo/inbox` absent from sitemap | Added the seller sample route to `public/sitemap.xml`. | `public routes keep the document skeleton and load without console errors`; sitemap curl assertion. | Live <https://client-catalogue-request.sociobot.in/sitemap.xml> returned 200 and contains `https://client-catalogue-request.sociobot.in/demo/inbox`. |

## Required acceptance checks beyond the four findings

- The first-screen action now targets `/?demo=1`. That URL and `/demo` share the isolated `demo:` namespace. The persistent banner exposes **Seller sample**, **Reset demo**, and **Start for real**. Leaving demo discards its keys.
- Route navigation preserves query strings, updates title, description, canonical, Open Graph, and Twitter metadata, announces the h1, and restores h1 focus on Back. The backend returns a real HTTP 404 for unknown paths.
- Privacy and Terms are present in every footer. The paid disclosure remains linked to Sociobot checkout; the public checkout returned HTTP 303.
- At 390 px, landing and demo `scrollWidth` equal `clientWidth`; demo banner controls remain at least 44 px. The mobile demo shows all six product records without clipping.
- The visual system remains the original illuminated-order-desk design: navy glass surfaces, mint actions, amber cautions, clipped sheet corners, self-hosted Sora, and the original generated order-desk art.

## Evidence index

- Landing verifier: [JSON](evidence/polish-1-landing/verify.json), [desktop](evidence/polish-1-landing/screenshot-desktop.png), [mobile](evidence/polish-1-landing/screenshot-mobile.png)
- Demo verifier: [JSON](evidence/polish-1-demo/verify.json), [desktop](evidence/polish-1-demo/screenshot-desktop.png), [mobile](evidence/polish-1-demo/screenshot-mobile.png)
- Lighthouse JSON: [polish-1-lighthouse.json](evidence/polish-1-lighthouse.json)
- Live browser suite: 43 passed, 3 intentional project-specific skips, 0 failed. Axe found no serious or critical issue on the routed pages.
- Lighthouse mobile: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.9 s, LCP 1.2 s, TBT 20 ms, CLS 0, total transfer 62 KiB.

No finding from review 1 remains open. There were no earlier `review-*.md` or `polish-*.md` reports.
