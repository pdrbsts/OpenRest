-- Spec §4.2 + §2.8: cliente passa a ter país (ISO2) e marca de esquecimento RGPD.
-- O esquecimento é distinto da anulação: anonimiza os dados pessoais mas
-- mantém o ID/código para integridade referencial em documentos fiscais
-- históricos (SAF-T não pode perder a ligação).
ALTER TABLE clientes ADD COLUMN pais TEXT NOT NULL DEFAULT 'PT';
ALTER TABLE clientes ADD COLUMN esquecido_em DATETIME;
CREATE INDEX IF NOT EXISTS idx_clientes_nif ON clientes(nif);
