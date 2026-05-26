# OpenRest — Catálogo de Flags de Impressão

> Referência completa das sequências especiais (flags) usadas em templates de documentos. Substituídas no momento da impressão pelos valores correntes.

## Casa / Estabelecimento

| Flag | Conteúdo | Origem |
|---|---|---|
| `\no` | Nome comercial da casa | licença |
| `\ds` | Designação social (denominação jurídica) | licença |
| `\mo` | Morada | licença |
| `\lo` | Localidade | licença |
| `\cp` | Código Postal | licença |
| `\pa` | País | licença |
| `\tf` | Telefone | licença |
| `\fx` | Fax | licença |
| `\cv` | Conservatória de Registo Comercial | licença |
| `\nr` | Nº de Registo na Conservatória | licença |
| `\cs` | Capital Social | licença |
| `\nc` | Nº de Contribuinte (NIF) | licença |

## Cliente

| Flag | Conteúdo |
|---|---|
| `\ol` | Nome do cliente |
| `\nx` | Nome do cliente OU da associação (se existir) |
| `\nl` | Nº do cliente |
| `\cl` | NIF do cliente |
| `\cx` | NIF do cliente OU da associação (se existir) |
| `\mc` | Morada do cliente |
| `\ll` | Localidade do cliente |
| `\xp` | Código Postal do cliente (OpenRest novo) |
| `\xz` | Zona (de morada) do cliente (OpenRest novo) |

## Data / Hora

| Flag | Conteúdo |
|---|---|
| `\dt` | Data actual (formato configurável) |
| `\da` | Data da abertura da mesa |
| `\sd` | Data do relógio do sistema |
| `\ho` | Hora HH:MM:SS |
| `\hc` | Hora HH:MM (curto) |
| `\xt` | Hora de emissão (ISO 8601) (OpenRest novo) |

## Documento

| Flag | Conteúdo |
|---|---|
| `\nd` | Nº de documento (do fecho do dia, conforme contexto) |
| `\ns` | Série do documento (OpenRest novo) |
| `\nx` | Identificador composto Série/Número (OpenRest novo) |
| `\atcud` | ATCUD (Portaria 195/2020) (OpenRest novo) |
| `\qr` | QR Code (substituído por bitmap; payload conforme legislação) (OpenRest novo) |
| `\hash` | Hash de assinatura (4 chars extraídos do hash Base64) (OpenRest novo) |
| `\versao` | Versão do software (OpenRest novo) |

## Empregado

| Flag | Conteúdo |
|---|---|
| `\ne` | Nº empregado |
| `\oe` | Nome do empregado |
| `\nm` | Nº da mesa |
| `\om` | Nome da mesa |

## Pessoas e Valor

| Flag | Conteúdo |
|---|---|
| `\np` | Nº de pessoas |
| `\pp` | Valor por pessoa |
| `\st` | Sub-total |
| `\vt` | Valor total |
| `\ve` | Valor total na moeda secundária |
| `\sx` | Total sem IVA |
| `\tx` | IVA total |
| `\ti` | Tabela de IVA (apenas em facturas, VDs, consultas) — sem cabeçalho |

## Pagamento

| Flag | Conteúdo |
|---|---|
| `\vc` | Valor cambiado (com factor de conversão do método) |
| `\vg` | Valor de gorjeta |
| `\fp` | Forma de pagamento (designação) |
| `\tr` | Troco |
| `\te` | Troco na moeda secundária |
| `\pg` | Valor pago |
| `\pe` | Valor pago na moeda secundária |

## Atributos

| Flag | Conteúdo |
|---|---|
| `\a1` | Atributo 1 |
| `\a2` | Atributo 2 |
| `\a3` | Atributo 3 |

**Em senhas de refeição** (semântica especial):
- `\a1` → Data da refeição reservada
- `\a2` → Para que refeição foi reservada
- `\a3` → Prato escolhido

## Formatação

| Flag | Efeito |
|---|---|
| `\s0` | Vermelho ON (ou negrito, conforme impressora) |
| `\s1` | Vermelho OFF |
| `\s2` | Tamanho duplo ON |
| `\s3` | Tamanho duplo OFF |
| `\s4` | Sublinhado ON |
| `\s5` | Sublinhado OFF |
| `\s6` | (Em impressoras EAN-13) início de código de barras |
| `\s7` | Centrar (OpenRest novo) |
| `\s8` | Alinhar à direita (OpenRest novo) |
| `\s9` | Alinhar à esquerda (default) (OpenRest novo) |

## Códigos de Barras / Bitmaps

| Flag | Conteúdo |
|---|---|
| `\bc` | Imprime código de barras CODE128 (bitmap por defeito ou ESC/POS se activo) |
| `\b0..\b9` | Bitmap N (até 10 bitmaps por documento). Cross com nome de ficheiros `WRSTSC00.8YX` |

## Outras

| Flag | Conteúdo |
|---|---|
| `\lc` | Nome do local. **Em senhas de refeição**: nome da loja destino. |

## Construção avançada (XML-like)

Para campos arbitrários da BD ou para flags com formatação avançada:

```html
<! type="tipo" id="campo" mask="máscara" align="alinhamento" default="default" offset="N" !>
```

### Atributos

