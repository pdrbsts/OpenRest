-- Fase 2 — Pagamento Parcial e Divisão de Conta.
-- Quando uma mesa é dividida em N facturas, ou apenas algumas linhas são
-- pagas em separado, criamos documentos-filho. Cada filho mantém o link ao
-- pai operacional (que nunca recebe número fiscal). A cadeia fiscal (série,
-- ATCUD, hash) corre nos filhos.
ALTER TABLE documents ADD COLUMN parent_document_id TEXT NULL REFERENCES documents(id);
CREATE INDEX IF NOT EXISTS idx_documents_parent ON documents(parent_document_id);
