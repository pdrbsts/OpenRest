# OpenRest — Módulo Plug-ins / Extensões

> Sistema de extensibilidade para terceiros. Todos os módulos opcionais (incluindo alguns historicamente "core" como SAF-T PT) são tratados como plug-ins de primeira classe.

## 1. Arquitectura

Cada plug-in vive em `plugins/<chave>/` com:

```
plugins/
└── <chave>/
    ├── plugin.json     Manifesto
    ├── README.md
    ├── icons/
    ├── config.schema.json
    ├── i18n/
    └── bin/ (ou) src/
```

### 1.1 Manifesto (`plugin.json`)

```json
{
  "key": "saft_pt",
  "name": {"pt": "SAF-T PT", "en": "SAF-T PT"},
  "version": "1.2.0",
  "author": "OpenRest Foundation",
  "license": "MIT",
  "homepage": "https://...",
  "openrest_min_version": "1.0.0",
  "modules_required": [],
  "execute": {
    "windows": {"command": "internal(saft_pt)", "parameters": "", "workdir": ""},
    "linux":   {"command": "internal(saft_pt)", "parameters": "", "workdir": ""}
  },
  "execution": {
    "server": true,
    "client": false,
    "maintenance": true,
    "normal": true
  },
  "access": {
    "exclude_levels": []
  },
  "events_subscribed": [
    "documento.emitido",
    "fecho_dia.confirmado"
  ],
  "ui": {
    "menu": "tools",
    "icon": "saft.png"
  }
}
```

### 1.2 API exposta

OpenRest expõe API ao plug-in:
- **Read API** — consultar entidades (com permissões)
- **Event Bus** — publicar/subscrever eventos
- **Document API** — emitir/reimprimir documentos
- **Print API** — enviar para impressora específica
- **UI Hook** — registar botões/janelas
- **Config API** — guardar configurações próprias
- **i18n API** — traduções

### 1.3 Linguagens suportadas

- **Internal** — plug-in compilado dentro do binário OpenRest (escolha histórica)
- **External (binary)** — executável separado comunicando por IPC (socket, pipe, gRPC)
- **External (script)** — Python, Lua, JS (Node) com sandbox configurável
- **Webhook** — apenas chamadas HTTP para serviço externo

## 2. Plug-ins de base (incluídos)

### 2.1 SAF-T PT

Exporta vendas em formato SAF-T PT (XML conforme Portaria 302/2016).

- Por intervalo de datas
- Auto-export mensal
- Validação de schema
- Inclui assinaturas / hashes (cadeia exigida)

### 2.2 Comunicação de Séries à AT (PT)

Comunica novas séries de documentos à Autoridade Tributária via WebService.
Retorna `CodigoValidacao` que vai compor o ATCUD.

### 2.3 Comunicação de Documentos à AT (PT)

Comunica resumos diários ou em tempo real (conforme legislação).

### 2.4 SAF-T equivalentes para outros países

- SAF-T AO (Angola)
- SAF-T LU (Luxemburgo)
- Modelos similares em ES, BR (NFC-e, NFC-S), etc.

### 2.5 Primavera Export

Exporta dados para sistemas Primavera (contabilidade).

### 2.6 MiliStore (Gestão de Stocks Básica)

Plug-in BackOffice básico:
- Importa artigos/famílias do OpenRest
- Custos diários de operação
- Comparar custos × vendas (margens)
- Encomendas e recepções
- Inventários
- Consulta existências

### 2.7 Ticket (Publicidade)

Modelo de negócio histórico (GrupoPIE): conteúdos publicitários impressos cíclicamente nos tickets.

Em OpenRest: framework genérico para publicidade nos tickets (próprios ou de terceiros), com:
- Campanhas activas
- Prioridades
- Censura local
- Sync com servidor central

### 2.8 Videovigilância

Liga eventos do POS a câmaras de vídeo. Eventos linkáveis:
- Abertura de caixa
- Fecho de documento (VD/conta/factura)
- Movimentos de caixa
- Cancelamentos
- Anulações / estornos
- Tirar conta
- Entrar / sair de manutenção
- Login de empregado

Para cada evento:
- Imagem actual do FO (sincronizada)
- Imagem do documento impresso
- Filme da câmara

Triggers:
- Email/SMS quando X anulações
- Alertas em tempo real

Hospedagem: PC servidor com web local; serviço IP dinâmico (tipo DynDNS) opcional para acesso remoto.

### 2.9 NetPay e Sistemas de Pagamento

Integração com terminais de pagamento via:
- Protocolo proprietário
- Norma EMV
- TEF (Brasil)
- SiBS (PT)

Tipo: dispositivo + plug-in.

### 2.10 W4 (Browser Embebido)

Browser controlado para acesso a portais de parceiros (distribuidores, prestadores, entidades financeiras) numa ambiente seguro:
- Imune a vírus
- Limitado a sites autorizados
- Optimizado para touchscreen

Em OpenRest: modernizado para WebView com whitelist + sandbox.

### 2.11 Máquina de Café (CCI/CSI)

