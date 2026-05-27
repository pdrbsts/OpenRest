# OpenRest — Ecrãs do POS (Detalhe)

> Especificação detalhada dos ecrãs principais do POS, com layouts, zonas, interacções e regras de navegação.

## 1. Ecrã de Selecção de Empregado

Mostra grelha de empregados activos:

```
+---------------------------------------------------+
|  ESCOLHE EMPREGADO                                |
+---------------------------------------------------+
|  [João]    [Maria]   [Pedro]   [Ana]              |
|   Cor1      Cor2      Cor3      Cor4              |
|                                                   |
|  [Anterior]                          [Seguinte]   |
+---------------------------------------------------+
|  Ler cartão / Led ID                              |
+---------------------------------------------------+
```

Acções:
- Premir botão → login
- Passar cartão → login automático
- Setas para paginar

Configuração: até N empregados por página (depende da resolução e do `botoes_na_janela_pedidos`).

## 2. Ecrã de Selecção de Mesa

```
+----------------------------------------------------+
| MESAS — Empregado: João          [Despacho] [Cons] |
+----------------------------------------------------+
|                                                    |
| Local: [Sala 1 ▼]                Sub-grupo: [1-50] |
|                                                    |
| [01]   [02]   [03]   [04]   [05]                   |
|  €23   €0     €45    €17    €0                     |
| ●      ◌      ●      ●      ◌                      |
|                                                    |
| ... (mais mesas)                                   |
|                                                    |
|  [Anterior]                          [Seguinte]    |
+----------------------------------------------------+
| [Mapa de Mesas]                       [Voltar]     |
+----------------------------------------------------+
```

Botões de mesa:
- Número/nome
- Valor actual
- Indicador de estado (cor / ícone)

Mesas inacessíveis (não pertencem ao empregado) ficam escurecidas e bloqueadas.

`Despacho` aparece se há locais delivery.
`Consumo` aparece se o empregado tem permissão `pedidos.consumo_proprio`.

`Mapa de Mesas` alterna para vista gráfica:

```
+----------------------------------------------------+
|  [Área 1] [Área 2] [Esplanada]                     |
+----------------------------------------------------+
|                                                    |
|     [Imagem da Sala]                               |
|        com pontos clicáveis                        |
|                                                    |
+----------------------------------------------------+
| [Selecção Numérica]                  [Voltar]      |
+----------------------------------------------------+
```

Empregado clica num ponto e o sistema selecciona a mesa cujo ponto mais próximo coincide.

## 3. Ecrã de Pedidos (modo normal — Touch)

```
+---------------------------------------------------+
| MESA 14 — Sala 1 — João  €23,50  Pessoas: 2       |
+---------------------------------------------------+
| PEDIDO EM CONSTRUÇÃO        |  CONSUMO ACTUAL     |
|  1x Café                    |   2x Cerveja  €4,00 |
|  2x Bolo                    |   1x Vinho    €5,00 |
|                             |                     |
+-----------------------------+---------------------+
|       Famílias              |    Sub-famílias     |
|  [Bebidas] [Café] [Doces]   |   [Cafés] [Leites]  |
+-----------------------------+---------------------+
|              ARTIGOS                              |
|  [Café]    [Café Cheio]   [Galão]                 |
|  [Carioca] [Cimbalino]    [Garoto]                |
|       ↑↓ paginação                                |
+---------------------------------------------------+
| Qt: [_____]    [Pedir]  [Sub-total]  [Cancelar]   |
| [Anular] [Oferta] [Recebe] [Transf.] [Voltar]     |
+---------------------------------------------------+
```

Layout zonas:
- **(1)** Lista do pedido em construção (top-left)
- **(2)** Lista de consumo actual (top-right)
- **(3)** Coluna/linha de famílias
- **(4)** Coluna/linha de sub-famílias
- **(5)** Grelha de artigos

Quando existe apenas 1 família com 1 sub-família e <56 artigos, modo "uma família": toda a página é dedicada a artigos.

Setas Anterior/Seguinte aparecem em qualquer secção paginada.

### Acções

