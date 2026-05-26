# OpenRest — Módulo Delivery (Entrega ao Domicílio)

> Funcionalidade dedicada ao serviço de delivery, incluindo identificação rápida do cliente por telefone, registo de pedidos, despacho, controlo de tempos e integração com call centers e redes remotas.

## 1. Activação

Activado pela criação de um **local com `tipo = delivery`**. Pode haver vários locais delivery (zonas de cidade, lojas remotas).

## 2. Identificação do Cliente

### 2.1 Entrada Activa (chamada recebida)

Se houver hardware de identificação de chamadas (`zyxel_callerid`, `fritzx_callerid`, modem genérico), uma chamada activa automaticamente a janela de identificação no posto configurado.

### 2.2 Entrada Manual

Operador prime o botão de Delivery e introduz nº telefone manualmente.

### 2.3 Pesquisa Avançada

Janela com filtros:
- Nome (parcial; suporta iniciais — "J A L" matches "José Alexandre Lago")
- Morada
- Telefone (com sintaxe `225551234/5/6,22555321`)

Procura por número de telefone:
- Por terminação: "1234" matches "225551234"
- Separador `/`: sufixos diferentes do mesmo número
- Separador `,`: números distintos

## 3. Criação de Cliente Novo

Se o telefone não existir na BD, abre janela de criação rápida (campos mínimos: nome, morada, telefone, zona).

Pode importar ruas de uma lista de ruas pré-configurada (acelera digitação).

## 4. Histórico do Cliente

Quando o cliente é identificado, mostram-se os **3 últimos pedidos**. Cada um tem botão "Copiar Pedido" que pré-carrega os artigos no ecrã actual.

Observações específicas:
- **Pedido** — observações impressas no documento de pedido para cozinha (ex: "bem passado")
- **Factura** — observações impressas no documento externo (ex: "bom apetite")
- **Cliente** — observações úteis ao operador (ex: "cliente exigente")
- **Morada** — observações adicionais à morada (ex: "frente ao restaurante Marina")

## 5. Janela de Despacho

Botão **Despacho** no ecrã de selecção de mesas.

Mostra todos os pedidos pendentes de entrega. Para cada:
- Mesa / número da encomenda
- Hora do pedido
- Tempo decorrido
- Cliente, morada, telefone
- Lista de artigos
- Estado (recebido, em preparação, pronto, despachado)

Operações:
- **Atribuir entregador**
- **Marcar pronto**
- **Despachar** (atribui entregador e timestamps)
- **Entregar** (regista entrega)

## 6. Controlo de Tempos

O sistema mede:
- Tempo de Preparação (do pedido até pronto)
- Tempo de Entrega (do pronto até entregue)

**Mecanismo**: imprimir flag `\bc` (código de barras) no pedido. Quando a cozinha tem o pedido pronto, lê o código de barras → marca como pronto.

Relatórios de delivery descriminam estes tempos.

## 7. Integração com Redes Remotas

Lojas centrais podem receber pedidos para depois despacharem nas lojas locais.

Configuração por loja:
- Quem trata pedidos recebidos remotamente
- Para que local entram
- Grava ou não localmente os pedidos enviados

Zona de morada do cliente associa-se a uma rede remota:
- "Boavista" → loja A
- "Foz" → loja B
- "Aveiro" → loja C

## 8. Modos de Funcionamento

### Loja Local
A loja recebe chamadas, identifica clientes, cria pedidos, despacha, entrega.

### Central Telefónica (Call Center)
Um posto recebe chamadas para várias lojas. O pedido entra na loja apropriada (pela zona do cliente) e segue lá. O operador fica livre para próxima chamada.

### Self-service (App / Web)
Pedidos chegam via API externa. Tratados como Delivery normais a partir daí.

## 9. Documentos relacionados

- **Pedido para cozinha** — com observações, código de barras se aplicável
- **Documento externo** (VD/Factura) — entregue ao cliente
- **Listagem de despacho** — para o entregador (ruas, contactos)

## 10. Estados de um pedido delivery

```
recebido → em_preparacao → pronto → despachado → entregue
                                                ↘ cancelado
```

Transições e timestamps registados em `pedido_delivery`.

## 11. Relatórios específicos

- **Pedidos pendentes** (em tempo real)
- **Performance**: média/desvio de tempos de preparação e entrega
- **Por entregador**: nº entregas, tempos médios, vendas
- **Por zona**: distribuição geográfica

## 12. Configurações úteis

- `Aviso de Reserva` — minutos antes da hora marcada para destacar a mesa como reservada (também aplica a delivery agendado)
- `Aceita só pedidos da mesma zona` — para macros restritas
- `Tempo de Preparação esperado` — para cor amarela/vermelha quando ultrapassado

## 13. Sugestão UX

Quando um pedido demora mais do que o esperado, deve haver sinalização visual e (configurável) sonora no posto de despacho. Cores:
- Verde: dentro do tempo normal
- Amarelo: tempo "médio" (configurável)
- Vermelho: ultrapassou tempo "grave"
