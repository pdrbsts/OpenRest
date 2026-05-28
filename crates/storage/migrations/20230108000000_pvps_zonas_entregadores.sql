-- tipos_preco (spec 3.1): 5 PVPs nomeados
CREATE TABLE IF NOT EXISTS tipos_preco (
    id TEXT PRIMARY KEY,
    codigo INTEGER NOT NULL UNIQUE,
    designacao TEXT NOT NULL
);

-- Artigo ganha pvp2..pvp5; price actual passa a chamar-se pvp1 (mantemos coluna).
-- O default 0 funciona como "preço não configurado": calling code cai no pvp1.
ALTER TABLE articles RENAME COLUMN price TO pvp1;
-- pvp2..pvp5 são nullable: NULL = "não configurado, usa pvp1"; 0 = "grátis".
ALTER TABLE articles ADD COLUMN pvp2 INTEGER;
ALTER TABLE articles ADD COLUMN pvp3 INTEGER;
ALTER TABLE articles ADD COLUMN pvp4 INTEGER;
ALTER TABLE articles ADD COLUMN pvp5 INTEGER;
ALTER TABLE articles ADD COLUMN tipo_artigo TEXT NOT NULL DEFAULT 'normal';

-- Zonas (spec 2.5) — agora com taxa de entrega para delivery.
CREATE TABLE IF NOT EXISTS zonas (
    id TEXT PRIMARY KEY,
    codigo INTEGER,
    designacao TEXT NOT NULL,
    taxa_entrega INTEGER NOT NULL DEFAULT 0,
    rede_remota_associada_id TEXT,
    anulado_em DATETIME
);
CREATE INDEX IF NOT EXISTS idx_zonas_codigo ON zonas(codigo);

-- Entregadores (entidade nova — separados de empregados).
CREATE TABLE IF NOT EXISTS entregadores (
    id TEXT PRIMARY KEY,
    nome TEXT NOT NULL,
    telefone TEXT,
    externo INTEGER NOT NULL DEFAULT 1,
    ativo INTEGER NOT NULL DEFAULT 1,
    anulado_em DATETIME
);

-- Delivery: snapshots da zona + taxa, e entregador_id passa a apontar para entregadores.
ALTER TABLE pedidos_delivery ADD COLUMN zona_id TEXT;
ALTER TABLE pedidos_delivery ADD COLUMN taxa_entrega_cents INTEGER NOT NULL DEFAULT 0;