| Acção | Resultado |
|---|---|
| Premir artigo | Adiciona 1 unidade ao pedido em construção (zona 1) |
| Premir balança | Lê peso da balança para Qt |
| `Qt` + número → premir artigo | Adiciona N unidades |
| Botão Pedir | Submete (move para zona 2), imprime nas zonas configuradas |
| Botão Cancelar | Remove linha seleccionada da zona 1 |
| Botão Anular | Abre ecrã de anulação (zona 2) |
| Botão Oferta | Abre ecrã de oferta/desconto |
| Botão Sub-total | Imprime consulta de mesa |
| Botão Recebe | Abre ecrã de recebimento |
| Botão Transf. | Abre ecrã de transferência |
| Botão Voltar | Volta à selecção de mesa |
| Long-press num artigo | Mostra detalhes/info do artigo |

### Complementos automáticos

Se família tem `mostra_complementos_automaticos`, ao seleccionar artigo principal aparecem botões com complementos mais usados, à direita ou em popup. Quick-add.

## 4. Ecrã de Pedidos — Take-Away

```
+---------------------------------------------------+
| TAKE-AWAY — €0,00                                 |
+---------------------------------------------------+
| Pedido:                          |                |
|                                  | [€5]  [€10]    |
|                                  | [€20] [€50]    |
|                                  | [Exacto]       |
|                                  |                |
+-----------------------------+----+----------------+
|       Famílias / Artigos    |    Troco: €0,00     |
|  [Bebidas] [Café] [Doces]   |                     |
|     ↓ grelha                |                     |
+-----------------------------+---------------------+
| Qt: [_____]    [Pedir & Fechar]                   |
| [Cancelar]  [Modo Normal]  [Voltar]               |
+---------------------------------------------------+
```

5 botões de troco rápido (valores configuráveis na caixa).
Troco visível em tempo real.
Botão "Modo Normal" para acessar funcionalidades que TA não permite (oferta, CC).

## 5. Ecrã de Pedidos — Take-Away Seguro

Igual ao Take-Away mas:
- Total **não aparece** ao operador nem ao display de cliente até confirmação
- Botão "Pedir" precisa ser premido duas vezes para fechar a mesa

## 6. Ecrã de Pedidos — PUB

```
+---------------------------------------------------+
| MESA 1 (primeira mesa do PUB) — €18,50            |
+---------------------------------------------------+
| Pedidos pendentes (1ª mesa):                      |
|  1x Whisky        €5,00                           |
|  1x Cerveja       €3,00                           |
|                                                   |
+-----------------------------+---------------------+
|       Artigos               |    Troco rápido     |
|     ↓                       |    [€5]  [€10]      |
+-----------------------------+---------------------+
| [Pedir & Fechar]  [Pedir & Manter (transferir)]   |
| [Transferir → Cliente "Pedro"]  [Voltar]          |
+---------------------------------------------------+
```

Dois modos de pedir:
- Fechar a mesa (venda directa)
- Manter a mesa em espera (transformar em mesa normal)

Botão Transferir abre janela específica do PUB onde se atribui nome ao cliente.

## 7. Ecrã de Pedidos — Delivery

```
+---------------------------------------------------+
| DELIVERY — Mesa 101 — Pedro Silva                 |
| Telefone: 912345678  Morada: Rua X, Lisboa        |
+---------------------------------------------------+
| Pedidos:                                          |
|  1x Pizza Família  €12,00                         |
|  1x Coca 1.5L      €3,50                          |
|                                                   |
| Obs Pedido: bem passado                           |
| Obs Factura: bom apetite                          |
| Obs Cliente: cliente exigente                     |
| Obs Morada: frente Marina                         |
+---------------------------------------------------+
|       Artigos / Famílias                          |
|     ↓                                             |
+---------------------------------------------------+
| Histórico: [Pedido 1] [Pedido 2] [Pedido 3]       |
| [Copiar Pedido N]                                 |
+---------------------------------------------------+
| [Pedir]  [Identificar Cliente]  [Voltar]          |
+---------------------------------------------------+
```

Identificação por:
- Chamada activa (CID)
- Pesquisa manual

## 8. Janela de Identificação de Cliente

```
+---------------------------------------------------+
| IDENTIFICAR CLIENTE                                |
+---------------------------------------------------+
| Pesquisa:                                          |
|  Nome: [_____________]                             |
|  Cartão: [______]                                  |
|  Código: [______]                                  |
|  Telefone: [______]                                |
|  Zona: [Cualquera ▼]                               |
|                                                    |
+----------------------------------------------------+
| Resultados:                                        |
|  Pedro Silva   912.... Lisboa   €0      Limite €200 |
|  Pedro Costa   913.... Porto   −€50    Limite €100  |
|  ...                                               |
+----------------------------------------------------+
| [Seleccionar] [Criar Novo] [Avançado] [Voltar]     |
+----------------------------------------------------+
```

