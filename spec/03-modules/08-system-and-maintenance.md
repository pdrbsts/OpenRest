# OpenRest — Módulo Sistema e Manutenção

> Operações de baixo nível: redireccionamento de impressoras, mensagens remotas, acerto de relógio, listagem de hardware, teste de impressão, bloqueio de posto, acessos técnicos e manutenção avançada.

## 1. Janela "Sistema"

Acedida sem manutenção. Disponibiliza operações comuns ao utilizador final/operador:

### 1.1 Redireccionar Impressoras

Para cada impressora:
- **Normal** — imprime normalmente
- **Espera** — guarda fila (até reactivação)
- **Ignorar** — descarta documentos
- **Redirecciona** → outra impressora

Operação típica: "a impressora do bar avariou — redireccionar para a impressora da caixa enquanto se substitui".

Adicionalmente, **Imprime últimos** reimprime documentos guardados (`Documentos guardados após a impressão`, configurável, default 2).

Reimpressão exige permissão específica.

### 1.2 Consulta (Receptor de Comandos)

Mostra terminais rádio activos:
- Empregado, nº comando, última mesa, última mensagem

Permite enviar mensagens remotas (cross-store).

#### Mensagens Remotas

Mensagens entre lojas (texto formatado).

- Apagar / criar / enviar
- Formato: Ctrl+1..6 altera tamanho, Alt+1..8 altera cor
- Destinatário: rede remota

### 1.3 Bloquear Posto

Bloqueia o terminal até password de desbloqueio (`Password de Bloqueio`).

### 1.4 Mensagens de Erro

Configurar se mensagens de erro vindas de terminais rádio devem ser também impressas.

Tipos:
- **Movimentação** — erros normais (mesa inexistente, sem stock, etc.)
- **Sessão** — relacionadas com abrir/fechar sessão

### 1.5 Acertar Relógio

Acertar data/hora do sistema operativo sem sair da aplicação. Acesso configurável.

### 1.6 Listar Hardware

Apenas no servidor. Listagem detalhada de todos os postos ligados (hardware, unidades de rede, espaço em disco).

### 1.7 Teste de Impressão

Imprime página de teste em impressora seleccionada ou em todas.

### 1.8 Plug-ins (atalho)

Lista de plug-ins disponíveis filtrada pelos acessos do utilizador.

## 2. Janela "Manutenção" (Técnico)

Acessível por senha. Duas formas de entrada:

### 2.1 Acesso Técnico

Senha calculada da matriz de campos visíveis (último dígito do dia, primeiro dos segundos, etc.) — modelo legado.

OpenRest moderniza para:
- **Password técnica configurada** (matriz alfa) — empregado fornece dígitos por letra: A=4, B=2, etc.
- Ou senha completa (modo standard)

### 2.2 Acesso de Início (Assistência)

Para iniciar manutenção prolongada (intervenção remota). Posto entra em **modo manutenção** com símbolo intermitente. Pode aceder a manutenção sem nova senha.

**Importante**: terminar com `Fim` para sair do modo. Fecho do dia força saída automática.

`manutencao_da_acesso_a_tudo` (definição geral): no modo manutenção, todos os acessos estão concedidos.

## 3. Operações de Manutenção

### 3.1 Definições Gerais

Configurações de sistema globais (afectam todo o sistema, independentemente do posto). Ver `02-data-model/01-entities.md §11.1`.

Principais:
- Formato dinheiro
- Botões na janela de pedidos (máximo)
- Idioma default
- Zona (prefixo numérico de séries)
- Quantidade máxima por linha
- Importações automáticas (segundos)
- Folga semanal
- Transição para Euro / Conversão de valores
- SmartChoice
- Ficheiros a exportar (vendas, mestres, parâmetros)
- Mostra fichas anuladas, muda nome empregado, cursor rotativo, código de pedido nos botões, fracções, formato data
- Transacções em disco (segurança vs velocidade)
- Arquivo de documentos (cria F<aammdd><zona>)
- Letras pequenas na janela de pedidos
- Numeração de documentos no apuramento de dia
- Imprime documentos na moeda não-base
- 4 casas decimais internas
- Conta cliente como método padrão
- Manutenção dá acesso a tudo

### 3.2 Controlo de Acessos

