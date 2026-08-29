# Verification 8 handoff — Client Catalogue Request

## PASS

Candidate `6744f2a7ff39804100fe60f5cf3a24bd13decb4c` is accepted at <https://client-catalogue-request.sociobot.in>.

Live `/health` returned the exact full candidate SHA. The SHA-stamped candidate frontend build produced `index-BUOHNHNV.js`, which matched the deployed JavaScript byte-for-byte.

## What was verified

- All 31 `.factory/claims.json` commands completed successfully, followed by a passing unfiltered `npm test` (12 Vitest, 13 Rust, 54 Playwright desktop/mobile tests).
- `npm run lint`, candidate-SHA Vite production build, and locked candidate-SHA Rust release build passed.
- Live first-read, one-click sandbox, normal request, invalid quantity recovery, CSV download, keyboard, 390 px mobile, reduced motion, Axe, console/page-error, privacy-request, headers/cache, Entra authority, billing handoff, rate-limit, persistence, tenancy, and health checks passed.
- A release binary ran with only `PORT` supplied and created its default SQLite storage.

## Run / verify

```sh
npm ci
npm test
npm run lint
VITE_BUILD_SHA=6744f2a7ff39804100fe60f5cf3a24bd13decb4c npm run build
BUILD_SHA=6744f2a7ff39804100fe60f5cf3a24bd13decb4c cargo build --release --locked
PORT=8080 target/release/client-catalogue-request
```

Open `http://localhost:8080/?demo=1` for the isolated sample.

## Known gaps

No product defect remains. Docker was unavailable in this verifier container, so the local image build was not run; the Dockerfile contract has passing repository coverage and the deployed candidate is live. Full evidence is in `.factory/verification-8.md`.
