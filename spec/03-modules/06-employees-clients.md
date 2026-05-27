# OpenRest — Módulo Empregados e Clientes

> Gestão de pessoas: empregados (utilizadores do sistema) e clientes (consumidores). Inclui acessos, comissões, autoconsumo, fidelidade, conta-corrente.

## 1. Empregados

### 1.1 Identificação

- Código numérico (1–999.999)
- Cartão (banda magnética, RFID, código barras)
- Username + password (PIN simples por defeito; pode ser palavra completa)
- Led ID (proximidade)
- Foto (UI moderna)

### 1.2 Acessos

Cada empregado pertence a um `nivel_acesso` (1–9). Cada nível tem permissões granulares (ver `02-data-model/04-access-matrix.md`).

**Acesso condicionado**: empregado de nível X só entra mediante autorização de empregado de nível Y. Implementado por dupla autenticação.

### 1.3 Mesas atribuídas

Conjunto de mesas que o empregado **abre por sua iniciativa**. Empregados sem acesso a mesas de outros não conseguem entrar em mesas abertas por colegas.

Sintaxe de conjunto (`02-data-model/03-set-syntax.md`).

### 1.4 Comissões

#### Variáveis (matriz)

`Grupo Empregado × Grupo Artigo → Percentagem`. Aplicada sobre vendas.

#### Fixas

Valor monetário fixo por sessão. Empregado escolhe qual aplicar no fecho de sessão.

#### Em grupo

- `Produz para` — empregados a quem este distribui parte das suas comissões
- `Recebe de` — empregados que distribuem para este

Suporta equipas, divisão por turno, etc.

Configurações relacionadas (caixa):
- `Comissões para o empregado de fecho`
- `Comissões sem IVA`
- `Anulações para o empregado que pediu` (vs anulou)

### 1.5 Consumo Próprio

Tipicamente, refere-se ao consumo dos próprios funcionários. Normalmente há produtos que não se pagam (gratuitos) e outros que são pagos a um preço reduzido. O local associado ao consumo próprio usa tipicamente uma tabela de preços (PVP) diferente dos restantes locais.

- `Base de Consumo` — valor mensal/diário sem custo
- `PVP de Consumo` — PVP usado para calcular o valor real consumido
- `% de Consumo` — percentagem que o empregado paga (do PVP)

Cálculo: valor a pagar = (Σ valor real - base) × percentagem

Ajuste:
- `Acerta CC no fecho da sessão` — emite movimento de caixa de ajuste imediatamente

Mesa especial automática para consumo do empregado (no local de consumo próprio). Operações:
- Fecha mesa só no fecho da sessão
- Não imprime consulta de mesa
- Não transfere
- Ofertas controladas pelo `base_ofertas` do empregado

### 1.6 Ofertas

Empregado pode oferecer descontos até `base_ofertas`. Acima, paga a diferença (igual ao consumo).

### 1.7 Cor e Idioma

- Cor: aparece nos botões do empregado e nas linhas de detalhe.
- Idioma: a UI muda para a língua do empregado quando se identifica.

### 1.8 Dados pessoais

- Morada, localidade, código postal
- NIF (9 dígitos validados)
- BI
- Telefones
- Data de admissão
- Data de nascimento

### 1.9 Histórico

- Sessões abertas/fechadas (lista)
- Marcações de ponto (timeclock)
- Vendas, comissões, consumo, ofertas (apuramentos)

### 1.10 Operações

- Novo / Anular / Recuperar
- Listagem configurável
- Atribuição de cartão por leitura (passa cartão → preenche)
- Reset de password (admin)
- Foto (upload na UI moderna)

## 2. Clientes

### 2.1 Identificação

- Código numérico (1–999.999)
- Nº cartão (até 9 caracteres) com validade
- Nome (40 chars)
- NIF (9 dígitos)

### 2.2 Associação

`associacao_cliente_id` — cliente "mãe" que paga conta dos associados. Exemplo: empresa paga refeições dos colaboradores; pai paga conta do filho.

- Saldo da associação = Σ saldos negativos dos associados + próprio
- Pagamento à associação liquida todos

`parente_mononivel_id` — referência informativa (sem CC consolidada).

### 2.3 Conta-Corrente (CC)

- Total a Débito
- Total a Crédito
- Saldo Actual
- Limite de Crédito
- Empregados com permissão `pode_passar_limite_credito` podem ultrapassar

