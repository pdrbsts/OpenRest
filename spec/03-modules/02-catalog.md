# OpenRest — Módulo Catálogo (Ficheiros)

> Define o catálogo do estabelecimento (famílias, sub-famílias, artigos, promoções, happy hour, pratos do dia, páginas rápidas, complementos, automatismos) e as tabelas de parâmetros que o suportam.

## 1. Tabelas paramétricas

Acedidas via `Ficheiros → Tabelas`. Conforme `02-data-model/01-entities.md` secção 1–3.

### 1.1 Tabelas de Empregados
1. **Níveis de Acesso** (1–9)
2. **Grupos de Comissão de Empregado** (1–9)
3. **Grupos de Comissão de Artigos** (1–9)
4. **Tabela de Comissões** (matriz)
5. **Comissões Fixas** (1–9)

### 1.2 Tabelas de Clientes
1. **Qualidade de Cliente**
2. **Grupos de Desconto de Cliente**
3. **Grupos de Desconto de Artigo**
4. **Tabela de Desconto** (matriz)

### 1.3 Tabelas Gerais
1. **Tipos de Preço** (PVP1–PVP5)
2. **Métodos de Pagamento** (1–9; 1=Numerário, 9=CC)
3. **Taxas de IVA** (1–9)
4. **Unidades de Movimento**
5. **Tamanhos**
6. **Zonas** (de morada, para delivery)
7. **Atributos** (até 3, com N valores cada)

## 2. Centros de Custos / Armazéns

Cada artigo é abatido a um armazém, definido pela zona de impressão (`centro_custo_zona`). Permite ao MiliStore/Reports calcular consumo por armazém.

## 3. Ficha de Empregado

Campos detalhados em `02-data-model/01-entities.md §4.1`. Operações:

- **Novo** — cria novo empregado, gera código se omitido
- **Anular** (soft delete)
- **Recuperar** — desfaz anulação (se `mostra_fichas_anuladas`)
- **Lista** — imprime listagem configurável

UI dividida em 2 ecrãs (Avançado).

### Mesas atribuídas
Sintaxe de conjunto (`02-data-model/03-set-syntax.md`).

### Cor e idioma
Cor do botão e cor nas listas de detalhe. Idioma faz a UI mudar quando o empregado se identifica.

### Comissões
- `Grupo de Comissão` (artigos cruzam)
- `Produz para` / `Recebe de` — distribuição em equipa

## 4. Ficha de Cliente

Campos detalhados em `§4.2`. Inclui:

- Associação (cliente paga conta de outros)
- Parente Mononível (referência informativa)
- Conta corrente (totais débito/crédito, saldo, limite)
- Pontos de fidelidade
- Cartão com validade
- Identificação fiscal completa

Múltiplos telefones com sintaxe `225551234/5/6,22555321`.

## 5. Ficha de Família / Sub-família

Família = código múltiplo de 100. Sub-família = restantes códigos. Sub-família herda defaults da família. Artigos herdam defaults da sub-família.

Campos: nome, nome curto, tipo (normal/complemento/informativo/consumo), PVP1..PVP5, IVA na mesa, IVA venda directa, zona impressão, grupo comissão, grupo desconto, unidade, tara, peso unitário, tamanho, ordem impressão (1–9), nome botão, PVP variável, cor, imagem, mostra complementos automáticos.

Operações: Novo, Anular, Recuperar, Listagem.

Não se pode anular família com sub-famílias agregadas (ou sub-família com artigos).

## 6. Ficha de Artigo

Inclui (além dos defaults herdados):

- **Artigo Meia-Dose** — aponta para versão menor (1/2 dose). Permite ciclo de complementos por tamanho.
- **Tipo de Artigo**: normal, complemento, informativo, consumo, gorjeta.
- **Código de Barras** — leitura directa para entrada. Sintaxe especial `xxcccccpppppy` para códigos > 12 dígitos com quantidade encriptada.
- **Tara** e **Peso Unitário** — usar balança para entrada.
- **Cor**, **Imagem** (BMP/TGA 24bit no directório /Images, optimizado, ≤8 chars no nome, comprimível em GZ).

## 7. Listagens

A opção "Listagens" gera relatórios configuráveis sobre qualquer tabela:
- Escolha de campos
- Ordenação por qualquer campo
- Disposição na linha (col, largura, justificação)

Cada configuração é persistida; pode ser executada para qualquer impressora.

## 8. Exclusões

Por **posto** e/ou **local**, exclui:
- Famílias / Sub-famílias
- Artigos
- Empregados

Adicionalmente, exclui **operações**:
- Introduzir quantidades
- Transferir artigos
- Fechar mesas
- Fazer estornos
- Pagar com postos

Útil para isolar comportamento por terminal.

