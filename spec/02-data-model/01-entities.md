# OpenRest — Modelo de Dados: Entidades

> Catálogo exaustivo das entidades do domínio. Não é ainda um esquema relacional — é a especificação conceptual. O esquema físico vive em `02-schema-sql.md`.

## Convenções

- **PK** — chave primária. Em OpenRest todas as entidades usam UUIDv7 + chave de negócio adicional onde existe (código curto numérico).
- **Soft delete** — todas as entidades suportam `anulado_em` (timestamp) em vez de delete físico, para preservar histórico/listagens passadas.
- **Audit** — `criado_em`, `criado_por`, `alterado_em`, `alterado_por` em todas as fichas.
- **i18n** — campos `designacao` admitem variante por locale (tabela `designacao_locale`).
- **Multi-loja** — entidades operacionais têm `loja_id`; entidades de catálogo podem ser globais ou por loja conforme política da rede.
- **Eventos** — operações relevantes geram eventos em `event_log` (ver `03-event-log.md`).

---

## 1. Tabelas de parâmetros (Empregados)

### 1.1 `nivel_acesso`
Define grupos de permissões.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9 (PK numérica) |
| `designacao` | string(20) | Ex: "Caixa", "Empregado", "Gerente" |
| `custo_hora` | money | Custo/hora dos empregados deste nível (para gráficos de custo) |
| `permissoes` | json | Conjunto de flags (ver `02-data-model/04-access-matrix.md`) |
| `acesso_condicionado` | bool | Exige autorização superior para entrar |

### 1.2 `grupo_comissao_empregado`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9 |
| `designacao` | string(20) | |

### 1.3 `grupo_comissao_artigo`
Igual ao anterior mas para artigos.

### 1.4 `tabela_comissoes`
Matriz cruzada.

| Campo | Tipo | Notas |
|---|---|---|
| `grupo_empregado` | FK | |
| `grupo_artigo` | FK | |
| `percentagem` | decimal(4,2) | -9.99 a 99.99 |

### 1.5 `comissao_fixa`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9 |
| `designacao` | string(20) | |
| `valor` | money | Valor fixo por sessão |

---

## 2. Tabelas de parâmetros (Clientes)

### 2.1 `qualidade_cliente`
Categoria descritiva. `codigo` 1–9, `designacao` 20 chars.

### 2.2 `grupo_desconto_cliente` / 2.3 `grupo_desconto_artigo`
Idêntico em estrutura aos grupos de comissão.

### 2.4 `tabela_descontos`
Matriz cruzada cliente×artigo, percentagem -9.99 a 99.99.

### 2.5 `zona`
Para entrega ao domicílio.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(3) | 1–999 |
| `designacao` | string(20) | |
| `rede_remota_associada` | FK→rede_remota | Nullable |

---

## 3. Tabelas de parâmetros (Geral)

### 3.1 `tipo_preco`
Define a nomenclatura global dos 5 níveis de PVP. Não contém os preços em si; os valores monetários residem estritamente nas colunas estáticas `pvp1..pvp5` das tabelas `artigo` e `familia`. Não existem tabelas relacionais adicionais de preços.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–5 |
| `designacao` | string(10) | "Mesa", "Take-Away", "Festa", … |

### 3.2 `metodo_pagamento`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9; 1=Numerário, 9=Conta Corrente (convenção herdada) |
| `designacao` | string(20) | |
| `abreviatura` | string(2) | |
| `factor_conversao` | decimal(3,2) | -1.00 a 1.99 (legado: conversão moeda) |
| `simbolo` | string(8) | Texto ou ícone |
| `tipo` | enum | dinheiro, cartão, cheque, vale, conta_corrente, voucher, pagamento_movel |
| `automatico` | bool | Pagamento automático via dispositivo (kiosk) |
| `pode_dar_troco` | bool | |
| `aceita_valor_parcial` | bool | |
| `imprime_recibo_separado` | bool | |
| `cor_botao` | colour | |

### 3.3 `taxa_iva`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9 |
| `designacao` | string(20) | "Isento", "Reduzido", "Intermédio", "Normal", … |
| `percentagem` | decimal(5,2) | |
| `vigencia_de` | date | Permite mudanças legais |
| `vigencia_ate` | date | Nullable |
| `pais` | iso2 | Para multi-país (PT, ES, …) |

### 3.4 `unidade_movimento`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | string(2) | "Un", "Kg", "L", "ml", "dz" |
| `designacao` | string(20) | |
| `casas_decimais` | int(1) | |

### 3.5 `tamanho`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(1) | 1–9 |
| `designacao` | string(20) | "Pequeno", "Médio", "Grande" |

### 3.6 `atributo` / `atributo_valor`
Até 3 atributos com até N valores cada. Cada documento pode receber 1 valor por atributo.

```
atributo (id, ordem 1..3, nome, ui)
atributo_valor (atributo_id, codigo, nome, ordem)
```

