# OpenRest — Arquitectura: Stack Tecnológica

> Stack proposta para OpenRest. Decisões sujeitas a revisão; o objectivo é encontrar o equilíbrio entre experiência moderna, autonomia offline, robustez transaccional e baixa fricção de instalação.

## 1. Princípios de selecção

1. **Mainstream** — preferir tecnologias com comunidade activa, documentação extensa, contratação fácil.
2. **Cross-platform** — Windows, Linux (incluindo distros embarcadas em POS) e Android.
3. **Capaz de offline** — postos podem operar sem rede; sincronização eventual.
4. **Performance** — UI responsiva (<100ms para acções comuns).
5. **Footprint razoável** — não exigir hardware de gama alta.
6. **Estendível** — facilidade de criar plug-ins.

## 2. Stack proposta — visão consolidada

```
┌─────────────────────────────────────────────┐
│           UI / Posto (Desktop)              │
│  Tauri (Rust + Web Frontend)                │
│  Frontend: SolidJS ou Svelte                │
│  Estilização: TailwindCSS                   │
└─────────────────┬───────────────────────────┘
                  │ HTTP / WebSocket / IPC
┌─────────────────▼───────────────────────────┐
│           UI / Posto (Mobile/Tablet)        │
│  PWA (mesmo frontend) ou Flutter            │
└─────────────────┬───────────────────────────┘
                  │
┌─────────────────▼───────────────────────────┐
│             Servidor Local                  │
│  Linguagem: Rust ou Go                      │
│  Framework: Axum (Rust) / Echo (Go)         │
│  Async runtime: Tokio (Rust)                │
│  ORM: SQLx / SeaORM (Rust)                  │
└─────────────────┬───────────────────────────┘
                  │
        ┌─────────┴─────────┐
        │                   │
┌───────▼─────┐    ┌────────▼────────┐
│  SQLite     │    │  Event Bus      │
│  (BD local) │    │  (NATS embed)   │
└─────────────┘    └─────────────────┘

         ↕ Sync (opcional)
┌─────────────────────────────────────────────┐
│            Servidor Central                 │
│  PostgreSQL                                 │
│  API REST + gRPC                            │
└─────────────────────────────────────────────┘
```

## 3. Componentes

### 3.1 Backend (servidor local na loja)

**Linguagem**: **Rust** (preferido) ou Go (alternativa).

Razões para Rust:
- Performance comparável a C++
- Garantias de segurança em memória
- Excelente ecosistema async (Tokio)
- Compilação cross-platform
- Distribuição como binário único (sem runtime)
- Comunidade de POS/embedded em crescimento

Razões para Go (alternativa):
- Curva de aprendizagem mais suave
- Compilação simples
- Boas bibliotecas para POS (algumas)

**Framework Web**: Axum (Rust) ou Echo/Fiber (Go).

**Persistência**:
- **SQLite** local (uma BD por loja).
- WAL mode para concorrência.
- `sqlite-vec` ou similar para busca semântica (se necessário).
- Sincronização: change-log + CRDT ou event-sourcing.

**Event Bus**:
- **NATS** embebido (single-node) ou **Redpanda** (multi-node).
- Pub/sub para eventos do sistema.
- Comunicação postos-servidor.

**Background workers**:
- Tarefas async (exportação SAF-T, sync, retries de impressão).

**Cache**:
- In-memory para fichas frequentemente acedidas.

### 3.2 Frontend (UI do posto)

**Plataforma**: **Tauri** (binário desktop com webview leve).

Razões:
- Bundle pequeno (~10MB vs Electron ~100MB)
- Backend em Rust (mesma linguagem do servidor)
- Acesso nativo (hardware, ficheiros)
- Cross-platform

**Framework Frontend**: **SolidJS** ou **Svelte**.

Razões:
- Compiladas (sem virtual DOM overhead)
- Performance superior a React em UI densa
- Componentes reactivos

**Estilização**: **TailwindCSS** + componentes customizados.

**Comunicação posto ↔ servidor**:
- HTTP REST (operações)
- WebSocket (events, state sync)
- IPC Tauri (acesso a hardware local)

### 3.3 Mobile / Tablet

**Opção A**: **PWA** (Progressive Web App) — mesmo código que desktop, instalável.

**Opção B**: **Flutter** — UI nativa rica, melhor performance em hardware fraco.

A escolher conforme avaliação de hardware-alvo. Default: PWA.

### 3.4 Servidor Central (multi-loja)

**Linguagem**: Rust (consistência).

**Persistência**: **PostgreSQL 15+**.

**Cache**: **Redis**.

**Object Storage**: **MinIO** (self-hosted) ou S3-compatível.

**Mensageria**: **NATS Streaming** ou **Kafka**.

**Deployment**: **Kubernetes** (para escala) ou Docker Compose (para casas).

### 3.5 Sincronização

**Modelo**: hybrid event-sourcing + CRDT.

