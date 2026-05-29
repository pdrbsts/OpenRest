-- Correcção fiscal (PT): os documentos de venda são apenas três — fatura
-- simplificada (FS), fatura-recibo (FR) e fatura a crédito (FT). "Venda a
-- Dinheiro" é um conceito legado (WinREST) sem correspondência fiscal em
-- Portugal; o equivalente moderno do pagamento imediato é a fatura-recibo.
--
-- Esta migration converte o template `venda_dinheiro` em `fatura_recibo`
-- (nominativo, com bloco de recebimento) e ajusta a `fatura` para a natureza
-- "a crédito" (sem bloco de pagamento recebido; liquidação por recibo).

UPDATE documento_templates SET
    tipo_documento = 'fatura_recibo',
    designacao = 'Factura-Recibo',
    cabecalho = '\s7\no
\s7\ds
\s7\mo
\s7\cp \lo
\s7NIF: \nc
------------------------------------------------
\s7FACTURA-RECIBO  (Original)
\s7\nx
ATCUD: \atcud
------------------------------------------------
Exmo(a). Sr(a).
\ol
NIF: \cl
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
\s7Recebi(emos) a quantia acima indicada.
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
WHERE tipo_documento = 'venda_dinheiro';

UPDATE documento_templates SET
    rodape = '------------------------------------------------
\ti
------------------------------------------------
TOTAL<! type="flag" id="vt" mask="###########################################" align="right" !>
------------------------------------------------
\s7Documento emitido a crédito.
\s7Liquidação por recibo (RC).
------------------------------------------------
ATCUD: \atcud
Hash: \hash
\s7Processado por programa certificado
------------------------------------------------
\qr'
WHERE tipo_documento = 'fatura';
