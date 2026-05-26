# OpenRest — Mapeamento WinREST → OpenRest

> Para integradores e utilizadores históricos do WinREST FrontOffice PRO. Mapeia conceitos, ficheiros, configurações e funcionalidades.

## Hierarquia conceptual

| WinREST | OpenRest | Notas |
|---|---|---|
| Posto | Posto | Idem (com tipos fundamental/importante/secundário) |
| Local | Local | Idem (com vários tipos: normal, take-away, etc.) |
| Mesa | Mesa | Identidade própria, suporta nome excepcional |
| Pedido | Pedido | Atómico (todas as linhas ou nenhuma) |
| Conta | Documento (consulta, factura, VD) | Tipos diferenciados |
| Sessão | Sessão de Empregado | Por dia, por caixa |
| Caixa | Caixa | Acumulador de dinheiro |
| Turno | Turno | Subdivisão da caixa |
| Bolsa | Bolsa | Caixa pessoal de empregado |
| Dia | Dia Lógico de Caixa | Independente da data do relógio |
| Zona de Impressão | Zona de Impressão | Mantida |
| Origem | Origem | Mantida |
| Hardlock | Licenciamento Open | Modelo alternativo |
| Plug-in | Plug-in | WebAssembly preferido |
| Cabeçalho/Rodapé (C/R) | Documento Template | Multi-variante (1-9 para alguns tipos) |

## Ficheiros legados → entidades OpenRest

| Ficheiro WinREST | Entidade OpenRest |
|---|---|
| WRSTMART.000 | artigo |
| WRSTMFAM.000 | familia |
| WRSTMCLI.000 | cliente |
| WRSTMEMP.000 | empregado |
| WRSTMARM.000 | armazem |
| WRSTTLOC.000 | local |
| WRSTTMAP.000 | mapa_mesas_area |
| WRSTPNT.000 | mapa_mesas_ponto |
| WRSTTALI.000 | mesa.nome |
| WRSTCAIX.000 | caixa |
| WRSTSES.000 | sessao_empregado (in-memory) |
| WRSTTANT.000 | (lista negra de mesas + reservas) — separado |
| WRSTACCS.000 | nivel_acesso + permissoes |
| WRSTTPOS.000 | posto |
| WRSTTPRM.000 | definicoes_gerais |
| WRSTTSER.000 | serie_documento |
| WRSTTCDC.000 | documento_template |
| WRSTTCOM.000 | terminal_radio_state |
| WRSTTIMP.000 | dispositivo (impressora) |
| WRSTDRVS.* | dispositivo (geral) |
| WRSTPROM.000 | promocao + promocao_nivel(_item) |
| WRSTHAPY.000 | happy_hour |
| WRSTDIAR.000 | prato_do_dia_config + prato_do_dia |
| WRSTTRES.000 | reserva_refeicao |
| WRSTRSV.000 | reserva |
| WRSTTPAG.000 | pagina_rapida |
| WRSTEXCL.000 | exclusao |
| WRSTTAUT.000 | artigo_automatico |
| WRSTTTEC.000 | tecla_config |
| WRSTTZAR.000 | centro_custo_zona |
| WRSTTZON.000 | zona_impressao + zona_morada |
| WRSTLSDC.000 | listagem_config |
| WRSTLSCP.000 | listagem_campos |
| WRSTSCKB.000 | smart_choice_kb |
| WRSTENVO.000 | envelope |
| WRSTRANK.000 | complemento_recente |
| WRSTDATA.000 | definicoes_gerais.data_logica |
| IBERSALD.000 | fecho_financeiro_saldo |

## Configurações `winrest.ini` → `openrest.toml`