| Atributo | Valor possível | Significado |
|---|---|---|
| `type` | `flag` | Equivale a usar a flag por nome |
| `type` | `field` | Acede a campo de tabela |
| `type` | `uid` | Gera UID para senha de refeição |
| `id` | string | Nome da flag, ou caminho `tabela.campo` |
| `mask` | string | Máscara de formatação; `#` representa dígito |
| `align` | `left` `right` `center` | Alinhamento (aplicado antes da máscara) |
| `default` | string | Valor por defeito se vazio |
| `offset` | int | (UID only) offset para evitar colisões cross-store |

### Exemplos

```html
Cabeçalho centrado:
<! type="flag" id="no" align="center" mask="#####################################" !>

Campo da BD acedido por path:
<! type="field" id="fb_c_empa.memp_nome" align="left" mask="############" !>

Total formatado:
\s2Total <! type="field" id="fb_c_vtot" align="right" mask="############" !>\s3

UID para senha:
<! type="uid" id="12345" offset="5000" !>
```

### Caminhos de field

Caminhos relativos à tabela principal do documento:
- Em **factura / VD / consulta**: cabeçalho da mesa (`fb_c_*`)
- Em **recibo**: tabela de caixa (`mov_*`)
- Em **pedido**: cabeçalho do pedido (`p_*`)

Pode descer com `.` para tabelas relacionadas: `fb_c_empa.memp_nome` (empregado de abertura → tabela empregado → campo nome).

## Catálogo de campos (referência)

### Cabeçalho de documento (consulta/factura/VD)

- `fb_c_proc` — Nº processo
- `fb_c_data` — Data
- `fb_c_hora` — Hora
- `fb_c_mesa` — Nº mesa
- `fb_c_nome_mesa` — Nome da mesa
- `fb_c_empa` — FK empregado abertura
- `fb_c_empf` — FK empregado fecho
- `fb_c_cli` — FK cliente
- `fb_c_nif` — NIF cliente snapshot
- `fb_c_np` — Nº pessoas
- `fb_c_vtot` — Total
- `fb_c_vsiva` — Total sem IVA
- `fb_c_vdesc` — Total desconto
- `fb_c_vofer` — Total ofertas
- `fb_c_vtxs` — Total taxa serviço
- `fb_c_metp` — FK método pagamento principal
- `fb_c_obs_pe` — Obs. pedido
- `fb_c_obs_fa` — Obs. factura
- `fb_c_obs_cl` — Obs. cliente
- `fb_c_obs_mo` — Obs. morada
- `fb_c_a1`, `fb_c_a2`, `fb_c_a3` — Atributos

### Detalhe

- `fb_d_qtd` — Quantidade
- `fb_d_art` — Código artigo
- `fb_d_design` — Designação
- `fb_d_punit` — Preço unitário
- `fb_d_pcusto` — Preço sem IVA
- `fb_d_perc_desc` — % desconto
- `fb_d_val_desc` — Valor desconto
- `fb_d_iva_cod` — Código IVA
- `fb_d_iva_perc` — Taxa IVA
- `fb_d_total_linha` — Total linha
- `fb_d_zona_imp` — Zona impressão
- `fb_d_emp_pedido` — Empregado de pedido
- `fb_d_hora` — Hora de pedido

### Pagamento

- `fb_p_val` — Valor
- `fb_p_pago` — Valor pago
- `fb_p_troco` — Troco
- `fb_p_metp` — Método

### Empregado

- `memp_codigo`
- `memp_nome`
- `memp_nivel`

### Cliente

- `mcli_codigo`
- `mcli_nome`
- `mcli_nif`
- `mcli_morada`

### Caixa (recibo)

- `mov_data`
- `mov_tipo`
- `mov_valor`
- `mov_emp`
- `mov_cli`
- `mov_obs`

## Configuração de detalhes

Editor visual permite definir, para cada campo a imprimir numa linha de detalhe:

- Posição (coluna inicial)
- Tamanho (largura)
- Justificação (esquerda, direita, centro)
- Tamanho duplo (toggle)
- Negrito / Vermelho

Para pedidos:
- Quantidade
- Nome curto
- Nome artigo
- Código
- Preço unitário
- Preço total
- SubTotal (apenas em impressoras de cartões)

Para contas (factura / VD / consulta):
- Os mesmos + Código IVA + Taxa IVA

Opções globais por documento:
- Imprime complementos (com preço / sem preço / não imprime preço)
- Não imprime detalhes (só total)
- Não imprime nomes dos campos

## ESC sequences (configuráveis por impressora)

OpenRest mantém compatibilidade com as configurações de ESC sequences do WinREST:

- Inicialização
- Reset / Limpar
- Vermelho ON/OFF
- Sublinhado ON/OFF
- Tamanho duplo ON/OFF
- Centrar / esquerdo / direito
- Corte papel
- Abrir gaveta (par 1 e par 2)
- Sino sonoro / Sequência de senha EAN-13
- Estado da impressora (status query)
- Codepage
- Linhas a saltar
- Linhas por página (para impressoras de cartões)

Cada impressora tem ficheiro de driver com as suas sequências (formato JSON em OpenRest, equivalente a `WRSTSC00.70?` no WinREST).
