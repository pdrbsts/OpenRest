# OpenRest — Módulo POS / Operação

> Módulo central. Tudo o resto suporta este. Define os ecrãs de venda, a manipulação de mesas, pedidos, recebimentos e movimentações de venda no chão de loja.

## 1. Ecrã principal

A *home* do POS apresenta 6 grupos de operações (configurável):

| Grupo | Operação |
|---|---|
| **Mesas** | Consulta de estado das mesas, totais, mesas abertas, reservas, lista negra |
| **Pedidos** | Registar pedidos (vários modos) |
| **Ficheiros** | Tabelas, fichas, promoções, happy hour, páginas rápidas, pratos do dia |
| **Caixa** | Aberturas, movimentos, fechos, apuramentos, relatórios |
| **Plug-ins** | Módulos extensíveis |
| **Sistema** | Operações de sistema (redirecções, mensagens, listagem hardware) |

Cada botão pode ter `Activo`, `Pede Utilizador`, `Pede Código` configurados por posto. Tempo de inactividade (`tempo_volta_inicial`) leva ao ecrã inicial.

### Zonas do ecrã

1. **Zona de Selecção de Opções** — coluna lateral com os 6 botões.
2. **Zona de Trabalho** — área principal onde aparecem janelas modais ou ecrãs de operação.
3. **Zona de Informação** — barra com data/hora, indicador de rede, antena, caps lock, mensagens (manutenção, erro, happy hour).
4. **Zona de Retorno** — equivalente a Escape; logo da casa abre toolbar (abrir gaveta, alternar moeda, calibrar touch, comutar consola).

## 2. Botão "Mesas"

Mostra o mapa de mesas, com paginação se exceder o ecrã. Cada botão de mesa apresenta:

- Número/nome da mesa
- Valor total acumulado
- Cor de semáforo:
  - **Verde** — livre, já foi usada hoje
  - **Vermelha** — ocupada, com sub-total impresso
  - **Azul** — ocupada
  - **Aloquete** — reservada

Premir uma mesa abre **Janela de Consulta de Mesa**:

- Lista de artigos pedidos (código, qt, preço, hora, empregado)
- Hora de abertura
- Empregados intervenientes
- Nº de pessoas
- Empregado actual
- Total actual
- Acumulado diário (tempo, pessoas, utilizações, facturação)
- Dados de reserva, se aplicável

## 3. Botão "Totais"

Janela com totais por mesa/cliente:

| Métrica | Recebido | Pendente | Total |
|---|---|---|---|
| Nº mesas | X | Y | X+Y |
| Clientes | … | … | … |
| Vendas s/ IVA | … | … | … |
| Vendas c/ IVA | … | … | … |

## 4. Botão "Mesas Abertas"

Listagem (filtrável por empregado) de mesas em aberto. Permite impressão de listagem rápida (útil em corte de energia com UPS).

## 5. Botão "Reservas"

### 5.1 Cronograma

Vista temporal (mês ou dia) mostrando distribuição das reservas. Reservas mensais aparecem como traço azul; diárias mostram intervalos.

### 5.2 Editor de Reservas

CRUD de reservas com filtro.

Campos:
- Dia inicial, dia final
- Hora inicial, hora final
- Cliente (escolha ou criação rápida com nome+telefone)
- Mesas (lista)
- Nº pessoas

Pesquisa por nome, telefone, código, zona.

## 6. Botão "Lista Negra"

CRUD de mesas/cartões bloqueados. Suporta exclusão individual e em bloco com intervalo `De..Até`. Aviso visível quando uma mesa em uso é bloqueada.

## 7. Botão "Pedidos"

Fluxo: **escolher empregado → escolher mesa → ecrã de pedidos**.

### 7.1 Escolher empregado

Botões com todos os empregados activos (paginados). Identificação alternativa por cartão (banda magnética/RFID/código barras) ou Led ID.

### 7.2 Escolher mesa