---

## 4. Entidades operacionais principais

### 4.1 `empregado`
Identifica utilizadores que operam o sistema.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | PK técnica |
| `codigo` | int(6) | Chave de negócio numérica |
| `nome` | string(40) | |
| `nivel_acesso_id` | FK | |
| `password_hash` | string | argon2id |
| `numero_cartao` | string(20) | Cartão magnético ou código de barras |
| `mesas_atribuidas` | range_set | Sintaxe `1:5,10:20:2` (sintaxe legada — ver `09-set-syntax.md`) |
| `base_consumo` | money | Limite mensal de consumo grátis |
| `pvp_consumo` | enum(pvp1..pvp5,local) | PVP usado para calcular consumo |
| `perc_consumo` | decimal(5,2) | % do PVP que o empregado paga |
| `base_ofertas` | money | Limite ofertas a clientes |
| `pvp_ofertas` | enum(pvp1..pvp5,local) | |
| `perc_ofertas` | decimal(5,2) | |
| `grupo_comissao_id` | FK | |
| `produz_para` | array<FK empregado> | Distribui comissões para… |
| `recebe_de` | array<FK empregado> | Recebe comissões de… |
| `cor_botao` | colour | Aparece nos botões e listas |
| `lingua` | locale | "pt-PT", "es-ES", … |
| `morada`, `localidade`, `cod_postal`, `nif`, `bi`, `telefone`, `data_admissao` | — | Dados pessoais |
| `acerta_cc_fecho_sessao` | bool | Emite ajuste automático ao fechar sessão |
| `anulado_em` | timestamp | Soft delete |

### 4.2 `cliente`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int(6) | |
| `nome` | string(40) | |
| `associacao_cliente_id` | FK cliente | "Cliente paga conta de outro" |
| `parente_mononivel_id` | FK cliente | Relação informativa |
| `numero_cartao` | string(9) | |
| `validade_cartao` | date | |
| `observacoes` | text | |
| `grupo_desconto_id` | FK | |
| `qualidade_cliente_id` | FK | |
| `limite_credito` | money | |
| `morada`, `localidade`, `cod_postal`, `data_nascimento`, `nif`, `telefone`, `telefax`, `email` | — | |
| `zona_id` | FK zona | |
| `total_debito_atual` | money | Calculado |
| `total_credito_atual` | money | Calculado |
| `saldo_atual` | money | Calculado = crédito − débito |
| `pontos` | int | Fidelidade |
| `anulado_em` | timestamp | |

> Os números de telefone aceitam a sintaxe legada `260100\1\2,260200` (várias terminações/números).

### 4.3 `familia`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int(4) | Família de topo ≡ múltiplo de 100 (600, 700). |
| `familia_superior_id` | FK familia | Nullable para família raiz. Permite hierarquia em vários níveis (N níveis de sub-famílias). |
| `nome_curto` | string(10) | |
| `designacao` | string(40) | |
| `nome_botao` | string(20) | |
| `tipo` | enum | normal, complemento, informativo, consumo |
| `tipo_movimento_id` | FK | un/peso |
| `unidade_id` | FK | |
| `grupo_comissao_id` | FK | |
| `grupo_desconto_id` | FK | |
| `iva_mesa_id` | FK | |
| `iva_venda_directa_id` | FK | |
| `pvp1..pvp5` | money | |
| `zona_impressao_id` | FK | Default da família |
| `pvp_variavel` | bool | |
| `ordem_impressao` | int(1) | 1–9 |
| `tamanho_id` | FK | Default |
| `tara` | weight | |
| `peso_unitario` | weight | |
| `mostra_complementos_automaticos` | bool | |
| `cor_botao` | colour | |
| `imagem` | blob | |
| `anulado_em` | timestamp | |

### 4.4 `artigo`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int(4) | |
| `familia_id` | FK familia | Pode ser associado a qualquer nível da hierarquia de famílias |
| `codigo_pedido` | int | Código curto para teclado |
| `nome_curto` | string(10) | |
| `designacao` | string(40) | |
| `nome_botao` | string(20) | |
| `artigo_meia_dose_id` | FK artigo | Aponta para "homólogo menor" |
| `tipo_artigo` | enum | normal, complemento, informativo, consumo, gorjeta |
| `tipo_movimento_id` | FK | un/peso |
| `contabilizacao` | decimal(4,2) | Factor de quantidade |
| `unidade_id` | FK | |
| `grupo_comissao_id` | FK | |
| `grupo_desconto_id` | FK | |
| `iva_mesa_id` | FK | |
| `iva_venda_directa_id` | FK | |
| `pvp1..pvp5` | money | Os 5 preços de venda fixos. Não existem outras tabelas de preços de artigos. |
| `pvp_variavel` | bool | |
| `zona_impressao_id` | FK | |
| `tamanho_id` | FK | |
| `tara` | weight | |
| `peso_unitario` | weight | |
| `codigo_barras` | string(13) | Múltiplos códigos permitidos (tabela auxiliar) |
| `cor_botao` | colour | |
| `imagem` | blob | |
| `classe_dispositivo` | int | Para máquina de café (acerto, dose…) |
| `taxa_servico_percentagem` | decimal(5,2) | Só para tipo=gorjeta |
| `taxa_servico_base` | money | |
| `taxa_servico_arredondamento` | money | |
| `anulado_em` | timestamp | |