Configuração de permissões por nível de acesso. Ver `02-data-model/04-access-matrix.md`.

Também:
- Password Mestra
- Password de Bloqueio
- Password de Manutenção (activa matriz)

### 3.3 Configuração de Caixas

Já documentado em `03-cash-register.md §11`. Inclui aberturas automáticas, opções no comando, apuramentos automáticos, relatórios.

### 3.4 Configuração de Zonas de Impressão

Cria zonas, mapeia zonas × locais × origens → impressoras, configura agrupamentos e tipos de documento por contexto.

Já documentado em `05-kitchen-and-routing.md`.

### 3.5 Licenciamento

- Carregar ficheiro de licença (`.lic`)
- Verificar hardlock (USB/série) ou HardServer
- Mostrar dados da licença (casa, NIF, módulos activos, postos máximos)

OpenRest moderniza: licença online com revalidação periódica, ou ficheiro local com chave pública assinada.

### 3.6 Configuração de Locais

Já documentado em `02-catalog.md` (`local` é referenciado em catálogo + módulos). Inclui:

- Designação, intervalo de mesas, tipo (normal/take-away/etc.), tipo de preço, método pagamento default, taxa de serviço, limite consumo, imprime conta acima de
- Mapa de mesas (área com imagem + pontos)
- Imprime sub-total em / conta em / fecha mesa ao pedir
- IVA (mesa vs venda directa, IVA excluído)
- Cor empregado nas listas
- Impressora directa de pedidos
- Pede nova mesa depois fechar / após pedido
- Indica pessoas / só na abertura / permite zero
- Aloca mesas dinamicamente / circular
- Inclui desconto nos preços
- Artigos auto sem preço
- Carregamento rápido de mesas (10k)
- Só imprime pedidos com complementos
- Lista grande de pedidos
- Mesas só uma vez por dia
- Facturação externa (hotel)
- Não agrupa detalhes na conta
- Permite encaixe promoções / Separa antes encaixe
- Permite mesas abertas no fim do dia
- Pode identificar cliente no pedido (cantina)
- Obriga indicar valor pago

### 3.7 Configuração de Documentos

Já documentado em `02-catalog.md §16`. Inclui editor de cabeçalhos/rodapés com flags e construções avançadas.

### 3.8 Configuração de Terminais Rádio

Configurar comandos:
- Display iluminado (com horário)
- Pergunta nº pessoas
- Pergunta forma de pagamento
- Pergunta se imprime
- Pergunta com/sem desperdício
- Anulação faz consulta (envia todos os artigos para edição)
- Pede nº de empregado em vez de nº pessoas
- Actualiza comandos em automático
- Envia "cartão" em vez de "mesa"

Hardware rádio identifica casas pelo nº de licenciamento.

### 3.9 Propriedades do Posto

Já documentado em `02-data-model/01-entities.md §4.13`.

### 3.10 Hardware do Posto

Configura hardware ligado por porta de teclado (touchscreen, leitores, rato). E:

#### Drivers
Lista de dispositivos. Auto-detecção para escolher driver adequado.

#### Filtros
Sequências de transformação de leituras (`%d`, `%3d`, `%*c`, `%[1-4]`, etc.).

#### Redes Remotas
Outras lojas/instâncias. Configuração de:
- Código (corresponde à zona)
- Endereço (TCP/IP ou número de linha)
- Net
- Dispositivo de ligação
- Quem trata pedidos delivery recebidos
- Local destino dos pedidos
- Cliente call-center? Grava localmente?

### 3.11 Configuração de Teclas

Cada tecla (ou combinação) pode ser configurada para uma função:
- Artigo, Família, Mesa, Empregado (com valor opcional)
- Quantidade, Preço, Pedir, Anular, Cancelar
- Sub-total, Conta, Factura, Gaveta
- Transferências, Consulta Registos
- Troco, Pagamento Parcial, Desconto, Nº Pessoas
- Pedir Parcial / Anular Parcial / Sub-total Parcial (mantêm a mesa)
- Limpa Empregado
- Delivery

**Cartão**: configurar um código de barras/cartão como se fosse uma tecla. Permite operação completa por leitor.

**Sequências de teclas**: programar uma tecla para executar múltiplas teclas em série (ex: tecla "." envia três zeros).

