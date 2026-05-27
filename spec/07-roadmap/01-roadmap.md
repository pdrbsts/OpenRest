# OpenRest — Roadmap

> Plano de fases / milestones para chegar de zero a software de POS completo, certificável e estendível. Datas estimadas — não compromissos.

## Visão de alto nível

```
Fase 0 — Fundação              (mes 1-2)
Fase 1 — POS Mínimo             (mes 3-5)
Fase 2 — POS Funcional Completo  (mes 6-9)
Fase 3 — Caixa Avançada e Reports (mes 10-12)
Fase 4 — Compliance PT Total    (mes 13-15)
Fase 5 — Multi-Loja              (mes 16-18)
Fase 6 — Mobile / Plug-ins      (mes 19-21)
Fase 7 — Certificação e GA      (mes 22-24)
Fase 8+ — Crescimento contínuo
```

## Fase 0 — Fundação (M0 – M2)

**Objectivo**: ambiente de desenvolvimento, modelo de domínio, infraestrutura de eventos, fundamentos da BD.

Entregas:
- Repositório com estrutura monorepo
- CI/CD pipeline
- Modelo de domínio definido (Rust crates)
- Schema SQLite e migrations
- Sistema de eventos (NATS embebido)
- Documentação técnica interna
- Testes unitários para domínio
- Logging estruturado

Critérios de saída:
- Build passa em Windows, Linux, macOS
- Cobertura >70% nos crates de domínio
- Spec aprovada e em sync com implementação

## Fase 1 — POS Mínimo (M3 – M5)

**Objectivo**: vender um café. Configurar um artigo, abrir uma mesa, registar, fechar mesa, imprimir.

Entregas:
- Backend HTTP com endpoints básicos
- Frontend Tauri com ecrã de pedidos básico
- Catálogo: famílias, sub-famílias, artigos
- Mesas (modelo normal)
- Empregados básicos
- Recebimento simples (1 método)
- Documento de Pedido + Consulta de Mesa
- Impressora genérica ESC/POS
- Configuração mínima
- Fatura legal PT (QR Code, ATCUD, dados fiscais da empresa)

Critérios de saída:
- Posso vender 1 produto, ver na consulta de mesa, fechar, imprimir fatura legal PT
- Latência alvo cumprida nos fluxos básicos

## Fase 2 — POS Funcional Completo (M6 – M9)

**Objectivo**: cobrir 90% dos casos de uso de uma casa standard.

Entregas:
- Locais com configuração completa
- Modos: Normal, Take-Away, Delivery, Consumo Próprio
- Zonas de impressão + mapping
- Pedidos secundários
- Anulação (com/sem desperdício) e Cancelamento
- Transferência de mesa
- Ofertas
- Pagamento parcial, múltiplos métodos, divisão de conta
- Identificação de cliente (com NIF)
- Promoções (menus) com escolha de itens
- Happy Hour
- Artigos automáticos
- Macros
- Páginas Rápidas
- Mapa de mesas (clique em imagem)
- Display de cliente VFD / Monitor de cliente
- Balança
- Leitor de cartões e códigos de barras
- Documentos configuráveis (cabeçalhos/rodapés com flags)
- Templates VD e Factura
- Caixa básica (abrir, fechar)
- Sessões de empregados
- Comunicação de séries com a AT por webservice

Critérios de saída:
- Loja-piloto consegue operar um dia completo
- Todos os tipos de local funcionam
- Performance dentro dos targets

## Fase 3 — Caixa Avançada e Reports (M10 – M12)

**Objectivo**: ferramentas de gestão financeira e reporting.

Entregas:
- Hierarquia completa Dia/Turno/Caixa/Sessão
- Bolsas
- Todos os movimentos de caixa (compras, vales, fundos, retiradas, empréstimos, envelopes)
- Fecho de Caixa, Fecho de Dia
- Fecho Financeiro (mapa económico + financeiro)
- Apuramentos: Sessão, Caixa, Turno, Dia
- Estatísticas (gráficos: vendas/clientes/custos)
- Relatórios diários:
  - Assiduidade
  - Vendas por Turno
  - Apuramento de IVA
  - Consulta de registos
  - Refeições
  - Descontos a Clientes
  - Relatório Diário
  - Saldo Clientes
  - Vendas Negativas
- Configuração de relatórios por dia/turno/caixa/sessão
- Botões de atalho na janela Caixa
- Comissões (variáveis e fixas)
- Conta Corrente de Cliente (totais, limite, associação)
- Pontos de fidelidade

Critérios de saída:
- Fecho do dia funciona com todos os apuramentos
- Comissões calculadas correctamente em cenários complexos
- CC funciona com clientes associados

## Fase 4 — Compliance PT Total (M13 – M15)

**Objectivo**: pronto para certificação fiscal em Portugal.

Entregas:
- Hash de assinatura (Portaria 363/2010)
- ATCUD
- QR Code com payload completo
- SAF-T PT export (validado contra XSD)
- Comunicação de séries à AT (plug-in)
- Comunicação de documentos à AT (plug-in)
- Cadeia de documentos íntegra
- Notas de crédito (estorno)
- Imutabilidade fiscal estrita
- Audit log inviolável
- Identificação eventual de cliente (PT — janela NIF)
- Validação de NIF PT
- Limites legais (FS sem NIF > €1000)

Critérios de saída:
- Auditor independente certifica integridade da cadeia
- SAF-T validado por software fiscal oficial
- Submissão para AT

## Fase 5 — Multi-Loja (M16 – M18)

**Objectivo**: suportar cadeias e franchising.

