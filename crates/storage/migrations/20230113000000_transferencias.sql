-- Spec §9 Transferência: novas permissões e tabela de auditoria.
ALTER TABLE niveis_acesso ADD COLUMN transfere_pedidos INTEGER NOT NULL DEFAULT 0;
ALTER TABLE niveis_acesso ADD COLUMN transfere_pedidos_com_conta_impressa INTEGER NOT NULL DEFAULT 0;

-- Audit trail das transferências de linhas entre documentos.
CREATE TABLE IF NOT EXISTS transferencias (
    id TEXT PRIMARY KEY,
    from_document_id TEXT NOT NULL,
    to_document_id TEXT NOT NULL,
    line_id TEXT NOT NULL,
    article_id TEXT NOT NULL,
    qty INTEGER NOT NULL,
    employee_id TEXT,
    transferida_em DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
CREATE INDEX IF NOT EXISTS idx_transferencias_from ON transferencias(from_document_id);
CREATE INDEX IF NOT EXISTS idx_transferencias_to ON transferencias(to_document_id);

-- Promover níveis existentes: Admin e Chefe transferem; Admin transfere mesmo após sub-total.
UPDATE niveis_acesso SET transfere_pedidos = 1, transfere_pedidos_com_conta_impressa = 1 WHERE codigo = 1;
UPDATE niveis_acesso SET transfere_pedidos = 1 WHERE codigo = 2;
