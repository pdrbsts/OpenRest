# OpenRest — Módulo Caixa

> Gestão de movimentos monetários: aberturas, transacções, transferências, encerramentos, apuramentos, relatórios. A hierarquia Dia/Turno/Caixa/Sessão é a coluna do modelo.

## 1. Filosofia operacional

O programa **assume conhecer todo o movimento de dinheiro do estabelecimento** desde a abertura do dia até ao fecho. Por isso, todos os pagamentos, compras, vales, fundos, retiradas, depósitos e empréstimos devem ser registados.

### Hierarquia

```
Dia
└── Caixa (1..N caixas físicas)
    └── Turno (1..N por caixa)
        └── Sessão de Empregado (1..N por turno)
            └── Bolsa (opcional, para empregados que não trabalham directamente para a caixa)
```

### Trabalho directo vs Bolsa

- **Empregado directo**: fecha mesa → dinheiro entra imediatamente na caixa.
- **Empregado com bolsa**: fecha mesa → dinheiro entra na bolsa. No fecho da sessão, a bolsa é transferida para a caixa.

**Implicação prática**: na passagem de turno, empregados directos não podem fechar mesas durante a transição. Empregados com bolsa podem.

## 2. Janela da Caixa

Estrutura em 6 secções:

| Secção | Acções |
|---|---|
| **Registos** | Consulta, Estatísticas |
| **Aberturas** | Abrir Caixa, Abrir Sessão |
| **Movimentos** | Pagamento CC (entrada), Fundo (entrada), Empréstimo (entrada), Retirada (saída), Envelopes, Compras (saída), Vale (saída) |
| **Transferências** | Vendas Activas (entre empregados), Turno |
| **Encerramentos** | Fechar Sessão, Fechar Caixa, Fechar Dia, Fecho Financeiro |
| **Apuramentos** | Sessão, Caixa, Turno, Dia, Relatórios |

## 3. Registos / Consulta

### 3.1 Consulta de Registos

Lista de documentos do dia (ou anteriores). Filtros: empregado, mesa, hora, nº documento, local.

Operações:
- **Reimprimir** (2ª via, marca "Duplicado") — exige permissão
- **Imprimir listagem** (com ou sem detalhes)
- **Detalhes de processo** — linha a linha (tipo, qt, preço, desconto, hora, empregado)
- **Estornar** — emite estorno (com/sem desperdício)
- **Editar** — alterar empregado de abertura/fecho, método de pagamento

Campos do cabeçalho mostrados:
- Dados do cabeçalho (nº, data, série, ATCUD)
- Empregado de abertura / fecho
- Método de pagamento
- Total

### 3.2 Estatísticas

Gráficos disponíveis (configuráveis em estilo de barras, cores, largura):
- **Vendas por hora**
- **Clientes por hora**
- **Custos de Pessoal** (precisa custos/hora configurados)
- **Custos/Vendas** (margem)
- **Vendas/Clientes**

Períodos: 60, 30 ou 15 minutos.

## 4. Aberturas

### 4.1 Abertura de Caixa

Abre uma `caixa_dia` com:
- Saldo transportado (= saldo de transporte do dia anterior)
- Verificação se data do programa == data do computador (aviso caso contrário)

Pode ser automática no arranque (`abre_no_arranque`).

### 4.2 Abertura de Sessão

Para um empregado, definir:
- Caixa a que vai trabalhar (Automático = caixa configurada para o comando)
- Com bolsa? Fundo de maneio para a bolsa?
- Comando associado?

Imprime documento de abertura (configurável).

## 5. Movimentos

### 5.1 Pagamento de Conta-Corrente (entrada)

Cliente paga directamente na caixa, aumentando saldo positivo da CC.

Cliente associado: pode pagar a conta de todos os associados de uma vez. Recibo descrimina cada cliente.

Procedimento:
1. Seleccionar cliente (com pesquisa avançada por nome/cartão/zona)
2. Introduzir valor
3. Método de pagamento
4. Enter → emite recibo

### 5.2 Fundo (entrada)

Reforço do fundo de maneio. Aumenta valor em caixa.

