-- Version two removes the global owner model. Every seller is keyed by the
-- stable Entra object id, so one visitor can never claim another seller's desk.
CREATE TABLE IF NOT EXISTS sellers (
  subject TEXT PRIMARY KEY,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS tenant_settings (
  seller_subject TEXT PRIMARY KEY REFERENCES sellers(subject) ON DELETE CASCADE,
  business_name TEXT NOT NULL DEFAULT 'My catalogue',
  price_label TEXT NOT NULL DEFAULT 'Trade price',
  tax_note TEXT NOT NULL DEFAULT 'Prices exclude tax',
  currency TEXT NOT NULL DEFAULT 'GBP'
);
CREATE TABLE IF NOT EXISTS tenant_products (
  seller_subject TEXT NOT NULL REFERENCES sellers(subject) ON DELETE CASCADE,
  id TEXT NOT NULL,
  sku TEXT NOT NULL,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  category TEXT NOT NULL DEFAULT 'Products',
  price_cents INTEGER,
  stock_note TEXT NOT NULL DEFAULT 'Ask about availability',
  active INTEGER NOT NULL DEFAULT 1,
  PRIMARY KEY (seller_subject, id),
  UNIQUE (seller_subject, sku)
);
CREATE TABLE IF NOT EXISTS tenant_links (
  token TEXT PRIMARY KEY,
  seller_subject TEXT NOT NULL REFERENCES sellers(subject) ON DELETE CASCADE,
  label TEXT NOT NULL,
  active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  revoked_at TEXT
);
CREATE TABLE IF NOT EXISTS tenant_requests (
  id TEXT PRIMARY KEY,
  seller_subject TEXT NOT NULL REFERENCES sellers(subject) ON DELETE CASCADE,
  link_token TEXT NOT NULL,
  client_name TEXT NOT NULL,
  company TEXT NOT NULL,
  email TEXT NOT NULL,
  po_number TEXT NOT NULL DEFAULT '',
  note TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'New',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
  deleted_at TEXT
);
CREATE TABLE IF NOT EXISTS tenant_request_lines (
  request_id TEXT NOT NULL REFERENCES tenant_requests(id) ON DELETE CASCADE,
  product_id TEXT NOT NULL,
  sku TEXT NOT NULL,
  name TEXT NOT NULL,
  quantity INTEGER NOT NULL,
  price_cents INTEGER,
  PRIMARY KEY (request_id, product_id)
);
-- Demo payloads are stored separately from tenant data and expire by timestamp.
CREATE TABLE IF NOT EXISTS demo_workspaces (
  id TEXT PRIMARY KEY,
  expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS demo_requests (
  workspace_id TEXT NOT NULL REFERENCES demo_workspaces(id) ON DELETE CASCADE,
  id TEXT NOT NULL,
  position INTEGER NOT NULL,
  payload TEXT NOT NULL,
  PRIMARY KEY (workspace_id, id)
);
