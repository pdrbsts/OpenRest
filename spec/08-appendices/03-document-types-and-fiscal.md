# OpenRest — Tipos de Documento e Cadeia Fiscal

> Anexo técnico com a taxonomia completa dos documentos, suas regras, transições e relações.

## Catálogo de Tipos

| Código | Designação | Natureza | Fiscal | Numerado? |
|---|---|---|---|---|
| `FT` | Factura | Documento de venda nominativo | ✓ | Série |
| `FR` | Factura-Recibo | Factura + recibo no mesmo documento | ✓ | Série |
| `FS` | Factura Simplificada (PT) | Equivalente moderno à "Venda a Dinheiro" | ✓ | Série |
| `NC` | Nota de Crédito | Estorno de factura/FR/FS | ✓ | Série |
| `ND` | Nota de Débito | Cobrança adicional | ✓ | Série |
| `RC` | Recibo | Quitação de conta-corrente | ✓ | Série |
| `GT` | Guia de Transporte | Para deliveries (em alguns países) | ✓ | Série |
| `GR` | Guia de Remessa | Movimento de stock entre lojas | ✓ | Série |
| `DC` | Documento de Conferência (Consulta de Mesa) | Não-fiscal | ✗ | Sequencial interno |
| `OS` | Orçamento / Cotação | Não-fiscal | ✗ | Sequencial interno |
| `PD` | Pedido de Cozinha | Não-fiscal | ✗ | Sequencial interno |
| `SE` | Senha (refeição) | Não-fiscal | ✗ | Sequencial interno + UID |
| `AP` | Apuramento (sessão/caixa/turno/dia) | Não-fiscal | ✗ | Sequencial interno |
| `RB` | Recibo de Bolsa (movimento empregado) | Não-fiscal | ✗ | Sequencial interno |
| `EX` | Documento de Facturação Externa | Não-fiscal (emitido por outro sistema) | — | Externo |

## Estados do Documento

```
[draft]  →  [emitido]  →  [pago]
                ↘
                  [anulado_por_NC]
```

- **draft**: ainda não emitido (consulta de mesa)
- **emitido**: nº de série atribuído, imutável
- **pago**: recibo emitido (em FR é simultâneo)
- **anulado_por_NC**: existe NC que o anula; cadeia fiscal mantém o original

## Sequências e Numeração

### Regras

- **Sequencial estrita** dentro da série
- Sem gaps permitidos
- Não reutilização
- Comunicada à AT (PT)
- ATCUD = `CodigoValidacao-NumeroDoc`

### Estrutura de série

```
serie = {
  tipo_documento,
  prefixo (ex: "FS", "FT", "FR_2024"),
  ano,
  proximo_numero,
  ATCUD_validacao,  // devolvido pela AT
  estado: activa | suspensa | encerrada
}
```

### Gaps e falhas

Quando há falha técnica que provoca gap:
1. Sistema regista evento `serie.gap_detected`
2. Audit log mantém razão
3. Documento "ficticio" com tipo "ANULADO_FALHA_TECNICA" pode ser emitido (configurável)
4. Auditoria fiscal deve ser informada

## Cadeia de Hash (PT)

Cada documento de venda assinado com:

```
content = data_doc + "Z" + datetime_emissao + ";" + nome_doc + ";" + valor_total + ";" + hash_anterior
hash = sign(content, chave_privada_RSA)  // PKCS#1 v1.5 SHA-1
hash_impresso = hash[0] + hash[10] + hash[20] + hash[30]  // 4 chars
```

`hash_anterior` é o hash do **documento anterior na mesma série**. Primeiro doc da cadeia: `hash_anterior = ""`.

## QR Code (PT)

Payload em pares `chave:valor` separados por `*`:

```
A:<NIFEmit>*B:<NIFCli ou "999999990">*C:<paisCli>*D:<TipoDoc>*E:<Estado=N|A|F|R>
*F:<DataDoc YYYYMMDD>*G:<NumDoc>*H:<ATCUD>
*I1:<paisIVA>*I2:<BaseIsento>*I3:<Base6>*I4:<IVA6>*I5:<Base13>*I6:<IVA13>*I7:<Base23>*I8:<IVA23>
*N:<TotalIVA>*O:<TotalComIVA>*Q:<4charsHash>*R:<SoftCertNum>
```

## ATCUD