Pode pesquisar combinando vários campos.
Botão "Criar Novo" abre janela de cliente novo.
"Avançado" filtra por múltiplos campos.

## 9. Janela de Recebimento

Já especificado em `03-modules/01-pos-operation.md §8`. Layout sugerido:

### 9.1 Modo simples

```
+---------------------------------------------------+
| RECEBER MESA 14 — Total: €23,50                   |
+---------------------------------------------------+
| Cliente: [Anónimo]              Pessoas: [2]      |
|                                                   |
|              [Numerário] [Multibanco] [Visa]      |
|              [MB Way]    [Cheque]   [Vale]        |
|              [Conta Corrente]                     |
|                                                   |
| Valor pago: [_______]    Troco: €0,00             |
|                                                   |
+---------------------------------------------------+
| [Imprime] [Imprime N.Desc.] [Avançado]            |
| [OK]  [Cancelar]                                  |
+---------------------------------------------------+
```

Botões grandes para os métodos comuns.
Valor pago opcional (em alguns locais, obrigatório).

### 9.2 Modo Avançado (múltiplos pagamentos)

```
+---------------------------------------------------+
| RECEBER MESA 14 — Total: €23,50                   |
+---------------------------------------------------+
| Pagamentos:                                       |
|  €10,00  Numerário                                |
|  €13,50  Multibanco                               |
|                                                   |
| Recebido: €23,50    Em Falta: €0,00               |
|                                                   |
| Valor: [_____]      [Numerário] [Multibanco] ...  |
|                                                   |
+---------------------------------------------------+
| [Anular pagamento] [Parcial] [Múltipla (divisão)] |
| [OK]  [Imprime]  [Cancelar]                       |
+---------------------------------------------------+
```

### 9.3 Pagamento Parcial (selecção de artigos)

```
+---------------------------------------------------+
| PAGAMENTO PARCIAL                                  |
+---------------------------------------------------+
| Lista de Artigos:                                  |
|  [ ] 1x Café     €1,00                            |
|  [x] 1x Bolo     €2,50                            |
|  [x] 2x Cerveja  €4,00                            |
|                                                   |
| Total Selecionado: €6,50                          |
+---------------------------------------------------+
| [OK]  [Cancelar]                                   |
+---------------------------------------------------+
```

### 9.4 Divisão de Conta

```
+---------------------------------------------------+
| DIVIDIR CONTA — Total €23,50  em [2 ▼] contas     |
+---------------------------------------------------+
| Conta 1: €11,75      |    Conta 2: €11,75         |
|  1x Café   €1,00     |    1x Bolo   €2,50         |
|  1x Cerveja €2,00    |    1x Cerveja €2,00        |
|  ...                 |    ...                     |
+---------------------------------------------------+
| ← Transferir Artigo →                              |
| [Divisão Automática]                               |
| [Conta 1: Pagamento ▼] [Conta 2: Pagamento ▼]    |
+---------------------------------------------------+
| [OK]  [Cancelar]                                   |
+---------------------------------------------------+
```

## 10. Janela de Ofertas

Igual à de transferência mas com campo de % ou valor:

```
+---------------------------------------------------+
| OFERTAS — Mesa 14                                  |
+---------------------------------------------------+
| Artigos selecionáveis:                             |
|  [x] 1x Bolo     €2,50  Desconto: 10% = €0,25     |
|  [x] 2x Cerveja  €4,00  Desconto: 10% = €0,40     |
|                                                   |
| Desconto: [__10__] %   ou: [____] €               |
| Total Desconto: €0,65                             |
+---------------------------------------------------+
| [Eliminar Ofertas Anteriores]                     |
| [OK]  [Cancelar]                                   |
+---------------------------------------------------+
```

## 11. Janela de Transferência

