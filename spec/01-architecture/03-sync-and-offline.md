# OpenRest — Sincronização e Modo Offline

> Como múltiplas lojas mantêm consistência, e como uma loja continua a operar quando perde ligação ao servidor central.

## 1. Princípios

1. **Offline-first**: a operação local **nunca** depende de ligação à internet.
2. **Eventually consistent**: sincronização é eventual; conflitos resolvidos por regras claras.
3. **Auditável**: cada operação é um evento imutável.
4. **Idempotente**: re-aplicar uma operação não cria duplicados.
5. **Recuperável**: state pode ser reconstruído a partir do log de eventos.

## 2. Modelo

### 2.1 Event Sourcing local

Cada loja mantém um **event log** local com todas as operações:

```
{
  "id": "uuid",
  "agregado_tipo": "documento",
  "agregado_id": "uuid",
  "tipo_evento": "documento.emitido",
  "payload": { ... },
  "actor": { ... },
  "loja_id": "uuid",
  "criado_em": "2024-...",
  "vector_clock": { "loja_a": 123, "loja_b": 0 },
  "correlation_id": "uuid"
}
```

### 2.2 Projection / View Models

A "vista actual" das entidades é derivada do event log:

```
event_log → projections → tabelas (artigos, mesas, documentos, …)
```

Projections atualizadas em real-time ao consumir eventos.

### 2.3 Sync com servidor central

Bidireccional:
- **Push** (loja → central): eventos novos
- **Pull** (central → loja): eventos de outras lojas, alterações de catálogo central

Protocolo: gRPC stream bidireccional com:
- Acknowledgements
- Resume from last offset
- Retry com exponential backoff
- Encriptação TLS

## 3. Domínios de Sincronização

Cada entidade pertence a um **domínio de sync** com política própria.

### 3.1 Catálogo (Master Data)

Inclui: artigos, famílias, sub-famílias, promoções, happy hour, tabelas de IVA, tipos de preço, etc.

**Política**: central é fonte da verdade.
- Edição central → push para todas as lojas
- Edição local opcional (configurável) → push para central → conflito resolvido por timestamp (last-write-wins)
- Em caso de divergência, central prevalece

**Frequência**: imediata (push contínuo) ou periódica (ex: 5 minutos).

### 3.2 Empregados

**Política**: gestão central, login local.
- Empregados criados/editados centralmente
- Login funciona offline (cache local de credenciais)
- Permissões podem ser cacheadas

### 3.3 Clientes

**Política**: ambidireccional.
- Loja pode criar cliente novo (especialmente em delivery)
- Sincroniza para central
- Central distribui para outras lojas
- Conflito (mesmo NIF): merge por regras configuráveis

### 3.4 Operação (Documentos, Movimentos)

**Política**: cada loja é fonte da verdade da sua operação.
- Documentos da loja A não são alterados pela central
- Central armazena cópia para reports
- Conflitos impossíveis (cada loja tem sua numeração)

### 3.5 Configuração

**Política**: por loja.
- Hardware, postos, locais, layouts: locais à loja
- BackOffice pode oferecer templates / wizards
- Não sincronizam

### 3.6 Reservas inter-loja

**Política**: bidireccional, com particularidades.
- Reserva de refeição na loja A com consumo na loja B
- Loja A envia para loja B (via servidor central ou directo)
- UID é validável em ambas
- Sync em quase-real-time

### 3.7 Mensagens remotas

**Política**: bidireccional simples.
- Loja A envia mensagem para loja B
- Push assíncrono
- Queue local até confirmação

### 3.8 Pedidos delivery cross-loja

**Política**: bidireccional.
- Cliente liga para call center → encomenda registada no central
- Roteamento por zona → enviado para loja apropriada
- Loja confirma recepção
- Atualização de estado → push back para central

## 4. Vector Clock

Para detectar concorrência:

```
vector_clock = {
  "loja_a": 145,   // último evento conhecido de A
  "loja_b": 92,
  "central":  201
}
```

Cada evento incrementa o contador da fonte.

Comparação:
- `clk1 < clk2`: clk1 happened-before clk2
- `clk1 == clk2`: mesmo evento
- `clk1 || clk2`: concorrência → conflito

## 5. Resolução de Conflitos

### 5.1 Estratégias

| Estratégia | Quando usar | Risco |
|---|---|---|
| **Last-Write-Wins** (LWW) | Edições simples, alteração de preço | Perda silenciosa |
| **Merge automático** | Adicionar a uma lista | Pode duplicar |
| **Manual review** | Casos críticos (fiscalidade) | Lentidão |
| **Domain-specific** | Conta-corrente: soma operações | Complexidade |

### 5.2 Configuração por entidade

