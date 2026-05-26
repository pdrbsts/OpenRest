# OpenRest — Visão e Escopo

> **Documento**: 00.01 Visão e Escopo
> **Estado**: Aprovado (rascunho de baseline)
> **Versão**: 0.1.0
> **Origem**: derivado dos manuais técnico e de utilizador do WinREST FrontOffice PRO (GrupoPIE, 1996–2008)

---

## 1. Propósito

OpenRest é a reconstrução open-source de uma aplicação de **Front Office para restauração** (ponto de venda, gestão de mesas, caixa, cozinha, delivery, cantinas), tomando como referência funcional o **WinREST FrontOffice PRO**. O objectivo não é apenas clonar a aplicação histórica, mas:

1. Preservar todo o conhecimento de domínio embutido naquela aplicação (modelos, fluxos, regras de negócio, integrações com hardware típico do sector).
2. Modernizar a stack tecnológica para uma plataforma actual (web/desktop multiplataforma, cloud-aware, *offline-first*, com sincronização entre lojas).
3. Manter compatibilidade conceptual com a operação do dia-a-dia do restaurador português / ibérico, incluindo a fiscalidade nacional (PT — certificação AT, SAF-T PT, comunicação de séries, ATCUD, QR Code, hash de assinatura).

## 2. Visão

> *Um sistema de Front Office completo, livre, auditável e estendível, que qualquer restaurante — desde um café de aldeia até uma cadeia franchisada — possa instalar, configurar e operar sem depender de licenciamento proprietário, mantendo a robustez e a riqueza funcional que os utilizadores de POS profissionais esperam.*

## 3. Público-alvo

| Persona | Necessidade central |
|---|---|
| **Restaurador independente** | POS simples, fiável, com fecho de dia, IVA, factura. |
| **Cadeia de restauração / franchising** | Multi-loja, sincronização central, reports consolidados. |
| **Restauração rápida / Fast-food / Take-Away** | Vendas directas, troco rápido, ecrã optimizado para volume. |
| **Discoteca / Bar / Pub** | Cartões de consumo, alocação dinâmica de mesas, lista negra. |
| **Pizzaria / Hamburguer / Sushi** | Promoções compostas, complementos com tamanhos, meia-dose. |
| **Pizzaria com entrega** | Delivery com identificação de cliente por telefone, despacho. |
| **Cantina / Restauração colectiva** | Pratos do dia, senhas com UID, torniquete, reservas. |
| **Hotel / Resort** | Facturação externa, integração com PMS, consumo próprio. |
| **Agente técnico / integrador** | Instalação, configuração e suporte remoto a múltiplas casas. |

## 4. Princípios orientadores

1. **Domínio em primeiro lugar.** O modelo de domínio (mesas, sessões, caixa, pedidos, documentos, IVA, comissões) é a coluna vertebral; a UI e a stack são consequência.
2. **Offline-first.** Uma loja deve continuar a vender se a internet ou o servidor central caírem; sincroniza quando volta a haver ligação.
3. **Atómico e auditável.** Todas as operações que mexem em fichas, caixas e documentos são transaccionais e deixam *audit trail*; os documentos fiscais nunca podem "desaparecer".
4. **Acessível e estendível.** Plug-ins de primeira classe (pagamentos, balanças, impressoras, máquinas de café, sistemas de videovigilância, integrações verticais).
5. **Toque primeiro, teclado depois.** A operação principal é optimizada para *touch-screen*; teclado e leitor de códigos de barras são caminhos alternativos completos.
6. **Multilingue e multi-moeda.** Permite operar lojas em vários países com legislações fiscais diferentes.
7. **Configurável sem programar.** Locais, zonas de impressão, documentos, teclas, dispositivos, fluxos — tudo deve ser configurável pela interface ou por ficheiro versionável.
8. **Compatibilidade conceptual com o WinREST.** Quem operou o WinREST deve conseguir operar o OpenRest sem reformação radical.

## 5. Escopo (in-scope)

- Operação de POS com gestão de mesas, sessões, caixa e dia.
- Configuração completa de catálogo (famílias, sub-famílias, artigos, preços, IVA, promoções, happy hour, pratos do dia, complementos, menus).
- Gestão de empregados (acessos, comissões, consumo, identificação por cartão).
- Gestão de clientes (fichas, conta corrente, fidelidade, pontos, descontos, NIF, associação).
- Múltiplos modos de operação por local: mesa, take-away, take-away seguro, pub, delivery, consumo próprio, restauração colectiva.
- Impressão configurável: pedidos para cozinha/bar, sub-totais, contas, facturas, vendas a dinheiro, recibos, senhas, etiquetas.
- Integração de hardware: impressoras térmicas/matriciais/fiscais, gavetas, displays de cliente, balanças, leitores de cartão e código de barras, antenas para terminais rádio, botoneiras, máquinas de café, dispositivos de controlo de acessos.
- Caixa: aberturas, movimentos, transferências, encerramentos, apuramentos, relatórios, fecho financeiro.
- Sincronização multi-loja (mensagens remotas, reservas inter-loja, delivery centralizado).
- Fiscalidade: SAF-T PT, hash de assinatura, ATCUD, QR Code, comunicação de séries à AT, certificação como software de facturação (objectivo de longo prazo).
- API HTTP/XMLRPC para integração com BackOffice/Reports/Store/PMS externos.
- Servidor "VNC-like" (ou alternativa moderna) para postos *thin* / PDAs.

## 6. Fora de escopo (out-of-scope) — pelo menos na primeira versão

