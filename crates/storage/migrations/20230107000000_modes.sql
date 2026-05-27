-- Cliente (spec 4.2 — subset Fase 2)
CREATE TABLE IF NOT EXISTS clientes (
    id TEXT PRIMARY KEY,
    codigo INTEGER,
    nome TEXT NOT NULL,
    nif TEXT,
    telefone TEXT,
    morada TEXT,
    cod_postal TEXT,
    localidade TEXT,
    email TEXT,
    observacoes TEXT,
    numero_cartao TEXT,
    limite_credito INTEGER NOT NULL DEFAULT 0,
    zona_id TEXT,
    anulado_em DATETIME
);
CREATE INDEX IF NOT EXISTS idx_clientes_telefone ON clientes(telefone);
CREATE INDEX IF NOT EXISTS idx_clientes_codigo ON clientes(codigo);
CREATE INDEX IF NOT EXISTS idx_clientes_nome ON clientes(nome);

-- Document ganha contexto de cliente, observações e modo
ALTER TABLE documents ADD COLUMN customer_id TEXT;
ALTER TABLE documents ADD COLUMN local_id TEXT;
ALTER TABLE documents ADD COLUMN observacoes_pedido TEXT;
ALTER TABLE documents ADD COLUMN observacoes_factura TEXT;
ALTER TABLE documents ADD COLUMN observacoes_cliente TEXT;
ALTER TABLE documents ADD COLUMN observacoes_morada TEXT;
ALTER TABLE documents ADD COLUMN delivery_morada TEXT;
ALTER TABLE documents ADD COLUMN delivery_telefone TEXT;

-- Pedido delivery (spec 12.2)
CREATE TABLE IF NOT EXISTS pedidos_delivery (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL UNIQUE,
    cliente_id TEXT,
    morada_snapshot TEXT,
    telefone_snapshot TEXT,
    recebido_em DATETIME NOT NULL,
    recebido_via TEXT NOT NULL DEFAULT 'balcao',
    entregador_id TEXT,
    pronto_em DATETIME,
    despachado_em DATETIME,
    entregue_em DATETIME,
    estado TEXT NOT NULL DEFAULT 'recebido'
);
CREATE INDEX IF NOT EXISTS idx_pedidos_delivery_estado ON pedidos_delivery(estado);

-- Empregado ganha campos de consumo próprio (spec 4.1 subset)
ALTER TABLE employees ADD COLUMN perc_consumo INTEGER NOT NULL DEFAULT 10000;  -- basis points (100% por defeito)
ALTER TABLE employees ADD COLUMN base_consumo INTEGER NOT NULL DEFAULT 0;       -- limite mensal cêntimos