| Variável WinREST | Equivalente OpenRest |
|---|---|
| `ExternalClient` | `clientes.aceita_externos` |
| `CompressExport` | `export.comprimir` |
| `Console` | `posto.consola_linux` |
| `Country` | `localizacao.pais` |
| `DeleteOld` | `manutencao.purga_anulados_dias` |
| `DirectLock` | `seguranca.direct_lock` (legacy) |
| `HardLock` | `licenciamento.hardlock_com` (legacy) |
| `HardServer` | `licenciamento.hardserver` (legacy) |
| `Iber` | `modulos.iberia` |
| `Pointer` | `posto.touch.pointer` |
| `Posto` | `posto.id` |
| `Net` | `posto.net_id` |
| `NetTimeOut` | `rede.timeout_seg` |
| `NumLockOff` | `teclado.numlock_off` |
| `RunWizard` | `setup.executar_wizard` |
| `SlowCPU` | `performance.optimizar_cpu_lento` |
| `TouchDevice` | `posto.touch.device_path` |
| `WarnClients` | `pedidos.aviso_pessoas_acima_de` |
| `VNCServer/Exclusive/Depth/Compression` | `vnc.*` |
| `WServer/WServerPort` | `vnc.wserver`, `vnc.wserver_port` |
| `HTTP_Port/DocumentRoot/...` | `http_server.*` |

## Operações UI

| Botão WinREST | Acção OpenRest |
|---|---|
| Mesas | `/mesas` (consulta) |
| Reservas | `/reservas` |
| Lista Negra | `/mesas/lista-negra` |
| Pedidos | Ecrã de pedidos |
| Página Rápida | Toggle no ecrã de pedidos |
| Recebimento | Janela de fecho de mesa |
| Transferência | Janela de transferência |
| Anulação | Janela de anulação |
| Cancelar | Botão para limpar pedido em construção |
| Oferta | Janela de desconto |
| Pedidos por Código | Modo alternativo de entrada |
| Sub-total | Imprime consulta |
| Despacho | Ecrã de delivery |
| Consumo | Mesa especial de empregado |
| Ficheiros | Menu de configuração |
| Caixa | Ecrã de operações de caixa |
| Plug-ins | Lista de extensões |
| Sistema | Operações de sistema |
| Manutenção (Técnico) | Modo admin |

## Tipos de Documento

| Sigla / Designação WinREST | Tipo OpenRest | Variantes C/R |
|---|---|---|
| Venda a Dinheiro (VD) | factura_simplificada (PT moderna) | 9 |
| Factura | factura | 9 |
| Factura-Recibo | factura_recibo | 9 |
| Nota de Crédito | nota_credito | 9 |
| Consulta de Mesa | consulta_mesa | 9 |
| Pedido (cozinha) | pedido | 9 |
| Recibo | recibo | 1 |
| Senha | senha | 1 |
| Pontos | pontos | 1 |
| Apuramento de Dia | apuramento_dia | 1 |
| Facturação Externa | factura_externa | 1 |
| Validação de Cliente | validacao_cliente | 1 |

## Sequências especiais (flags) — mapeamento

Mantidas idênticas para compatibilidade conceptual. Lista completa em `02-printer-flags.md`.

| Flag | Conteúdo | Mantida? |
|---|---|---|
| `\no` | Nome da casa | ✓ |
| `\ds` | Designação social | ✓ |
| `\mo` `\lo` `\cp` `\pa` | Morada/Localidade/CP/País | ✓ |
| `\tf` `\fx` | Telefone/Fax | ✓ |
| `\cv` `\nr` `\cs` | Conservatória/Registo/Capital | ✓ |
| `\nc` | NIF da casa | ✓ |
| `\dt` `\da` `\sd` `\ho` `\hc` | Datas e horas | ✓ |
| `\nd` | Nº documento | ✓ |
| `\sX` | Sequências especiais (0–5) | ✓ |
| `\ne` `\oe` | Empregado | ✓ |
| `\nm` `\om` | Mesa | ✓ |
| `\bc` | Código de barras | ✓ |
| `\bX` | Bitmap | ✓ |
| `\ol` `\nx` `\nl` `\cl` `\cx` `\mc` `\ll` | Cliente | ✓ |
| `\np` `\pp` | Pessoas | ✓ |
| `\st` | Sub-total | ✓ |
| `\vt` | Valor total | ✓ |
| `\ve` | Valor moeda secundária | ✓ |
| `\sx` | Total sem IVA | ✓ |
| `\tx` | IVA | ✓ |
| `\ti` | Tabela IVA | ✓ |
| `\vc` `\vg` `\fp` `\tr` `\te` `\pg` `\pe` | Pagamento | ✓ |
| `\lc` | Local | ✓ |
| `\aX` | Atributos | ✓ |
| **Novas em OpenRest:** | | |
| `\atcud` | ATCUD | + |
| `\qr` | QR Code (PT) | + |
| `\hash` | Hash de assinatura (4 chars) | + |
| `\versao` | Versão do software | + |
| `\saft_versao` | Versão SAF-T | + |