Janela com mesas organizadas por local:
- Cor escura = mesa de outro empregado, sem acesso
- Caixa de selecção do local
- Sub-grupos `1..50, 51..100, …` se forem muitas

Pode comutar para **selecção gráfica** (mapa de mesas), em que se clica numa zona da imagem da sala.

Botões adicionais:
- **Despacho** — só aparece se houver locais de delivery
- **Consumo** — só aparece se o empregado tiver `pedidos.consumo_proprio`

### 7.3 Ecrã de Pedidos (Touch-Screen)

Layout em 5 zonas:

1. **Lista do pedido em construção** (linhas ainda não submetidas)
2. **Lista do consumo actual** (linhas já pedidas)
3. **Coluna de famílias** (botões com cores)
4. **Coluna de sub-famílias**
5. **Grelha de artigos** (paginada com setas)

Acções:
- Premir artigo → adiciona 1 unidade ao pedido em construção
- `Qt + número + Artigo` → adiciona N unidades
- `Pedir` → submete o pedido, imprime nas zonas configuradas, move linhas para "consumo actual"
- `Cancelar` → remove linhas do pedido em construção (antes de pedir)
- `Anular` → remove linhas do consumo (após pedir), com escolha "com/sem desperdício"
- `Sub-total` → imprime consulta de mesa
- `Recebimento` → janela de fecho
- `Transferência` → mover artigos para outra mesa
- `Oferta` → desconto sobre artigos seleccionados (% ou valor absoluto)
- `Pedidos por código` → introduzir código via teclado virtual
- Setas de navegação entre artigos

Se houver balança ligada, peso entra automaticamente no campo Qt.
Se houver apenas 1 família e 1 sub-família com <56 artigos, modo "uma família": toda a página é grelha de artigos.

### 7.4 Modos especiais de Pedidos

#### Take-Away
- Cinco teclas de troco rápido (valores configuráveis)
- Visualização do troco em tempo real
- Botão para escran normal (para casos onde precisa ofertas/CC)
- Fecha a mesa ao premir Pedir (gera VD imediato)

#### Take-Away Seguro
- Igual ao Take-Away mas:
  - Empregado e cliente não vêem total até confirmação (anti-fraude)
  - Permite registo ao balcão
  - Mesas não encerram ao pedir (clica 2x para fechar)

#### PUB
- Pedidos entram todos na "primeira mesa"
- Transferir pode atribuir nome de cliente à mesa destino
- Alocação dinâmica
- Conta volta a ficar disponível em modo "Em Espera"

#### Delivery
- Botão de chamada (telefone) abre janela de identificação
- Pesquisa rápida por nº telefone (com sintaxe especial), morada, nome
- Histórico dos 3 últimos pedidos do cliente, com botão "copiar pedido"
- Observações específicas: pedido, factura, cliente, morada
- Requer configuração de **Áreas de Entrega** (zonas geográficas com respectiva taxa de entrega) e de **Entregadores** (lista de nomes dos "motoboys", que podem ser recursos externos em vez de empregados regulares).
- Janela de **Despacho**: atribui pedidos a entregadores, imprime documento, mede tempos de preparação e entrega

#### Consumo Próprio
- Mesa especial automática por empregado (tipicamente para o consumo dos próprios funcionários)
- Normalmente existem produtos gratuitos e outros pagos a preço reduzido
- O local associado a este modo usa habitualmente uma tabela de preços (PVP) diferente dos restantes
- Restrições: não fecha mesa avulsa (fecha com fecho de sessão), não imprime consulta, não transfere, oferta pelo `base_ofertas`
- Valor consumido vs valor a pagar diferenciados (PVP × percentagem)

#### Pedidos via Teclado
- Ecrã em modo texto para uso massivo com teclado físico/programável
- Workflow: `nº mesa + Mesa | qt + Quantidade | código + Artigo | … | Pedir`
- Suporta múltiplas teclas com sequências configuradas
- Correção via teclas de cursor + introdução de novo valor

