# OpenRest — Compliance Fiscal (Portugal e outros)

> Requisitos fiscais (cumulativos com a operação). O OpenRest tem como objectivo a **certificação como software de facturação em Portugal** (AT) — requisito legal para vendas a partir de 2013/2020+ para empresas com volume de negócios > limite. Esta secção é vinculativa.

## 1. Portugal — enquadramento legal

### 1.1 Legislação relevante

- **DL 28/2019** — obriga uso de software certificado
- **Portaria 363/2010** — define método de assinatura e hash da cadeia de documentos
- **Portaria 195/2020** — define ATCUD e QR Code obrigatórios
- **Portaria 302/2016** (e actualizações) — define estrutura SAF-T PT
- **Comunicação à AT** — séries de documentos, documentos e ficheiros SAF-T

### 1.2 Documentos abrangidos

- **Factura** (FT)
- **Factura-Recibo** (FR)
- **Factura Simplificada** (FS) — substitui a histórica "Venda a Dinheiro"
- **Nota de Crédito** (NC)
- **Nota de Débito** (ND)
- **Documento de Conferência** (DC) — consulta de mesa, não fiscal
- **Recibo** (RC)
- **Guia de Transporte / Remessa** — para deliveries, em alguns casos

## 2. Numeração

### 2.1 Séries

Cada tipo de documento tem 1..N séries activas. Cada série tem:
- Prefixo configurável (ex: "FT", "FS")
- Ano em curso (geralmente um suffix `/2024`)
- Próximo número sequencial
- Estado: activa / suspensa / encerrada

### 2.2 Regras

- Numeração **estritamente sequencial** dentro da série. Sem gaps.
- Não pode reutilizar números.
- Anulação não remove número — emite-se Nota de Crédito.
- Série deve ser **comunicada à AT antes** do primeiro documento.

### 2.3 ATCUD

Sigla: Código Único de Documento.
Formato: `[ATCUD-Validacao]-[Numero]`
Exemplo: `JFG3-12345`

`Validacao` é devolvido pela AT na comunicação da série.

ATCUD impresso em **todos os documentos fiscais** desde 2023.

## 3. Hash de Assinatura (Portaria 363/2010)

Cada documento de venda é assinado.

### 3.1 Conteúdo da assinatura

Concatenação de:
- Data do documento (`YYYY-MM-DD`)
- Data/hora de emissão (`YYYY-MM-DDThh:mm:ss`)
- Identificador do documento (Série + Número)
- Valor total
- Hash do documento anterior (na mesma série e ano)

### 3.2 Algoritmo

- Assinatura: RSA-2048 PKCS#1 v1.5 com SHA-1
- Chave privada gerada na primeira execução, **comunicada à AT** (apenas chave pública)
- Hash impresso no documento: 4 caracteres extraídos das posições 1, 11, 21, 31 da string Base64 do hash

### 3.3 Cadeia

Cada documento referencia o anterior por hash:
- `Hash_n = Sign(content_n || Hash_{n-1})`
- Primeiro documento da cadeia (`n=1`): `Hash_0 = ""` (vazio).

Inviolabilidade: alterar um documento intermédio invalida toda a cadeia subsequente.

### 3.4 Implementação OpenRest

- Chave privada armazenada cifrada (com `key encryption key` derivada da master key)
- Chave pública incluída na licença (validação)
- Pipeline atómico: gerar documento → calcular hash → escrever → emitir
- Se falha na escrita, anula o número e tenta de novo (mas mantém o gap registado em audit log)
- Migração de chaves: documento de "transição" cria nova série quando rota

## 4. QR Code (Portaria 195/2020)

Imprimido em **todos os documentos fiscais** (factura, FR, FS, NC, ND).

### 4.1 Payload

Estrutura em pares `chave:valor`, separados por `*`:

```
A:<NIF Emitente>*B:<NIF Adquirente>*C:<País>*D:<TipoDoc>*E:<Estado>
*F:<Data YYYYMMDD>*G:<NumDoc>*H:<ATCUD>
*I1:<País taxa>*I2:<Base IVA Isento>*I3:<Base 6%>*I4:<IVA 6%>
*I5:<Base 13%>*I6:<IVA 13%>*I7:<Base 23%>*I8:<IVA 23%>
*N:<Total IVA>*O:<Total c/ IVA>*Q:<4chars hash>*R:<Versão>
```

Comprimento ≈ 200 caracteres. QR Code versão 6 ou superior.

### 4.2 Implementação OpenRest

- Geração no momento da emissão
- Imprimível pela impressora térmica via flag `\qr`
- Inclui apoio para QR colorido em impressoras compatíveis

## 5. SAF-T PT

### 5.1 Conteúdo

XML normalizado contendo:
- **Header** — identificação da empresa, versão SAF-T, período
- **MasterFiles** — clientes, fornecedores, produtos, taxas de IVA, contas contabilísticas
- **GeneralLedgerEntries** — registos contabilísticos (opcional para FrontOffice)
- **SourceDocuments** — facturas, pagamentos, guias, recibos
  - **WorkingDocuments** — documentos de conferência
  - **MovementOfGoods** — guias de transporte
  - **Payments** — recibos

### 5.2 Periodicidade

- **Mensal**: comunicação obrigatória até dia 5 do segundo mês seguinte
- **Anual**: relativo a contabilidade
- **A pedido**: para fiscalização

### 5.3 Estrutura

Conforme Portaria 302/2016 e actualizações (versão actual à data da especificação).