### 4.5 `artigo_codigo_barras`
Tabela secundária. Um artigo pode ter N códigos.

### 4.6 `complemento_recente` (rank de complementos automáticos)
Mantém histórico para sugerir complementos mais usados quando o utilizador escolhe um artigo principal.

```
artigo_principal_id, complemento_id, contagem, ultima_utilizacao
```

### 4.7 `armazem` (centro de custos)
Stock lógico abatido pelas vendas via zona de impressão.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int(3) | 1–999 |
| `designacao` | string(40) | |

### 4.8 `centro_custo_zona`
Mapa zona de impressão → armazém.

### 4.9 `local`
Conjunto lógico de mesas com configuração própria.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `designacao` | string(30) | |
| `mesas_definicao` | range_set | "1:50" ou "100:200:2" |
| `tipo` | enum | normal, take_away, take_away_seguro, pub, delivery, consumo_proprio, restauracao_colectiva |
| `tipo_preco_id` | FK | |
| `metodo_pagamento_default_id` | FK | |
| `taxa_servico_artigo_id` | FK artigo | Artigo do tipo gorjeta a aplicar |
| `limite_consumo` | money | 0 = sem limite |
| `imprime_conta_acima_de` | money | Força VD acima do valor |
| `nome_generico_mesa` | string(30) | "Mesa \nm" |
| `imprime_subtotal_em` | json | {pedido, anulacao, pagamento_parcial, fecho, comando} |
| `imprime_conta_em` | json | idem |
| `fecha_mesa_ao_pedir` | enum | nunca, comando, sempre |
| `usa_iva_venda_directa` | bool | |
| `iva_excluido_dos_precos` | bool | "Preços sem IVA" |
| `cor_empregado_na_lista` | bool | |
| `impressora_directa_pedidos_id` | FK | Monitor de pedidos |
| `pede_nova_mesa_depois_de_fechar` | bool | |
| `pede_nova_mesa_apos_pedido` | bool | |
| `indica_pessoas_obrigatorio` | bool | |
| `indica_pessoas_apenas_abertura` | bool | |
| `permite_zero_pessoas` | bool | |
| `aloca_mesas_dinamicamente` | bool | |
| `alocacao_circular` | bool | |
| `inclui_desconto_nos_precos` | bool | |
| `artigos_automatico_sem_preco` | bool | |
| `carregamento_rapido_mesas` | bool | |
| `so_imprime_pedidos_com_complementos` | bool | |
| `lista_grande_pedidos` | bool | |
| `mesas_uma_vez_por_dia` | bool | |
| `facturacao_externa` | bool | |
| `nao_agrupa_detalhes_na_conta` | bool | |
| `permite_encaixe_promocoes` | bool | |
| `separa_artigos_antes_encaixe` | bool | |
| `permite_mesas_abertas_fim_do_dia` | bool | |
| `pode_identificar_cliente_no_pedido` | bool | |
| `obriga_indicar_valor_pago` | bool | |
| `usa_desenho_mesas` | bool | |
| `imagem` | blob | Imagem de fundo se usa_desenho_mesas |
| `largura` | int | Largura da área de desenho |
| `altura` | int | Altura da área de desenho |
| `anulado_em` | timestamp | |

### 4.10 `mesa`
Define a configuração estática e visual de uma mesa no sistema.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `local_id` | FK | |
| `codigo` | int | Único por local |
| `nome` | string(30) | Sobrepõe-se ao genérico se != null |
| `nomeobjecto` | string | |
| `posx` | int | |
| `posy` | int | |
| `imagem` | blob | |
| `fntname` | string | |
| `fntsize` | int | |
| `fntcolor` | colour | |
| `fontx` | int | |
| `fonty` | int | |
| `fontstyle` | string | |
| `estadox` | int | |
| `estadoy` | int | |
| `reservax` | int | |
| `reservay` | int | |
| `altura` | int | |
| `largura` | int | |
| `criada_em` | timestamp | |

