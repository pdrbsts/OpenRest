-- Conceito de Dia operacional (spec §57 "Data Lógica de Caixa", §398
-- `data_documento`). Distinto de `created_at`, que é o instante do relógio:
-- `data_dia` é a data do "Dia de facturação", podendo recuar para o dia
-- civil anterior quando o pedido entra antes da hora de mudança de dia.
-- Exemplo: café com cutoff 02:00 → pedido às 00:30 pertence ao dia anterior.
ALTER TABLE documents ADD COLUMN data_dia DATE;
CREATE INDEX IF NOT EXISTS idx_documents_data_dia ON documents(data_dia);