## Construções XML-like

| Construção WinREST | OpenRest |
|---|---|
| `<! type="flag" id="..." !>` | Mantida |
| `<! type="field" id="..." !>` | Mantida |
| `<! type="uid" id="..." !>` | Mantida + UID modernizado |

## Hardware

### Portas

| WinREST | OpenRest |
|---|---|
| Porta Paralela | Mantida (legacy) |
| Porta Série | Mantida |
| DOS File | Substituída por File Path |
| Porta Nula | Mantida (mock) |
| Socket Port | Mantida (TCP client) |
| Server Socket Port | Mantida (TCP server) |
| — | + USB (nova) |
| — | + Bluetooth (nova) |
| — | + WebSocket (nova) |
| — | + gRPC (nova) |

### Impressoras

Suporte mantido a todos os modelos genéricos. Modelos específicos legacy continuam funcionais via drivers.

### Hardlock

Substituído por:
- Licenciamento local (chave assinada em ficheiro)
- Licenciamento cloud (revalidação periódica)
- Modo Community (sem licença para versões open)

## Modos especiais de Local

| WinREST | OpenRest |
|---|---|
| Normal | normal |
| Take-Away | take_away |
| Take-Away Seguro | take_away_seguro |
| PUB | pub |
| Entrega ao Domicílio | delivery |
| Consumo Próprio | consumo_proprio |
| (Restauração Colectiva) | restauracao_colectiva |

Configurações por local mantidas conceptualmente.

## Permissões (Acessos)

Mantida estrutura 1–9 níveis de acesso com permissões granulares. Permissões reorganizadas em árvore lógica (ver `02-data-model/04-access-matrix.md`).

## Tabelas paramétricas

Todas mantidas com mesma cardinalidade conceptual (geralmente 1–9 ou 1–999 conforme tabela).

## Métodos de Pagamento

Mantida a convenção:
- Código 1 = Numerário
- Código 9 = Conta Corrente
- Códigos 2–8 = configuráveis

Novos métodos suportados em OpenRest:
- MB Way
- Apple Pay / Google Pay
- Crypto (opcional)
- Vouchers / Gift Cards

## Importação a partir do WinREST

OpenRest oferece importador:
- Lê ficheiros `wrst*.000`
- Mapeia para entidades OpenRest
- Valida integridade
- Resolve conflitos (UI guida)
- Gera log de importação

Limitações:
- Histórico de documentos não é importado (recomenda-se manter sistema antigo em consulta)
- Configurações de hardware: refazer no novo sistema

## Diferenças intencionais

| Aspecto | WinREST | OpenRest | Razão |
|---|---|---|---|
| Storage | Ficheiros ASCII por entidade | SQLite | Atomicidade, queries |
| Arquitectura | Monolito C/C++ | Cliente-servidor Rust | Modernização |
| UI | Win32 / X11 desktop | Web / Tauri | Cross-platform melhor |
| API externa | Nenhuma estruturada | REST + gRPC + WebSocket | Integrações |
| Licenciamento | Hardlock USB | Licença assinada | Open / Cloud |
| Sync inter-loja | Mensagens remotas básicas | Event sourcing completo | Robustez |
| Multi-loja | Limitado | Primeira classe | Cadeias modernas |
| Plug-ins | Internal-only | WebAssembly + REST | Extensibilidade real |
| Logs | Ficheiros texto | Estruturados (JSON) | Observabilidade |
| Backups | Manual | Automático | Robustez |
| Fiscalidade PT | Manual / limitada | Integrada + certificação | Requisito legal moderno |
