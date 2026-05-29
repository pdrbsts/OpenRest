-- Refinamento dos templates de Factura (FT) e Venda a Dinheiro (VD) — Fase 2.
-- Acrescenta, face à factura simplificada: bloco de identificação do cliente
-- (FT), coluna de preço unitário no detalhe, cabeçalho de colunas alinhado
-- (campos com `default`+`mask` partilham as larguras com o detalhe), marca
-- "Original" e bloco de pagamento.
--
-- Layout do detalhe (largura 48): qtd(4) ' ' design(22) punit(7) iva(5) total(9).

UPDATE documento_templates SET
    designacao = 'Factura',
    cabecalho = '\s7\no
\s7\ds
\s7\mo
\s7\cp \lo
\s7NIF: \nc
\s7Cap. Social: \cs
------------------------------------------------
\s7FACTURA  (Original)
\s7\nx
ATCUD: \atcud
------------------------------------------------
Exmo(a). Sr(a).
\ol
NIF: \cl
\mc
\xp \ll
\dt \ho
------------------------------------------------
<! type="field" id="h" default="Qtd" mask="####" align="right" !> <! type="field" id="h" default="Artigo" mask="######################" !><! type="field" id="h" default="P.Unit" mask="#######" align="right" !><! type="field" id="h" default="IVA" mask="#####" align="right" !><! type="field" id="h" default="Total" mask="#########" align="right" !>',
    linha_detalhe = '<! type="field" id="fb_d_qtd" mask="####" align="right" !> <! type="field" id="fb_d_design" mask="######################" !><! type="field" id="fb_d_punit" mask="#######" align="right" !><! type="field" id="fb_d_iva_perc" mask="#####" align="right" !><! type="field" id="fb_d_total_linha" mask="#########" align="right" !>',
    rodape = '------------------------------------------------
\ti
------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
Forma de pagamento: \fp
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
WHERE tipo_documento = 'fatura';

UPDATE documento_templates SET
    designacao = 'Venda a Dinheiro',
    cabecalho = '\s7\no
\s7\ds
\s7\mo
\s7NIF: \nc
------------------------------------------------
\s7VENDA A DINHEIRO
\s7\nx
ATCUD: \atcud
------------------------------------------------
NIF Cliente: \cl
\dt \ho
------------------------------------------------
<! type="field" id="h" default="Qtd" mask="####" align="right" !> <! type="field" id="h" default="Artigo" mask="######################" !><! type="field" id="h" default="P.Unit" mask="#######" align="right" !><! type="field" id="h" default="IVA" mask="#####" align="right" !><! type="field" id="h" default="Total" mask="#########" align="right" !>',
    linha_detalhe = '<! type="field" id="fb_d_qtd" mask="####" align="right" !> <! type="field" id="fb_d_design" mask="######################" !><! type="field" id="fb_d_punit" mask="#######" align="right" !><! type="field" id="fb_d_iva_perc" mask="#####" align="right" !><! type="field" id="fb_d_total_linha" mask="#########" align="right" !>',
    rodape = '------------------------------------------------
\ti
------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
Forma de pagamento: \fp
Total pago: \pg
Troco: \tr
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
WHERE tipo_documento = 'venda_dinheiro';
