# OpenRest — Decisões de Design (ADRs)

> Architectural Decision Records: trade-offs assumidos no modelo de dados. Cada decisão tem contexto, opções, escolha e consequências.

## ADR-001: Mesa Viva vs Documento Pendente

**Contexto**: O domínio precisa de representar uma mesa em curso (pedidos pendentes) e um documento fiscal final (factura/VD). Estas são entidades distintas com naturezas diferentes (uma é volátil, outra é imutável).

**Opções**:
1. **Separadas**: `mesa_sessao` + `mesa_sessao_detalhe` (vivas) e `documento` + `documento_detalhe` (finais).
2. **Unificadas**: tudo é `documento` com `estado` (pendente, emitido, anulado).

**Escolha**: **Opção 1 (Separadas)**.

**Razões**:
- Os dados de mesa viva têm semântica diferente: podem ser alterados (anulação, oferta) antes do fecho.
- Documento fiscal é imutável após emissão; misturar com volátil complica regras.
- Performance: `mesa_sessao` típica tem 5-20 linhas; `documento` em produção tem milhões.
- Audit: separação ajuda a auditar fluxos (quando passa de "viva" a "documento").

**Consequências**:
- Transferências entre mesas afectam só `mesa_sessao`.
- Ao fechar mesa, snapshot do estado vai para `documento` (com hashes, IVA, etc.).
- Reports usam `documento` quase sempre.
- API expõe ambas, mas claramente diferenciadas.

---

## ADR-002: Soft Delete vs Hard Delete

**Contexto**: Fichas (artigos, clientes, empregados) podem ser "removidas". O WinREST usa soft delete (marca como anulado).

**Opções**:
1. **Soft delete** com flag `anulado_em`.
2. **Hard delete** com cascata.
3. **Archive table** (mover para tabela separada).

**Escolha**: **Opção 1 (Soft delete)**, com possibilidade de archive periódico.

**Razões**:
- Histórico exige integridade referencial (factura antiga referencia artigo que pode estar "anulado").
- Reports retroactivos precisam de ler dados anulados.
- Compatibilidade com WinREST (`mostra_fichas_anuladas`).
- "Recuperar" um anulado é trivial.

**Consequências**:
- Todas as queries operacionais filtram `WHERE anulado_em IS NULL`.
- UI tem opção "mostrar anulados".
- Purga (hard delete) após N dias é opcional (definição `manutencao.purga_anulados_dias`).
- Códigos curtos (1–9) não podem ser reutilizados imediatamente.

---

## ADR-003: Códigos curtos numéricos vs UUIDs

**Contexto**: WinREST usa códigos numéricos curtos (1–9, 1–999, 1–6 dígitos). Identificadores modernos preferem UUIDs.

**Opções**:
1. **Só código curto** (legado).
2. **Só UUID**.
3. **Ambos** — UUID interno (PK técnica), código curto (chave de negócio).

**Escolha**: **Opção 3**.

**Razões**:
- UUID v7: chave técnica universal, suporta multi-loja sem colisões.
- Código curto: human-friendly, usado em pedidos por teclado, comandos rádio, leitores.
- Compatibilidade WinREST.

**Consequências**:
- Cada entidade tem `id` (UUID) e `codigo` (numérico).
- API expõe ambos; URLs usam UUID por defeito, código curto opcionalmente.
- Pesquisa por código é índice secundário.

---

## ADR-004: Sintaxe de Conjunto de Mesas

**Contexto**: WinREST tem sintaxe compacta para conjuntos de mesas (`1:5,20:30:2`).

**Opções**:
1. **Sintaxe compacta** apenas.
2. **JSON estruturado** apenas.
3. **Ambos** — compacto + parsed em forma estruturada.

**Escolha**: **Opção 3** (`02-data-model/03-set-syntax.md`).

**Razões**:
- Compactidade para edição rápida.
- Estruturada para UI guiada.
- Compatibilidade com importação WinREST.

**Consequências**:
- Parser e gerador entre formas.
- Operações de pertença, intersecção implementadas eficientemente.

---

## ADR-005: Event Sourcing vs CRUD direct

