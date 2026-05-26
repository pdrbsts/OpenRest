# OpenRest — Hardware e Periféricos (Visão Geral)

> Catálogo dos tipos de dispositivos suportados, suas funcionalidades, ligações e configuração. Detalhes específicos de cada classe ficam em documentos próprios.

## 1. Filosofia

Cada dispositivo é uma entidade `dispositivo` na BD com `tipo`, `pai_dispositivo_id` (cadeia: gaveta → impressora → porta), e `configuracao` em JSON.

Configuração de hardware reflecte topologia física. Ex: gaveta ligada a impressora série numa porta COM1:

```
porta_serie(COM1)
└── impressora_generica(modelo TM-T88)
    └── gaveta_generica
```

Mecanismo **AUTO** acelera configuração: carrega defaults pelo modelo escolhido.

## 2. Classes de dispositivos

### 2.1 Portas

Caminho físico/lógico para comunicação.

| Tipo | Uso | Notas |
|---|---|---|
| **Porta Paralela** | Impressoras DOS-era; transmissão paralela (LPT) | Limitada a cabos curtos; alta velocidade |
| **Porta Série** | Impressoras, balanças, leitores, antenas | Configurável (baud, paridade, controlo de fluxo) |
| **DOS File** | Streaming para ficheiro ou porta DOS | Legado |
| **Porta Nula** | Dispositivo "null sink" — descarta tudo | Útil para desactivar impressões |
| **Socket Port** | Cliente TCP a servidor remoto | Impressoras IP, dispositivos de controlo de acessos |
| **Server Socket Port** | Servidor TCP aguardando ligações | Aceitação de pedidos remotos |

Modernizadas em OpenRest:
- **USB** — porta virtual com driver
- **Bluetooth** — impressoras móveis
- **Ethernet/IP** — primeira classe
- **WebSocket** — comunicação browser-based
- **gRPC** — para extensões cloud

### 2.2 Impressoras

| Tipo | Uso |
|---|---|
| **Impressora Ecrã** | Mostra documento no display (sem hardware) |
| **Monitor de Pedidos (KDS)** | Ecrã na cozinha que recebe pedidos em vez de imprimir |
| **Impressora Genérica** | Impressora de talão / matricial / térmica |
| **Impressora Fiscal** | Hardware certificado fiscalmente (BR, IT) |

Subconfigurações:
- **Codepage** (437, 850, 858, 1252, …)
- **Nº cópias por documento**
- **Sublinhar/tracejado** entre linhas
- **Imprime cartões** com formato fixo (linhas por página/cabeçalho/rodapé)
- **Inverte documentos** (impressoras montadas de parede)
- **Imprime códigos de barras ESC/POS / ITF**
- **Documentos guardados após impressão** (para reimpressão)
- **Sequências ESC personalizadas**
- **Tipo de bitmap** (raster, column, GS/v0, ...)
- **Troca de cor a meio de linha** (capacidade do hardware)

### 2.3 Dispositivos de produção controlada

| Tipo | Uso |
|---|---|
| **Máquina de Café** (CCI/CSI) | Controla doses |
| **Dispenser de bebidas** | Idem genérico |

Já documentado em `03-modules/05-kitchen-and-routing.md §9`.

### 2.4 Dispositivos de Pagamento

| Tipo | Uso |
|---|---|
| **NetPay** | Terminal Visa/MB integrado (PT/BPN — histórico) |
| **Terminais SiBS** | EMV/SiBS em Portugal |
| **PinPad genérico** | Norma EMV |

Configurados como dispositivo + método de pagamento associado.

### 2.5 Controlo de Acessos

| Tipo | Uso |
|---|---|
| **HW Controlo Acessos** | Torniquete / portaria |

Activado por:
- "Impressão" de documento na zona configurada
- Validação de UID por leitor associado

### 2.6 Displays de Cliente

| Tipo | Uso |
|---|---|
| **Display Genérico** | Display LCD de 2 linhas via porta série |
| **Display Interno (PAR/JarlTech)** | Integrado em POS dedicados |
| **NCR 7460 Display** | Específico modelo NCR |
| **Velleman MML30G** | Display matricial de marca específica |

Configurações:
- Codepage / encoding
- Sequências de inicialização e retorno
- Linhas e colunas
- Mensagem demo (em standby)

OpenRest moderno suporta também:
- Display tablet/monitor (segundo monitor exclusivo)
- Display via WebRTC para tablet do cliente

### 2.7 Leitores

| Tipo | Uso |
|---|---|
| **Leitor de Cartões Genérico** | Banda magnética, RFID emulado como teclado ou serial |
| **Leitor de Códigos de Barras** | Idem |
| **Led ID** | Receptor de identificação por proximidade do empregado |

Configurações:
- Sequência de fim de código
- Timeout (caso não haja sequência)
- Aceita só caracteres numéricos
- **Filtros** (transformações: `%d`, `%*3c`, `%[1-4]`, etc.)
- **Validação UID**: para senhas de refeição
- Dispositivo de display de validação
- Dispositivo de controlo de acessos a accionar
- Pedido automático de reservas (mesa+empregado)
- Dispositivo de bebidas a creditar (classe do artigo)

### 2.8 Terminais Rádio (Comandos)