Já documentado em `05-kitchen-and-routing.md §9`. Controla doses produzidas por máquina de café electrónica.

### 2.12 Restauração Colectiva

Pode ser modelado como plug-in. Activa novas opções de UI, novas configurações, gestão de senhas com UID. Já documentado em `07-collective-catering.md`.

### 2.13 Fecho Financeiro

Plug-in com mapa económico + financeiro. Substitui o ecrã de retirada por envelopes. Já documentado em `03-cash-register.md §8.4`.

### 2.14 Timeclock (Relógio de Ponto)

Marcação de assiduidade independente da sessão. Já documentado em `06-employees-clients.md §5`.

### 2.15 Reports

Relatórios avançados com intervalos de datas e configurações persistíveis. Já documentado em `03-cash-register.md §10`.

### 2.16 Reservas

Gestão de reservas de mesa (cronograma, edição, lista negra). Pode ser plug-in para casas que não precisam.

### 2.17 ComServer

Servidor de comunicação remota:
- Envia/recebe ficheiros de outras lojas
- Sync com servidor central
- WinREST Ticket comms
- Notificações push

### 2.18 Encomendas (B2C Web/App)

Recebe encomendas via API de canais externos (app de cliente, site, marketplaces).

## 3. Plug-ins de hardware

- Drivers para modelos específicos de impressora fiscal
- Drivers para balanças especiais
- Adaptadores para PMS (hotel)
- Adaptadores para integradores como Deliveroo / Uber Eats / Glovo

## 4. Gestão de plug-ins

### 4.1 Marketplace

UI no BackOffice com:
- Lista de plug-ins disponíveis (oficiais + comunidade)
- Instalação com um clique
- Actualizações automáticas
- Avaliações

### 4.2 Sandbox e segurança

- Plug-ins externos correm em container/sandbox
- Permissões explícitas (acesso a BD, hardware, rede)
- Auditoria de actividade

### 4.3 Configuração

Cada plug-in expõe `config.schema.json` que gera UI automaticamente:

```json
{
  "type": "object",
  "properties": {
    "api_key": {"type": "string", "format": "secret"},
    "url_endpoint": {"type": "string", "format": "uri"},
    "intervalo_envio_minutos": {"type": "integer", "minimum": 1, "default": 15}
  }
}
```

### 4.4 Acesso por nível

`exclude_levels` no manifesto define quem **não** pode executar. UI esconde para esses utilizadores.

### 4.5 Modo execução

- **Servidor** — só corre no servidor (relatórios pesados)
- **Cliente** — corre em postos
- **Manutenção** — só em modo manutenção
- **Normal** — em operação corrente

Pode ser ambos.

## 5. Eventos do sistema

O sistema emite eventos que plug-ins subscrevem:

- `app.started`, `app.stopping`
- `dia.aberto`, `dia.fechado`
- `caixa.aberta`, `caixa.fechada`
- `sessao.aberta`, `sessao.fechada`
- `mesa.aberta`, `mesa.fechada`
- `pedido.submetido`
- `documento.emitido`, `documento.estornado`, `documento.reimpresso`
- `mov_caixa.registado`
- `empregado.identificado`
- `cliente.identificado`, `cliente.criado`
- `reserva.criada`, `reserva.consumida`, `reserva.cancelada`
- `delivery.recebido`, `delivery.despachado`, `delivery.entregue`
- `pagamento.recebido`, `pagamento.estornado`
- `anulacao.aplicada`
- `oferta.aplicada`
- `transferencia.aplicada`
- `lista_negra.adicionada`, `lista_negra.removida`
- `comissao.calculada`
- `apuramento.gerado`
- `error.fatal`, `error.warning`

Payload em JSON estruturado.

## 6. API de impressão

Plug-ins podem:
- Emitir documentos com template próprio
- Imprimir em impressoras específicas
- Solicitar reimpressões
- Receber notificações de fila / falha

## 7. Casos de uso especiais

### 7.1 PMS de hotel

Plug-in que:
- Recebe consumos de bar/restaurante do hotel via OpenRest
- Devolve número de quarto/conta
- Push para o PMS quando consumo é "facturado externamente"
- Local específico `facturacao_externa=true`

### 7.2 Marketplaces (Uber Eats, Glovo)

Plug-in que:
- Recebe encomendas via webhook
- Cria pedidos delivery no OpenRest
- Sync de menu (artigos disponíveis, preços)
- Push de status (pronto, entregue)

### 7.3 Plataformas de reservas (TheFork)

Plug-in que:
- Recebe reservas da plataforma
- Cria entries em `reserva`
- Notifica plataforma de no-show / cancelamento

### 7.4 Sistemas de fidelidade (loyalty cards externos)

Plug-in que:
- Valida cartão externo
- Reporta pontos ganhos
- Aplica resgates

### 7.5 Cozinha conectada (Smart Kitchen)

Plug-in que:
- Envia pedidos para sistemas de "kitchen automation"
- Recebe estimativas de tempo
- Coordena com KDS