Procedimento:
1. Caixa destino
2. Valor
3. Método
4. Observação
5. Enter

### 5.3 Empréstimo (entrada)

Empregado empresta dinheiro à caixa → débito na CC do empregado.

### 5.4 Retirada (saída)

Para cofre, depósito bancário, etc. Não pode exceder valor em caixa (excepto se fecho cego).

### 5.5 Envelopes

Substitui Retirada quando módulo Fecho Financeiro está activo.

Operação: regista ensacamento. O dinheiro continua contabilizado como em caixa até ao fecho do dia.

Campos:
- Código do envelope
- Valor
- Data
- Empregado responsável

### 5.6 Compras (saída)

Saída para aquisições. Mesma estrutura que Retirada. Em Fecho Financeiro, exporta com identificação fiscal.

### 5.7 Vale (saída)

Adiantamento ao empregado. Saída de caixa + débito na CC do empregado.

## 6. Transferências

### 6.1 Vendas Activas

Quando se fecha sessão a um empregado com mesas abertas, pode-se transferir para outro empregado.

Por defeito, o empregado origem **perde direitos a comissões** das mesas transferidas.

Excepção: se `vendas_activas_mantem_empregado=true` na caixa, as comissões ficam com quem efectuou os pedidos.

### 6.2 Turno

Passa uma caixa para o turno seguinte (= fecho + abertura sem fechar sessões). Permite mudança de turno progressiva (caixa a caixa) com a casa em funcionamento.

## 7. Relógio de Ponto (Timeclock)

Módulo licenciado, independente do registo de sessões.

- Empregado marca entrada/saída independentemente da sessão (entra antes da abertura do dia, sai depois do fecho).
- Listagem de assiduidade nos relatórios.

## 8. Encerramentos

### 8.1 Fecho de Sessão

Encerra sessão de empregado:
- Se com bolsa: dinheiro da bolsa → caixa
- Se `acerta_cc_no_fecho`: ajustes automáticos por comissões, ofertas, consumo
- Imprime apuramento de sessão (se configurado)
- Selecciona comissão fixa aplicável (se há)

Pré-requisito: todas as mesas do empregado fechadas, transferidas, ou aceites como abertas (configuração especial).

Mesa pode ser transferida para o dia seguinte mantendo agregação ao mesmo empregado.

### 8.2 Fecho de Caixa

Encerra caixa:
- Define saldo a transportar
- Pré-requisito: todas as sessões para essa caixa fechadas
- Imprime apuramento de caixa (auto)
- Pode ser reaberta num turno acima

Se módulo Fecho Financeiro: oferece consulta de envelopes.

Se `introduz_valor_em_caixa=true`: pede contagem real antes do apuramento teórico (fecho cego).

### 8.3 Fecho de Dia

Operação terminal:
1. Garante todas as caixas fechadas
2. Pergunta ajuste de data (dia seguinte ou data manual)
3. Imprime apuramento de dia (auto)
4. Exporta todos os ficheiros do dia → directórios `export/` (e `modem/` se aplicável)
5. Aplica regras de `folga_semanal` (avança N dias se aplicável)
6. Faz arquivo (`gera_arquivo_documentos=true`): cria ficheiro `F<aammdd><zona>` com vendas, dinheiro e facturas

Data lógica:
- Pode diferir da data do computador (turnos pós-meia-noite, férias, etc.)
- Aviso se desfasamento

### 8.4 Fecho Financeiro

Módulo adicional com:

#### Mapa Económico
- Circuito de valores
- Locais
- Tipos de vendas

#### Mapa Financeiro
- Métodos de pagamento que originaram os valores
- Depósitos
- Outros movimentos (pagamento de serviços)

Antes do fecho do dia (e após fecho de caixa):
1. Ajustar recebimentos por método de pagamento
2. Registar compras/despesas (documentos, valores, IVA)
3. Configurar depósitos a efectuar

## 9. Apuramentos

Mapas impressos.

### 9.1 Apuramento de Sessão

