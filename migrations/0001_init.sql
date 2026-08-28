CREATE TABLE IF NOT EXISTS owner (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  password_hash TEXT NOT NULL,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS sessions (
  token TEXT PRIMARY KEY,
  expires_at TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS settings (
  id INTEGER PRIMARY KEY CHECK (id = 1),
  business_name TEXT NOT NULL DEFAULT 'My catalogue',
  price_label TEXT NOT NULL DEFAULT 'Trade price',
  tax_note TEXT NOT NULL DEFAULT 'Prices exclude tax',
  currency TEXT NOT NULL DEFAULT 'GBP'
);
INSERT OR IGNORE INTO settings(id) VALUES (1);
CREATE TABLE IF NOT EXISTS products (
  id TEXT PRIMARY KEY,
  sku TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  description TEXT NOT NULL DEFAULT '',
  category TEXT NOT NULL DEFAULT 'Products',
  price_cents INTEGER,
  stock_note TEXT NOT NULL DEFAULT 'Ask about availability',
  active INTEGER NOT NULL DEFAULT 1
);
CREATE TABLE IF NOT EXISTS client_links (
  token TEXT PRIMARY KEY,
  label TEXT NOT NULL,
  active INTEGER NOT NULL DEFAULT 1,
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS quote_requests (
  id TEXT PRIMARY KEY,
  link_token TEXT NOT NULL,
  client_name TEXT NOT NULL,
  company TEXT NOT NULL,
  email TEXT NOT NULL,
  po_number TEXT NOT NULL DEFAULT '',
  note TEXT NOT NULL DEFAULT '',
  status TEXT NOT NULL DEFAULT 'New',
  created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS request_lines (
  request_id TEXT NOT NULL,
  product_id TEXT NOT NULL,
  sku TEXT NOT NULL,
  name TEXT NOT NULL,
  quantity INTEGER NOT NULL,
  price_cents INTEGER,
  PRIMARY KEY(request_id, product_id)
);
