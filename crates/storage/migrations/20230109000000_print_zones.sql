-- Dispositivos (spec 10.1 subset): impressoras físicas (file-based em Fase 2).
CREATE TABLE IF NOT EXISTS dispositivos (
    id TEXT PRIMARY KEY,
    nome TEXT NOT NULL,
    tipo TEXT NOT NULL DEFAULT 'impressora_generica',  -- impressora_generica|escpos|email|...
    modelo TEXT,
    descricao TEXT,
    output_path TEXT,                                   -- ficheiro para impressoras file-based
    ativo INTEGER NOT NULL DEFAULT 1,
    anulado_em DATETIME
);

-- Zonas de impressão (spec 10.3): destino lógico (ex: 1=D.Externos, 2=Subtotais, 10=Cozinha, 11=Bar).
CREATE TABLE IF NOT EXISTS zonas_impressao (
    id TEXT PRIMARY KEY,
    codigo INTEGER NOT NULL UNIQUE,
    designacao TEXT NOT NULL,
    secundarios INTEGER NOT NULL DEFAULT 0,             -- pedidos cruzados
    anulado_em DATETIME
);

-- Mapping zona × local → impressora (spec 10.2). origem opcional.
CREATE TABLE IF NOT EXISTS impressora_zona_local (
    id TEXT PRIMARY KEY,
    zona_impressao_id TEXT NOT NULL,
    local_id TEXT NOT NULL,
    origem_id TEXT,
    dispositivo_id TEXT NOT NULL,
    agrupamento TEXT NOT NULL DEFAULT 'normal',
    numero_copias INTEGER NOT NULL DEFAULT 1
);
CREATE INDEX IF NOT EXISTS idx_izl_lookup
    ON impressora_zona_local(zona_impressao_id, local_id);

-- Artigo aponta para zona de impressão (kitchen routing).
ALTER TABLE articles ADD COLUMN zona_impressao_id TEXT;

-- Linha do documento marca-se como pedida (impressa em cozinha/bar).
ALTER TABLE document_details ADD COLUMN pedida_em DATETIME;