### 4.11 `posto`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `numero` | int | Identificador 1, 2, … |
| `descricao` | string | |
| `tipo` | enum | fundamental, importante, secundario |
| `mesa_default_id` | FK mesa | Ou usar 1ª do empregado |
| `usa_primeira_mesa_empregado` | bool | |
| `mesas_acessiveis` | range_set | |
| `opcao_default` | enum | mesas, pedidos, recebe, … |
| `impressora_sistema_id` | FK | |
| `display_cliente_id` | FK | |
| `pedidos_por_teclado` | bool | |
| `aviso_reserva_min` | int | Tempo de antecedência da reserva |
| `protecao_ecra_min` | int | 0 desactiva |
| `gaveta_id` | FK | |
| `caixa_fixa_id` | FK caixa | Nullable |
| `opcoes_activas` | json | Por opção: {activo, pede_utilizador, pede_codigo} |
| `tempo_volta_inicial` | int(seg) | |
| `monitor` | int | Multi-monitor |
| `resolucao` | enum | 320×240, 640×480, 800×600, 1024×768, 1280×800, fullhd |
| `rotacao` | enum | 0, 90, 180, 270 |
| `teclado_id` | FK | |
| `teclado_virtual` | bool | |
| `teclado_secundario_id` | FK | |
| `touchscreen_driver` | string | |
| `cores_personalizadas` | json | botões, janelas, fundo |
| `auto_desligar` | bool | |
| `consolas_virtuais` | array<int> | Postos a correr na mesma máquina |
| `anulado_em` | timestamp | |

---

## 5. Documentos e movimentos

### 5.1 `documento`
Cabeçalho fiscal/operacional. **Imutável** após emissão; alterações são via novo documento de tipo `estorno`/`anulacao`.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `tipo` | enum | factura, factura_recibo, venda_dinheiro, recibo, consulta_mesa, nota_credito, senha, sub_total, …pedido_cozinha, sub_total_local, factura_externa, pontos, apuramento_dia |
| `serie` | string | Série fiscal (PT: prefixo+ano) |
| `numero` | int | Sequencial dentro da série |
| `numero_local` | string | Para identificação humana ("FT 2024/1234") |
| `atcud` | string | Portugal — `CodigoValidacao-NumeroDoc` |
| `qr_code_payload` | string | Conforme Portaria 195/2020 (PT) |
| `hash_assinatura` | string | Hash conforme Portaria 363/2010 (PT) |
| `hash_versao_chave` | string | "1" |
| `data_emissao` | timestamp | |
| `data_documento` | date | Data lógica de caixa |
| `loja_id` | FK | |
| `posto_emissor_id` | FK | |
| `caixa_id` | FK | Nullable se documento de cozinha |
| `sessao_id` | FK | |
| `empregado_abertura_id` | FK | |
| `empregado_fecho_id` | FK | |
| `cliente_id` | FK | Nullable |
| `cliente_nome` | string | Snapshot |
| `cliente_nif` | string | Snapshot |
| `cliente_morada`, `cliente_localidade`, `cliente_codpostal`, `cliente_pais` | — | Snapshot |
| `mesa_id` | FK | Nullable em VDs directas |
| `mesa_nome_snapshot` | string | |
| `numero_pessoas` | int | |
| `total_sem_iva` | money | |
| `total_iva` | money | |
| `total` | money | |
| `total_desconto` | money | |
| `total_oferta` | money | |
| `total_taxa_servico` | money | |
| `arredondamento` | money | |
| `metodo_pagamento_principal_id` | FK | |
| `atributos` | json | {1: valor_id, 2: valor_id, 3: valor_id} |
| `observacoes_pedido`, `observacoes_factura`, `observacoes_cliente`, `observacoes_morada` | text | |
| `documento_anulado_por_id` | FK documento | |
| `documento_que_anula_id` | FK documento | |
| `anulado` | bool | |
| `estornado` | bool | |

### 5.2 `documento_detalhe`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `documento_id` | FK | |
| `linha` | int | Ordem |
| `nivel_promocao` | int | 0 se não pertence a promoção |
| `artigo_id` | FK | |
| `artigo_codigo` | string | Snapshot |
| `artigo_designacao` | string | Snapshot |
| `quantidade` | decimal(7,3) | |
| `unidade_id` | FK | |
| `preco_unitario` | money | |
| `preco_unitario_sem_iva` | money | |
| `desconto_percent` | decimal(5,2) | |
| `desconto_valor` | money | |
| `oferta` | bool | |
| `iva_codigo` | int(1) | |
| `iva_percentagem` | decimal(5,2) | |
| `total_linha` | money | |
| `total_linha_sem_iva` | money | |
| `zona_impressao_id` | FK | |
| `empregado_pedido_id` | FK | |
| `pedido_em` | timestamp | |
| `anulada` | bool | |
| `anulada_com_desperdicio` | bool | |
| `anulada_por` | FK empregado | |
| `anulada_em` | timestamp | |
| `nivel_macro` | int | |
| `complemento_de_detalhe_id` | FK documento_detalhe | Para complementos |

### 5.3 `documento_pagamento`
Permite múltiplos pagamentos por documento.

