-- Anulação inline na linha do documento (spec 5.2): mantém histórico de linha,
-- subtrai-se ao total apenas via cálculo aplicacional.
ALTER TABLE document_details ADD COLUMN anulada INTEGER NOT NULL DEFAULT 0;
ALTER TABLE document_details ADD COLUMN anulada_com_desperdicio INTEGER NOT NULL DEFAULT 0;
ALTER TABLE document_details ADD COLUMN anulada_em DATETIME;
ALTER TABLE document_details ADD COLUMN anulada_por TEXT;
ALTER TABLE document_details ADD COLUMN anulada_motivo TEXT;
