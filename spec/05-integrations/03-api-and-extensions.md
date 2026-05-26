# OpenRest — APIs e Extensibilidade

> Interface de programação para integrações externas: BackOffice, Reports, PMS, sistemas de cliente, marketplaces, etc.

## 1. Princípios

1. **API-first**: tudo o que se faz na UI tem que se poder fazer via API.
2. **Versionada**: cada endpoint tem versão; mudanças breaking → nova versão.
3. **Documentada**: OpenAPI 3.x (REST) + protobuf (gRPC) + tipos TypeScript gerados.
4. **Autenticada**: tokens com escopos granulares.
5. **Auditada**: cada chamada gera entrada no event_log.

## 2. Endpoints REST

Base: `https://servidor:porta/api/v1/`

### 2.1 Autenticação

```
POST /auth/login
{
  "username": "joao",
  "password": "...",
  "loja_id": "uuid"
}
→ {
  "access_token": "...",
  "refresh_token": "...",
  "expires_in": 3600
}
```

OAuth2-like com `scope`:
- `read:catalog`
- `write:catalog`
- `read:sales`
- `write:sales`
- `read:employees`
- `read:cashier`
- `manage:devices`
- `admin:*`

### 2.2 Catálogo

```
GET    /artigos?familia_id=&codigo=&page=&size=
POST   /artigos
GET    /artigos/{id}
PUT    /artigos/{id}
DELETE /artigos/{id}    # soft delete (anular)

GET    /familias
GET    /promocoes
GET    /happy-hour
```

### 2.3 Operação

```
GET    /mesas/abertas
GET    /mesas/{id}/estado
POST   /mesas/{id}/abrir
POST   /mesas/{id}/pedir
  body: {detalhes: [{artigo_id, qt, preco?, complementos?}], empregado_id, posto_id}
POST   /mesas/{id}/anular
POST   /mesas/{id}/transferir
POST   /mesas/{id}/oferta
POST   /mesas/{id}/sub-total
POST   /mesas/{id}/fechar
  body: {pagamentos: [{metodo_id, valor, valor_pago?}], cliente_id?, n_pessoas, atributos?}
GET    /mesas/{id}/sub-total
```

### 2.4 Documentos

```
GET    /documentos?serie=&numero=&data_de=&data_ate=
GET    /documentos/{id}
GET    /documentos/{id}/pdf
POST   /documentos/{id}/reimprimir
POST   /documentos/{id}/estornar
```

### 2.5 Caixa

```
POST   /caixa/abrir
POST   /caixa/fechar
POST   /caixa/movimento
  body: {tipo, valor, metodo_id, observacao?, cliente_id?, empregado_id?}
GET    /caixa/saldo
GET    /sessoes
POST   /sessoes/abrir
POST   /sessoes/fechar
GET    /apuramentos/sessao?empregado_id=&data=
GET    /apuramentos/caixa?caixa_id=&data=
GET    /apuramentos/dia?data=
POST   /dia/fechar
```

### 2.6 Empregados / Clientes

```
GET    /empregados
POST   /empregados
GET    /empregados/{id}

GET    /clientes
POST   /clientes
GET    /clientes/{id}
GET    /clientes/{id}/conta-corrente
POST   /clientes/{id}/conta-corrente/movimento
GET    /clientes/{id}/pontos
POST   /clientes/{id}/pontos/usar
```

### 2.7 Delivery

```
GET    /delivery/pedidos?estado=&entregador_id=
POST   /delivery/pedidos
PATCH  /delivery/pedidos/{id}/estado
  body: {estado: "pronto"|"despachado"|"entregue"|"cancelado", entregador_id?}
```

### 2.8 Reservas

```
GET    /reservas?dia=&mesa_id=&cliente_id=
POST   /reservas
PATCH  /reservas/{id}
DELETE /reservas/{id}
```

### 2.9 Restauração Colectiva

```
GET    /pratos-do-dia?data=
GET    /reservas-refeicao?cliente_id=&data=
POST   /reservas-refeicao
POST   /reservas-refeicao/{uid}/validar
```

### 2.10 Reports

```
GET    /reports/{tipo}?from=&to=&groupby=
GET    /reports/saft-pt?mes=&ano=
```

### 2.11 Sistema

```
GET    /sistema/saude
GET    /sistema/versao
GET    /sistema/licenca
GET    /sistema/impressoras/{id}/estado
POST   /sistema/impressoras/{id}/redireccionar
POST   /sistema/posto/{id}/bloquear
POST   /sistema/relogio/sincronizar
```

## 3. Endpoints WebSocket (Real-time)

Base: `wss://servidor:porta/ws/v1/`

### 3.1 Subscrições

```
{action: "subscribe", topics: ["mesa.alterada", "pedido.submetido", "documento.emitido"]}
```

Eventos recebidos em tempo real (formato JSON estruturado):