## 9. Promoções (Menus)

### 9.1 Estrutura

Promoção = artigo "cabeça" (família dedicada de Promoções) com preço fechado, decomposto em N níveis. Cada nível tem itens elegíveis (artigos) com `delta_preco`.

Exemplo "Menu do Dia" (4 €):
- Nível 1: Sopa (passar todos os artigos elegíveis para "Selecção"). Pode haver "Creme de marisco" com +0.5€.
- Nível 2: Prato (carnes/peixes).
- Nível 3: Café.

### 9.2 Pedido de promoção

No ecrã de pedidos por touch, premir a promoção abre janela de escolha de itens.

- `permite_encaixe_promocoes`: o cliente não precisa de pedir tudo de uma vez; podem ser registados após o pedido inicial. O sistema "encaixa-os" automaticamente.
- Sub-promoção: uma promoção pode ser item de outra promoção (não auto-referência).
- Quantidade múltipla: desmultiplica para optimizar (10 menus podem ter 7 Cocas + 3 Fantas).

### 9.3 Registo via comando

Sintaxe de pedido:
```
Menu 2 1
Água natural 1
Café 1
```

Item obrigatório (único do nível) não precisa ser registado.

### 9.4 Resolução de ambiguidades

Quando o mesmo item pode estar em vários níveis, o sistema escolhe o cenário de **menor custo total**.

### 9.5 Exclusivos

Lista de artigos vendíveis apenas como parte de promoções.

## 10. Pontos / Fidelidade

Configuração singleton:
- `activo`
- `valor_por_ponto` (X € = 1 ponto)
- `metodo_arredondamento`
- `permite_descontos_venda` (pontos pagam parte)
- `valor_por_ponto_venda`
- `impressora` (específica, sistema, ou nenhuma)

Documento de pontos configurável separadamente.

## 11. Happy Hour

Períodos com tipo de preço alternativo ou "2 pelo preço de 1" (`quantidade_bonus`).

Campos por entrada:
- Dia(s) da semana / data
- Hora início / fim
- Local (nullable = todos)
- Lista de artigos
- Tipo de preço a aplicar (PVPN)
- Comprar 1 leva 2: passar o artigo 2× para a lista Escolhidos

## 12. Páginas Rápidas

Ecrã alternativo de artigos misturados de várias famílias/sub-famílias. Configuração por nome.

Operações:
- Novo → editar
- Setas de transferência de artigos da lista de catálogo
- Mostra complementos automáticos (toggle)

A primeira página rápida configurada aparece no ecrã de pedidos.

## 13. Artigos em Automático

Eventos suportados:
- **Abertura de mesa** (couvert, consumo mínimo, taxa serviço)
- **Fecho de mesa** (perguntar quantidades caso `Qt=0`)
- **Início do dia** (mesas que ficam abertas — hotel)
- **Após pedido de outro artigo** (cerveja + amendoim)
- **Por transferência** (vender por fatias: regista-se o artigo numa "mesa origem" e transfere-se 1/8 para a mesa destino)

Campos:
- Artigo origem
- Quantidade (fixa ou variável)
- PVP usado (normal ou específico)
- Soma número de pessoas (factor × N + base)
- Arredondamento

## 14. Macros

Sistema de **níveis de macro** associados a pedidos.

Configuração:
- Artigo identificador de macro N (sobe o nível para os pedidos seguintes)
- Artigo execução de macro N (reimprime os pedidos do nível N)

Exemplo:
```
2 Bacalhau com natas   (nível 0)
1 Segue                (passa para nível 1)
1 Entrecosto Grelhado  (nível 1)
1 Costeletas Grelhadas (nível 1)
```
Mais tarde: `1 Pode Sair` reimprime os artigos do nível 1.

Recomenda-se `Aceita só pedidos da mesma zona` para que bebidas não fiquem misturadas com pratos.

## 15. Pratos do Dia

### 15.1 Configuração

- Tipo: Semanal, Diário, Configurável
- Nº dias (configurável)
- Data início (configurável)
- Nº refeições por dia (Almoço, Jantar…)
- Nomes das refeições
- Família dos Pratos do Dia (com artigos Prato Carne, Prato Peixe, etc.)

Para cada dia × refeição, definir qual `artigo correspondente` representa o prato real.

Definição de preço via "Artigos em Automático" → tipo de preço.

### 15.2 Refeição Actual

Selector que indica qual a refeição em curso. Determinante para os artigos correspondentes pedidos.

### 15.3 Restauração Colectiva

Módulo adicional para cantinas.

