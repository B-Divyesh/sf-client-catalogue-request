# Client Catalogue Request

Turn repeat orders into structured quote requests.

Client Catalogue Request is for small B2B sellers who already maintain a product list. A seller imports CSV rows, creates hard-to-guess client links, and receives quote requests with SKUs and quantities. It does not take payment or manage inventory.

Live product: <https://client-catalogue-request.sociobot.in>

## Try the sandbox

Open `/demo` or <https://client-catalogue-request.sociobot.in/demo>. It includes six realistic products, POA pricing, stock caveats, a request basket, and a sample seller inbox. The browser stores the isolated sample workspace and sample requests. The server does not retain demo requests. Use **Reset demo** to remove the sample data from this browser.

## Seller workflow

1. Open `/manage` and sign in with Sociobot. Your Sociobot account receives its own workspace.
2. Download the CSV template. Import a file with `sku` and `name` columns. Price, description, category, and stock note are optional.
3. Create a named client link and share it with that client.
4. Review incoming requests in the workspace. Export all lines to CSV or print a request to PDF.

The free workspace supports 12 catalogue rows and one client link. Request export stays free. The ₹1,499 one-time license supports more rows and links, with up to 5,000 rows per import. Checkout and license checks use Sociobot; no payment provider runs in this app.

## Run locally

Requirements: Node.js 22+, npm, current stable Rust, and SQLite build support.

```sh
npm ci
npm run build
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run
```

Open <http://localhost:8080>. The server creates `DATA_DIR` and its SQLite database on first boot. Only `PORT` is needed in production; it defaults to `8080`.

For frontend work, run the API with `cargo run`, then run `npm run dev` in another terminal. Vite proxies `/api` and `/health` to port 8080.

## Test and verify

```sh
npm test
npm run build
docker build --build-arg BUILD_SHA=local -t client-catalogue-request .
docker run --rm -p 8080:8080 client-catalogue-request
curl http://localhost:8080/health
```

`npm test` runs unit tests, the complete Rust API flow, claim tests in Chromium, a 390 px mobile pass, and automated accessibility checks. Every public claim and its sandbox evidence is listed in [`.factory/claims.json`](.factory/claims.json).

## CSV format

The header names are case-insensitive:

```csv
sku,name,description,category,price,stock_note
MUG-12,Stack mug 12 oz,Stoneware mug,Tableware,4.80,In stock
TRAY-OAK,Oak serving tray,Oil-finished oak,Service,,Made to order
```

A blank price becomes POA. Prices use major currency units in CSV and integer minor units in the API.

## Deployment and data

The root `Dockerfile` builds the Vite frontend and Rust server. It runs as a non-root user and persists SQLite under `/app/data`. Mount that path as a volume. `/health` returns the build SHA passed as `BUILD_SHA`.

Seller tenant data, catalogue rows, client tokens, and quote requests stay in SQLite. Browser storage contains an Entra sign-in token and any pasted Sociobot license. There are no analytics, advertising scripts, remote fonts, or runtime CDNs.

See [privacy](https://client-catalogue-request.sociobot.in/privacy), [terms](https://client-catalogue-request.sociobot.in/terms), [demo notes](.factory/demo.md), and [visual system](.factory/design.md).

## License

MIT. See [LICENSE](LICENSE).