## 8. Recebimento (Fecho de Mesa)

Janela principal:

```
+------------------------------------------+
| Total na Mesa: 23,50 €                   |
| Recebido: 0,00 €                          |
| Em Falta: 23,50 €                         |
|                                          |
| Métodos de pagamento (botões)            |
|                                          |
| Cliente: [Anónimo]    Nº Pessoas: 2      |
|                                          |
| [Imprime]  [Imprime N.Desc.]  [Avançado] |
| [OK]       [Cancelar]                    |
+------------------------------------------+
```

Acessível **sem** permissão para fechar mesas (só para alterar cliente/nº pessoas).

### 8.1 Identificação de cliente

Janela com filtros: nome (parcial), nº cartão, código, zona. Cartão lido directamente no campo de leitura. Identificação eventual (PT) — janela de NIF+nome para cliente sem ficha.

NIF validado por algoritmo (avisa mas não impede).

### 8.2 Janela Avançada (Recebimento Múltiplo)

Suporta divisão por método de pagamento e por documento.

Campos:
- Valor — valor parcial
- Nº Pessoas
- Total da mesa
- Total recebido
- Forma de pagamento (botões)
- Descrição
- **Parcial** — pagar só alguns artigos
- **Múltipla** — dividir o consumo em N facturas

#### Modelo de dados (rodapés de documento)

Um documento pode ter 1..N **rodapés de pagamento** (tabela `payments`), cada um com `payment_method_id`, `amount` em cêntimos e `descricao` opcional. O somatório dos rodapés cobre o `Document.total`; quando a soma excede o total, o excedente é gravado em `Document.troco_cents`. O fecho fiscal e o registo dos rodapés acontecem numa única transacção (tudo-ou-nada).

Fluxos:

#### Pagamento exacto múltiplo
1. Introduzir valor → premir método 1 → introduzir resto → premir método 2 → …
2. OK só fica activo quando totaliza.

> **Implementado** em v0.5.0-alpha. Endpoint `POST /api/documents/:id/close` aceita `{ payments: [{ payment_method_id, amount, descricao? }] }` e valida `sum >= total`.

#### Pagamento com troco
- Inserir valor maior que total → premir método (Numerário)
- Sistema calcula troco

> **Implementado** — `troco = sum - total` quando positivo é guardado em `documents.troco_cents` e impresso no recibo.

#### Pagamento Parcial
- Selecciona artigos → premir Pagamento Parcial → OK
- Gera factura/VD apenas com esses artigos
- Mesa continua aberta

> **Implementado** em v0.5.0-alpha. Endpoint `POST /api/documents/:id/partial-close` body `{ line_ids, payments }`: cria um documento-filho (`parent_document_id` aponta para o pai), move as linhas seleccionadas, fecha o filho fiscalmente. O pai mantém-se aberto com o saldo. Restrições: linhas têm de estar pedidas (`pedida_em IS NOT NULL`) e não anuladas. O filho **não recebe `table_id`** — só o pai detém a mesa, garantindo que o fecho fiscal do filho não a liberta.

#### Divisão de Conta
- Introduz nº contas → janela de divisão (manual ou automática)
- Setas de transferência de artigos entre contas
- Botão "Divisão Automática" tenta dividir mantendo o total da conta o mais próximo possível do pretendido

