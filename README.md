# Client Catalogue Request

Turn repeat orders into structured quote requests.

Client Catalogue Request is for small B2B sellers who already maintain a catalogue. A seller imports CSV rows, creates hard-to-guess client links, and receives quote requests with SKUs and quantities. The request form never collects payment-card data.

Live product: <https://client-catalogue-request.sociobot.in>

## Try the sandbox

Choose **Try it with sample data**, open `/?demo=1`, or visit <https://client-catalogue-request.sociobot.in/?demo=1>. The isolated demo includes six realistic products, two POA prices, stock notes, a request basket, and a seeded seller inbox. The browser stores the sample workspace and requests under `demo:` keys. The server does not retain demo requests. Use **Reset demo** to remove that sample data and create a clean workspace.

## Seller workflow

1. Open `/manage` and sign in with Sociobot. Your Sociobot account receives its own workspace.
2. Download the CSV template. Import a file with `sku` and `name` columns. Price, description, category, and stock note are optional.
3. Create a named client link and share it with that client.
4. Review incoming requests in the workspace. Export all lines to CSV or print a request to PDF.

The free workspace supports 12 catalogue rows and one client link. Request export stays free. The ₹1,499 one-time license supports more rows and links, with up to 5,000 rows per import. Checkout and license checks use the Sociobot billing API. The app has no payment-card fields.

## Run locally

Requirements: Node.js 22+, npm, current stable Rust, and SQLite build support.

```sh
npm ci
npm run build
DATA_DIR=./data WEB_DIST=./dist PORT=8080 cargo run
```

Open <http://localhost:8080>. The server creates `DATA_DIR` and its SQLite database on first boot. Only `PORT` is needed in production; it defaults to `8080`.

For frontend work, run the API with `cargo run`, then run `npm run dev` in another terminal.

## Test and verify

```sh
npm test
npm run build
docker build --build-arg BUILD_SHA=local -t client-catalogue-request .
docker run --rm -p 8080:8080 client-catalogue-request
curl http://localhost:8080/health
```

Every public product claim and its sandbox command is listed in [`.factory/claims.json`](.factory/claims.json).

## CSV format

The header names are case-insensitive:

```csv
sku,name,description,category,price,stock_note
MUG-12,Stack mug 12 oz,Stoneware mug,Tableware,4.80,In stock
TRAY-OAK,Oak serving tray,Oil-finished oak,Service,,Made to order
```

A blank price becomes POA.

## Deployment and data

The root `Dockerfile` builds the Vite frontend and Rust server. The container runs as a non-root user and keeps SQLite under `/app/data`. Mount that path as a volume. `/health` returns the build SHA passed as `BUILD_SHA`.

Seller workspaces, catalogue rows, client links, and quote requests stay in SQLite. Session storage holds the current sign-in token. Local storage holds a pasted Sociobot license and its last verification result. There are no analytics, advertising scripts, remote fonts, or runtime CDNs.

See [privacy](https://client-catalogue-request.sociobot.in/privacy), [terms](https://client-catalogue-request.sociobot.in/terms), [demo notes](.factory/demo.md), and [visual system](.factory/design.md).

## License

MIT. See [LICENSE](LICENSE).
