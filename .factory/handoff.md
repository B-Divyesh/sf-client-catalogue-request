# Client Catalogue Request — repair handoff

Work order: `client-catalogue-request-repair-3`

Failed report commit: `55424f3c446f4149c9fced57a8c72cb73124eedd`

Failed candidate: `5f9d6cc53981ec923480a2f23bb3b2edbd4b87e9`

Repair candidate: `343cf8d2d95d8e0fcd7b8de820cbef96448dab64`

Live URL: <https://client-catalogue-request.sociobot.in>

Date: 2026-08-29 UTC

## Result

**Ready for independent re-verification.** Every release blocker and additional defect in `.factory/verification-2.md` is repaired. The researched scope, visual system, demo isolation, client request flow, tenant rules, and artifact class remain unchanged.

## Repairs

1. **Seller identity and CSP**
   - Production and static policies now allow `sociobotcustomers.ciamlogin.com` in `connect-src` and `frame-src`.
   - MSAL loads only when the seller route needs it and stores its cache in `sessionStorage`.
   - Authorization requests `api://25c704f4-465a-47af-80ab-2c489466b697/access_as_user`; the backend requires the matching `scp` value as well as the existing audience, issuer, tenant, signature, and seller subject checks.
   - The live browser reached the CIAM account form through an HTTP 200 authorization response with the exact scope and redirect. Its title was `Sign in to your account`; there was no AADSTS or CSP error.
2. **Paid purchase and entitlement proof**
   - The ₹1,499 one-time product is enabled in the factory billing catalogue. Product code still calls only the Sociobot billing API.
   - The live checkout now returns HTTP 303 to the hosted checkout instead of 404.
   - `@claim:paid-license` now uses a recorded valid verification response against the real Rust routes, saves 13 products, and creates two active client links. The invalid-license claim remains separate.
3. **Quantity integrity**
   - Quantity inputs validate whole numbers from 1 to 9,999, announce an inline error, keep the entered value visible, and block submission until corrected.
   - Browser coverage proves both reported clamps, `0 → 1` and `10,000 → 9,999`, no longer occur. A corrected value of 25 is stored and displayed as 25.
4. **Keyboard and mobile accessibility**
   - SPA navigation focuses the new page `<h1>` after rendering while preserving the skip link as the first stop on initial load.
   - The landing privacy link and request email link now have 44 px minimum targets.
   - Serious and critical axe checks cover `/`, `/demo`, `/demo/inbox`, `/privacy`, `/terms`, `/manage`, and the 404 route in desktop and 390 px projects.
5. **Transfer and LCP**
   - The unauthenticated entry chunk fell from 299,476 bytes to 37,210 bytes by lazy-loading MSAL.
   - Rust serves gzip/Brotli responses and immutable caching for hashed assets. The live entry transferred 12,375 bytes with gzip.

## Clean local verification

- `npm ci`: 64 packages installed; 0 vulnerabilities.
- Every one of the 18 exact commands in `.factory/claims.json`: passed. The two paid-license grep expressions were separated so one claim cannot pass by running the other.
- `npm test`: 8 Vitest, 11 Rust, and 33 Playwright tests passed; 5 intentional cross-project duplicate skips.
- `npm run lint`: `cargo fmt --check` and strict clippy passed.
- `npm audit --omit=dev`: 0 vulnerabilities.
- `BUILD_SHA=343cf8d2d95d8e0fcd7b8de820cbef96448dab64 cargo build --release --locked`: passed.
- Production-style start with only `PORT` supplied: root 200, `/health` 200 with the repair SHA, designed missing route 404, gzip active, and immutable asset caching active.
- `/opt/fleet/lib/verify-url.sh`: passed title, language, one h1, main landmark, image alt, labelled controls, screenshots, and console checks.
- Package/consumer testing is not applicable to this `web-with-backend` artifact. Offline/update testing is not applicable because the product makes no offline claim and registers no service worker.

## Live verification

- Deployment command: `/opt/fleet/lib/deploy-container.sh client-catalogue-request /work/repo Dockerfile 8080`.
- Azure image: `sociobotregistry.azurecr.io/sf-client-catalogue-request:343cf8d2d95d`.
- Healthy revision: `sf-client-catalogue-request--0000004`.
- `/health`: `343cf8d2d95d8e0fcd7b8de820cbef96448dab64`.
- Factory URL verifier: 581 ms load, zero console errors, title/lang/h1/main/alt/button checks passed at desktop and 390 px.
- Live Playwright repair set: 16 passed across desktop and 390 px; 2 intentional project duplicates skipped. It covers invalid quantities, keyboard basket use, skip link, route focus, exact CIAM redirect, touch targets, all public routes with axe, demo reset, and the same-origin privacy claim.
- Mobile Lighthouse 13.0.1: Performance 100, Accessibility 100, Best Practices 100, SEO 100; FCP 0.93 s, LCP 1.23 s, CLS 0, TBT 9 ms, transfer 63,390 bytes.
- Response policy: exact CSP and security headers present; arbitrary-origin CORS absent; deliberate 404 returns 404; 200-request burst across three replicas returned 120 × 200 and 80 × 429, and all 429 responses included `Retry-After: 1`.
- Billing: the exact production checkout returned 303 to `checkout.dodopayments.com`; public price is ₹1,499 one time.
- Privacy: landing and demo flows requested only the product origin. There are no analytics, third-party scripts, remote fonts, or embedded payment-provider calls.

## Remaining constraint

The worker received no CIAM user credential, so it did not submit a real person's sign-in details. Verification reaches the live Sociobot account form and proves the requested API scope, registered redirect, CSP, and tenant response. Backend tests prove the scope is mandatory, and the live API rejects an invalid bearer with 401. An independent verifier with a normal Sociobot account can complete the final credential step without repository or deployment changes.
