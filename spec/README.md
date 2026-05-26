# OpenRest — Especificação

> Especificação completa do **OpenRest**, a versão open-source do WinREST FrontOffice PRO. Este conjunto de documentos é o ponto de partida vinculativo para a construção do produto. Os manuais técnico e de utilizador do WinREST estão na raiz do repositório e foram usados como referência de domínio.

**Versão da especificação**: 0.1.0 (baseline)
**Estado**: rascunho aprovado para implementação
**Data**: 2026-05-26

## Como ler

Recomenda-se a leitura por esta ordem:

1. **[Visão e Escopo](00-overview/01-vision.md)** — porque este projecto existe, para quem, o que é e o que não é.
2. **[Glossário](00-overview/02-glossary.md)** — terminologia herdada do domínio. Deixar aberta como referência.
3. **[Arquitectura](01-architecture/)** — stack, topologias, sincronização, segurança, bootstrap.
4. **[Modelo de Dados](02-data-model/)** — entidades, invariantes, sintaxes, permissões, decisões.
5. **[Módulos](03-modules/)** — funcionalidades por módulo (POS, Catálogo, Caixa, Delivery, Cozinha, Cantinas, Sistema, Plug-ins).
6. **[UI/UX](04-ui-ux/)** — princípios visuais e ecrãs detalhados.
7. **[Integrações](05-integrations/)** — hardware, compliance fiscal, API.
8. **[Não-Funcionais](06-non-functional/)** — performance, fiabilidade, internacionalização.
9. **[Roadmap](07-roadmap/)** — plano de fases.
10. **[Apêndices](08-appendices/)** — mapeamento WinREST→OpenRest, catálogo de flags, tipos de documento, troubleshooting.

## Índice completo

### 00 — Overview

- [01 — Visão e Escopo](00-overview/01-vision.md)
- [02 — Glossário](00-overview/02-glossary.md)

### 01 — Arquitectura

- [01 — Stack Tecnológica](01-architecture/01-stack.md)
- [02 — Topologias de Implementação](01-architecture/02-deployment-topologies.md)
- [03 — Sincronização e Modo Offline](01-architecture/03-sync-and-offline.md)
- [04 — Segurança](01-architecture/04-security.md)
- [05 — Bootstrap e Configuração Inicial](01-architecture/05-bootstrap.md)

### 02 — Modelo de Dados

- [01 — Entidades](02-data-model/01-entities.md)
- [02 — Invariantes e Regras](02-data-model/02-invariants.md)
- [03 — Sintaxe de Conjuntos (legado)](02-data-model/03-set-syntax.md)
- [04 — Matriz de Permissões](02-data-model/04-access-matrix.md)
- [05 — Decisões de Design (ADRs)](02-data-model/05-design-decisions.md)

### 03 — Módulos Funcionais

- [01 — POS / Operação](03-modules/01-pos-operation.md)
- [02 — Catálogo (Ficheiros)](03-modules/02-catalog.md)
- [03 — Caixa](03-modules/03-cash-register.md)
- [04 — Delivery](03-modules/04-delivery.md)
- [05 — Cozinha / Distribuição de Pedidos](03-modules/05-kitchen-and-routing.md)
- [06 — Empregados e Clientes](03-modules/06-employees-clients.md)
- [07 — Restauração Colectiva](03-modules/07-collective-catering.md)
- [08 — Sistema e Manutenção](03-modules/08-system-and-maintenance.md)
- [09 — Plug-ins e Extensões](03-modules/09-plugins.md)

### 04 — UI / UX

- [01 — Visão Geral](04-ui-ux/01-ui-overview.md)
- [02 — Ecrãs do POS (Detalhe)](04-ui-ux/02-pos-screens.md)

### 05 — Integrações

- [01 — Hardware (Visão Geral)](05-integrations/01-hardware-overview.md)
- [02 — Compliance Fiscal (PT e outros)](05-integrations/02-fiscal-compliance.md)
- [03 — APIs e Extensibilidade](05-integrations/03-api-and-extensions.md)

### 06 — Não-Funcionais

- [01 — Requisitos Não-Funcionais](06-non-functional/01-nfrs.md)

