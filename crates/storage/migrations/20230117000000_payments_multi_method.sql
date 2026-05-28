-- Fase 2 — Múltiplos métodos de pagamento por documento.
-- A tabela `payments` já modela o conceito de "rodapé de pagamento": uma linha
-- por método aplicado a um documento. Esta migration acrescenta:
--   * `descricao`: texto livre da janela Avançada (ex.: "Visa **1234").
--   * `documents.troco_cents`: troco em cêntimos quando soma(payments) > total.
ALTER TABLE payments ADD COLUMN descricao TEXT;
ALTER TABLE documents ADD COLUMN troco_cents INTEGER NOT NULL DEFAULT 0;