**Contexto**: Cada operação altera o estado. Event sourcing armazena eventos imutáveis; CRUD altera directamente.

**Opções**:
1. **CRUD direct** com audit log paralelo.
2. **Event sourcing total**.
3. **Hibrido**: CRUD + event log para acções relevantes.

**Escolha**: **Opção 3 (Híbrido)**.

**Razões**:
- Event sourcing total tem custo de implementação e performance para queries simples.
- CRUD é familiar e tem boa ferramenta.
- Audit + event log resolve a necessidade de rastreabilidade.

**Consequências**:
- BD tem tabelas "vivas" (estado actual) + `event_log` paralelo.
- Reconstruir estado é possível mas não default.
- Sincronização inter-loja usa eventos (vide ADR-006).

---

## ADR-006: Sincronização entre lojas

**Contexto**: Multi-loja precisa sincronização. Conflitos possíveis.

**Opções**:
1. **Master-Slave** (central manda).
2. **Multi-Master** com last-write-wins.
3. **Event Sourcing + CRDTs** para resolução automática.

**Escolha**: **Combinação**:
- Catálogo: master-slave (central manda) com LWW para excepções.
- Operação: multi-master (cada loja é dona da sua operação).
- Inter-loja (reservas, mensagens): event sourcing simples.

**Razões**:
- Domínios diferentes têm necessidades diferentes.
- CRDTs complexos só onde valem o overhead.
- Conflitos no catálogo são raros (gestão central).
- Operação não tem conflitos (cada loja tem seu sequencial).

**Consequências**:
- Modelo de sync por domínio (vide `03-sync.md`).
- Vector clocks só onde necessário.
- UI BackOffice tem screen de "conflitos pendentes".

---

## ADR-007: Snapshot vs Referência em Documentos

**Contexto**: Documento fiscal referencia artigo, cliente, empregado. Se a ficha mudar, o documento deve permanecer correcto?

**Opções**:
1. **Referência (FK)**: documento aponta para `cliente_id`; nome vem da tabela.
2. **Snapshot total**: documento inclui nome/NIF/morada do cliente como copy at-emit-time.
3. **Híbrido**: snapshot de dados críticos + FK para histórico.

**Escolha**: **Opção 3**.

**Razões**:
- Reimpressão fiel: documento de há 5 anos deve mostrar dados como estavam.
- Performance: queries de reports não precisam de joins constantes.
- Auditoria fiscal: snapshot de NIF/nome do cliente protegido contra alterações posteriores.
- FK mantida para histórico do cliente (consultar todas suas facturas).

**Consequências**:
- Cada `documento` tem `cliente_id`, `cliente_nome`, `cliente_nif`, etc.
- Catálogo (artigos) idem: `artigo_id` + `artigo_codigo` + `artigo_designacao` snapshot.
- Espaço em disco maior, justificado.

---

## ADR-008: Multi-tenant vs Single-tenant

**Contexto**: BackOffice central pode servir muitos tenants (SaaS) ou ser single-tenant.

**Opções**:
1. **Single-tenant**: cada instalação serve um cliente.
2. **Multi-tenant** com isolamento lógico.
3. **Multi-tenant** com isolamento físico (BD por tenant).

**Escolha**: **Default Single-tenant**, com **option Multi-tenant** para SaaS.

**Razões**:
- Cadeias gerem-se single-tenant.
- SaaS managed pode ter multi-tenant para reduzir custos.
- Segurança em fiscalidade exige isolamento estrito.

**Consequências**:
- Modelo de domínio é tenant-agnóstico.
- Coluna `tenant_id` opcional, activa em modo multi-tenant.
- Row-Level Security (PostgreSQL) no modo multi.

---

## ADR-009: Storage de Documentos Fiscais

**Contexto**: Documentos fiscais devem ser preservados por 10 anos. Onde guardá-los?

**Opções**:
1. **BD principal** (linhas em tabelas).
2. **Object storage** (S3) para PDFs + metadados na BD.
3. **Híbrido**: dados estruturados na BD + PDF arquivado em object storage.

**Escolha**: **Opção 3**.