| Campo | Tipo | Notas |
|---|---|---|
| `documento_id` | FK | |
| `linha` | int | |
| `metodo_pagamento_id` | FK | |
| `valor` | money | |
| `valor_pago` | money | Pode ser maior por troco |
| `troco` | money | |
| `cliente_id` | FK | Para CC |
| `referencia_externa` | string | Ex: nº de autorização Multibanco |

### 5.4 `documento_iva_breakdown`
Sumário por taxa de IVA (snapshot impresso na factura/VD).

| Campo | Tipo | Notas |
|---|---|---|
| `documento_id` | FK | |
| `iva_codigo` | int(1) | |
| `iva_percentagem` | decimal(5,2) | |
| `base` | money | |
| `valor_iva` | money | |

---

## 6. Operação (mesas vivas)

### 6.1 `mesa_estado`
Estado dinâmico e vivo da mesa (aberta, bloqueada, reservada).

| Campo | Tipo | Notas |
|---|---|---|
| `mesa_id` | FK | PK |
| `estado` | enum | livre, aberta, em_espera, reservada, bloqueada |
| `bloqueada_por_posto_id` | FK posto | |
| `bloqueada_motivo` | text | |
| `cliente_associado_id` | FK | Cliente atribuído à mesa |
| `numero_pessoas` | int | |
| `empregado_actual_id` | FK | |
| `aberta_em` | timestamp | |
| `subtotal_actual` | money | |
| `reservada_ate` | timestamp | |
| `reserva_pessoas` | int | |
| `reserva_cliente_id` | FK | |
| `reserva_observacoes` | text | |

---

## 7. Caixa

### 7.1 `caixa`
Acumulador físico/lógico de dinheiro.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int | |
| `designacao` | string | |
| `impressora_documentos_id` | FK | |
| `gaveta_automatica_id` | FK gaveta | |
| `vendas_activas_mantem_empregado` | bool | |
| `nao_permitir_estornos_apos_fecho_sessao` | bool | |
| `taxa_conversao_euro` | decimal | Legacy (transição EUR) |
| `max_oferta_isenta_iva` | money | |
| `valores_troco_rapido` | array<money> | Botões take-away |
| `documentos_excluidos` | json | Movimentos de caixa a não imprimir |
| `exclui_verificacao_cliente` | bool | |
| `abre_no_arranque` | bool | |
| `sessoes_em_comandos` | json | {abre, fecha, com_bolsa, fundo_maneio, caixa_associada} |
| `forca_abertura_arranque` | bool | |
| `apura_em_automatico` | json | |
| `introduz_valor_em_caixa` | bool | Fecho com contagem |
| `imprime_vendas_horarias` | bool | |
| `fecho_directo` | bool | Fecha tudo automaticamente |
| `comissoes_para_empregado_fecho` | bool | |
| `comissoes_sem_iva` | bool | |
| `inclui_mesas_abertas_apuramentos` | bool | |
| `anulacoes_para_empregado_pedido` | bool | |
| `apura_facturado_sessao_por` | enum | fecho, abertura, pedido |
| `anulado_em` | timestamp | |

### 7.2 `caixa_dia`
Estado de uma caixa num dia.

| Campo | Tipo | Notas |
|---|---|---|
| `caixa_id` | FK | |
| `data_caixa` | date | |
| `fundo_maneio_inicial` | money | |
| `fundo_maneio_transportado` | money | |
| `estado` | enum | aberta, fechada |
| `aberta_em` | timestamp | |
| `aberta_por` | FK empregado | |
| `fechada_em` | timestamp | |
| `fechada_por` | FK empregado | |
| `saldo_transporte` | money | Calculado |

### 7.3 `turno`
Subdivisões temporais da caixa.

| Campo | Tipo | Notas |
|---|---|---|
| `caixa_dia_id` | FK | |
| `numero` | int | 1, 2, … |
| `aberto_em` | timestamp | |
| `fechado_em` | timestamp | |

### 7.4 `sessao_empregado`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `empregado_id` | FK | |
| `caixa_dia_id` | FK | |
| `turno_id` | FK | |
| `com_bolsa` | bool | |
| `fundo_bolsa` | money | |
| `comando_associado_id` | FK | |
| `comissao_fixa_id` | FK | |
| `aberta_em` | timestamp | |
| `fechada_em` | timestamp | |
| `facturado` | money | Calculado |
| `comissoes` | money | Calculado |
| `consumo` | money | Calculado |
| `ofertas` | money | Calculado |
| `acerto_cc_no_fecho` | bool | |