> **Implementado** em v0.5.0-alpha. Endpoint `POST /api/documents/:id/split` aceita três modos via discriminator `mode`:
>
> | Modo | Body | Comportamento |
> |---|---|---|
> | `lines` | `{ mode: "lines", assignments: [{ line_ids: […] }, …] }` | Cada linha vai inteira para uma só conta. Totais por conta podem diferir. |
> | `quantidades` | `{ mode: "quantidades", num_accounts: N }` | Cada linha elegível é dividida fraccionariamente em N partes (cada filho recebe `qty_milli/N` e `total/N`). Cêntimos residuais ficam no pai → **todas as contas têm exactamente o mesmo total**. |
> | `encaixar` | `{ mode: "encaixar", assignments: [{ line_ids: […] }, …] }` | Linhas atribuídas à conta "primária" ficam intactas; sistema gera linhas de compensação positivas/negativas (com `descricao="Compensação <artigo>"`) para igualar totais. Cada conta = `total_elegível/N`. |
>
> O pai fica **operacionalmente fechado** (`is_closed=true` sem dados fiscais) e a mesa é libertada quando todas as linhas elegíveis foram movidas; cada filho corre na cadeia fiscal de forma independente. `GET /api/documents/:id/split/auto-plan?num_accounts=N` devolve uma sugestão greedy/LPT (Longest Processing Time first) para o modo `lines`/`encaixar`.

##### Modelo pai/filho

```
Document (pai, table_id=T, parent_document_id=NULL)
   ├─ DocumentDetail …  ← linhas originais
   ↓ split / partial-close move ou divide linhas
Document (filho, table_id=NULL, parent_document_id=pai.id, hash/ATCUD próprios)
   └─ DocumentDetail (referenciam o filho via UPDATE document_id ou novas linhas)
```

Invariantes:
* Pai nunca recebe número fiscal, ATCUD ou hash.
* Filhos têm `table_id IS NULL` para que `finalize_document_fiscal` não liberte a mesa do pai prematuramente.
* `mesa_estado.subtotal_actual` é decrementado pelo total movido quando linhas saem do pai.
* Uma divisão exige no mínimo 2 linhas elegíveis (pedidas, não anuladas).

##### Modelo de linhas fraccionárias

Para suportar os modos **Quantidades** e **Encaixar**, as linhas têm:

| Campo | Tipo | Semântica |
|---|---|---|
| `qty` | i32 | Quantidade inteira (legado/exibição quando inteiro). |
| `qty_milli` | i64 | Quantidade em milli-unidades (1.000 = 1 unidade). Pode ser **fraccionária** (e.g., 500 = 0,5) ou **negativa** (compensações Encaixar). |
| `total` | i64 cêntimos | Pode ser negativo (compensações). |
| `descricao` | text? | Sobrepõe o nome do artigo no recibo. Usado para rotular linhas de compensação como "Compensação Café". |

Cálculo de IVA: cada linha contribui para o bucket de IVA do seu `article_id`. Linhas de compensação positivas e negativas referenciam o mesmo artigo e cancelam-se entre contas, preservando o total de IVA do parente.

### 8.3 Conta Corrente do Cliente

- Selecciona cliente → método Conta Corrente
- Se ultrapassa `limite_credito`, exige permissão superior (ou bloqueia)
- Em cliente Associado, pagamento pode ser feito à associação (liquidando todos os associados)

## 9. Transferência

Janela com lista de artigos da mesa origem.

Operações:
- Seleccionar linha → premir botão "Transferir" → escolher mesa destino → OK
- Transferência total (todos os artigos)
- Para PUB: ao transferir, pode-se atribuir nome à mesa destino

Se `indica_pessoas_obrigatorio` na origem e destino, o sistema também redistribui pessoas.

## 10. Anulação

Janela idêntica à de transferência mas para anular. Exige permissão de empregado `pedidos.anula`. Se a conta (sub-total) já tiver sido impressa, exige adicionalmente a permissão `pedidos.anula.com_conta_impressa`.

Campos:
- Lista de artigos
- Qt a anular (default 1)
- Botão balança (para anular por peso exacto)
- Caixa de selecção "c/ Desperdício"

Confirmação imprime pedido de anulação na zona original.

## 11. Cancelar

Remove linhas do pedido em construção (antes de pedir). Esta operação exige a permissão de empregado `pedidos.cancelar`. Não imprime nada em zonas de preparação. O seu registo em base de dados (numa tabela de cancelamentos/auditoria) é opcional e configurável de acordo com as preferências do cliente, ao contrário da Anulação que é sempre registada.