```
+---------------------------------------------------+
| TRANSFERIR — Mesa 14 → ?                          |
+---------------------------------------------------+
| Mesa Origem: 14                                    |
| Artigos:                                           |
|   1x Café     €1,00     [→]                       |
|   1x Bolo     €2,50     [→]                       |
|   2x Cerveja  €4,00     [→]                       |
|                                                    |
| Para Mesa: [____]   [Procurar Mesa]                |
+---------------------------------------------------+
| [Transferir Selecionado] [Transferir Tudo]         |
| [OK]  [Cancelar]                                   |
+---------------------------------------------------+
```

## 12. Janela de Anulação

```
+---------------------------------------------------+
| ANULAR — Mesa 14                                   |
+---------------------------------------------------+
| Artigos no consumo:                                |
|  [_] 1x Café     €1,00                            |
|  [x] 1x Bolo     €2,50                            |
|  [_] 2x Cerveja  €4,00                            |
|                                                    |
| Qt a anular: [_1_]                                 |
| Com Desperdício: [x]   Sem Desperdício: [_]        |
+---------------------------------------------------+
| [OK]  [Cancelar]                                   |
+---------------------------------------------------+
```

Botão balança ao lado de "Qt" para inserir peso exacto.

## 13. Ecrã de Pedidos — Via Teclado

```
+---------------------------------------------------+
| MESA: [001] EMP: João  TOTAL: €23,50              |
+---------------------------------------------------+
| (1) Lista do detalhe atual:                       |
|     2x Café Cheio    €2,00                        |
|     1x Bolo          €2,50                        |
|                                                   |
| (2) Consumo actual da mesa:                       |
|     ...                                            |
|                                                   |
| (3) [MESA 001]      (4) Empregado: João           |
|                                                   |
| (5) Entrada: [____]                               |
+---------------------------------------------------+
| Teclas: Mesa  Qt  Artigo  Pedir  Anular           |
+---------------------------------------------------+
```

5 zonas:
1. Detalhe do pedido em construção
2. Consumo actual da mesa
3. Mesa actual
4. Empregado actual
5. Linha de entrada

Operação fluida com teclado físico:
```
nº_mesa  → Mesa
[qt]     → Quantidade
nº_art   → Artigo
... (repetir)
Pedir
```

Erros corrigíveis pelas setas do cursor.

Mudança de empregado: `código + Empregado`.

## 14. Ecrã de Despacho (Delivery)

```
+----------------------------------------------------+
| DESPACHO DELIVERY                                  |
+----------------------------------------------------+
| Pedidos pendentes:                                 |
|                                                    |
| #1234  10:35 (5 min)  Pedro Silva  Lisboa         |
|        Pizza, Coca                                 |
|        Estado: PRONTO    [Despachar para...]       |
|                                                    |
| #1235  10:38 (2 min)  Maria Santos  Lisboa        |
|        Sushi                                       |
|        Estado: EM PREP                             |
|                                                    |
+----------------------------------------------------+
| Entregadores disponíveis: João, Pedro              |
+----------------------------------------------------+
| [Refresh]  [Filtros]  [Voltar]                     |
+----------------------------------------------------+
```

Cores por tempo decorrido:
- Verde < 15min
- Amarelo 15-25min
- Vermelho > 25min (configurável)

## 15. Ecrã de Caixa

```
+----------------------------------------------------+
| OPERAÇÕES DE CAIXA — Caixa 1 — Turno 1            |
+----------------------------------------------------+
|                                                    |
|  REGISTOS         ABERTURAS       MOVIMENTOS      |
|  [Consulta]       [Abre Caixa]   [Pag. CC]        |
|  [Estatísticas]   [Abre Sessão]  [Fundo]          |
|                                  [Empréstimo]     |
|  TRANSFERÊNCIAS                  [Retirada]       |
|  [Vendas Activas]                [Envelopes]      |
|  [Turno]                         [Compra]         |
|                                  [Vale]           |
|                                                    |
|  ENCERRAMENTOS    APURAMENTOS    RELATÓRIOS       |
|  [Fecha Sessão]   [Sessão]       [Custom 1]       |
|  [Fecha Caixa]    [Caixa]        [Custom 2]       |
|  [Fecho Financ.]  [Turno]        ...              |
|  [Fecha Dia]      [Dia]                           |
|                                                    |
+----------------------------------------------------+
| Caixa: €245,30   Turno: 1   Em sessão: 3 emp.     |
+----------------------------------------------------+
```

Disposição em grelha de 5 secções + área de relatórios customizados (até 5 botões directos).