Entregas:
- Modelo multi-loja
- Servidor central (PostgreSQL)
- BackOffice web (gestão central)
- Sincronização bidireccional (eventos)
- Catálogo central com push para lojas
- Reports consolidados
- Reservas inter-loja
- Pedidos delivery cross-loja
- Mensagens remotas
- Sync resiliente (offline tolerant)
- Vector clocks / CRDT
- Conflict resolution

Critérios de saída:
- 3 lojas a operar em paralelo com sync
- Cair internet numa loja não afecta operação local
- Reports consolidados precisos

## Fase 6 — Mobile, Plug-ins e Hardware Extras (M19 – M21)

**Objectivo**: tablets, plug-ins, hardware especializado.

Entregas:
- App mobile (PWA + Android nativo)
- Sistema de plug-ins com WebAssembly
- Plug-in store
- Plug-ins essenciais:
  - SAF-T PT (já em F4)
  - Primavera Export
  - Videovigilância
  - Máquina de Café
  - Restauração Colectiva
  - Timeclock
  - Fecho Financeiro
- Drivers para mais hardware:
  - Impressoras fiscais BR / outras
  - Balanças adicionais (Toledo, Bizerba)
  - POS integrados (Bleep, NCR, …)
  - Identificadores de chamada
- Marketplace de plug-ins
- Integração TheFork (plug-in)
- Integração Uber Eats / Glovo (plug-in)

Critérios de saída:
- Tablet Android operando em delivery
- 5+ plug-ins funcionais
- Integração com 1+ plataforma externa

## Fase 7 — Certificação e GA (M22 – M24)

**Objectivo**: produto v1.0 GA, certificado AT, marketing.

Entregas:
- Submissão à AT (PT) + atribuição de número
- Documentação completa (técnica + utilizador) em PT, EN, ES
- Material de marketing
- Site público
- Vídeos / demos
- Programa de parceiros (integradores)
- Casos de estudo (lojas-piloto)
- Press releases
- Versão Enterprise comercial

Critérios de saída:
- v1.0.0 GA released
- Certificação AT recebida
- 100+ lojas a usar em produção

## Fase 8+ — Crescimento Contínuo (M25+)

**Tópicos prováveis**:
- Outras certificações fiscais (ES, BR, AO, LU, …)
- Integrações com mais marketplaces
- IA / Analytics (sugerir preços, optimizar menus, prever picos)
- Reservas online públicas
- Loyalty avançada (campanhas)
- POS Mobile nativo (não PWA)
- Self-order (kiosks)
- Self-pay (app cliente paga em mesa)
- Voice ordering
- Multi-tenant SaaS managed
- Mais idiomas

## Cronograma Visual (Gantt simplificado)

```
M0  M3  M6  M9  M12 M15 M18 M21 M24
|---|---|---|---|---|---|---|---|
F0
   F1
       F2
              F3
                  F4
                      F5
                         F6
                            F7
                                F8+
```

## Marcos / Demos públicas

- **M3 — Demo "Hello Café"**: vender 1 café num posto
- **M6 — Demo "Restaurante de Bairro"**: operação dia completo
- **M9 — Demo "Take-away + Delivery"**: 2 modos a operar
- **M12 — Demo "Cantina"**: restauração colectiva com torniquete
- **M15 — Demo "Fiscalidade PT"**: SAF-T válido, QR Code, ATCUD
- **M18 — Demo "Cadeia"**: 3 lojas sincronizadas
- **M21 — Demo "Tablet + Plug-ins"**: ecosistema completo
- **M24 — Launch v1.0**: GA

## Equipa estimada

### Fase 0-2 (small team)
- 1 architect / lead
- 2 backend engineers (Rust)
- 1 frontend engineer
- 1 devops (part-time)
- 1 product manager (part-time)

### Fase 3-5 (growing)
- + 2 engineers
- + 1 mobile engineer
- + 1 designer
- + 1 QA

### Fase 6-7 (ramp-up)
- + 2 plug-in specialists
- + 1 fiscal specialist
- + 1 technical writer
- + 1 customer success

Pico ≈ 12-15 pessoas. Pós-launch, manter 8-10 core.

## Riscos e Mitigações

| Risco | Probabilidade | Impacto | Mitigação |
|---|---|---|---|
| Certificação AT demora muito | Média | Alto | Começar cedo, plug-in dedicado |
| Compatibilidade legado WinREST complexa | Alta | Médio | Limitar a importação de mestres |
| Performance em hardware POS antigo | Média | Alto | Targetar Linux embarcado cedo |
| Adopção lenta (mercado conservador) | Alta | Alto | Parcerias com integradores |
| Bugs em produção fiscal | Baixa | Crítico | Testes exaustivos, beta-program |
| Concorrência (proprietária e open) | Alta | Médio | Foco em UX e ecossistema |

## Métricas de sucesso

### Adopção
- 100 lojas em produção até M30
- 1.000 lojas até M48
- 10.000 lojas até M72

### Comunidade
- 50 contribuidores até M24
- 200 forks / 500 stars até M24
- Discord activo (>500 membros até M24)

### Comercial (opcional)
- 10 customers Enterprise até M30
- 100 customers Enterprise até M48

### Qualidade
- Bug críticos resolvidos em <72h
- Updates seamless
- NPS > 50

## Versões alvo

- **v0.1.0** — Fim Fase 0 (interno)
- **v0.2.0** — Fim Fase 1 (alpha)
- **v0.5.0** — Fim Fase 2 (beta)
- **v0.7.0** — Fim Fase 3
- **v0.8.0** — Fim Fase 4
- **v0.9.0** — Fim Fase 5 (release candidate)
- **v0.99.0** — Fim Fase 6
- **v1.0.0** — Fim Fase 7 (GA)