```yaml
artigo:
  conflict_resolution: lww
familia:
  conflict_resolution: lww
cliente:
  conflict_resolution: manual_review
  fields:
    nif: lww
    nome: lww
    saldo: domain_specific  # soma operações
documento:
  conflict_resolution: impossible  # nunca há conflitos
```

### 5.3 Detecção e notificação

Quando há conflito não resolvido automaticamente:
- Operação suspensa
- Notificação ao admin
- UI no BackOffice para resolver

## 6. Funcionamento Offline

### 6.1 Detecção

- Heartbeat ao servidor (a cada 30s)
- 3 falhas consecutivas → modo offline
- Banner visível na UI

### 6.2 Operação no modo offline

- Operação local funciona normal
- Documentos emitidos ficam em fila para sync
- Hash de assinatura é gerado normalmente (cadeia local)
- Pode emitir SAF-T local

### 6.3 Restabelecimento

- Detectar reconectividade
- Push de eventos pendentes
- Pull de eventos da central
- Reconciliação
- Conflitos resolvidos
- Banner removido

### 6.4 Duração tolerada

- 24h: confortável (default)
- 7 dias: aceitável com aviso
- > 7 dias: aviso forte, eventual bloqueio (configurável; depende da política)

## 7. Sync de Catálogo — Detalhes

### 7.1 Operações suportadas

- Criar artigo
- Alterar artigo (qualquer campo)
- Anular artigo (soft delete)
- Recuperar artigo

Equivalente para família, promoção, etc.

### 7.2 Optimização

- Delta sync: só campos alterados
- Compressão (gzip)
- Batching: múltiplas mudanças num único push

### 7.3 Eventos

```
catalogo.artigo.criado
catalogo.artigo.alterado
catalogo.artigo.anulado
catalogo.artigo.recuperado
```

## 8. Sync de Operação — Detalhes

### 8.1 Push diário

Por defeito, no fecho do dia, exporta:
- Todos os documentos do dia
- Todos os movimentos de caixa
- Atributos diários

Push em batch para central.

### 8.2 Push em tempo real (opcional)

Para integradores que precisam:
- Cada documento → push imediato
- WebSocket persistente

### 8.3 Identificação de duplicados

- UUID do evento (ID único)
- Vector clock
- Idempotent operations: aplicar 2x = aplicar 1x

## 9. Falhas e Recuperação

### 9.1 Loja perde dados completos

- Restore do último backup
- Pull do central para dados em falta
- Replay de eventos perdidos

### 9.2 Central perde dados

- Restore do backup central
- Pull de TODAS as lojas
- Reconstrução por replay

### 9.3 Lojas e central em desacordo

- Identificar fonte da verdade por entidade
- Manual review se necessário

## 10. Modo "Estado degradado"

Estados possíveis:

| Estado | Causa | Permitido | Bloqueado |
|---|---|---|---|
| **Online** | Tudo OK | Tudo | — |
| **Offline parcial** | Central inacessível | Operação local | Reservas cross-loja, pedidos call center |
| **Offline total** | Sem rede | Operação local | Tudo cross-loja |
| **Inconsistente** | Pendências antigas | Operação com aviso | Operações que dependem de sync |
| **Bloqueado** | Limite ultrapassado | Apenas consulta | Operação |

## 11. Multi-master vs Single-master

OpenRest é **multi-master** com regras:
- Cada loja é master da sua operação
- Central é master do catálogo (com excepções configuráveis)
- Inter-loja: ambos podem editar, com resolução de conflito

Single-master (cloud-only) é uma opção de deployment, mas não default.

## 12. Sincronização end-to-end com criptografia

Para máxima segurança:
- TLS em transit
- Encriptação at-rest (BD)
- Assinatura digital de eventos (opcional)
- Encriptação de payloads sensíveis (cartões, dados pessoais GDPR)

## 13. Métricas e Health

Dashboard mostra:
- Latência de sync por loja
- Tamanho de queue pendente
- Conflitos por resolver
- Última sync com sucesso
- Saúde da ligação

## 14. Backup e Snapshot

### 14.1 Local

- BD SQLite → snapshot diário (compressed)
- Event log → append-only, snapshot semanal
- Configuração → versionada em ficheiro

### 14.2 Cloud

- Encriptado at-rest (S3-compatible)
- Retenção: 90 dias por defeito (configurável)
- Test de restore mensal

## 15. Modo "single server" sem central

Loja autónoma sem central. Toda a configuração local. Backup local + cloud opcional.

Útil para:
- Restaurantes únicos
- Cafés / bares
- Negócios pequenos sem necessidade de cadeia

Default para "OpenRest Self-hosted".

## 16. Migração de uma loja entre tenants

Suporte a migração:
- Export completo
- Import noutro tenant
- Reescrita de UUIDs (preserva referências)
- Validação de integridade

Útil em mudanças de proprietário / reorganizações.
