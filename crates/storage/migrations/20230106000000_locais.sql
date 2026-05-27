-- Local: conjunto lógico de mesas com configuração própria (spec 4.9).
CREATE TABLE IF NOT EXISTS locais (
    id TEXT PRIMARY KEY,
    designacao TEXT NOT NULL,
    mesas_definicao TEXT,                       -- range_set "1:50"
    tipo TEXT NOT NULL DEFAULT 'normal',        -- normal|take_away|take_away_seguro|pub|delivery|consumo_proprio|restauracao_colectiva
    tipo_preco_id TEXT,
    metodo_pagamento_default_id TEXT,
    taxa_servico_artigo_id TEXT,
    limite_consumo INTEGER NOT NULL DEFAULT 0,
    imprime_conta_acima_de INTEGER NOT NULL DEFAULT 0,
    nome_generico_mesa TEXT NOT NULL DEFAULT 'Mesa {nm}',
    imprime_subtotal_em TEXT NOT NULL DEFAULT '{}',
    imprime_conta_em TEXT NOT NULL DEFAULT '{}',
    fecha_mesa_ao_pedir TEXT NOT NULL DEFAULT 'nunca',
    usa_iva_venda_directa INTEGER NOT NULL DEFAULT 0,
    iva_excluido_dos_precos INTEGER NOT NULL DEFAULT 0,
    cor_empregado_na_lista INTEGER NOT NULL DEFAULT 0,
    impressora_directa_pedidos_id TEXT,
    pede_nova_mesa_depois_de_fechar INTEGER NOT NULL DEFAULT 0,
    pede_nova_mesa_apos_pedido INTEGER NOT NULL DEFAULT 0,
    indica_pessoas_obrigatorio INTEGER NOT NULL DEFAULT 0,
    indica_pessoas_apenas_abertura INTEGER NOT NULL DEFAULT 0,
    permite_zero_pessoas INTEGER NOT NULL DEFAULT 1,
    aloca_mesas_dinamicamente INTEGER NOT NULL DEFAULT 0,
    alocacao_circular INTEGER NOT NULL DEFAULT 0,
    inclui_desconto_nos_precos INTEGER NOT NULL DEFAULT 0,
    artigos_automatico_sem_preco INTEGER NOT NULL DEFAULT 0,
    carregamento_rapido_mesas INTEGER NOT NULL DEFAULT 0,
    so_imprime_pedidos_com_complementos INTEGER NOT NULL DEFAULT 0,
    lista_grande_pedidos INTEGER NOT NULL DEFAULT 0,
    mesas_uma_vez_por_dia INTEGER NOT NULL DEFAULT 0,
    facturacao_externa INTEGER NOT NULL DEFAULT 0,
    nao_agrupa_detalhes_na_conta INTEGER NOT NULL DEFAULT 0,
    permite_encaixe_promocoes INTEGER NOT NULL DEFAULT 0,
    separa_artigos_antes_encaixe INTEGER NOT NULL DEFAULT 0,
    permite_mesas_abertas_fim_do_dia INTEGER NOT NULL DEFAULT 1,
    pode_identificar_cliente_no_pedido INTEGER NOT NULL DEFAULT 0,
    obriga_indicar_valor_pago INTEGER NOT NULL DEFAULT 0,
    usa_desenho_mesas INTEGER NOT NULL DEFAULT 0,
    imagem TEXT,                                -- base64 data url da imagem de fundo
    largura INTEGER,                            -- dimensão da área de desenho (px)
    altura INTEGER,
    anulado_em DATETIME
);

-- Mesa: configuração estática e visual (spec 4.10).
ALTER TABLE tables ADD COLUMN local_id TEXT;
ALTER TABLE tables ADD COLUMN nomeobjecto TEXT;
ALTER TABLE tables ADD COLUMN posx INTEGER;
ALTER TABLE tables ADD COLUMN posy INTEGER;
ALTER TABLE tables ADD COLUMN imagem TEXT;
ALTER TABLE tables ADD COLUMN fntname TEXT;
ALTER TABLE tables ADD COLUMN fntsize INTEGER;
ALTER TABLE tables ADD COLUMN fntcolor TEXT;
ALTER TABLE tables ADD COLUMN fontx INTEGER;
ALTER TABLE tables ADD COLUMN fonty INTEGER;
ALTER TABLE tables ADD COLUMN fontstyle TEXT;
ALTER TABLE tables ADD COLUMN estadox INTEGER;
ALTER TABLE tables ADD COLUMN estadoy INTEGER;
ALTER TABLE tables ADD COLUMN reservax INTEGER;
ALTER TABLE tables ADD COLUMN reservay INTEGER;
ALTER TABLE tables ADD COLUMN altura INTEGER;
ALTER TABLE tables ADD COLUMN largura INTEGER;
ALTER TABLE tables ADD COLUMN criada_em DATETIME;
CREATE INDEX IF NOT EXISTS idx_tables_local ON tables(local_id);

-- O estado vivo da mesa passa a viver em mesa_estado (spec 6.1).
ALTER TABLE tables DROP COLUMN is_open;

CREATE TABLE IF NOT EXISTS mesa_estado (
    mesa_id TEXT PRIMARY KEY,
    estado TEXT NOT NULL DEFAULT 'livre',
    bloqueada_por_posto_id TEXT,
    bloqueada_motivo TEXT,
    cliente_associado_id TEXT,
    numero_pessoas INTEGER,
    empregado_actual_id TEXT,
    aberta_em DATETIME,
    subtotal_actual INTEGER NOT NULL DEFAULT 0,
    reservada_ate DATETIME,
    reserva_pessoas INTEGER,
    reserva_cliente_id TEXT,
    reserva_observacoes TEXT
);