### 7.5 `movimento_caixa`
Catálogo unificado de movimentos.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `tipo` | enum | abertura, pagamento_cc_entrada, fundo_entrada, emprestimo_entrada, retirada, envelope_deposito, compra, vale_saida, transferencia_bolsa_caixa, transferencia_caixa_bolsa, transferencia_turno, venda_dinheiro, recebimento_factura, ajuste_sessao, fecho |
| `caixa_dia_id` | FK | |
| `turno_id` | FK | |
| `sessao_empregado_id` | FK | Nullable |
| `metodo_pagamento_id` | FK | |
| `valor` | money | Positivo entrada, negativo saída |
| `documento_origem_id` | FK documento | Nullable |
| `cliente_id` | FK | Nullable |
| `empregado_envolvido_id` | FK | Nullable |
| `observacao` | text | |
| `criado_em` | timestamp | |
| `criado_por` | FK empregado | |

### 7.6 `envelope`
Depósitos identificáveis dentro de caixa.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | string | |
| `valor` | money | |
| `data` | date | |
| `empregado_id` | FK | |
| `caixa_dia_id` | FK | |
| `consultado` | bool | |
| `fechado` | bool | |

### 7.7 `marcacao_ponto`
Registo de assiduidade (módulo timeclock).

| Campo | Tipo | Notas |
|---|---|---|
| `empregado_id` | FK | |
| `instante` | timestamp | |
| `tipo` | enum | entrada, saida |

---

## 8. Promoções, fidelidade e ofertas

### 8.1 `promocao`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `artigo_id` | FK | "Cabeça" da promoção |
| `niveis` | json | Lista de níveis (ver 8.2) |

### 8.2 `promocao_nivel` / `promocao_nivel_item`
Cada nível tem N artigos elegíveis com `delta_preco`.

```
nivel (promocao_id, ordem, nome)
item (nivel_id, artigo_id, delta_preco, exclusivo, escolha_default)
```

### 8.3 `artigo_exclusivo`
Lista de artigos vendíveis apenas como item de promoção.

### 8.4 `happy_hour`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `dia_semana` | enum | seg..dom, ou data específica |
| `hora_inicio` | time | |
| `hora_fim` | time | |
| `local_id` | FK | Nullable = todos |
| `artigos` | array<FK artigo> | |
| `tipo_preco_id` | FK | |
| `quantidade_bonus` | int | "1 paga, 2 leva" → 1 |

### 8.5 `pontos_config`
Singleton.

| Campo | Tipo | Notas |
|---|---|---|
| `activo` | bool | |
| `valor_por_ponto` | money | |
| `metodo_arredondamento` | enum | up, down, nearest, banker |
| `permite_descontos_venda` | bool | |
| `valor_por_ponto_venda` | money | |
| `impressora_id` | FK | |

### 8.6 `cliente_pontos_movimento`
Histórico de ganho/uso de pontos.

---

## 9. Pratos do Dia / Restauração Colectiva

### 9.1 `prato_do_dia_config`
| Campo | Tipo | Notas |
|---|---|---|
| `tipo` | enum | semanal, diario, configuravel |
| `numero_dias` | int | |
| `data_inicio` | date | |
| `refeicoes_por_dia` | int | |
| `nomes_refeicoes` | array<string> | Almoço, Jantar… |
| `familia_pratos_id` | FK familia | |

### 9.2 `prato_do_dia`
Cada dia × refeição × correspondente.

| Campo | Tipo | Notas |
|---|---|---|
| `data` | date | |
| `refeicao` | int | |
| `artigo_prato_dia_id` | FK | "Prato Carne", "Prato Peixe" |
| `artigo_correspondente_id` | FK | Artigo real consumido |
| `tipo_preco_id` | FK | |

### 9.3 `refeicao_actual`
Aponta qual a refeição em curso (singleton por loja).

### 9.4 `reserva_refeicao`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `cliente_id` | FK | Nullable para eventual |
| `data` | date | |
| `refeicao` | int | |
| `prato_do_dia_id` | FK | |
| `loja_destino_id` | FK loja | |
| `senha_uid` | string | Encriptado |
| `senha_documento_id` | FK documento | |
| `estado` | enum | reservada, usada, anulada |

### 9.5 `reserva_refeicao_uid_config`
Configuração do esquema UID por loja: código secreto, offset.

---

## 10. Hardware / Dispositivos

### 10.1 `dispositivo`
Entidade polimórfica para todo o hardware.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int | |
| `posto_id` | FK posto | |
| `tipo` | enum | porta_paralela, porta_serie, dos_file, porta_nula, socket_port, server_socket_port, impressora_ecra, monitor_pedidos, maquina_cafe, netpay, controlo_acessos, impressora_generica, impressora_fiscal, botoneira_serie, botoneira_paralela, gaveta_generica, display_cliente, display_interno, ncr_7460_display, velleman_mml30g, leitor_cartoes, leitor_codigos_barras, receptor_terminais_radio, antena_v02, led_id, balanca_generica, balanca_bizerba, jarltech_8100, pos_par, ncr_7460, bleep_ts600, bleep_ts650, zyxel_callerid, fritzx_callerid, identificador_generico |
| `pai_dispositivo_id` | FK dispositivo | Cadeia (impressora→porta) |
| `nome` | string | |
| `configuracao` | json | Específico do tipo |
| `ligacoes_superiores_max` | int | Quantos podem ligar acima |
| `anulado_em` | timestamp | |

