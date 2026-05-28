-- Auditoria de anulações (spec 02-glossário, 03 §10).
-- Sempre populada quando uma linha já pedida é anulada.
CREATE TABLE IF NOT EXISTS anulacoes (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    document_detail_id TEXT NOT NULL,
    article_id TEXT NOT NULL,
    qty INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    total INTEGER NOT NULL,
    com_desperdicio INTEGER NOT NULL DEFAULT 0,
    motivo TEXT,
    empregado_id TEXT,
    anulada_em DATETIME NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_anulacoes_document ON anulacoes(document_id);
CREATE INDEX IF NOT EXISTS idx_anulacoes_anulada_em ON anulacoes(anulada_em);

-- Auditoria opcional de cancelamentos (spec 03 §11): populada apenas quando
-- a flag de config `registar_cancelamentos` está activa. As linhas canceladas
-- são apagadas fisicamente de `document_details` — este registo é a única
-- pista para histórico/auditoria interna.
CREATE TABLE IF NOT EXISTS cancelamentos (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    article_id TEXT NOT NULL,
    qty INTEGER NOT NULL,
    unit_price INTEGER NOT NULL,
    total INTEGER NOT NULL,
    motivo TEXT,
    empregado_id TEXT,
    cancelada_em DATETIME NOT NULL
);
CREATE INDEX IF NOT EXISTS idx_cancelamentos_document ON cancelamentos(document_id);