**Razões**:
- Estrutura na BD permite queries e reports.
- PDFs em object storage poupam espaço da BD.
- Reimpressão usa template + dados (não PDF cached).
- Backup do object storage separado da BD.

**Consequências**:
- Coluna `pdf_url` opcional em `documento`.
- Geração de PDF on-demand para reimpressão.
- Object storage encriptado e versionado.

---

## ADR-010: Identificação de Cliente em Recebimento

**Contexto**: PT exige NIF em facturas; cliente pode ou não ter ficha.

**Opções**:
1. **Sempre criar ficha** quando pede factura nominativa.
2. **Permitir snapshot eventual** sem criar ficha.
3. **Ambos**.

**Escolha**: **Opção 3**.

**Razões**:
- Clientes regulares devem ter ficha (pontos, descontos).
- Clientes eventuais não querem registo.
- Reimpressão deve preservar dados.

**Consequências**:
- `documento` tem campos `cliente_id` (nullable) + snapshot `cliente_nome`, `cliente_nif`, etc.
- UI: botão "Criar Ficha" no recebimento se NIF dado mas não associado.

---

## ADR-011: Localização e Multi-país

**Contexto**: Diferentes países têm requisitos fiscais diferentes.

**Opções**:
1. **Hard-coded** por país.
2. **Plugin-based** (cada país = plug-in).
3. **Configurável** (matriz de regras por país).

**Escolha**: **Opção 2 + 3**.

**Razões**:
- Regras simples podem ser configuráveis.
- Compliance complexa precisa de plug-in (PT, BR, ES).
- Permite contribuição da comunidade.

**Consequências**:
- `pais_locale` na BD com regras.
- Plug-ins por país: `fiscal-pt`, `fiscal-es`, `fiscal-br`.
- Núcleo agnóstico a país.

---

## ADR-012: Linguagem de Templates de Documento

**Contexto**: Templates configuráveis precisam de linguagem de substituição.

**Opções**:
1. **Manter sintaxe WinREST** (`\flag`).
2. **Sintaxe moderna** (Handlebars, Liquid).
3. **Ambos** com migração gradual.

**Escolha**: **Opção 3**, default WinREST para compatibilidade, suporta moderna.

**Razões**:
- Reformação dos integradores históricos.
- Limitações da sintaxe WinREST (sem condicionais, loops complexos).
- Moderna abre porta a templates mais sofisticados.

**Consequências**:
- Parser dual.
- Migração assistida (importar template legado → reescrever).

---

## ADR-013: Idempotência de Operações

**Contexto**: Sync e retries podem repetir operações.

**Opções**:
1. **Idempotência opcional**.
2. **Idempotência obrigatória** em todas as APIs mutadoras.

**Escolha**: **Opção 2**.

**Razões**:
- Resiliência em redes pouco fiáveis.
- Sync entre lojas exige.

**Consequências**:
- Cada request mutador tem `idempotency_key`.
- BD regista chaves vistas (cache TTL).
- Documentos têm UUID gerado pelo cliente.

---

## ADR-014: Sequencial de Documentos

**Contexto**: Quem gera o próximo número?

**Opções**:
1. **BD com lock pessimista** em cada emissão.
2. **Counter atómico** (incremento atómico).
3. **Pre-allocation** (reservar bloco de N números).

**Escolha**: **Opção 2** em single-server; **Opção 3** em multi-loja.

**Razões**:
- Performance: lock pessimista bottleneck.
- Multi-loja precisa garantir não-colisão.

**Consequências**:
- Cada loja com gama própria.
- Coordenação central para alocar gamas.

---

## ADR-015: Anulação de Linha em Documento de Mesa Viva

**Contexto**: Antes de fechar a mesa, posso anular uma linha. Apago ou marco?

**Opções**:
1. **Apagar fisicamente**.
2. **Marcar como anulada** (mantém linha visível com riscado).
3. **Lista paralela de anulações**.

**Escolha**: **Opção 2** (mantém histórico operacional).

**Razões**:
- Auditoria: empregado consegue ver o que aconteceu.
- Reports de "vendas negativas" precisa das anulações.
- Lista de detalhe original do documento de cozinha não muda.

