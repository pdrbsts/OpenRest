-- Famílias passam a ser uma árvore (uma família pode ter pai).
ALTER TABLE families ADD COLUMN parent_id TEXT;
CREATE INDEX IF NOT EXISTS idx_families_parent ON families(parent_id);

-- Artigo ganha taxa de IVA (basis points: 1300 = 13%).
ALTER TABLE articles ADD COLUMN vat_rate INTEGER NOT NULL DEFAULT 1300;

-- Séries fiscais
CREATE TABLE IF NOT EXISTS document_series (
    id TEXT PRIMARY KEY,
    document_type TEXT NOT NULL,
    prefix TEXT NOT NULL,
    year INTEGER NOT NULL,
    next_number INTEGER NOT NULL DEFAULT 1,
    atcud_validation TEXT NOT NULL,
    is_active INTEGER NOT NULL DEFAULT 1,
    UNIQUE(document_type, prefix, year)
);

-- Cadeia fiscal nos documentos
ALTER TABLE documents ADD COLUMN series_id TEXT;
ALTER TABLE documents ADD COLUMN document_type TEXT;
ALTER TABLE documents ADD COLUMN document_number INTEGER;
ALTER TABLE documents ADD COLUMN atcud TEXT;
ALTER TABLE documents ADD COLUMN hash TEXT;
ALTER TABLE documents ADD COLUMN hash_short TEXT;
ALTER TABLE documents ADD COLUMN previous_hash TEXT;
ALTER TABLE documents ADD COLUMN issued_at DATETIME;
ALTER TABLE documents ADD COLUMN qr_payload TEXT;
