# Demo sandbox

- One-click and direct URL: `/?demo=1` (the `/demo` alias opens the same sandbox)
- Seller-side sample: `/demo/inbox?demo=1`
- Sample data: six products across desk, packing, and shop-floor categories. Two have POA prices and three have stock caveats. The seller inbox starts with one two-line request from Juniper Corner.
- Reset: use **Reset demo** in the amber banner. It removes the browser's sample workspace and requests, then creates a clean sample.
- Storage namespace: `demo:client-catalogue-request:requests` stores the random workspace ID and `demo:client-catalogue-request:submitted` stores submitted sample requests. Leaving demo deletes both keys. Demo API calls are stateless and never read, write, or retain seller or demo data on the server.
- Verification: use a fresh browser context. Add an item, send a request, open **Seller sample**, then export the CSV.
