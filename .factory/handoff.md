# Review 1 handoff — Client Catalogue Request

Reviewed 2026-08-29 UTC. Work order: `client-catalogue-request-review-1`.

## Result: FAIL

No product code was modified. The full review is in `.factory/review-1.md`.

The live product passes the cold-read, one-click demo, request submission/inbox/reset, same-origin privacy, route/metadata, accessibility, and declared-test checks. All 18 commands listed in `.factory/claims.json` passed from a fresh clone, as did `npm test` and `npm run lint`.

Four findings remain:

1. **Blocking:** several factual landing/README statements lack a matching claims-manifest entry and observable sandbox test.
2. **Minor:** slogan/metaphor headings do not name their sections.
3. **Minor:** “catalogue” and “product list” name the same concept.
4. **Minor:** the public `/demo/inbox` route is missing from the sitemap.

## How to reproduce verification

```sh
npm ci
npm test
npm run lint
```

Then open `https://client-catalogue-request.sociobot.in` in a new 390 px browser context, choose **Try it with sample data**, submit a sample request, open **Seller sample**, and choose **Reset demo**. See `.factory/review-1.md` for the exact claim commands, quote locations, and expected outcomes.