### 3.12 Dispositivos

Já documentado em `05-integrations/` exaustivamente. Suporta:

Portas:
- Paralela, Série, DOS File, Porta Nula, Socket Port, Server Socket Port

Periféricos:
- Impressora Écran, Monitor Pedidos, Máquina Café, NetPay, Controlo Acessos, Impressora Genérica, Impressora Fiscal
- Botoneira (Série, Paralela)
- Gaveta Genérica
- Displays: Genérico, Interno (PAR/JarlTech), NCR 7460, Velleman MML30G
- Leitores: Cartões, Códigos de Barras
- Receptor Terminais Rádio, Antena V.02, Led ID
- Balanças: Genérica, Bizerba
- POS integrados: Jarltech 8100, POS PAR, NCR 7460, Bleep TS600/TS650
- Identificadores de chamadas: ZyXEL omni.net, AVM Fritz!X PC, Genérico (modem)

#### Mecanismo AUTO

Botão para carregar configurações padrão para um tipo de hardware. Acelera setup.

Passos típicos:
1. Adicionar Porta Série
2. Premir AUTO → escolher impressora → carrega config da porta
3. Adicionar Impressora Genérica
4. Premir AUTO → escolher modelo → carrega sequências ESC
5. Adicionar Gaveta Genérica

## 4. Ficheiros e Estrutura

### 4.1 Estrutura de directórios

```
openrest/
├── bin/                  Executáveis
├── files/
│   ├── data/             Tabelas
│   ├── export/           Exportações diárias
│   ├── import/           Importações (vindas do Store)
│   ├── modem/            Exportações secundárias (para envio remoto)
│   ├── archive/          Histórico (delivery, etc.)
│   ├── images/           Imagens dos botões
│   ├── timages/          Imagens das áreas de mapa de mesas
│   ├── ticket/           Campanhas (módulo Ticket)
│   └── tmp/              Temporários
├── config/
│   ├── device.cfg
│   ├── version.txt
│   ├── dictionary.yml    Dicionário i18n
│   ├── license.json      Licenciamento
│   ├── posto/            Configurações por posto
│   ├── fonts/
│   ├── icons/
│   ├── textures/
│   └── printer-drivers/
├── plugins/              Plug-ins (subdirectório por plug-in)
├── log/                  Logs (várias classes)
└── tools/                Utilitários (scancode, stp, hardtest, …)
```

### 4.2 Ficheiros principais (legado WinREST → equivalente OpenRest)

| Legado | Conteúdo | Equivalente OpenRest |
|---|---|---|
| `WRSTACCS.000` | Acessos | `nivel_acesso` + `acesso_permissao` |
| `WRSTCAIX.000` | Caixas | `caixa` |
| `WRSTDATA.000` | Data sistema | `definicoes_gerais.data_logica` |
| `WRSTDIAR.000` | Pratos do dia | `prato_do_dia_config` + `prato_do_dia` |
| `WRSTDRVS.WIN` | Dispositivos | `dispositivo` |
| `WRSTENVO.000` | Envelopes | `envelope` |
| `WRSTEXCL.000` | Exclusões | `exclusao` |
| `WRSTHAPY.000` | Happy Hour | `happy_hour` |
| `WRSTLSDC.000` | Configurações listagens | `listagem_config` |
| `WRSTMARM.000` | Mestre armazéns | `armazem` |
| `WRSTMART.000` | Mestre artigos | `artigo` |
| `WRSTMCLI.000` | Mestre clientes | `cliente` |
| `WRSTMEMP.000` | Mestre empregados | `empregado` |
| `WRSTMFAM.000` | Mestre famílias | `familia` |
| `WRSTPNT.000` | Pontos mapa mesas | `mapa_mesas_ponto` |
| `WRSTPROM.000` | Promoções | `promocao` |
| `WRSTRSV.000` | Reservas | `reserva` |
| `WRSTSCKB.000` | SmartChoice base | `smart_choice_kb` |
| `WRSTSES.000` | Temporário sessões | `sessao_empregado` (in-memory) |
| `WRSTTALI.000` | Nomes das mesas | `mesa.nome` |
| `WRSTTAUT.000` | Artigos auto | `artigo_automatico` |
| `WRSTTCDC.000` | Documentos | `documento_template` |
| `WRSTTCOM.000` | Imagem comandos | `terminal_radio_state` |
| `WRSTTIMP.000` | Impressoras | `dispositivo` |
| `WRSTTLOC.000` | Locais | `local` |
| `WRSTTMAP.000` | Mapas mesas | `mapa_mesas_area` |
| `WRSTTPAG.000` | Páginas rápidas | `pagina_rapida` |
| `WRSTTPOS.000` | Postos | `posto` |
| `WRSTTPRM.000` | Parâmetros | `definicoes_gerais` |
| `WRSTTRES.000` | Reservas refeição | `reserva_refeicao` |
| `WRSTTSER.000` | Contadores | `serie_documento` |
| `WRSTTTEC.000` | Teclas | `tecla_config` |
| `WRSTTZAR.000` | Centros custo | `centro_custo_zona` |
| `WRSTTZON.000` | Zonas | `zona_impressao` |

