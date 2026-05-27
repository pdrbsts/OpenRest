-- Phase 1: simple single-method receipt
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    payment_method_id TEXT NOT NULL,
    amount INTEGER NOT NULL,
    created_at DATETIME NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_payments_document ON payments(document_id);
