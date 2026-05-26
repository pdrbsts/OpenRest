# OpenRest — Módulo Cozinha / Distribuição de Pedidos

> Define como os pedidos são distribuídos pelos diferentes pontos de produção (cozinha, bar, grelhador, café, dispenser de bebidas, máquinas de café automáticas) e como esses pontos comunicam o estado de volta.

## 1. Conceitos

### 1.1 Zona de Impressão

Sítio **lógico** para onde um artigo deve ser impresso. Exemplos:
- D.Externos - Faturas-Recibo / Faturas Simplificadas (zona 1, reservada)
- D.Internos - Sub-Totais / Consultas de Mesa (zona 2, reservada)
- Bar
- Cozinha
- Grelhador
- Pastelaria
- Café

### 1.2 Impressora

Dispositivo **físico** ligado a uma porta.

### 1.3 Mapa zona → impressora

Configurado por `local` e (opcionalmente) por `origem`. Permite que o "vinho da casa" seja impresso no Bar 1 quando pedido na Sala 1 e no Bar 2 quando pedido na Sala 2.

```
zona_impressao × local × origem → impressora(s) com agrupamento, tipo doc, secundário, complemento
```

### 1.4 Origem

Conjunto de postos + comandos que partilham a mesma matriz de zonas. Permite ter postos com routing diferente do default.

## 2. Distribuição

Quando um pedido é submetido:

1. Para cada `documento_detalhe` no pedido:
   a. Determinar `zona_impressao` (do artigo ou herdada).
   b. Determinar `origem` do posto/comando que pediu.
   c. Lookup `impressora_zona_local[zona × local × origem]`.
   d. Adicionar linha à fila da impressora.
2. Para cada impressora envolvida:
   a. Construir documento conforme `tipo_pedido` configurado.
   b. Aplicar `agrupamento` da impressora ou da zona.
   c. Imprimir.
3. Se há zonas marcadas `secundarios=true`:
   a. Construir pedidos espelho para essas zonas, marcando que sai junto com pedido principal.
   b. Imprimir.

## 3. Modos de Agrupamento

| Modo | Comportamento |
|---|---|
| **Individual** | Cada artigo num pedido com quantidade 1. 2 cafés → 2 pedidos. |
| **Por Artigo** | Cada linha agrupada por artigo. 2 cafés + 1 descafeinado → 2 pedidos (um com 2 cafés). |
| **Agrupado** | Um único pedido com todos os artigos. |
| **Individual + Agrupado** | Imprime individuais + um sumário agrupado. |
| **Por Artigo + Agrupado** | Por artigo + sumário agrupado. |
| **Agrupa Zonas** (na impressora) | Junta zonas configuradas como Agrupa Zonas no mesmo pedido. |
| **Agrupa Tudo** (na impressora) | Mantém ordem de pedido, sem agrupar por zona. |

Configurável a dois níveis: **zona** (default) e **impressora** (override). `Normal` na impressora = herda da zona.

## 4. Pedidos Secundários (Cross-zone)

Cenário: pizza+saladas devem sair ao mesmo tempo da Pizza (cozinha) e do Bar (saladas).

Marcar zonas Cozinha e Bar como `secundarios=true`. Quando há pedido com artigos de ambas, cada zona recebe:
- Pedido principal (artigos dessa zona)
- Indicação "sai junto com" (artigos das outras zonas)

Tipo de documento `tipo_secundario` permite usar letra mais pequena para esses extras.

## 5. Complementos

Artigos do tipo `complemento` saem na **mesma zona do principal** por defeito.

Excepção: tipo `tipo_complemento` permite formatação diferente (ex: letra menor).

Em tamanhos diferentes (ex: Bem Passado, Molho Especial, Mal Passado): vê `02-data-model/01-entities.md §4.4` (artigo meia-dose forma ciclo).

## 6. Documento de Pedido

Formato configurável (ver `02-catalog.md §16`).

Campos típicos:
- Cabeçalho com nome do empregado, mesa, hora
- Linhas de detalhe: Qt, Nome (curto ou completo), preço unitário, preço total (opcional)
- SubTotal (opcional, em impressoras de cartões)
- Sem cabeçalho fiscal (não é documento externo)

