-- Documentos configuráveis (spec 08-appendices/02-printer-flags.md): cabeçalho,
-- linha de detalhe e rodapé por tipo de documento, com flags (\xx) e a
-- construção XML-like <! type="..." ... !>. Renderizados no momento da impressão
-- pelo motor `devices::template`.
CREATE TABLE IF NOT EXISTS documento_templates (
    id TEXT PRIMARY KEY,
    tipo_documento TEXT NOT NULL UNIQUE,   -- fatura_simplificada|fatura|venda_dinheiro|consulta_mesa|pedido
    designacao TEXT NOT NULL,
    cabecalho TEXT NOT NULL DEFAULT '',
    linha_detalhe TEXT NOT NULL DEFAULT '',
    rodape TEXT NOT NULL DEFAULT '',
    nao_imprime_detalhes INTEGER NOT NULL DEFAULT 0,
    largura INTEGER NOT NULL DEFAULT 48,
    anulado_em DATETIME
);

-- Default: Factura Simplificada. Reproduz o recibo legal montado em código na
-- Fase 1, agora editável. Largura 48: qtd(5) espaço(1) artigo(30) iva(4) total(8).
INSERT OR IGNORE INTO documento_templates
    (id, tipo_documento, designacao, cabecalho, linha_detalhe, rodape)
VALUES (
    'd0000000-0000-0000-0000-000000000001',
    'fatura_simplificada',
    'Factura Simplificada',
    '\s7\no
\s7\ds
\s7\mo
\s7\cp \lo
\s7NIF: \nc
------------------------------------------------
\s7Factura Simplificada
\s7\nx
------------------------------------------------
\om
\dt \ho
Adquirente: Consumidor Final
------------------------------------------------
Qtd Artigo                            IVA   Total',
    '<! type="field" id="fb_d_qtd" mask="#####" align="right" !> <! type="field" id="fb_d_design" mask="##############################" !><! type="field" id="fb_d_iva_perc" mask="####" align="right" !><! type="field" id="fb_d_total_linha" mask="########" align="right" !>',
    '------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
\ti
------------------------------------------------
Pago: \pg   Troco: \tr
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
);

-- Default: Factura (com identificação de cliente).
INSERT OR IGNORE INTO documento_templates
    (id, tipo_documento, designacao, cabecalho, linha_detalhe, rodape)
VALUES (
    'd0000000-0000-0000-0000-000000000002',
    'fatura',
    'Factura',
    '\s7\no
\s7\ds
\s7\mo
\s7\cp \lo
\s7NIF: \nc
------------------------------------------------
\s7FACTURA
\s7\nx
------------------------------------------------
Cliente: \ol
NIF: \cl
\dt \ho
------------------------------------------------
Qtd Artigo                            IVA   Total',
    '<! type="field" id="fb_d_qtd" mask="#####" align="right" !> <! type="field" id="fb_d_design" mask="##############################" !><! type="field" id="fb_d_iva_perc" mask="####" align="right" !><! type="field" id="fb_d_total_linha" mask="########" align="right" !>',
    '------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
\ti
------------------------------------------------
Pago: \pg   Troco: \tr
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
);

-- Default: Venda a Dinheiro (VD).
INSERT OR IGNORE INTO documento_templates
    (id, tipo_documento, designacao, cabecalho, linha_detalhe, rodape)
VALUES (
    'd0000000-0000-0000-0000-000000000003',
    'venda_dinheiro',
    'Venda a Dinheiro',
    '\s7\no
\s7\ds
\s7\mo
\s7NIF: \nc
------------------------------------------------
\s7VENDA A DINHEIRO
\s7\nx
------------------------------------------------
\dt \ho
------------------------------------------------
Qtd Artigo                            IVA   Total',
    '<! type="field" id="fb_d_qtd" mask="#####" align="right" !> <! type="field" id="fb_d_design" mask="##############################" !><! type="field" id="fb_d_iva_perc" mask="####" align="right" !><! type="field" id="fb_d_total_linha" mask="########" align="right" !>',
    '------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
\ti
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
);

-- Default: Consulta de Mesa (documento interno, sem dados fiscais).
INSERT OR IGNORE INTO documento_templates
    (id, tipo_documento, designacao, cabecalho, linha_detalhe, rodape)
VALUES (
    'd0000000-0000-0000-0000-000000000004',
    'consulta_mesa',
    'Consulta de Mesa',
    '\s7\no
------------------------------------------------
\s7CONSULTA
\om   \np pessoas
Empregado: \oe
\dt \ho
------------------------------------------------
Qtd Artigo                                  Total',
    '<! type="field" id="fb_d_qtd" mask="#####" align="right" !> <! type="field" id="fb_d_design" mask="##################################" !><! type="field" id="fb_d_total_linha" mask="########" align="right" !>',
    '------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
\s7** Documento não fiscal **'
);

-- Default: Pedido (talão de cozinha/bar — sem totais nem IVA).
INSERT OR IGNORE INTO documento_templates
    (id, tipo_documento, designacao, cabecalho, linha_detalhe, rodape)
VALUES (
    'd0000000-0000-0000-0000-000000000005',
    'pedido',
    'Pedido',
    '\s7\s2PEDIDO\s3
------------------------------------------------
\om   \oe
\dt \ho
------------------------------------------------',
    '<! type="field" id="fb_d_qtd" mask="#####" align="right" !> <! type="field" id="fb_d_design" mask="#########################################" !>',
    '------------------------------------------------'
);