Por (data, empregado, sessão). Pode descriminar:
- Anulações
- Ofertas
- Consumo
- Comissões
- Factores de conversão
- Vendas por: nada, famílias, fam/sub-fam, tudo

Empregado "Todos" imprime para todos.

Comissões em grupo podem incluir movimentação completa durante a sessão.

### 9.2 Apuramento de Caixa

Por (data, turno, caixa). Descrimina:
- Totais parciais (entradas vs saídas)
- Vendas por método de pagamento
- Factores de conversão

Após fecho do dia, valores por método são zerados (compensação).

### 9.3 Apuramento de Turno

Por (data, turno). Descrimina movimentos ou só totais.

### 9.4 Apuramento de Dia

Por data. Descrimina:
- Famílias, sub-famílias, artigos, locais
- Descontos e ofertas
- Vendas horárias (gráfico)
- Numeração de documentos (primeiro/último por tipo)

## 10. Relatórios (módulo Reports)

Lista de relatórios disponíveis com configuração persistível por relatório:

- **Assiduidade** (precisa timeclock) — marcações de empregados
- **Vendas por Turno** — quantidades por turno e família
- **Apuramento de IVA** — vendas por taxa, em 3 níveis: artigo, sub-fam, fam
- **Consulta de Registos** — documentos do intervalo, totalizador
- **Refeições** — total com/sem associação
- **Descontos a Clientes** — descontos por cliente
- **Relatório Diário** — totais por tipo doc, contadores, vendas por artigo/local, IVA
- **Saldo Clientes** — listagem com débito/crédito/saldo
- **Vendas Negativas** — estornos, ofertas, descontos, anulações

### Configuração

Cada relatório suporta configurações múltiplas (alternativas). Cada configuração:
- Imprime no fecho de dia?
- Imprime no fecho de turno?
- Imprime no fecho de caixa?
- Imprime no fecho de sessão?
- Mostra botão de atalho na janela principal (máx 5)
- Parâmetros específicos (intervalos de datas, agrupamentos, níveis)

## 11. Configurações especiais de Caixa

- **Abre no arranque** — caixas e sessões abrem automaticamente
- **Força abertura no arranque** — janela de cancelamento não aparece
- **Apura em automático** — apuramentos no fecho
- **Introduz valor em caixa** — fecho com contagem real
- **Imprime vendas horárias** — gráfico no apuramento
- **Fecho directo** — fecha tudo automaticamente confirmando só mesas abertas
- **Vendas Activas mantêm empregado de pedido**
- **Não permitir estornos após fecho de sessão**
- **Taxa de Conversão para Euro** — legacy (usado em arquivos antigos)
- **Máximo valor de oferta isenta de IVA** — para ofertas grandes onde IVA é cobrado
- **Valores para troco rápido** — botões Take-Away

## 12. Movimentos de Caixa — visão consolidada

| Tipo | Direcção | Origem | Destino |
|---|---|---|---|
| Venda directa | Entrada | Cliente | Caixa |
| Venda via bolsa | Entrada | Cliente | Bolsa |
| Fundo de Maneio | Entrada | Externo | Caixa |
| Empréstimo | Entrada | Empregado | Caixa (cria CC débito) |
| Pagamento CC | Entrada | Cliente | Caixa (liquida CC) |
| Transferência Bolsa→Caixa | Interna | Bolsa | Caixa |
| Transferência Turno | Interna | Caixa anterior | Caixa actual |
| Retirada | Saída | Caixa | Cofre/Banco |
| Compra | Saída | Caixa | Fornecedor |
| Vale | Saída | Caixa | Empregado (cria CC) |
| Depósito (envelope) | Informativo | Caixa | Envelope (até fecho) |
| Ajuste sessão | Misto | Caixa | CC empregado (ou inverso) |
| Fundo Maneio Saída | Saída | Caixa | Bolsa (na abertura) |

Cada movimento gera entrada em `movimento_caixa` com `valor` signed.

## 13. Documentos de Caixa

Configuráveis para serem suprimidos (alguns):
- Passagem de dinheiro da bolsa para a caixa (no fecho)
- Validação de cliente (recibo CC)