## 7. Monitor de Pedidos (KDS - Kitchen Display System)

Dispositivo "impressora especial" que mostra pedidos em ecrã em vez de papel.

Funcionalidades:
- Fila de pedidos, com fila de espera (símbolo se cheio)
- Acompanhamento do estado da mesa (cores muda com o tempo):
  - **Verde** — Atraso normal
  - **Amarelo** — Atraso médio
  - **Vermelho** — Atraso grave
  Limites configuráveis.
- Apagar pedido (DEL), Recuperar (INS), Próximo/Anterior (PgUp/PgDn) — teclas em botoneira.
- Configurar `Acompanha o estado da mesa` e `Apaga o pedido no fecho da mesa`.
- Configurações de colunas e fonte.

Em locais com Monitor de Pedidos, pode definir-se uma `impressora_directa_pedidos` que vai actualizando o monitor à medida que o operador picka artigos (sem esperar pelo "Pedir").

## 8. Impressora directa de pedidos

Quando o local tem `impressora_directa_pedidos_id`, cada artigo seleccionado no ecrã de pedidos vai aparecendo no monitor de cozinha em tempo real — mesmo antes de premir Pedir. Útil para fluxos contínuos.

## 9. Máquina de Café (controlo de doses)

Tratada como impressora especial. Quando um café é pedido no POS:
1. Sistema marca crédito +1 na "classe do café" do dispositivo.
2. Operador prime botão da máquina → máquina pergunta ao POS se há crédito.
3. Se sim → produz dose, débito −1.
4. Se não → bloqueia (configurável: créditos extra negativos limitados).

Configurações por máquina:
- Botões físicos da máquina mapeados como: 1 unidade, 2 unidades, Acerto, Não usado, Uso livre
- Número de classes (café, leite, …) e atribuição de botões
- Créditos extra disponíveis

Anulação:
- **Com desperdício** — artigo já foi produzido → crédito mantém-se
- **Sem desperdício** — crédito devolvido

## 10. Controlo de Acessos (Torniquete)

Dispositivo tratado como impressora. Quando é "impresso" um documento na zona configurada → torniquete abre.

Ou alternativamente: leitor de códigos de barras associado valida UID → torniquete abre + pedido automático para mesa pré-configurada.

## 11. Redireccionamento de Impressoras

Em runtime, qualquer empregado com permissão pode aceder ao **Redireccionamento de Impressoras** (Sistema):

Estados:
- **Normal** — imprime
- **Espera** — guarda fila até reactivação
- **Ignorar** — descarta documentos
- **Redirecciona** → outra impressora

Documentos guardados (configurável): podem ser reimpressos.

Este mecanismo é a "rede de segurança" quando a impressora avaria. Permite continuar a operação enquanto se substitui o hardware.

## 12. Imprimir códigos de barras

- **CODE128** via flag `\bc` (bitmap por defeito)
- **ESC/POS nativo**: opção na impressora `Imprime códigos de barras ESC/POS` (mais rápido)
- **ITF (interleaved)**: opção adicional para códigos numéricos pares
- **EAN-13**: para senhas, sequência `1D6B430C` (Epson) precedida por `\s6`

## 13. Códigos de barras como entrada (filtros)

Configuráveis filtros de leitor para extrair códigos com formato específico:
- `%d` — número decimal completo
- `%n` (variável) — até N dígitos
- `%*` — ignora
- `%[...]` — só caracteres do conjunto
- Saída com `%d` ou `%Ld` (para grandes códigos como EAN13)

Permite cartões com estruturas diversas (timestamps, IDs encriptados) serem mapeados ao código do cliente/empregado.

## 14. Sugestões para o redesign

1. **KDS como cidadão de primeira classe**: tablets na cozinha em vez de impressoras térmicas, com bumps por gesto.
2. **Routing por estação não só por zona**: cada zona pode ter sub-estações (Fritadeira, Salamandra, etc.) com lógica adicional.
3. **Pedidos com fotos**: incluir foto do artigo nos pedidos para cozinhas com staff rotativo.
4. **Tempos sugeridos por artigo**: cozinha sabe quanto deve durar cada item, alarma se ultrapassa.
5. **Reentregas de pedidos**: se KDS falhar, fila de pendentes vai para outro display.