### 07 — Roadmap

- [01 — Roadmap por Fases](07-roadmap/01-roadmap.md)

### 08 — Apêndices

- [01 — Mapeamento WinREST → OpenRest](08-appendices/01-winrest-to-openrest-mapping.md)
- [02 — Catálogo de Flags de Impressão](08-appendices/02-printer-flags.md)
- [03 — Tipos de Documento e Cadeia Fiscal](08-appendices/03-document-types-and-fiscal.md)
- [04 — Troubleshooting](08-appendices/04-troubleshooting.md)

## Convenções

### Linguagem

- Esta especificação é escrita em **Português Europeu**.
- Termos de domínio históricos são mantidos (mesa, sessão, caixa, …) para minimizar fricção a integradores legados.
- Termos técnicos em inglês quando standard (UUID, REST, gRPC, …).

### Estado dos documentos

- **Aprovado** — baseline, mudanças exigem revisão.
- **Rascunho** — em construção, sujeito a mudanças.
- **Adiado** — decisão pendente.

Todos os documentos desta primeira versão estão em estado **Aprovado (rascunho de baseline)** — aprovado conceptualmente, sujeito a refinamento durante a implementação.

### Granularidade

- **Visão** — princípios.
- **Especificação** — o que tem que ser construído.
- **Decisões** — porquê.
- **Detalhes técnicos finos** (algoritmos, queries, schemas SQL) vivem no código e na docs auto-geradas.

### Versionamento

- A especificação segue SemVer.
- 0.x → pré-implementação, mudanças livres.
- 1.0 → coincide com release GA do produto.
- Major bumps requerem ADR.

## Mapeamento de objectivos

Para cada objectivo de alto nível do projecto, eis onde está documentado:

| Objectivo | Documentos chave |
|---|---|
| Reconstruir funcionalmente o WinREST | `03-modules/*`, `08-appendices/01-winrest-to-openrest-mapping.md` |
| Modernizar a stack | `01-architecture/01-stack.md` |
| Suportar multi-loja com sincronização | `01-architecture/02-deployment-topologies.md`, `01-architecture/03-sync-and-offline.md` |
| Cumprir fiscalidade PT (e outros) | `05-integrations/02-fiscal-compliance.md`, `08-appendices/03-document-types-and-fiscal.md` |
| Suportar todos os modelos de operação | `03-modules/01-pos-operation.md` (todos os tipos de local) |
| Permitir extensão por terceiros | `03-modules/09-plugins.md`, `05-integrations/03-api-and-extensions.md` |
| Garantir robustez offline | `01-architecture/03-sync-and-offline.md` |
| Manter a UX optimizada para touch | `04-ui-ux/*` |
| Suportar todos os tipos de hardware típicos | `05-integrations/01-hardware-overview.md` |
| Documentar tudo | este conjunto de documentos |

## Próximos passos

1. **Aprovação formal** desta baseline pela equipa.
2. **Setup do repositório** com a estrutura proposta em `01-architecture/01-stack.md`.
3. **Início da Fase 0** do [roadmap](07-roadmap/01-roadmap.md): fundação.
4. **Iteração** — cada PR de código vem com actualizações de spec quando relevante.
5. **Releases iniciais** — alpha, beta, GA conforme roadmap.

## Contribuir

Esta especificação é um documento vivo. Para contribuir:

- Issues no repositório para discussão.
- PRs alterando os ficheiros .md desta pasta.
- ADRs novos em `02-data-model/05-design-decisions.md`.
- Discussões mais amplas em `discussions/`.

## Atribuição

Esta especificação deriva conceptualmente dos manuais do **WinREST FrontOffice PRO**, propriedade da **GrupoPIE Portugal, S.A.** (1996–2008). O OpenRest é uma reconstrução independente, open-source, sem reutilização de código proprietário.

A terminologia de domínio é mantida para minimizar fricção de adopção e por respeito à história deste produto que serviu o sector da restauração em Portugal durante décadas.

## Licença da especificação

Esta documentação é disponibilizada sob licença **CC-BY-SA 4.0**. O código do produto OpenRest tem licença separada (a definir, ver `02-data-model/05-design-decisions.md` D-001).