OpenRest gera SAF-T automaticamente:
- Validador contra XSD oficial
- Encripta para envio seguro (TLS)
- Mantém histórico das exportações
- Comunicação directa via webservice (futura)

## 6. Comunicação à AT

### 6.1 Comunicação de séries

Quando se cria nova série: webservice AT recebe e devolve `CodigoValidacao` (que vai compor ATCUD).

Plug-in `comunicacao_series_at` faz isto.

### 6.2 Comunicação de documentos

Resumos diários ou em tempo real:
- WebService SOAP / REST
- Dados-chave de cada documento
- Confirmação retornada

Plug-in `comunicacao_documentos_at` faz isto.

### 6.3 Comunicação SAF-T

Upload do XML mensal.

## 7. Regras de impressão

### 7.1 Obrigatório nos documentos fiscais

- Logo / Designação Social da casa (`\ds`)
- NIF da casa (`\nc`)
- Conservatória e registo (`\cv`, `\nr`)
- Capital social (`\cs`) — se aplicável
- Designação do cliente (se não consumidor final) (`\ol`/`\nx`)
- NIF do cliente (se não consumidor final) (`\cl`/`\cx`)
- Morada do cliente (se factura)
- Data (`\dt`)
- Tipo de documento e número (`\nd`)
- ATCUD
- QR Code
- Hash de assinatura (4 caracteres)
- Versão do software certificado
- IVA por taxa (tabela ou linha a linha)

### 7.2 Designação social obrigatória

Em PT, se `\ds` ou `\nc` faltam no template, o sistema **bloqueia a impressão** com mensagem de erro. Isto força a configuração correcta.

### 7.3 IVA na linha de detalhe

Cada linha tem que mostrar:
- IVA percentagem ou código
- Base (preço × qt)
- Valor IVA

### 7.4 Reimpressão (2ª via)

- Carimba "DUPLICADO" (ou "2ª VIA")
- Não pede novo NIF (mantém o original)
- Não conta para numeração

### 7.5 Anulação

- Não apaga o documento original
- Emite Nota de Crédito (referenciando o original)
- Anulação aparece nos relatórios

## 8. Limites e validações

### 8.1 NIF obrigatório

Em PT, factura simplificada (FS) sem NIF é permitida até valor limite (€1000 IVA incluído, à data, para particulares). Acima exige NIF.

Sistema deve:
- Avisar quando ultrapassa o limite
- Bloquear emissão de FS sem NIF acima do limite
- Sugerir mudança para Factura

### 8.2 Validação de NIF

- Algoritmo de check digit PT
- Aviso, não bloqueio (NIF pode ser estrangeiro)
- Toggle alfanumérico para NIFs estrangeiros

### 8.3 NIF repetido

- Aviso (cliente já existe na BD)
- Não bloqueia (pode ser intencional em casos especiais)

## 9. Outros países

### 9.1 Espanha (ES)
- VeriFactu / NF-e equivalente em curso
- Imprime NIF do estabelecimento também (em consultas de registos)

### 9.2 Brasil (BR)
- NFC-e (Nota Fiscal de Consumidor Eletrônica)
- SAT (Sistema Autenticador e Transmissor)
- ECF (Emissor de Cupom Fiscal) — depreciado mas suportado
- IVA brasileiro com regras próprias

### 9.3 Suécia (SE)
- Relatório "Resultado de Vendas" substitui apuramento

### 9.4 Turquia (TR)
- Tratamento especial de I com/sem ponto

### 9.5 Angola (AO)
- SAF-T AO

### 9.6 Luxemburgo (LU)
- SAF-T LU

### 9.7 Genérico

Configuração `pais_locale` permite:
- IVA em detalhe obrigatório?
- ATCUD obrigatório?
- Limite NIF obrigatório
- Métodos SAF-T equivalentes
- Hash de assinatura obrigatório?
- Formatos data por defeito
- Símbolos moeda

## 10. Audit Log

Toda operação fiscal gera entrada em `event_log`:
- Emissão de documento
- Reimpressão
- Anulação
- Mudança de série
- Comunicação à AT
- Geração de SAF-T

Imutável (apenas append).
Retenção mínima: **10 anos** (PT) ou conforme país.

## 11. Backup e Recuperação

### 11.1 Garantias

- Backup automático diário (mínimo)
- Backup local + cloud (configurável)
- Recuperação testada periodicamente
- Documentação dos procedimentos

### 11.2 Disaster Recovery

- Tempo máximo de inactividade (RTO): 4h
- Perda máxima de dados (RPO): 24h em modo conservador

### 11.3 Migração

Procedimento de migração entre instalações:
- Export completo dos dados
- Verificação de hashes na cadeia
- Reimport com validação

## 12. Certificação

### 12.1 Processo em PT

1. Submissão do software à AT
2. Auditoria de código (review)
3. Testes de não-conformidade
4. Atribuição de número de software certificado
5. Renovação a cada alteração relevante

### 12.2 OpenRest

Como software open-source, certificação é desafiadora:
- Risco: utilizador pode modificar e quebrar conformidade
- Mitigação: certificar **builds oficiais** assinados; alertar sobre uso de builds não-oficiais
- Alternativa: certificação de "OpenRest com extensão fechada de fiscalização" como módulo separado

### 12.3 Roadmap

- Fase 1: SAF-T export funcional
- Fase 2: Hash de assinatura completo
- Fase 3: ATCUD + QR Code
- Fase 4: Comunicação webservice AT
- Fase 5: Submissão para certificação