Características:
- Criar artigos "Reserva" e "Usa Reserva".
- Definir tipos de preço para clientes registados vs eventuais (separados por compra/utilização).
- Senhas com **UID** (código encriptado em EAN-13) para validação por torniquete.
- Configurar `pode_identificar_cliente_no_pedido` no local de reservas (substitui o campo quantidade por botão de identificação).
- Reservas inter-loja: senha emitida na loja A vale na loja B (configurável via rede remota e `offset` no UID).

Fluxo de reserva:
1. Empregado entra no local de reservas
2. Identifica cliente (regular) ou usa eventual
3. Selecciona prato → selecciona dia/refeição (em janela) → confirma
4. Imprime senha(s) com código de barras

Fluxo de consumo:
1. Cliente apresenta senha
2. Leitor lê código de barras → valida UID → marca como usado
3. Pedido automático do artigo correspondente para a mesa pré-configurada
4. Torniquete abre

Cliente sem reserva: aplica-se PVP normal.

## 16. Configuração de Documentos (cabeçalhos e rodapés)

Acedido via Manutenção → Documentos, mas conceptualmente parte do catálogo.

### 16.1 Tipos de documento

| Tipo | Nº variantes (C/R) |
|---|---|
| Factura | 9 |
| Venda a Dinheiro | 9 |
| Consulta de Mesa | 9 |
| Pedido (cozinha) | 9 |
| Recibo | 1 |
| Senha | 1 |
| Pontos | 1 |
| Apuramento de Dia | 1 |
| Facturação Externa | 1 |
| Validação de Cliente | 1 |

### 16.2 Editor de detalhes

Cada linha de detalhe escolhe campos (qt, nome, preço, IVA…) com:
- Coluna inicial
- Tamanho
- Justificação (esq, dir, centro)
- Tamanho duplo
- Negrito / Vermelho

### 16.3 Cabeçalhos e Rodapés

Texto livre com **flags especiais** (ver `08-appendices/02-printer-flags.md`).

Sequências essenciais:
- `\no` — Nome da casa
- `\ds` — Designação social
- `\nc` — NIF
- `\mo, \lo, \cp, \pa, \tf, \fx, \cv, \nr, \cs` — endereço, contactos, registo
- `\dt, \da, \sd, \ho, \hc` — datas
- `\nd` — Nº documento
- `\sX` — formatação (0=vermelho, 2=duplo, 4=sublinhado, …)
- `\ne, \oe` — empregado
- `\nm, \om` — mesa
- `\bc, \bX` — código barras, bitmap
- `\ol, \nx, \nl, \cl, \cx, \mc, \ll` — cliente
- `\np, \pp` — pessoas
- `\st, \vt, \ve, \sx, \tx, \ti` — totais e IVA (\ti = tabela de IVA)
- `\vc, \vg, \fp, \tr, \te, \pg, \pe` — pagamento
- `\lc` — local
- `\aX` — atributos

Em senhas de refeição, `\a1..\a3` têm semântica especial (data, refeição, prato).

### 16.4 Construção avançada (XML-like)

Permite referenciar **campos da BD** directamente:

```html
<! type="flag" id="no" align="center" mask="######…" !>
<! type="field" id="fb_c_vtot" align="right" mask="############" !>
<! type="uid" id="codigo_secreto" offset="5000" !>
```

`type`: `flag`, `field`, `uid`
`id`: chave (flag) ou caminho `tabela.campo`
`align`: left, right, center
`mask`: formato com `#` para dígitos
`default`: valor por defeito
`offset`: para UIDs cross-store

### 16.5 UIDs para senhas

Geração: encriptar `loja(125) || artigo(30) || data || refeição(4) || contador(15000)` com código secreto.
Encoding em EAN-13.
Validador: leitor reverte e verifica não-reuso.
Probabilidade de colisão aleatória ≈ 1/30M.
Para imprimir como código de barras: ESC/POS `1D6B430C` ou flag `\s6` antes do UID.

## 17. Wizard de configuração

Assistente que importa bases de dados "modelo" (famílias e artigos pré-configurados) de directório `databases/<modelo>/`.

Activado por `RunWizard=1` no `winrest.ini` (legado) ou primeira execução em OpenRest. Pode ser saltado e revisitado depois.

## 18. Ordem recomendada de configuração inicial

(Documentado em `01-architecture/04-deployment-and-bootstrap.md`)

1. Hardware do posto (input)
2. Licenciamento
3. Hardware (dispositivos, postos secundários)
4. Tipos de preço, locais, caixas, zonas de impressão (criar, não configurar ainda)
5. Configurações automáticas de caixa
6. Documentos (cabeçalhos/rodapés)
7. Mapear zonas de impressão a impressoras
8. Tabelas paramétricas
9. Famílias (cuidado especial — herdadas)
10. Artigos
11. Empregados
12. Propriedades dos postos
13. Teclas (se aplicável)
