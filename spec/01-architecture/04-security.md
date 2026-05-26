# OpenRest — Segurança

> Modelo de ameaças, controlos, autenticação, autorização, encriptação, audit.

## 1. Modelo de Ameaças

### 1.1 Actores

- **Empregado honesto** — uso normal
- **Empregado desonesto** — fraude (anulações, ofertas falsas, "boca")
- **Gerente** — auditor / actor com elevados privilégios
- **Cliente** — utilizador externo; pode tentar manipular displays, comprovativos
- **Concorrente / Atacante externo** — acesso à rede, dados
- **Hacker remoto** — exploração da API
- **Adversário fiscal** — manipulação de documentos para evasão

### 1.2 Activos críticos

1. Cadeia de documentos fiscais (integridade)
2. Catálogo (preços não podem ser alterados sem rasto)
3. Dados de clientes (GDPR)
4. Dados de pagamento (PCI-DSS, mesmo que tokens)
5. Histórico de operação (audit trail)
6. Credenciais
7. Chave privada de assinatura fiscal

### 1.3 Ameaças

| Ameaça | Vetor | Mitigação |
|---|---|---|
| Fraude por anulação | Empregado anula após cobrar | Logging + permissões + reports de "vendas negativas" |
| Vendas "fantasma" | Empregado não regista venda | Câmaras (videovigilância plug-in) + auditoria contra stock |
| Alteração de preço silenciosa | Edição de artigo sem rasto | Audit log inviolável + assinatura |
| Exfiltração de dados de cliente | Acesso indevido à BD | Permissões + encriptação |
| Acesso físico ao posto | Empregado deixou logado | Lock screen automático |
| Phishing / Engenharia social | Senha mestra revelada | Rotação + matriz de password |
| Ataque na rede | Sniffing / MITM | TLS + autenticação mútua |
| Vulnerabilidade em dependências | CVE pública | Dependabot + builds reproduzíveis |
| Quebra de cadeia fiscal | Edição directa de BD | Hash assinado + audit |
| Manipulação SAF-T | Reescrever exportação | Hash + assinatura |
| Deny of service | Inundar com pedidos | Rate limit |

## 2. Autenticação

### 2.1 Empregado

Login por:
- Código + password (PIN curto ou palavra)
- Cartão magnético / RFID
- Código de barras
- Led ID (proximidade)
- (futuro) Biometria
- (futuro) SSO em ambiente corporativo

Password storage: argon2id com salt único.
PIN curto (4-6 dígitos): aceitável para POS mas com:
- Lock após N tentativas
- Audit das tentativas

### 2.2 Manutenção (Técnico)

Senha técnica especial. Modos:
- **Standard**: password única.
- **Seguro (matriz)**: configurada uma palavra; o sistema gera matriz alfabética e pede dígitos. Empregado vê só a palavra; técnico vê só os dígitos.

Senha mestra: para reset de senhas de utilizador (recomendada rotação).

### 2.3 API

OAuth2 / JWT.
Tokens com expiração curta (1h).
Refresh tokens para renovação.
Revogação possível (token blacklist).

### 2.4 Posto a Posto

mTLS — cada posto tem certificado.
Servidor verifica certificado do cliente.

### 2.5 Loja a Central

mTLS + token.
Certificate pinning.

## 3. Autorização (RBAC)

Já documentado em `02-data-model/04-access-matrix.md`.

Modelo:
- **Empregado** → **Nível de Acesso** → **Permissões granulares** (lista de flags)

Extensões:
- **ABAC** (Attribute-Based) para casos especiais — ex: permissão só durante horário de manutenção.
- **Acesso temporário** — gerente delega temporariamente a outro empregado.
- **Override**: gerente pode "actuar como" empregado para correcções (com audit).

## 4. Encriptação

### 4.1 At-Rest

- BD SQLite: encriptação **opcional** (SQLCipher), default off
- BD PostgreSQL central: TDE (Transparent Data Encryption)
- Backups: encriptados sempre
- Chaves: armazenadas separadamente (HSM em deployments avançados)

Campos sensíveis (mesmo sem encriptação total):
- Cartões: nunca armazenar PAN completo (PCI-DSS)
- Tokens de pagamento (BIN+last4 ok)
- Hashes de password
- Chave privada de assinatura fiscal

### 4.2 In Transit

- TLS 1.3 obrigatório (excepto em rede LAN explícita com toggle)
- HSTS habilitado
- Cipher suites: modernas (AES-GCM, ChaCha20-Poly1305)
- Certificados: Let's Encrypt automático ou customizado

### 4.3 In Use

- Memória limpa após uso (chaves)
- No-swap para processos críticos
- Rust limita exposição (vs C/C++)

## 5. Auditoria

### 5.1 Event Log

Todas as operações fiscais e administrativas:
- Imutável (apenas append)
- Cada entrada com hash do anterior (chain)
- Retenção mínima: 10 anos (PT)

### 5.2 Operações auditadas