| Tipo | Uso |
|---|---|
| **Receptor de Terminais Rádio (legado)** | Antena receptora dos terminais antigos |
| **Antena V.02** | Antena unificada moderna |

Em OpenRest moderno, comandos rádio são tipicamente substituídos por:
- **Aplicação mobile** (tablet/smartphone do empregado) via WiFi
- **Suporte legado** para terminais rádio existentes através de adaptador

### 2.9 Balanças

| Tipo | Uso |
|---|---|
| **Balança Genérica** | Por porta série com protocolo configurável |
| **Balança Bizerba** | Driver específico |
| **Outras (Toledo, …)** | Adicionar como plug-ins |

Configurações:
- Sequência recebida (`xxxx.nnnnnnnxxx` formato)
- Sequência a enviar (para balanças que precisam de "ping")
- Período (s)
- Caracteres especiais hex `\x0A`

### 2.10 POS Integrados

| Tipo | Componentes |
|---|---|
| **Jarltech Series 8100** | Display cliente, LCD, 2 gavetas, 4 portas auxiliares, 4 impressoras |
| **POS PAR Microsystems** | Gaveta, leitor cartões, display, LCD |
| **NCR 7460** | Leitor de cartões |
| **Bleep TS600 (EUROtouch)** | Display, leitor, gaveta |
| **Bleep TS650** | Display, leitor, 2 gavetas |

OpenRest moderniza para POS Android-based.

### 2.11 Identificação de Chamadas (Delivery)

| Tipo | Uso |
|---|---|
| **ZyXEL omni.net TA** | 2 portas analógicas + dados RDIS |
| **AVM Fritz!X PC** | Idem |
| **Identificador Genérico (modem)** | AT-commands para detecção CallerID |

Em OpenRest moderno:
- Soft phone / SIP integrado
- Push notifications de plataformas de delivery
- Integração API com WhatsApp Business para encomendas

### 2.12 Botoneiras

| Tipo | Uso |
|---|---|
| **Botoneira Série** | Até 12 botões via porta DB9 |
| **Botoneira Paralela** | Até 5 botões (legado) |

Cada botão envia uma tecla configurada.
Pino 15 em paralela pode actuar como Ctrl (modificador).

Em OpenRest:
- USB-HID (teclas customizadas)
- Streamdeck-like (botões com display)
- Pedais USB (para uso hands-free em cozinha)

### 2.13 Gavetas

| Tipo | Uso |
|---|---|
| **Gaveta Genérica** | Ligada a impressora ou directa por relé |

Configurações:
- Sensor de gaveta aberta (alguns modelos)
- Inversão de sinal (modelo-específico)
- Período de polling
- Posto a notificar

## 3. Cabos e ligações

Documentação herdada de portas paralela e série RS-232 mantém-se em `08-appendices/03-cables.md` para retro-compatibilidade.

Pin-outs:
- DB25 / Centronics paralela
- DB9 / DB25 / RJ46 série
- Cabos de impressora série de 3 fios (TX, GND, protocolo)
- Cabos de antena (com alimentação +12V no pino 1)

## 4. Comunicação rádio (legado)

Antenas configuradas:
- Receptor: 4800 baud, n,8,1, sem flow control
- Antena V.02: 38400, n, 8, 1, sem flow control

Hardware identifica casas pelo nº de licenciamento.
Configurações enviadas só quando há mensagem para o terminal (não imediato).

## 5. Configuração automática (AUTO)

Em qualquer ponto da árvore de dispositivos, premir AUTO em vez de Configurar:
- Para Porta Série: escolhe tipo de dispositivo conectado → configura baud, paridade, etc.
- Para Impressora: escolhe modelo → carrega ESC sequences, codepage
- Para Balança: escolhe modelo → carrega protocolo

Acelera setup em 80% dos casos.

## 6. Resolução de problemas

Listada exaustivamente em `08-appendices/04-troubleshooting.md`.

Categorias:
- Impressoras (não imprime, caracteres errados, duplicados, não abre gaveta, mudança de linha dupla)
- Hardware rádio
- Outros periféricos
- Software (variáveis de ambiente, drivers, rede)

## 7. Mudanças propostas para OpenRest

### 7.1 Drivers em pacote

Em vez de configurar manualmente cada modelo, ter pacotes de driver com:
- Sequências ESC
- Codepage
- Tamanho de papel
- Capacidades suportadas
- Templates exemplo

Marketplace: comunidade contribui drivers.

### 7.2 Auto-discovery

USB plug-and-play: detectar impressora/balança automaticamente e oferecer configuração.

Mesma coisa para rede: SSDP/Bonjour/UPnP discovery.

### 7.3 Hot-swap

Mudar impressora avariada por outra do mesmo modelo sem intervenção técnica: sistema reconfigura ao detectar.

### 7.4 Cloud printing

Pedido enviado a serviço cloud que distribui a impressoras IP geograficamente distribuídas. Útil para cadeias com múltiplas lojas geridas centralmente.

### 7.5 Mock devices

Dispositivos virtuais para testes:
- Impressora "Console" (imprime para log)
- Display de cliente em janela
- Balança simulada
- Leitor simulado (via clipboard)

### 7.6 Telemetria

Cada dispositivo reporta:
- Estado actual (ok, erro, sem papel, sem tinta)
- Volumes (páginas impressas, leituras)
- Última utilização
- Necessidade de manutenção
