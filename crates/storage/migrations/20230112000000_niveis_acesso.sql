-- Níveis de acesso (spec 1.1). Phase 2 MVP modela as permissões granulares
-- relevantes para cancelar/anular; outras permissões juntam-se quando os
-- módulos respectivos forem implementados.
CREATE TABLE IF NOT EXISTS niveis_acesso (
    id TEXT PRIMARY KEY,
    codigo INTEGER NOT NULL UNIQUE,
    designacao TEXT NOT NULL,
    cancela_pedidos INTEGER NOT NULL DEFAULT 0,
    anula_pedidos INTEGER NOT NULL DEFAULT 0,
    anula_pedidos_com_conta_impressa INTEGER NOT NULL DEFAULT 0,
    anulado_em DATETIME
);

ALTER TABLE employees ADD COLUMN nivel_acesso_id TEXT;

-- Marca quando o sub-total/consulta de mesa foi impresso pela primeira vez —
-- afecta o requisito de permissão extra para anular após esse momento.
ALTER TABLE documents ADD COLUMN subtotal_impresso_em DATETIME;
