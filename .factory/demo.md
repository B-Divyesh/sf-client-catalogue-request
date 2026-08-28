# Demo sandbox

- URL: `/demo`
- Seller-side sample: `/demo/inbox`
- Sample data: six products across desk, packing, and shop-floor categories. Two have POA prices and three have stock caveats. The seller inbox starts with one two-line request from Juniper Corner.
- Reset: use **Reset demo** in the amber banner. It deletes the ephemeral workspace and creates a clean one.
- Storage namespace: `demo:client-catalogue-request:requests` stores only the random workspace ID. Request data stays in an isolated in-memory backend workspace for up to 24 hours. Demo routes never read or write the seller database.
- Verification: use a fresh browser context. Add an item, send a request, open **Seller sample**, then export the CSV.
