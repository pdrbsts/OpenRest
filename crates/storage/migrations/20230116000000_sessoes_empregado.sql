-- Sessão de Empregado (spec §7.4). MVP sem Caixa/Turno: o vínculo a
-- `caixa_dia_id`/`turno_id` fica reservado para quando esses módulos chegarem.
-- Por enquanto a sessão liga-se directamente à `data_dia` operacional.
CREATE TABLE IF NOT EXISTS sessoes_empregado (
    id TEXT PRIMARY KEY,
    empregado_id TEXT NOT NULL,
    data_dia DATE NOT NULL,
    com_bolsa INTEGER NOT NULL DEFAULT 0,
    fundo_bolsa INTEGER NOT NULL DEFAULT 0,
    observacao_abertura TEXT,
    observacao_fecho TEXT,
    aberta_em DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    aberta_por TEXT,
    fechada_em DATETIME,
    fechada_por TEXT
);

-- Spec §30: um empregado só pode ter UMA sessão aberta por loja.
-- Aplicado via índice único parcial sobre as sessões abertas.
CREATE UNIQUE INDEX IF NOT EXISTS uniq_sessao_aberta_por_empregado
    ON sessoes_empregado(empregado_id) WHERE fechada_em IS NULL;

CREATE INDEX IF NOT EXISTS idx_sessoes_data_dia ON sessoes_empregado(data_dia);

-- Spec §402 (documents.sessao_id): trace fiscal do documento à sessão activa.
ALTER TABLE documents ADD COLUMN sessao_id TEXT;
CREATE INDEX IF NOT EXISTS idx_documents_sessao_id ON documents(sessao_id);
