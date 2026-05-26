# OpenRest — Requisitos Não-Funcionais

> Performance, fiabilidade, escalabilidade, manutenibilidade, internacionalização, acessibilidade. Métricas medíveis sempre que possível.

## 1. Performance

### 1.1 Latência de UI

| Operação | Objectivo (P95) | Limite (P99) |
|---|---|---|
| Premir botão (feedback visual) | 50ms | 100ms |
| Abrir mesa | 100ms | 200ms |
| Adicionar artigo ao pedido | 50ms | 100ms |
| Submeter pedido (1 linha) | 200ms | 500ms |
| Submeter pedido (20 linhas, 3 zonas) | 500ms | 1500ms |
| Imprimir conta | 300ms | 1000ms |
| Pesquisa de cliente (10k registos) | 200ms | 500ms |
| Abrir Caixa | 500ms | 2000ms |
| Fechar Dia (loja média) | 5s | 15s |

### 1.2 Throughput

- 100 pedidos/min por loja (modelo médio)
- 1000 documentos/dia por loja (modelo médio)
- Pico: 5× o normal sem degradação

### 1.3 Recursos

- **CPU**: idle < 5%, pico < 50%
- **RAM**: posto < 500MB, servidor (1 loja) < 2GB
- **Disco**: 1GB/ano por loja média
- **Rede**: 1Mbps suficiente para sync (low usage)

### 1.4 Tempo de arranque

- Posto frio: < 5s para ecrã principal
- Servidor frio: < 10s para aceitar conexões
- Quentinho (cache): < 1s para o posto

## 2. Fiabilidade

### 2.1 Uptime

- Posto: 99.9% (excluindo manutenção programada)
- Servidor local: 99.95%
- Servidor central: 99.99%

### 2.2 MTBF / MTTR

- MTBF (Mean Time Between Failures): > 30 dias
- MTTR (Mean Time To Recovery): < 15min

### 2.3 Atomicidade

- Operações que afectam múltiplos ficheiros: transaccionais (all-or-nothing)
- Em caso de crash, reaplica/cancela transacções pendentes
- Garantia: nenhum gap na numeração de documentos por falha técnica
- WAL (Write-Ahead Logging) ou equivalente

### 2.4 Recuperação de falhas

- UPS no servidor (mínimo 30min)
- Detecção de falha de rede em < 30s
- Reconciliação automática quando rede volta
- Backups diários, retenção 90 dias

### 2.5 Modo Degradado

OpenRest **nunca para de vender**. Mesmo com:
- Sem central
- Sem internet
- Sem balança / leitor / display
- Sem impressora (avisa e oferece alternativa)

## 3. Escalabilidade

### 3.1 Por loja

- Até 50 postos por servidor (rede LAN)
- Até 10.000 mesas/cartões
- Até 100.000 artigos
- Até 1.000.000 documentos/ano

### 3.2 Por rede de lojas

- Até 1.000 lojas por servidor central
- Pode escalar horizontalmente (sharding por loja)

### 3.3 Catálogo

- Até 1.000.000 artigos na BD central
- Sync delta-only para evitar push completo

## 4. Manutenibilidade

### 4.1 Logging

- Logs estruturados (JSON)
- Níveis: TRACE, DEBUG, INFO, WARN, ERROR
- Rotação automática
- Centralização opcional (Loki, Elasticsearch)

### 4.2 Monitoring

- Métricas Prometheus
- Dashboards Grafana
- Alertas configuráveis
- Health endpoint `/health`

### 4.3 Debugging

- Modo debug com mais logs
- Replay de eventos para reproduzir bugs
- Crash dumps opcional (Sentry-compatible)

### 4.4 Updates

- Updates seamless (sem downtime)
- Rollback automático em caso de falha
- Migrations de BD versionadas
- Backward compatibility entre minor versions

## 5. Internacionalização

### 5.1 Línguas

Suporte day-1:
- Português (PT)
- Português (BR)
- Espanhol (ES, ES-AR, ES-MX)
- Inglês (UK, US)
- Francês (FR)

Roadmap:
- Italiano, Alemão, Russo, Polaco
- Chinês simplificado / tradicional
- Árabe (RTL)

### 5.2 Localização

- Formatação de números (separador decimal, milhares)
- Formatação de datas / horas
- Moeda (símbolo, posição, casas decimais)
- Calendário (Gregoriano, Hijri opcional)
- Direção (LTR, RTL)
- Pluralização (regras CLDR)

### 5.3 Locales por instalação

- Locale global (default)
- Locale por empregado (UI muda no login)
- Locale por documento (alguns templates podem ser multilingue)

## 6. Acessibilidade

### 6.1 Padrões

- WCAG 2.1 nível AA
- ARIA roles e labels
- Navegação por teclado completa

### 6.2 Suporte