### 4.3 Variáveis de ambiente (`winrest.ini` → `openrest.toml`)

```toml
[global]
Net = 134
SlowCPU = true
Console = 9
Pointer = "MICROTOUCH"
TouchDevice = "/dev/tts/1"
DeleteOld = 90  # purga registos anulados há mais de N dias

[Country]
Code = "PT"

[Posto.1]
HardServer = true
[Posto.2]
TouchDevice = "/dev/tts/0"
VNCServer = true
VNCExclusive = true
VNCDepth = 2
WServerPort = 11002
WServer = "127.0.0.1:11002"
```

Variáveis essenciais (herdadas):
- `ExternalClient` — aceita cartões fora da BD
- `CompressExport` — comprimir registos diários
- `Console` — consola Linux
- `Country` — PT, ES, BR, SE, TR
- `DeleteOld` — dias para purga
- `DirectLock` — acesso directo a porta de hardlock
- `HardLock` — COM do hardlock
- `HardServer` — usar serviço de rede
- `Iber` — activar módulos externos (Iberia)
- `Pointer` / `TouchDevice` — drivers Linux
- `Posto` — número do posto
- `Net` — rede
- `NetTimeOut` — segundos (default 12)
- `NumLockOff` — teclado numérico desligado no arranque
- `RunWizard` — força wizard inicial
- `SlowCPU` — optimizações gráficas
- `WarnClients` — aviso para nº pessoas > 12

VNC: `VNCServer`, `VNCExclusive`, `VNCDepth`, `VNCCompression`, `WServer`, `WServerPort`.

HTTP: `HTTP_Port` (8080), `HTTP_DocumentRoot` (`www`), `HTTP_DefaultDocument` (`index.htm`), `HTTP_Compression`, `HTTP_UserName`, `HTTP_PassWord`.

## 5. Logs

OpenRest mantém o modelo de logging por classes do WinREST e estende-o:

Classes:
- `Control`, `Data`, `Debug`, `Deliv`, `Device`, `Flow`, `File`, `Gui`, `HTTP`, `Net`, `Radio`, `VNC`, `Wserver`
- Novas: `Sync`, `Audit`, `Security`, `Perf`

Parâmetros:
- `Append` — mantém log entre execuções
- `Commit` — força flush
- `Compress` — comprime (delog para descomprimir)
- `NoFlush` — agrupa escritas
- `Socket` — envia para socket em vez de ficheiro
- `FileName=log-(DATE).txt` — divisão por dia

Suporte a `[linux]` / `[windows]` para configurações cross-platform.

Visualização em tempo real: `OpenRest Logger` (UI separada).

## 6. Utilitários de diagnóstico

Pasta `tools/`:
- **scancode** — captura scancodes de leitores
- **stp** — terminal de porta série
- **hardtest** — diagnóstico de hardlocks
- **dbcheck** — verificação de integridade da BD
- **simctl** — simula dispositivos para testes
- **delog** — descomprime logs

## 7. Resolução de Problemas (catálogo)

Documento `08-appendices/04-troubleshooting.md` lista problemas comuns com causas e soluções, dividido por:
- Impressoras
- Hardware de comunicação rádio
- Outros periféricos
- Software
- Rede
- Performance
- Sincronização
- Fiscalidade