**Consequências**:
- `mesa_sessao_detalhe` tem `anulada_em`, `anulada_por`, `anulada_com_desperdicio`.
- UI mostra anuladas com tracejado.
- Documento fiscal final só inclui não-anuladas.

---

## ADR-016: Time Zones

**Contexto**: Cadeia multi-país tem fusos diferentes.

**Opções**:
1. **Tudo UTC**.
2. **Tudo local da loja**.
3. **UTC armazenado, local apresentado**.

**Escolha**: **Opção 3**.

**Razões**:
- UTC para sincronização.
- Local para utilizador.
- Cadeia fiscal usa local (timezone da loja).

**Consequências**:
- `loja.fuso_horario` armazenado.
- Timestamps em UTC na BD.
- Conversões em UI / templates.

---

## ADR-017: Versionamento de Schema

**Contexto**: Schema da BD evolui com versões.

**Opções**:
1. **Auto-migrate** ao arrancar.
2. **Manual com tooling**.
3. **Híbrido**: auto-migrate para minor, manual para major.

**Escolha**: **Opção 3**.

**Razões**:
- Updates seamless em prod (minor).
- Major exige planeamento (backup, janela).
- Rollback testável.

**Consequências**:
- Ferramenta `openrest-migrate` com diff/dry-run/apply/rollback.
- Backup obrigatório antes de major.

---

## ADR-018: Resolução de Conflitos no Catálogo

**Contexto**: Loja edita artigo localmente; central edita também.

**Opções**:
1. **LWW** (Last-Write-Wins).
2. **Pessimista**: central manda sempre.
3. **Manual**: UI para resolver.

**Escolha**: **Default LWW**, com **opção pessimista por entidade**.

**Razões**:
- LWW resolve 95% dos casos com baixo overhead.
- Casos críticos (preço, IVA) podem ser pessimistas.
- Operadores não querem UIs de conflito.

**Consequências**:
- Per-entity policy.
- Avisos no log quando LWW resolve conflito (não silencioso).

---

## ADR-019: Cache de Catálogo no Posto

**Contexto**: Postos precisam do catálogo. Buscar sempre ao servidor é lento.

**Opções**:
1. **Sem cache** (buscar sempre).
2. **Cache full** sincronizada.
3. **Cache LRU**.

**Escolha**: **Opção 2** (cache total sincronizada).

**Razões**:
- Catálogos típicos têm <10.000 artigos: cabem em memória.
- Sync push é eficiente.
- Latência crítica para UX.

**Consequências**:
- Memória de cada posto: ~50MB para catálogo grande.
- Invalidação por evento (push).
- Boot do posto: pull inicial.

---

## ADR-020: Backups

**Contexto**: Quão frequentes? Onde?

**Opções**:
1. **Diários locais**.
2. **Diários cloud**.
3. **Híbrido**: locais frequentes + cloud diários.

**Escolha**: **Opção 3**.

**Razões**:
- Locais protegem contra falha de software/disco.
- Cloud protege contra incêndio/roubo.
- Frequência adapta ao volume.

**Consequências**:
- Snapshots locais a cada hora.
- Upload cloud diário.
- Retenção: locais 7 dias, cloud 90 dias.

---

## ADR-021: Linguagem Backend

**Contexto**: Vide `01-architecture/01-stack.md`. Rust escolhida.

**Trade-offs**:
- ✅ Performance
- ✅ Safety
- ✅ Concurrency
- ❌ Curva de aprendizagem
- ❌ Compilation time

**Mitigação**: equipa core treinada; comunidade aberta recebe contribuições em Rust (e via plug-ins em outras linguagens).

---

## Decisões adiadas (Pending)

- **D-001**: Licenciamento final (AGPL vs MIT vs Apache).
- **D-002**: Suporte a múltiplas moedas simultâneas em transição cambial.
- **D-003**: Mecanismo de extensão para fiscalidade BR (NFCe vs SAT).
- **D-004**: Modelo de tenancy em SaaS managed (isolated vs shared).
- **D-005**: Estratégia de marketing e GTM.

Decisões adiadas ficam até evento que as force (ex: requisito de cliente).
