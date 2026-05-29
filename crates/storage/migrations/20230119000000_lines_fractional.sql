-- Fase 2 — Divisão fraccionária e modo Encaixar (linhas de compensação).
--
-- `qty_milli` representa a quantidade em milli-unidades (1.000 = 1 unidade).
-- Permite shares fraccionários (e.g., 500 = 0,5 unidade) sem perder a
-- semântica inteira já existente. Linhas antigas conservam o seu `qty` e
-- recebem `qty_milli = qty * 1000` na backfill.
--
-- `descricao` permite sobrepor o nome da linha no recibo (e.g., "Compensação
-- divisão Café" quando uma linha foi gerada pelo split em modo Encaixar).
--
-- Valores negativos são permitidos em `qty_milli` e em `total` para suportar
-- linhas de compensação (e.g., -75 cêntimos contra o lado positivo de outra
-- conta-filho). O fecho fiscal trata-as como contribuições normais ao bucket
-- de IVA do artigo referenciado.
ALTER TABLE document_details ADD COLUMN qty_milli BIGINT;
ALTER TABLE document_details ADD COLUMN descricao TEXT;
UPDATE document_details SET qty_milli = CAST(qty AS BIGINT) * 1000 WHERE qty_milli IS NULL;
