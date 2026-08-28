# Visual thesis: the illuminated order desk

Client Catalogue Request uses a **luminous glass data landscape**. The product sits between an old PDF price list and a heavy commerce system, so its world resembles an order desk at dusk: quiet navy space, translucent inventory slips, sharp mint selection light, and warm amber stock cautions. It should feel private and precise, not like a public shop.

## Palette

- `--ink-950 #07131c`: night desk background; the default canvas.
- `--ink-900 #0b1d29`: raised app field.
- `--glass #102c3a`: translucent catalogue panels.
- `--glass-bright #173f50`: selected and hover planes.
- `--paper #f4fbf8`: primary text on dark fields.
- `--mist #b9cbc9`: secondary text; tested above 4.5:1 on the canvas.
- `--mint #71f5c6`: primary action and basket signal.
- `--mint-ink #052219`: text on mint.
- `--amber #ffc66d`: stock and POA cautions.
- `--danger #ff8b8b`: validation and failed delivery.
- Light legal pages invert the relationship with `#eef8f4` paper, `#09222c` ink, and deep teal actions.

Color never carries state alone. Stock and request states always include words or symbols.

## Type and spacing

Display headings use **Sora**, self-hosted as a compact WOFF2 subset. Body and data use the system UI stack, with tabular figures for quantities and money. This pairing keeps the catalogue human while making request rows scan like an order sheet. The scale is 14, 16, 20, 28, 44, and 64 px. Body text is never below 16 px.

Spacing follows an 8 px unit with 4 px for tight label relationships. Section gaps use 64–112 px. Catalogue rows form a staggered landscape instead of a generic equal-card grid. Corners are clipped with asymmetric 10/26 px radii, like overlapping acetate dividers.

## Composition and interaction grammar

The landing page is split like a sales desk: the plain job statement occupies the left rail while a luminous request sheet rises on the right. Thin ruled lines and small index labels connect sections. The product UI keeps filters in a horizontal glass shelf, product records in a dense, responsive landscape, and the basket as a bright-edged working layer.

Primary buttons fill mint. Secondary actions use a clear glass edge. Links remain underlined. Focus uses a 3 px amber outline and 3 px offset. Every target is at least 44 px.

The signature motion is a **sheet lift**: selected lines rise 4 px and a mint edge travels into the basket over 220 ms. Route changes cross-fade over 180 ms. There is no looping animation. With `prefers-reduced-motion: reduce`, transforms and smooth scrolling are removed and state changes are instant.

## Original asset plan

One generated hero illustration shows a dark glass order desk with floating catalogue slips flowing into one structured request sheet. It explains the product's job without showing fake UI. No people, brands, text, logos, coins, carts, or payment imagery.

Prompt sheet: *Editorial still life of a private B2B order desk at night, layered translucent glass catalogue sheets with abstract product tiles and small barcode-like rules flowing into one neatly aligned request document, luminous sea-mint edges, restrained amber status pin, deep ink navy room, fine frosted texture, oblique 35 mm lens, crisp controlled studio light, generous negative space, premium technical realism, no text, no letters, no watermark, no logos, no people, no shopping cart, no credit cards, no gradients, no neon cyberpunk clutter.*

Generated with the Param Factory Azure image deployment (`factory-image`) on 2026-08-28. The selected source and prompt sidecar live in `assets/src/`. The shipped WebP and social preview are derived from this original and are licensed as part of this MIT project. Hand-authored interface icons are inline SVG.

## Performance and responsive policy

The hero has an explicit aspect ratio and responsive WebP sizes; the mobile source stays below 300 KB. It is the only high-priority image. At 390 px, the visual becomes a shallow landscape below the action, the facts stack, filters scroll only if needed, catalogue rows use one column, and the basket opens as a full-screen dialog. At wide sizes, the basket is a side sheet. The UI ships without a component framework so first-load JavaScript stays below 200 KB gzip.