## 16. Ecrã de Manutenção

Ecrã técnico, layout em grelha de ícones:

```
+----------------------------------------------------+
| MANUTENÇÃO                                         |
+----------------------------------------------------+
|  [Definições Gerais]   [Acessos]   [Caixas]       |
|  [Zonas Impressão]     [Licença]   [Locais]       |
|  [Documentos]          [Comandos]  [Postos]       |
|  [Hardware]            [Dispositivos] [Teclas]    |
|                                                    |
+----------------------------------------------------+
| [Sair Manutenção]                  [Fim]           |
+----------------------------------------------------+
```

### 16.1 Configuração de Zonas de Impressão e Mapeamento

Por decisão estrutural, para minimizar a fricção com os utilizadores da versão legacy (WinREST) e permitir uma transição suave, o ecrã de configuração de **Zonas de Impressão** mantém exatamente o mesmo paradigma de interface (UI) e a mesma lógica de roteamento cruzado:
- **Painéis em Cascata**: Os eixos `Origem (Configuração)`, `Zona de Impressão` (incluindo tanto destinos de produção como destinos lógicos de conta: `D. Externos` e `D. Internos`) e `Local` coabitam e são selecionáveis em painéis list-box lado a lado.
- **Workflow de Seleção**: O utilizador seleciona uma Origem, seleciona a Zona, depois clica no Local. No painel final de `Impressoras`, marca as impressoras que devem processar essa combinação (`Origem x Zona x Local`) através de caixas de seleção.
- **Overrides Locais**: A mesma filosofia de herança visual é mantida; propriedades como `Agrupamento` e `Tipo de Pedido` colocadas diretamente na impressora sobrepõem-se aos defaults definidos para a Zona.

## 17. Fluxos cross-screen

### 17.1 Abrir o dia (servidor)

```
Arranque → verificar licença → carregar BD
→ se houve falha pendente: recuperar transacção
→ caixas configuradas para abrir auto? → abrir
→ sessões para abrir auto? → abrir
→ ecrã principal
```

### 17.2 Atender mesa

```
Ecrã principal → Pedidos → escolher empregado → escolher mesa
→ ecrã de pedidos → seleccionar artigos → Pedir
→ aguardar consumo / Sub-total
→ Recebimento → escolher método → OK
→ mesa fecha → ecrã de pedidos com nova mesa OU mapa de mesas
```

### 17.3 Fechar dia

```
Ecrã principal → Caixa → Fecha Sessão (cada empregado)
→ Fecha Caixa (cada caixa) → Fecha Dia
→ confirmar data → apuramentos → exportações
→ se folga: avança data
→ ecrã principal (data nova)
```

## 18. Ecrã de Cliente (Segundo Monitor)

Este ecrã é apresentado num monitor secundário virado para o cliente. Não tem controlos interativos (touchscreen desativado ou ignorado no painel do cliente).

```text
+---------------------------------------------------+
|  [LOGO DA CASA]                    [12:30]        |
+---------------------------------------------------+
|  (1) CONTA ACTUAL      |  (2) ZONA DE MARKETING   |
|                        |                          |
|  1x Café         €1,00 |  +--------------------+  |
|  1x Bolo         €2,50 |  |                    |  |
|  2x Cerveja      €4,00 |  |   IMAGEM / VÍDEO   |  |
|                        |  |    PROMOCIONAL     |  |
|                        |  |                    |  |
|  --------------------  |  +--------------------+  |
|  TOTAL:          €7,50 |                          |
|                        |  "Peça o nosso menu!"    |
|                        |                          |
+---------------------------------------------------+
|  (3) MENSAGEM RODAPÉ: Obrigado pela preferência!  |
+---------------------------------------------------+
```

Zonas principais:
1. **Conta Atual**: Apresenta as linhas do pedido em tempo real, descontos aplicados e o valor total. Quando o operador entra no ecrã de pagamento, destaca o valor recebido e o troco.
2. **Zona de Marketing**: Carousel rotativo de imagens ou vídeos (suporta media local ou via CDN). Pode ser configurada para ecrã inteiro quando a mesa está fechada/inativa (Modo Standby).
3. **Rodapé**: Mensagem rotativa ou fixa com cumprimentos, horários de funcionamento, ou informações fiscais legais (quando obrigatório).