### 10.2 `impressora_zona_local`
Mapa zona × local → impressora(s).

| Campo | Tipo | Notas |
|---|---|---|
| `zona_impressao_id` | FK | |
| `local_id` | FK | |
| `origem_id` | FK origem | Nullable = default |
| `dispositivo_impressora_id` | FK dispositivo | |
| `agrupamento` | enum | normal, individual, por_artigo, agrupado, individual_agrupado, por_artigo_agrupado, agrupa_zonas, agrupa_tudo |
| `tipo_pedido_documento_id` | FK | |
| `tipo_secundario_documento_id` | FK | |
| `tipo_complemento_documento_id` | FK | |
| `tipo_subtotal_documento_id` | FK | |
| `tipo_conta_documento_id` | FK | |
| `numero_copias` | int | |

### 10.3 `zona_impressao`
| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int | 1=D.Externos, 2=Sub-totais por convenção |
| `designacao` | string | |
| `tipo_pedido_id` | FK documento_tipo | |
| `tipo_secundario_id` | FK | |
| `tipo_complemento_id` | FK | |
| `secundarios` | bool | Pedidos cruzados |

### 10.4 `origem`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `nome` | string | |
| `postos` | range_set | |
| `comandos` | range_set | |

### 10.5 `documento_template`
Cabeçalho/rodapé configurável.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `tipo_documento` | enum | factura, vd, recibo, consulta_mesa, pedido, senha, pontos, apuramento_dia, factura_externa, validacao_cliente |
| `numero_cr` | int | 1–9 |
| `colunas` | int | |
| `cabecalho` | text | Com flags `\no`, `\nd`, etc. |
| `rodape` | text | Idem |
| `campos_detalhe` | json | Quais campos de detalhe e formato (col, largura, justificação, fonte) |
| `imprime_complementos` | json | {com_preco, sem_preco, nao_imprime_preco} |
| `nao_imprime_detalhes` | bool | |
| `nao_imprime_nomes_campos` | bool | |

### 10.6 `tecla_config`
Mapeamento de teclas/cartões para funções.

| Campo | Tipo | Notas |
|---|---|---|
| `posto_id` | FK | |
| `tecla` | string | "Enter", "F1", "Ctrl+Alt+P" |
| `cartao_codigo` | string | Alternativa por leitor |
| `funcao` | enum | artigo, familia, mesa, empregado, quantidade, apaga_linha, apaga_tudo, pedir, sub_total, conta, preco, anular, gaveta, factura, transferencias, consulta_registos, troco, pag_parcial, desconto, num_pessoas, pedir_parcial, anular_parcial, subtotal_parcial, limpa_empregado, delivery |
| `valor` | int | Código associado à função |
| `sequencia` | array<keystroke> | Para macros de teclado |

### 10.7 `rede_remota`
Outras lojas/instâncias.

| Campo | Tipo | Notas |
|---|---|---|
| `codigo` | int | Equivalente à zona |
| `endereco` | string | IP/host:port ou nº de linha |
| `net` | int | |
| `dispositivo_id` | FK dispositivo | TCP/IP, ZyXEL TA, etc. |
| `empregado_delivery_id` | FK empregado | Responsável pelos pedidos recebidos |
| `local_delivery_id` | FK local | Para onde entram pedidos |
| `grava_pedidos_localmente` | bool | |
| `cliente_call_center` | bool | |

---

## 11. Sistema / Configuração global

### 11.1 `definicoes_gerais`
Singleton com todas as flags globais (formato dinheiro, língua, zona, Qt. máxima, importações automáticas, folga semanal, transição moeda, conversão valores, SmartChoice, ficheiros a exportar, mostra fichas anuladas, muda nome empregado, cursor rotativo, mostra código pedido, mostra fracções, formato data, transacções em disco, arquivo documentos, letras pequenas, transferência vazia, manutenção dá acesso a tudo, imprime moeda não base, numeração no apuramento, 4 casas decimais, conta cliente método padrão, etc.).

### 11.2 `licenca`
| Campo | Tipo | Notas |
|---|---|---|
| `chave` | string | |
| `tipo` | enum | demo, perpetua, renting, comunidade |
| `validade` | date | |
| `hardlock_numero` | string | Legacy |
| `cloud_key` | string | Modelo moderno |
| `casa_designacao_social` | string | |
| `casa_nif` | string | |
| `casa_morada`, `localidade`, `cod_postal`, `pais`, `tel`, `fax`, `conservatoria`, `nr_registo`, `capital_social` | — | |
| `modulos_activos` | array<string> | reservas, fecho_financeiro, dispenser, colectiva, w4, timeclock, http_api, vnc, reports, store, ticket, dispenser, comserver, … |
| `max_postos`, `max_artigos`, `max_empregados`, `max_lojas` | int | |