## 12. Oferta (Desconto)

Janela com lista de artigos. Seleccionar + introduzir % ou valor absoluto. Aplica-se às linhas seleccionadas.

Limites pelo `base_ofertas` do empregado.

## 13. Pedidos por Código (sem teclado físico)

Janela de pedido com:
- Campo de código
- Campo de quantidade
- Caixa de rolamento de artigo (para escolha visual)
- Botão OK

Útil quando não há teclado físico mas só touchscreen.

## 14. Sub-Total

Imprime consulta de mesa na zona de sub-totais. Permitido N vezes. Se configurado, bloqueia anulações após sub-total impresso.

## 15. Botões adicionais (via configuração)

Cada local pode ter botões adicionais visíveis no ecrã de pedidos:
- Página Rápida (artigos mais vendidos)
- Selecção Gráfica (mapa de mesas)
- Despacho (delivery)
- Consumo (próprio)
- Conta com vs sem detalhes
- Imprime factura (forçar VD nominativa)

## 16. Sequência tipo de operação

```
[Início do dia: Caixa aberta automaticamente ou manualmente]
  └── Sessão abre (empregado entra)
       └── Mesa abre (selecciona/cria)
            ├── Pedido(s) — 1..N pedidos parciais
            ├── Sub-total impresso, alterações, ofertas
            ├── Identificação de cliente
            └── Recebimento
                 ├── Único método
                 ├── Múltiplos métodos
                 ├── Parcial
                 └── Divisão de conta
       └── Mesa fecha
  └── Sessão fecha (apuramento de sessão)
[Fim do dia: Caixa fecha → Apuramento → Fecho de Dia]
```

## 17. Operações por terminal rádio (comando)

Suporta as operações principais:
- Login do empregado por código + password
- Identificar mesa
- Pedir artigos com `código + Artigo` ou via tabelas
- Conta (imprime e fecha)
- Anular
- Pagamento parcial
- Pedir nº pessoas e/ou forma de pagamento (configurável)
- Apuramentos

Mensagens de erro são também enviadas ao display do comando.

## 18. Mensagens de erro

Configuráveis para serem também impressas:
- **Movimentação**: erros normais durante pedidos ("Mesa XX não existe")
- **Sessão**: erros relacionados com abrir/fechar/mudar sessão

## 19. Sincronização com Ecrã de Cliente

Quando um posto está configurado com um ecrã de cliente secundário (Customer Display), o comportamento operacional reflete-se nesse ecrã de acordo com as seguintes regras de sincronização:

- **Atualização em Tempo Real**: Cada linha introduzida e validada pelo operador no ecrã principal (após seleção de artigo e quantidade) surge imediatamente na área de "Conta Atual" do ecrã de cliente, não sendo necessário aguardar pelo "Sub-total".
- **Ocultação de Dados (Privacidade Operacional)**: Operações sensíveis ou exclusivas do operador (como os diálogos de Anular, Cancelar, ou Transferir) não apresentam a janela modal ao cliente. O ecrã do cliente reflete apenas o resultado final na lista de consumo (ex: a linha é removida da lista ou riscada e o total é reduzido de imediato).
- **Transição de Estados**:
  - **Mesa Livre / Terminal Bloqueado**: O ecrã de cliente entra em modo "Standby", ocultando a área da conta e expandindo a Zona de Marketing para ecrã inteiro.
  - **Ecrã de Recebimento**: Assim que o operador inicia o pagamento, o ecrã de cliente foca-se no Total a Pagar e nos métodos de pagamento selecionados.
  - **Pós-Pagamento**: O ecrã destaca o Valor Recebido e o Troco. Esta informação mantém-se visível por um período de tempo configurável (ex: 10 segundos) após a confirmação, antes de o ecrã regressar ao modo Standby.