- BackOffice completo (Store/Reports). OpenRest exporta dados e expõe API; o BackOffice é um produto irmão.
- Gestão de stock avançada com fornecedores, encomendas, recepções complexas. O equivalente ao "MiliStore" é tratado como módulo separado.
- Gestão de RH e folhas de salário (apenas regista assiduidade/comissões).
- Reservas online públicas (site/widget para o cliente final). A reserva é interna; integrações com plataformas (TheFork, etc.) ficam para plug-ins.
- Aplicação para o cliente final (auto-pedido em mesa, pagamento por QR). Pode ser produto irmão.
- E-commerce / web shop.

## 7. Não-objectivos explícitos

- **Não** é objectivo clonar pixel-a-pixel a UI dos anos 2000 do WinREST. A UI é redesenhada para padrões actuais mantendo a *familiaridade operacional*.
- **Não** é objectivo manter o formato binário/ASCII histórico dos ficheiros `wrst*.000`. OpenRest usa um modelo de dados próprio; pode haver importador.
- **Não** é objectivo suportar todos os modelos de hardware listados no manual técnico (DOS-era). Suportam-se classes de hardware (impressora ESC/POS, balança protocolo Toledo, etc.) e modelos representativos.

## 8. Critérios de sucesso

1. Uma loja-tipo (1 servidor + 2 postos + 1 impressora de cozinha + 1 impressora de talões + gaveta + display + balança) consegue operar um dia de serviço completo sem falhas: abre, vende, factura, fecha o dia e exporta SAF-T válido.
2. A reconstrução suporta os modos de operação: **mesa, take-away, delivery, pub, consumo próprio, restauração colectiva**.
3. A configuração inicial é guiada (Wizard) e completa-se em <2 horas para uma instalação simples.
4. Tempo de resposta da UI no posto: <100ms para acções comuns (abrir mesa, pedir artigo, fechar mesa).
5. Funcionamento offline garantido até 24h sem perda de dados.
6. Open-source com licença permissiva ou copyleft fraco (a decidir), com documentação suficiente para integradores externos contribuírem.

## 9. Glossário rápido (terminologia herdada do domínio)

| Termo | Definição operacional |
|---|---|
| **Posto** | Terminal físico de POS (PC, tablet, all-in-one) com ecrã, impressoras, gaveta, etc. |
| **Local** | Conjunto lógico de mesas (Sala 1, Esplanada, Take-Away, Delivery, Bar). |
| **Mesa** | Acumulador de pedidos. Pode ser uma mesa física, um cartão de consumo, um lugar de balcão. |
| **Sessão** | Período em que um empregado está autenticado e a operar. |
| **Caixa** | Acumulador físico/lógico de dinheiro. Cada gaveta corresponde a uma caixa. |
| **Turno** | Subdivisão temporal da Caixa (manhã, tarde, noite). |
| **Bolsa** | "Caixa pessoal" de um empregado que recebe dinheiro fora da caixa principal. |
| **Zona de impressão** | Sítio lógico onde sai um pedido (Bar, Grelhador, Cozinha, D.Externos). |
| **Origem** | Conjunto de postos/comandos que partilham uma configuração de zonas. |
| **C/R** | Cabeçalho/Rodapé de documento. Há 9 variantes para alguns tipos de documento. |
| **Comando / Terminal rádio** | Dispositivo portátil sem fios usado para pedir à mesa. |
| **PPV / PVP** | Preço de Venda ao Público. Existem até 5 PVPs por artigo. |
| **D.Externos** | Documentos externos: factura, venda a dinheiro, factura-recibo, recibo. |
| **Apuramento** | Mapa de fecho (de sessão, de caixa, de turno, de dia). |
| **Fecho do dia** | Operação que congela o dia, exporta dados e avança a data lógica. |
| **Mesa em automático** | Artigo que é colocado em automático na mesa em determinados eventos (abertura, fecho, início do dia). |
| **Macro** | Conjunto de artigos previamente pedidos que pode ser invocado em pedido posterior ("pode sair"). |
| **Atributo** | Etiqueta livre que classifica um documento (ex: tipo de comida, época do ano). |
| **UID** | Código único impresso em senhas de refeição para validação por torniquete. |
| **Hardlock** | Dongle de licenciamento (USB/série) — substituído em OpenRest por modelo open. |

## 10. Stakeholders

- **Maintainers** do projecto OpenRest.
- **Comunidade open-source** (contribuidores).
- **Restauradores** (utilizadores finais).
- **Integradores / parceiros técnicos** que instalam e suportam.
- **Autoridade Tributária e Aduaneira (PT)** e equivalentes em outros países, no contexto da certificação fiscal.

## 11. Documentação de referência

Os documentos seguintes detalham cada aspecto desta visão:

- [Arquitectura técnica](../01-architecture/) — stack, deployment, serviços, sincronização, offline.
- [Modelo de dados](../02-data-model/) — entidades, relações, invariantes.
- [Módulos funcionais](../03-modules/) — POS, Catálogo, Caixa, Cozinha, Reports, Delivery, Cantinas.
- [UI/UX](../04-ui-ux/) — ecrãs, fluxos, atalhos, configuração visual.
- [Integrações e hardware](../05-integrations/) — impressoras, balanças, displays, terminais rádio, fiscalidade.
- [Requisitos não-funcionais](../06-non-functional/) — performance, fiabilidade, segurança, compliance.
- [Roadmap](../07-roadmap/) — fases, MVP, evolução.
- [Apêndices](../08-appendices/) — mapas WinREST→OpenRest, vocabulário técnico, decisões adiadas.