- Leitor de ecrã (NVDA, JAWS, VoiceOver)
- Alto contraste
- Tamanhos de fonte ajustáveis
- Daltonismo: cores adaptadas

### 6.3 UI Operadores

- Botões ≥ 48×48px
- Toques duplos opcionais
- Confirmação para acções destrutivas

## 7. Compatibilidade

### 7.1 OS

- Windows 10 / 11 (x64)
- Windows Server 2019+
- Ubuntu 22.04 LTS / 24.04 LTS
- Debian 12+
- macOS 13+ (apenas BackOffice)
- Android 10+ (mobile)
- iOS 16+ (PWA somente)
- Linux embarcado (POSReady-like distros)

### 7.2 Browser (BackOffice)

- Chrome / Edge últimas 2 versões
- Firefox últimas 2 versões
- Safari últimas 2 versões

### 7.3 Hardware

- POS modernos (Intel/AMD x64, ARM)
- Tablets Android e iOS
- Impressoras ESC/POS (Epson, Star, Bixolon, …)
- Impressoras fiscais (BR: Bematech, Daruma; outras conforme país)
- Leitores via teclado (HID)
- Leitores via série (RS232, USB-Serial)
- Gavetas via impressora ou directas
- Balanças padronizadas (Toledo, Bizerba, …)
- Displays VFD / LCD via série
- Câmaras IP / USB
- Pagamento: TPA via porta série, USB, IP

### 7.4 Compatibilidade WinREST (legado)

- Importação de mestres `wrst*.000`
- Suporte conceptual (mesmas operações, mesma terminologia)
- Drivers de hardware comuns

## 8. Segurança (resumo)

Documentado em `01-architecture/04-security.md`. Resumo:

- Senhas argon2id
- TLS 1.3 entre componentes
- mTLS opcional
- RBAC granular
- Audit log inviolável
- Encriptação at-rest opcional
- GDPR + PCI-DSS compliance

## 9. Compliance Fiscal

Documentado em `05-integrations/02-fiscal-compliance.md`. Resumo:

- SAF-T PT
- ATCUD
- QR Code
- Hash de assinatura PT
- Comunicação à AT
- Equivalentes em outros países

## 10. Documentação

- Manuais para utilizador, técnico, integrador
- Multi-idioma (mesmas línguas do produto)
- Pesquisa full-text
- Versionada (cada versão major mantém sua doc)
- Inclui vídeos / GIFs para fluxos complexos

## 11. Suporte

### 11.1 Canais

- Issues no GitHub (comunidade)
- Forum de discussão
- Discord/Matrix para chat
- Suporte comercial (empresas)

### 11.2 SLAs (versão comercial)

- Resposta inicial: 4h (críticos), 24h (normal)
- Resolução: depende da gravidade

### 11.3 Bugs críticos (open-source)

- Triagem em 48h
- Fix em days para bugs críticos
- CVEs reportados publicamente

## 12. Testes

### 12.1 Cobertura

- Unitários: 80%+
- Integração: cobertura dos fluxos principais
- E2E: cenários completos de utilização
- Performance: benchmarks repetidos por commit

### 12.2 Tipos

- Unit (Rust: cargo test)
- Integration (testcontainers)
- E2E (Playwright)
- Performance (Criterion)
- Mutation (cargo-mutants)
- Property-based (proptest)
- Chaos engineering (toxiproxy)

### 12.3 Ambientes

- Local (developer)
- CI (cada PR)
- Staging (pré-produção)
- Production (canary deploys)

## 13. Versionamento

SemVer (MAJOR.MINOR.PATCH):
- MAJOR: breaking changes
- MINOR: novas features compatíveis
- PATCH: bug fixes

LTS (Long Term Support):
- 1 LTS por ano
- Suporte mínimo 24 meses
- Suporte estendido (comercial) opcional

## 14. Distribuição

- Releases públicas via GitHub
- Mirrors em CDN (downloads grandes)
- Repositórios de pacotes (apt, dnf, brew, chocolatey, scoop)
- Docker Hub / GHCR

## 15. Privacidade

### 15.1 Coleta de Dados

- Por defeito: zero telemetria
- Opt-in para anonymous usage stats
- Crash reports opt-in

### 15.2 Cookies / Web

BackOffice: cookies estritamente necessários (sessão). Sem trackers.

### 15.3 Dados de Clientes (do utilizador final)

Vê GDPR em `04-security.md §8`.

## 16. Sustentabilidade

- Carbon footprint medido
- Optimização para hardware existente (não forçar upgrade)
- LCD com baixo brilho por defeito
- Modo "eco" reduz refreshes desnecessários

## 17. Acessibilidade financeira

- Versão Community totalmente gratuita
- Versão Enterprise paga com suporte
- Plug-ins comerciais opcionais
- Sem lock-in de dados (export sempre possível)