```
{
  "event": "mesa.alterada",
  "timestamp": "2024-...",
  "data": {
    "mesa_id": "...",
    "estado_anterior": "aberta",
    "estado_actual": "fechada"
  },
  "actor": {"empregado_id": "...", "posto_id": "..."}
}
```

### 3.2 Comandos

```
{action: "command", op: "imprimir-talão", args: {documento_id: "..."}}
```

## 4. gRPC (alta performance)

Para integrações intensivas (Reports, sync entre lojas).

```protobuf
service FrontOffice {
  rpc ConsultarMesa(MesaQuery) returns (Mesa);
  rpc PedirArtigos(stream Pedido) returns (PedidoResposta);
  rpc EventStream(EventFilter) returns (stream Event);
}
```

## 5. WQL (Query Language)

Linguagem de query interna, semelhante a SQL.

Exemplo:
```sql
SELECT empregado_nome, COUNT(*), SUM(total)
FROM documento
WHERE data_documento = TODAY AND tipo = 'factura_simplificada'
GROUP BY empregado_id
ORDER BY SUM(total) DESC
```

Acedido por:
- API `/query` (POST com query)
- UI de Reports (custom queries)
- Plug-ins

Permissões aplicadas (não acede a tudo).

## 6. SDKs

### 6.1 JavaScript / TypeScript

```ts
import { OpenRestClient } from '@openrest/client';

const client = new OpenRestClient({
  baseURL: 'https://posto.local',
  apiKey: '...'
});

const mesas = await client.mesas.abertas();
await client.mesas.pedir(mesaId, {
  detalhes: [{ artigo_id: '...', qt: 2 }],
  empregado_id: '...'
});
```

### 6.2 Python

```python
from openrest import Client

client = Client(base_url='https://posto.local', api_key='...')
mesas = client.mesas.abertas()
client.mesas.pedir(mesa_id, detalhes=[...])
```

### 6.3 Outros

- C# / .NET
- Java / Kotlin
- Go
- Ruby
- PHP

Gerados automaticamente do OpenAPI.

## 7. Webhooks

Sistema regista listeners externos:

```
POST /webhooks
{
  "url": "https://meu-pms.example.com/openrest-events",
  "events": ["documento.emitido", "mesa.fechada"],
  "secret": "...",
  "ativo": true
}
```

Push com payload:
```json
{
  "event": "documento.emitido",
  "timestamp": "...",
  "data": { ... },
  "signature": "sha256=..."
}
```

## 8. Configuração de Plug-ins

### 8.1 Manifest

Já documentado em `03-modules/09-plugins.md`.

### 8.2 Hooks da UI

```js
// plugin manifest declara:
"ui_hooks": [
  { "place": "pedidos.toolbar", "label": "Imprimir QR", "icon": "qr.svg", "command": "qr.imprimir" },
  { "place": "caixa.fecho_dia", "command": "saft.gerar" }
]
```

### 8.3 Comandos do plug-in

Cada plug-in expõe comandos invocáveis:

```js
plugin.command('qr.imprimir', (args) => {
  // ...
});
```

## 9. Eventos do Sistema (catálogo completo)

Já listado em `03-modules/09-plugins.md §5`. Resumo:

- `app.*`: started, stopping, error
- `dia.*`: aberto, fechado
- `caixa.*`: aberta, fechada, movimento
- `sessao.*`: aberta, fechada, ajuste
- `turno.*`: transferido
- `mesa.*`: aberta, alterada, fechada, transferida, anulada
- `pedido.*`: submetido, secundario_emitido
- `documento.*`: emitido, reimpresso, estornado, anulado
- `mov_caixa.*`: registado
- `empregado.*`: autenticado, sessao_aberta, sessao_fechada
- `cliente.*`: identificado, criado, alterado, anulado
- `reserva.*`: criada, alterada, consumida, cancelada, expirada
- `delivery.*`: recebido, atribuido, despachado, entregue, cancelado
- `pagamento.*`: recebido, estornado, ajustado
- `lista_negra.*`: adicionada, removida
- `comissao.*`: calculada, paga
- `apuramento.*`: gerado, impresso
- `licenca.*`: validada, expirada, alterada
- `manutencao.*`: iniciada, terminada
- `error.*`: fatal, warning, info

## 10. Versionamento

- API REST: `/api/v1`, `/api/v2`, …
- Cada versão mantém-se durante pelo menos 18 meses após substituição
- Deprecation Notices: header `Sunset:` em respostas
- Changelog público

## 11. Rate Limiting

- Burst: 100 req/s por token
- Sustained: 10000 req/h por token
- Configurável por escopo

## 12. Segurança

- TLS obrigatório (excepto em rede local explícita com toggle)
- Tokens com expiração
- Rotação automática de chaves
- IP allowlist opcional
- Audit log de todas as chamadas
- CORS configurável

## 13. Documentação Auto-gerada

- `/api/docs` — Swagger UI
- `/api/openapi.json` — OpenAPI 3.x
- `/api/postman-collection.json` — coleção pronta a importar