### 11.3 `serie_documento`
| Campo | Tipo | Notas |
|---|---|---|
| `tipo_documento` | enum | |
| `prefixo` | string | "FT", "FR", "VD", "NC" |
| `ano` | int | |
| `proximo_numero` | int | |
| `comunicada_at` | bool | PT — comunicada à AT |
| `atcud_validacao` | string | Código devolvido pela AT |

### 11.4 `atcud`
Códigos ATCUD atribuídos pela Autoridade Tributária.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `tipodoc` | enum | |
| `serie` | string | |
| `ano` | int | |
| `atcud` | string | Código de validação ATCUD |
| `data_inicio` | date | Data de início de vigência |
| `data_registo` | timestamp | |
| `ativo` | bool | |

### 11.5 `loja`
Em modelos multi-loja.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `codigo` | int | |
| `designacao_comercial` | string | |
| `designacao_social` | string | |
| `nif`, `morada`, `cod_postal`, `localidade`, `pais` | — | |
| `taxa_iva_default_id` | FK | |
| `moeda` | iso4217 | |
| `fuso_horario` | string | |

### 11.6 `pais_locale`
Configuração de regras nacionais.

| Campo | Tipo | Notas |
|---|---|---|
| `pais` | iso2 | PT, ES, BR, SE, TR, … |
| `regras` | json | {iva_em_detalhe_obrigatorio, atcud_obrigatorio, nif_obrigatorio_acima_de, saft_export, hash_assinatura, datas_dd_mm_aaaa, simbolos_moeda} |

### 11.7 `event_log`
*Audit trail* universal.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `agregado_tipo` | string | "documento", "mesa", "caixa", "empregado" |
| `agregado_id` | uuid | |
| `tipo_evento` | string | "documento.emitido", "mesa.aberta", "anulacao.aplicada" |
| `payload` | json | |
| `actor_empregado_id` | FK | |
| `actor_posto_id` | FK | |
| `loja_id` | FK | |
| `criado_em` | timestamp | |
| `correlation_id` | uuid | |

---

## 12. Comunicação remota

### 12.1 `mensagem_remota`
| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `direcao` | enum | enviada, recebida |
| `rede_remota_id` | FK | |
| `assunto` | string | |
| `corpo` | text | |
| `enviada_em` | timestamp | |
| `recebida_em` | timestamp | |
| `lida` | bool | |

### 12.2 `pedido_delivery`
Encomenda de entrega ao domicílio.

| Campo | Tipo | Notas |
|---|---|---|
| `id` | uuid | |
| `cliente_id` | FK | |
| `morada_snapshot` | string | |
| `telefone_snapshot` | string | |
| `recebido_em` | timestamp | |
| `recebido_via` | enum | telefone, balcão, app, web, call_center |
| `posto_origem_id` | FK | |
| `rede_remota_origem_id` | FK | |
| `loja_destino_id` | FK | |
| `entregador_id` | FK empregado | |
| `despachado_em` | timestamp | |
| `entregue_em` | timestamp | |
| `tempo_preparacao_seg` | int | |
| `tempo_entrega_seg` | int | |
| `documento_id` | FK | |
| `estado` | enum | recebido, em_preparacao, pronto, despachado, entregue, cancelado |

---

## 13. Plug-ins (extensões)

Os plug-ins são descritos por entidade `plugin` e configurados por entidade `plugin_config`. Modelos específicos vivem dentro do próprio plug-in (ex: `videovigilancia_evento`, `cafe_credito`, `netpay_transacao`, `ticket_campanha`).

| Campo | Tipo | Notas |
|---|---|---|
| `chave` | string | Único: "videovigilancia", "saft_pt", "primavera_export", … |
| `versao` | string | |
| `activo` | bool | |
| `configuracao` | json | |
| `niveis_acesso_excluidos` | array<int> | |
| `executa_em_manutencao` | bool | |
| `executa_em_normal` | bool | |
| `executa_servidor` | bool | |
| `executa_postos` | bool | |

---

## Decisões importantes adiadas

1. **Mesa viva ≡ documento pendente?** — Decidir se o modelo unifica `mesa_sessao_detalhe` com `documento_detalhe` (versão "evento de venda" com estado) ou os mantém separados (decisão em `02-data-model/03-design-decisions.md`).
2. **Estados de documento** — Como reconciliar a imutabilidade fiscal portuguesa com edições de pré-fecho (consulta de mesa, sub-totais).
3. **Snapshots vs referência** — Quanto snapshot guardar nos documentos quando o catálogo muda.
4. **Multi-tenant** — Será o OpenRest *single-tenant* na instalação, com sincronização entre lojas; ou *multi-tenant* em SaaS? (a especificação assume single-tenant por defeito, com sync.)

Estas decisões são tratadas em [03-design-decisions.md](./03-design-decisions.md).