- **Catálogo** (artigos, famílias, clientes, empregados): replicação central-para-loja com `last-write-wins`.
- **Operação** (documentos, movimentos): event-sourcing por loja, agregação central.
- **Configuração de hardware**: por loja, não sync.

Protocolo: gRPC streams bidireccionais com resilience.

### 3.6 Plug-ins

**Runtime opções**:
- **WebAssembly** (recomendado): sandbox seguro, performante, multi-linguagem (Rust, Go, AssemblyScript, etc.).
- **Lua** embebido: simples para scripts curtos.
- **External process**: para plug-ins pesados (executáveis comunicando por IPC/HTTP).

API Plug-in expõe:
- Read API
- Event Bus
- Print API
- UI Hooks
- Storage isolado

### 3.7 Compatibilidade legado

Para integradores que já têm sistemas integrados com WinREST:
- API REST com endpoints "compatibilidade" (mimics legado)
- Importador de dados do WinREST (ficheiros `wrst*.000`)
- Driver de Hardlock virtual (para integradores que precisam)

## 4. Stack alternativa (mais conservadora)

Para equipas com menos experiência em Rust:

```
Backend: .NET 8 (C#) ou Java 21 (Spring Boot)
Frontend: Electron + React/Vue
DB: PostgreSQL (mesmo localmente)
Mobile: React Native
```

Trade-offs:
- ✅ Equipa mais facilmente recrutada
- ✅ Ferramentas maduras
- ❌ Footprint maior
- ❌ Binários maiores
- ❌ Startup mais lento

## 5. Embarcado / IoT

Para hardware antigo (POS de baixa especificação):
- Versão "light" do backend (sem todas as features)
- Frontend ainda mais leve (HTML + JS vanilla)
- Targetar Linux embarcado (Yocto, Buildroot)

## 6. Versões

- **OpenRest Server**: o backend completo
- **OpenRest Posto (desktop)**: a UI no terminal
- **OpenRest Posto (mobile)**: PWA / Flutter
- **OpenRest Cloud**: BackOffice central
- **OpenRest Kit**: imagem pré-configurada (Linux + OpenRest) para POS dedicados

## 7. Build & Distribuição

### 7.1 Builds

- CI: GitHub Actions
- Cross-compilation para Windows, Linux (x86_64, aarch64), macOS, Android
- Releases assinados
- Updates automáticos (opcional)

### 7.2 Distribuição

- Binários nas releases do GitHub
- Pacotes:
  - `.exe` instalador (Windows)
  - `.deb`, `.rpm`, AppImage (Linux)
  - `.dmg` (macOS — para BackOffice)
  - `.apk` (Android — Posto mobile)
  - `.flatpak` / `.snap` (Linux desktop)
- Docker images
- Helm charts (Kubernetes)

### 7.3 Estrutura do repositório

Monorepo com múltiplos crates/módulos:

```
openrest/
├── crates/
│   ├── server/           Backend principal
│   ├── domain/           Modelo de domínio
│   ├── storage/          Persistência
│   ├── sync/             Sincronização
│   ├── api-rest/         REST API
│   ├── api-grpc/         gRPC
│   ├── api-ws/           WebSocket
│   ├── devices/          Drivers de hardware
│   ├── fiscal-pt/        Conformidade PT
│   ├── fiscal-es/        Conformidade ES
│   ├── fiscal-br/        Conformidade BR
│   └── …
├── apps/
│   ├── posto/            Tauri app
│   ├── mobile/           PWA / Flutter
│   ├── backoffice/       Web app
│   └── cli/              Linha de comandos
├── plugins/
│   ├── saft-pt/
│   ├── primavera-export/
│   └── …
├── docs/                 Especificações (este projecto)
├── tools/                Utilitários
└── .github/
```

## 8. Telemetria e Observabilidade

- Logging estruturado (JSON)
- Tracing distribuído (OpenTelemetry)
- Métricas (Prometheus)
- Dashboards (Grafana)
- Alertas (configurável)
- Crash reports opcionais (Sentry-compatible, self-hosted)

## 9. Segurança da Stack

- Dependencies scanning (Dependabot)
- SBOM (Software Bill of Materials) em cada release
- Vulnerability disclosures: política RFC 9116 (`security.txt`)
- Auditoria periódica (pentest)
- Compliance: ISO 27001 (longo prazo), PCI-DSS (se processamento de cartão)

## 10. Padrões de Código

- Linguagem: Inglês para código, comentários, commits, PRs
- Português PT-PT para documentação de utilizador (com tradução en/pt-BR/es)
- Estilo: rustfmt, gofmt, prettier
- Lint: clippy, eslint
- Testes: 80%+ coverage
- Documentação: rustdoc, godoc, jsdoc

## 11. Licenciamento

Decisão pendente entre:
- **MIT** (permissiva) — uso comercial sem restrições
- **AGPL** (copyleft forte) — protege contra forks fechados em SaaS
- **MPL** (copyleft fraco) — meio-termo

Recomendação inicial: **AGPL** para protecção, com excepção para o cliente final (utilizador não-distribuidor).
