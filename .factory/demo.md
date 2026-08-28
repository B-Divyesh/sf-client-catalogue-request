# Demo sandbox

- URL: `/demo`
- Seller-side sample: `/demo/inbox`
- Sample data: six products across desk, packing, and shop-floor categories. Two have POA prices and three have stock caveats. The seller inbox starts with one two-line request from Juniper Corner.
- Reset: use **Reset demo** in the amber banner. It deletes the demo request key and reloads the sample catalogue.
- Storage namespace: `demo:client-catalogue-request:requests`. Demo mode never reads the real session key and never calls request or catalogue API routes.
- Verification: use a fresh browser context. Add an item, send a request, open **Seller sample**, then export the CSV.