### 2.4 Descontos

`grupo_desconto_cliente` cruza com `grupo_desconto_artigo` (matriz).

Aplicado automaticamente quando cliente é identificado no recebimento.

Configurável a inclusão do desconto no preço impresso (`inclui_desconto_nos_precos` no local).

### 2.5 Pontos / Fidelidade

- `pontos` — total acumulado
- Ganho automático: `pontos += floor(total / valor_por_ponto)` no fecho
- Uso: pode pagar parte da conta (`valor_por_ponto_venda`)
- Edição manual: permissão `ficheiros.clientes.edita_pontos`

### 2.6 Dados de contacto

- Morada (35 chars), Localidade (35 chars), Código Postal (25 chars)
- Telefone, telefax (com sintaxe especial multiplexada `252290600\1\2,252290601`)
- Email
- Zona (de morada, para delivery)
- Data de nascimento (para campanhas)
- Qualidade de cliente (descritivo)
- Observações (texto livre)

### 2.7 Histórico

- Últimos N pedidos (com botão "copiar pedido")
- Movimentos de CC
- Reservas
- Senhas de refeição
- Pontos ganhos/usados

### 2.8 Operações

- Novo (com criação rápida em delivery)
- Anular (mantém histórico)
- Recuperar
- Listagem configurável
- Importação em massa (CSV/Excel)
- Exportação para CRM externo
- GDPR: exportar dados pessoais, anonimizar

### 2.9 Identificação eventual (PT)

Quando o cliente não tem ficha mas quer factura nominativa: janela de NIF + nome aparece antes de imprimir a VD/factura (com país=PT activo).

- Validação de check digit do NIF (avisa mas não impede)
- NIF alfanumérico (estrangeiros) suportado (toggle)
- Dados gravados em snapshot para reimpressão

## 3. Tabelas de suporte

### 3.1 Qualidade de Cliente

Categoria descritiva (VIP, Empresa, Particular). 1–9.

### 3.2 Grupo de Desconto

Tabela 1–9 para cliente e para artigo. Matriz cruzada.

### 3.3 Zonas

Zonas de morada (1–999). Útil para:
- Agrupar clientes (estatísticas)
- Roteamento delivery (associar zona a rede remota)
- Filtros de pesquisa

### 3.4 Nível de Acesso

Já descrito em `02-data-model/04-access-matrix.md`.

### 3.5 Grupos de Comissão

Empregado: 1–9, descritivo.
Artigo: 1–9, descritivo.
Matriz cruzada com percentagens.

### 3.6 Comissão Fixa

Valores fixos seleccionáveis no fecho de sessão (1–9).

## 4. Sessões de Empregado

### 4.1 Estados

```
inactivo → autenticado (acessos consultáveis)
           → sessao_aberta (pode operar caixa)
                          → fechada
```

### 4.2 Operações

- Login (código + password ou cartão)
- Logout (sem fechar sessão, só passa para ecrã inicial)
- Abrir sessão (escolhe caixa, bolsa, comando)
- Fechar sessão (ajustes automáticos, apuramento)
- Mudar empregado (no posto, sem fechar sessão)
- Mudar password (próprio empregado)

### 4.3 Acesso temporário a outras mesas

Empregado A transfere mesa para empregado B → B tem acesso temporário a essa mesa, perdido no fecho.

## 5. Marcação de Ponto (Timeclock)

Módulo independente da sessão.

- Entrada antes da abertura do dia
- Saída após o fecho do dia
- Sequência alternada
- Listagem de assiduidade

UI: identificação + botão "Registar".

## 6. Mensagens entre empregados

Sistema simples de mensagens (post-it) entre empregados, opcionalmente entre lojas (redes remotas):

- Receber: indicador visível na UI
- Enviar: pode incluir formatação (Ctrl+1..6 muda tamanho, Alt+1..8 muda cor)
- Apagar / arquivar

## 7. UI sugerida (modernização)

- Foto + nome em vez de só código
- Atalhos de identificação rápida (leitor RFID, biometria opcional)
- Modo "shift handover" — passa-se a tablet entre empregados
- Suporte a "primary user" no posto + "actor" (gerente faz override mas a venda fica ao empregado)
- Mobile companion app: ver acumulado da sessão, comissões, mensagens