- Login / logout
- Mudança de password
- Acessos elevados (manutenção)
- Emissão / anulação / reimpressão de documentos
- Mudança de catálogo (preços, IVA)
- Mudança de empregado / nível
- Movimentos de caixa
- Conflitos de sincronização
- Acesso a dados sensíveis (cliente)

### 5.3 Visualização

Apenas a quem tem `audit.read`. Filtros, exportação.

### 5.4 Detecção de anomalias

Análises automatizadas:
- Volume excessivo de anulações por empregado
- Anulações ao fim do dia
- Picos de vendas/anulações
- Acessos fora de horário

Alertas configuráveis.

## 6. Segurança Física

### 6.1 Posto

- Lock screen automático após inactividade
- Password de desbloqueio
- Senha mestra para situações de emergência
- Modo "exclusivo" (sem UI local) para minimizar manipulação

### 6.2 Servidor

- UPS
- Acesso físico restrito
- Disco encriptado
- Sem entrada de USB não autorizado (USB lock)

### 6.3 Comandos / Tablets

- Pin de desbloqueio
- Remote wipe se perdido
- Logout automático

## 7. PCI-DSS (Pagamentos)

OpenRest deve ser:
- **Out of scope** (sempre que possível): redireccionar processamento para terminal certificado
- **Tokenização**: armazenar tokens em vez de PANs
- Compliance scope mínimo

Quando processa cartão directamente:
- Nunca armazenar PAN, CVV
- Mascaramento (last 4 + BIN)
- Encriptação ponto-a-ponto

## 8. GDPR (Dados Pessoais)

### 8.1 Direitos do titular

- **Acesso**: exportar todos os dados de um cliente
- **Rectificação**: corrigir
- **Esquecimento**: anonimizar histórico
- **Portabilidade**: export em formato standard
- **Objecção**: opt-out de marketing/profiling

### 8.2 DPIA

Aplicação inclui guidelines para DPIA (Data Protection Impact Assessment).

### 8.3 Anonimização

Procedimento de "right to be forgotten":
- Substitui dados PII por hash
- Mantém integridade fiscal (factura ainda referencia "Cliente anonimizado UUID")
- Audit log mantém o pedido

### 8.4 Consentimento

UI de criação de cliente inclui toggles:
- Comunicações de marketing
- Análise de comportamento
- Partilha com parceiros

## 9. Hardening

### 9.1 Code

- Reviews obrigatórias em PRs
- Static analysis (clippy, Semgrep)
- Dependency scanning
- Fuzz testing nas partes críticas (parser de documentos, network)

### 9.2 Build

- Reproducible builds
- Signed releases
- SBOM (Software Bill of Materials)

### 9.3 Deployment

- Containers minimal (distroless)
- Read-only filesystem onde possível
- Non-root user
- AppArmor/SELinux profiles
- Network policies

### 9.4 Runtime

- Privilege de-escalation onde possível
- Resource limits (CPU, memória, FDs)
- Rate limits API

## 10. Resposta a Incidentes

### 10.1 Plano de IR

- Detectar (alertas)
- Conter (revogar acessos)
- Eliminar (patch / rollback)
- Recuperar (restore se necessário)
- Lições aprendidas (post-mortem)

### 10.2 Comunicação

- Bug bounty / responsible disclosure
- security.txt (RFC 9116)
- CVE assignment para vulnerabilidades

### 10.3 Notificação

- Em violação de dados pessoais: 72h à autoridade (GDPR)
- Aos titulares: quando aplicável

## 11. Atualizações de Segurança

- Cadência regular (mensal mínimo)
- Patches críticos: out-of-band
- LTS branches com correcções de segurança apenas
- Notificação de fim-de-vida (EOL)

## 12. Plug-ins

- Sandbox obrigatório (WebAssembly preferido)
- Permissões explícitas (declaradas no manifesto)
- Revogação possível
- Code signing dos plug-ins oficiais
- Aviso ao instalar plug-ins não-oficiais

## 13. Modelo de "Defense in Depth"

```
Camada 1: Rede (firewall, VPN, mTLS)
Camada 2: Aplicação (autenticação, autorização)
Camada 3: Dados (encriptação, audit)
Camada 4: Sistema (OS hardening, monitoring)
Camada 5: Físico (acesso restrito)
Camada 6: Pessoas (formação, processos)
```

## 14. Conformidade

- **ISO 27001** — objectivo de longo prazo
- **PCI-DSS** — quando processa pagamentos
- **GDPR** — operacional desde o dia 1
- **Certificação AT (PT)** — vinculativa
- **NIS2** — relevante em alguns países

## 15. Threat Model Diagram (DFD simplificado)

```
[Cliente] → [Posto] → [Servidor Loja] → [Servidor Central]
                          ↓
                   [Impressora/Gaveta]
                          ↓
                   [Pagamento (terceiro)]

Threats:
T1: MITM no canal Posto→Servidor → mitigação: mTLS
T2: SQLi na API → mitigação: prepared statements
T3: XSS no BackOffice → mitigação: CSP + sanitização
T4: Privilege escalation → mitigação: RBAC strict
T5: Tampering de documentos → mitigação: hash chain
```