Sigla: Código Único de Documento.
Formato: `<CodigoValidacao>-<NumeroDoc>`.
Exemplo: `JFG3-12345`.

Obrigatório em todos os documentos fiscais (PT) desde 2023.

`CodigoValidacao` devolvido pela AT após comunicação da série.

## Imutabilidade

- Documento emitido **nunca** é alterado
- Para correcção: NC referenciando o original
- Reimpressão: nova cópia idêntica + marca "DUPLICADO"
- Anulação não muda dados; estado passa a `anulado_por_NC`

## Eventos relacionados

```
documento.emitido            // novo documento criado
documento.reimpresso         // 2ª via solicitada
documento.estornado          // NC emitida que o anula
documento.alterado_metadata  // alteração de dados não fiscais (ex: anotação)
documento.pago               // pagamento associado
documento.sincronizado       // enviado para central
documento.comunicado_AT      // comunicado à AT
```

## Relações entre documentos

```
factura/VD/FR/FS ← (nota_credito anula)
documento_pedido_cozinha → (associa) → documento_principal
recibo → (paga) → factura
senha → (origem) → reserva
factura ← (origem) → recibo (quando separados)
```

## Validação fiscal antes de emitir

OpenRest valida antes de emitir cada documento:

1. NIF do cliente válido (se obrigatório)
2. Designação social e NIF da casa preenchidos
3. Total > 0
4. Pelo menos uma linha de detalhe (ou doc tipo recibo)
5. Cadeia de hash íntegra (hash anterior existe)
6. Numeração sequencial (próximo número correcto)
7. Data lógica plausível
8. Empregado com permissões
9. Caixa aberta (para documentos de venda)
10. Mesa associada existe / em estado correcto

Falha qualquer validação → bloqueio + log + alerta.

## Modelo de Estorno

```
1. Doc original "FS 2024/1234" emitido com total €10
2. Cliente reclama 1 item; emite-se "NC 2024/5" referenciando "FS 2024/1234"
   - NC com valor +€2 (item devolvido)
3. Estado de FS 2024/1234 muda para "parcialmente_estornado"
4. Pode emitir-se nova NC ou nova FS para correcção

Se for anulação total:
1. Doc "FS 2024/1234" emitido com €10
2. Emite-se "NC 2024/6" com valor total
3. Estado de FS muda para "anulado_por_NC"
4. Em reports: doc original aparece a zero
```

## Restrições por país

### Portugal
- IVA obrigatório em detalhe
- ATCUD + QR Code + Hash
- SAF-T mensal
- Comunicação de séries

### Espanha
- VeriFactu (em curso)
- NIF para B2B

### Brasil
- NFC-e ou SAT
- ECF (legado)
- Modelo fiscal específico (CFOP, CSOSN, NCM)

### Suécia
- Box / Control Box obrigatória

### Genérico
- IVA pode ou não aparecer
- Hash pode ou não ser obrigatório

Configurável via `pais_locale`.

## Reimpressão (2ª via)

- Marca "DUPLICADO"
- Não conta para numeração
- Mantém NIF original
- Conta para audit (reimpressão é evento)
- Em PT: limitada a casos válidos (perda do original)

## Modelos de Documento

Mantidos em `documento_template`. Cada tipo tem 1..N variantes (`C/R` 1-9 em alguns tipos).

Template tem:
- Cabeçalho (texto com flags)
- Rodapé (texto com flags)
- Configuração de detalhes (campos, posições, tamanhos)
- Opções (imprime complementos, não imprime detalhes, …)

## Configuração por país (resumo)

```yaml
pais: PT
  iva_em_detalhe_obrigatorio: true
  atcud_obrigatorio: true
  qr_code_obrigatorio: true
  hash_assinatura: PT_363_2010
  saft_export: mensal
  comunicacao_at: true
  nif_obrigatorio_acima_de: 1000.00 EUR
  formato_data: DD/MM/YYYY
  separador_decimal: ","
  moeda: EUR

pais: BR
  iva_em_detalhe_obrigatorio: false
  emissao_fiscal: NFCe | SAT | ECF
  ...
```

## Numeração interna vs fiscal

OpenRest mantém 2 numerações:

1. **Numeração fiscal** — gestionada pelas séries (visível ao cliente, imutável)
2. **Numeração interna** — UUID + sequencial técnico (para operação)

Em UI mostra-se a fiscal; em logs e API a interna está sempre disponível.
