# OpenRest — Módulo Restauração Colectiva

> Vertical específica para cantinas, refeitórios, refeitórios escolares e similares. Combina pratos do dia, senhas com UID encriptado, reservas de refeições, controlo por torniquete e preçário diferenciado.

## 1. Conceitos

- **Refeição** — almoço, jantar, lanche, etc. Pode haver 1..4 refeições por dia.
- **Prato do Dia** — artigo "Prato Carne"/"Prato Peixe" cujo correspondente real varia por dia.
- **Senha** — documento que titula uma reserva. Tem UID encriptado em código de barras EAN-13.
- **Reserva** — pré-pagamento (parcial ou total) de uma refeição para um dia futuro.
- **Utilização** — consumo da reserva no acto da refeição.
- **Cliente regular vs eventual** — clientes registados na BD podem ter preço diferenciado.

## 2. Configuração inicial

### 2.1 Pratos do Dia

Como definido em `02-catalog.md §15`. Configurar:
- Família dos Pratos do Dia
- Tipo (semanal/diário/configurável)
- Nº refeições por dia + nomes
- Mapeamento dia × refeição → artigo correspondente

### 2.2 Artigos especiais

Criar dois artigos:
- **"Reserva"** — pedido deste artigo cria uma nova reserva
- **"Usa Reserva"** — pedido deste artigo consome uma reserva existente

### 2.3 Local de Reservas

Local com:
- `pode_identificar_cliente_no_pedido = true`
- Campo Qt do ecrã é substituído por botão de identificação de cliente

### 2.4 Preços diferenciados

Na janela de configuração da restauração colectiva (`Ficheiros → Pratos do Dia → Reservas`):

- **Compra de reserva**
  - Artigo de compra: "Reserva"
  - Imprime senha? (toggle por tipo de cliente)
  - PVP para cliente regular
  - PVP para cliente eventual
- **Utilização de reserva**
  - Artigo de utilização: "Usa Reserva"
  - PVP para cliente regular
  - PVP para cliente eventual

PVP `(normal)` = aplica PVP do local. Outros = força um PVP específico.

Permite cobrar:
- Tudo no acto da reserva
- Tudo no consumo
- Parte na reserva + parte no consumo

## 3. Fluxo de Reserva

1. Empregado entra no Local de Reservas
2. Pede artigo "Reserva"
3. Janela de selecção de Prato + Dia/Refeição:
   ```
   [Reserva]         | (selectiona prato)
   Prato Carne       | [_][_][_][_][_][_][_] (dias)
   Prato Peixe       | [Almoço]
   Prato Vegetariano | [Jantar]
   ```
4. Premir botão `dia × refeição` adiciona a reserva.
5. Para várias refeições, repetir.
6. Para vários pratos no mesmo dia/refeição, premir mesmo botão N vezes.
7. Identificar cliente (regular) ou eventual.
8. Confirmar pedido.

Resultado:
- Documento de pedido normal
- N senhas impressas (uma por reserva)
- Reservas gravadas em BD

## 4. Senhas

Documento de tipo `senha` com 1 c/r configurável.

Campos típicos:
- Logo da casa
- Nome da loja destino (se cross-store)
- Data da refeição reservada
- Refeição (almoço/jantar)
- Prato escolhido
- Cliente (se regular)
- Código de barras EAN-13 com UID

Sequência especial usada: `<! type="uid" id="codigo_secreto" offset="X" !>`.

Em senhas, `\a1`, `\a2`, `\a3` têm semântica especial:
- `\a1` — data da refeição
- `\a2` — refeição (nome)
- `\a3` — prato escolhido

## 5. UID (Unique ID)

Estrutura encriptada:
- Loja destino (até 125 — 7 bits)
- Código do artigo reservado (1–30 — 5 bits)
- Data (data encoded)
- Refeição (1–4 — 2 bits)
- Contador (0–15000 — ~14 bits)

Total cabe em 12 dígitos úteis do EAN-13.

Encriptação com `código secreto` (até 9 dígitos) escolhido pela instalação.

Validação: leitor desencripta com mesmo código, verifica plausibilidade dos campos e não-reuso.

**Probabilidade de aceitar UID aleatório como válido**: ~1/30M.

### Cross-store

Várias lojas podem partilhar reservas. Para evitar colisões no contador:
- Cada loja tem `offset` diferente no documento senha:
  - Loja 1: offset 0
  - Loja 2: offset 5000
  - Loja 3: offset 10000

## 6. Fluxo de Consumo

### 6.1 Cliente identificado por leitura

1. Cliente apresenta senha
2. Leitor de códigos de barras associado (modo `Valida UID`) lê
3. Sistema desencripta, valida, marca como usado
4. Torniquete abre (se `Dispositivo de controlo de acessos` configurado)
5. Sistema pede automaticamente o artigo correspondente para mesa pré-configurada e empregado pré-configurado

### 6.2 Identificação manual

1. Empregado entra no Local
2. Identifica cliente (regular) → reservas pendentes aparecem na lista
3. Pede artigo "Usa Reserva"
4. Confirma

### 6.3 Cliente eventual sem reserva

1. Cliente compra directamente o prato do dia (à parte do sistema de reservas)
2. PVP normal aplicado

### 6.4 Eventual com senha (sem ficha)

1. Empregado pede "Usa Reserva"
2. Janela mostra todos os pratos com reservas pendentes daquela refeição
3. Empregado selecciona o indicado pela senha
4. Sistema verifica plausibilidade e regista uso

## 7. Display de Validação

Configurar `Display de validação` no leitor para que o cliente veja imediatamente:
- ✓ Verde — válido
- ✗ Vermelho — inválido / reutilizado / expirado

Útil em torniquetes onde o cliente está distante do operador.

## 8. Listagens e Relatórios

- **Reservas para hoje** (por refeição)
- **Clientes com reservas não consumidas** (listagem dos eventuais que não vieram)
- **Resumo de refeições por dia / mês**
- **Inventário previsível** (quantos pratos preparar)

## 9. Limitações

- O sistema **não impõe limite máximo** de reservas por refeição. Recomenda-se que a cantina configure manualmente.
- Para impor limite, usar pré-registo numa "mesa origem" (slice) com stock controlado.
- Não há hora limite para reservar — gestão humana.
- Reservas para o passado **não** são permitidas.

## 10. Integração

- BackOffice (Store/Reports) consolidando reservas/consumos de várias lojas
- Cartões de cliente (RFID) para identificação rápida
- App móvel para o cliente final reservar com antecedência (futuro)
- Integração com sistema escolar (importação de alunos como clientes)
- Faturação mensal a empresas/escolas (associação)

## 11. Configurações por local

- `pode_identificar_cliente_no_pedido` (campo Qt vira botão de identificação)
- `permite_zero_pessoas` (para senhas onde não interessa)
- Tipo de preço apropriado
- `Não imprime conta acima de` igual a zero (não imprime VD no acto)
