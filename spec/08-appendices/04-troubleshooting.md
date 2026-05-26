# OpenRest — Resolução de Problemas (Troubleshooting)

> Catálogo de problemas comuns com sintomas, causas prováveis e soluções. Herdado e expandido do manual técnico do WinREST.

Convenção:
- **P** — Problema
- **C** — Causa
- **V** — Verificação
- **S** — Solução

## 1. Impressoras

### P1. A impressora não imprime nada
- **C1.1** Porta mal configurada nos Dispositivos.
- **C1.2** Impressora off-line ou sem alimentação.
- **C1.3** Cabo ou hardware danificado.
  - **V1.3** Em Linux: `echo "test" > /dev/ttyS0`. Em Windows: `mode COM2:`, `dir > COM2:`.
- **C1.4** Parâmetros de comunicação errados (DIP-switch).
  - **S1.4** Confirmar com manual da impressora; ajustar DIP-switch.
- **C1.5** Redireccionamento activo para "Ignorar" ou "Espera".
  - **V1.5** Sistema → Redireccionar Impressoras.

### P2. Caracteres errados
- **C2.1** Parâmetros mal configurados (baud, paridade).
- **C2.2** Codepage errada.
  - **S2.2** Configurar codepage correcta (geralmente 850, 858 ou 1252).
- **C2.3** Cabo danificado.
- **C2.4** Modelo de impressora mal escolhido (sequências ESC erradas).

### P3. Imprime em DOS mas não no programa
- **C3.1** Configuração específica errada em Dispositivos.
  - **V3.1** Teste de impressão em Sistema → Teste de Impressão.
- **C3.2** Sem zonas de impressão mapeadas naquela impressora.
- **C3.3** Sem artigos a sair na zona pretendida do local em uso.
- **C3.4** Impressora redireccionada.

### P4. Sempre duplicados
- **C4.1** Número de cópias mal configurado.

### P5. Imprime mas não abre gaveta
- **C5.1** Gaveta mal configurada nas Propriedades do Posto.
- **C5.2** Impressora com 2 saídas, cabo usa a errada.
  - **S5.2** Configurar para 2ª saída em dispositivos.
- **C5.3** Cabo da gaveta foi para outro modelo de impressora.

### P6. Duas mudanças de linha em vez de uma
- **C6.1** Impressora configurada para fazer LF automático.
  - **S6.1** DIP-switch para desactivar LF automático.

### P7. Impressora térmica corta papel a meio
- **C7.1** Configuração ESC do auto-cut errada.
- **C7.2** Erro do hardware (sensor entupido).

### P8. Códigos de barras não imprimem
- **C8.1** Impressora não suporta CODE128 nativo.
  - **S8.1** Activar "Imprime códigos de barras ESC/POS" ou usar bitmap (default).
- **C8.2** Sequência ESC de início errada.

## 2. Hardware Rádio (legado)

### P1. Nada no écran da antena
- **C1.1** Sem alimentação externa.
- **C1.2** Alimentada pelo PC com cabo mal feito.

### P2. Antena não recebe pedidos
- **C2.1** Porta mal configurada.
  - **V2.1** Em Dispositivos: 4800, n, 8, 1, sem flow control.
- **C2.2** Cabo errado.

### P3. Comandos ficam em "Enviando..."
- **C3.1** Antena não passa recepção.
- **C3.2** Antena sem alimentação.
- **C3.3** Nº de licenciamento no terminal ≠ no programa.
  - **S3.3** Em Acerca do WinREST, ver nº e configurar nos terminais.
- **C3.4** Números internos diferentes.
- **C3.5** Bateria fraca do terminal.
- **C3.6** Obstruções metálicas / paredes.

### P4. Terminais não carregam artigos
- **C4.1** Terminal não configurado para receber.
  - **V4.1** No terminal: `9005 3 255 #` → coloca em modo 255.

## 3. Outros Periféricos

### P1. Touchscreen TPIS não funciona
- **C1.1** Cabo da porta de teclado desligado.
- **C1.2** Programa mal configurado.
- **C1.3** Saturação luminosa.

### P2. Touchscreen TPIS retorna sempre a mesma posição
- **C2.1** Hardware avariado.

### P3. Touchscreen capacitivo (moderno) não detecta toque
- **C3.1** Driver não instalado.
- **C3.2** Calibração necessária.
  - **S3.2** Acertar zonas via menu Toolbar.
- **C3.3** Modo `VNCExclusive=1` activo sem touch redirected.

### P4. Leitor de cartões não funciona
- **C4.1** Leitor desligado ou mal configurado.
  - **V4.1** Testar no DOS prompt (legado) ou com `evtest` (Linux moderno).
- **C4.2** Cartão mal configurado na ficha.
- **C4.3** Empregado sem sessão aberta.

### P5. Rato não aparece
- **C5.1** Driver não carregado (Linux: `xinput list`).
- **C5.2** Programa não configurado para rato.
- **C5.3** Touchscreen compatível com rato a ocupar lugar.

### P6. Balança não devolve peso
- **C6.1** Sequência de leitura mal configurada.
- **C6.2** Modo "Envia sequência" desactivado quando deveria estar activo.
- **C6.3** Pino de alimentação errado.

### P7. Display de cliente em branco
- **C7.1** Sequência de inicialização errada.
- **C7.2** Baud rate diferente.
- **C7.3** Velocidade demasiado alta — adicionar pausas.

## 4. Software

### P1. Programa não executa
- **C1.1** Variável `POSTO` não definida (legado).
- **C1.2** Driver `DISPLAY.SYS` no `CONFIG.SYS` (legado).
- **C1.3** Tenta correr sob DPMI (legado).
- **C1.4** `EMM386.EXE` não carregado (legado).
- **C1.5** IPX não disponível (legado).
- **C1.6** Servidor não encontrado.
  - **V1.6** Verificar `net` no posto = `net` do servidor; verificar firewall.

### P2. "Foi perdida a conexão com o posto X"
- **C2.1** Posto desligado / sem rede.
- **C2.2** `NetTimeOut` curto demais.
  - **S2.2** Aumentar `NetTimeOut` em `winrest.ini`.

### P3. Programa lento
- **C3.1** Hardware fraco.
  - **S3.1** Activar `SlowCPU=true`.
- **C3.2** BD com milhões de registos, sem índices.
- **C3.3** Log com classes muito detalhadas (ClassFile=3, ClassFlow=3) — desactivar.

### P4. Programa crasha aleatoriamente
- **C4.1** Memória insuficiente.
- **C4.2** Driver de hardware com bug.
  - **V4.2** Activar log com `ClassDevice=3`, reproduzir, analisar.

### P5. Documentos não exportam
- **C5.1** Definições Gerais → Ficheiros a exportar com tipos errados.
- **C5.2** Directório `export/` sem permissões de escrita.
- **C5.3** Disco cheio.

### P6. Importação falha
- **C6.1** Estrutura do ficheiro mudou entre versões.
- **C6.2** Linhas corrompidas (ASCII inválido).

## 5. Rede

### P1. Sync entre lojas não funciona
- **C1.1** Firewall bloqueando porta.
- **C1.2** Certificado expirado.
- **C1.3** Tokens inválidos.
- **C1.4** Vector clock divergente — exige resolução manual.

### P2. Mensagens remotas não chegam
- **C2.1** Rede remota mal configurada (IP errado).
- **C2.2** ComServer não a correr no outro lado.

### P3. API HTTP retorna 401
- **C3.1** Token expirado.
- **C3.2** Permissões insuficientes.

### P4. WebSocket cai constantemente
- **C4.1** Proxy intermédio fecha conexões longas.
  - **S4.1** Configurar reconnect automático + keepalive.

## 6. Fiscalidade

### P1. Documento não emite — falta NIF da casa
- **C1.1** Template sem flag `\nc`.
  - **S1.1** Configurar nas Definições Gerais e no template.

### P2. Hash de assinatura inválido
- **C2.1** Cadeia quebrada (documento anterior em falta).
- **C2.2** Chave privada perdida — gravíssimo, requer migração.

### P3. SAF-T inválido contra XSD
- **C3.1** Campo obrigatório vazio.
- **C3.2** Encoding errado (devia ser UTF-8).
- **C3.3** Versão SAF-T errada.

### P4. ATCUD em branco
- **C4.1** Série ainda não comunicada à AT.
  - **S4.1** Plug-in de comunicação de séries.

### P5. QR Code não scaneável
- **C5.1** Versão do QR demasiado baixa (pequeno demais).
- **C5.2** Brilho da impressão fraco.

## 7. Operação

### P1. Empregado não consegue entrar
- **C1.1** Cartão diferente.
- **C1.2** Password errada (lock após N tentativas).
- **C1.3** Nível de acesso revogado.
- **C1.4** Sessão "fantasma" aberta de outro posto.
  - **S1.4** Fechar sessão manualmente em ferramentas de admin.

### P2. Mesa "fantasma" sem dono
- **C2.1** Crash durante operação deixou mesa órfã.
  - **S2.1** Ferramenta de "reaplicar transacção pendente" no arranque resolve.
- **C2.2** Empregado fechou sessão sem fechar mesa (e `permite_mesas_abertas_fim_do_dia=false`).

### P3. Numeração saltou um número
- **C3.1** Falha técnica durante emissão.
  - **S3.1** Verificar event log; emitir "documento anulado" com esse número para fechar o gap.

### P4. Caixa não fecha
- **C4.1** Sessões abertas pendentes.
- **C4.2** Mesas abertas (no local errado).
- **C4.3** Valor introduzido < valor teórico (fecho cego com diferença) — pode ser intencional, registar como `quebra_de_caixa`.

### P5. Apuramento não imprime
- **C5.1** Impressora de sistema mal configurada no posto.
- **C5.2** Documento template `apuramento_dia` não existe ou está vazio.

### P6. Conta corrente do cliente fica negativa
- **C6.1** Limite de crédito ultrapassado mas empregado tem permissão.
- **C6.2** Pagamento de outro cliente não associado contou.

## 8. Performance

### P1. Pedidos demoram a aparecer na cozinha
- **C1.1** Rede congestionada.
- **C1.2** Impressora série a baixa velocidade.
- **C1.3** Bitmap pesado a ser imprimido.

### P2. UI engasga ao listar mesas
- **C2.1** Demasiadas mesas no local (10k+).
  - **S2.1** Activar `carregamento_rapido_mesas=true`.

### P3. Login demora
- **C3.1** Hash de password muito caro (argon2 com parâmetros altos).
  - **S3.1** Ajustar parâmetros sem comprometer segurança.

## 9. Backups e Recuperação

### P1. Backup não correu
- **C1.1** Disco de destino cheio.
- **C1.2** Permissões.
- **C1.3** Scheduler desactivado.

### P2. Restore não recupera operação corrente
- **C2.1** Backup é de antes da última operação — perda esperada.
- **C2.2** Backup encriptado — chave necessária.

## 10. Plug-ins

### P1. Plug-in não aparece
- **C1.1** Manifesto inválido.
- **C1.2** Versão do OpenRest muito antiga.
- **C1.3** Excluído pelo nível de acesso.

### P2. Plug-in crasha
- **C2.1** Sandbox restritivo.
  - **S2.1** Verificar permissões declaradas no manifesto.

### P3. Plug-in lento
- **C3.1** Plug-in WebAssembly não optimizado.
- **C3.2** Plug-in externo com latência de rede.

## 11. Atualizações

### P1. Update falha a meio
- **C1.1** Disco insuficiente.
- **C1.2** Sistema operacional não suportado.
- **S1**: Rollback automático para versão anterior.

### P2. Migration de BD falha
- **C2.1** Dados em conflito com nova estrutura.
- **C2.2** Migration script com bug.
- **S2**: Restore do snapshot pré-update.

## 12. Hardware desconhecido

### P1. Modelo de impressora não está na lista
- **S1.1** Usar Impressora Genérica + configurar sequências manualmente.
- **S1.2** Pedir/contribuir driver para o projecto.

### P2. Leitor de cartões com formato estranho
- **S2.1** Configurar filtro com regex.
- **S2.2** Usar `tools/scancode` (legado) para capturar scancodes.

## 13. Utilitários de diagnóstico

Pasta `tools/`:

| Ferramenta | Função |
|---|---|
| `openrest-scancode` | Captura scancodes de leitores |
| `openrest-stp` | Terminal de porta série |
| `openrest-hardtest` | Diagnóstico de hardlocks (legado) |
| `openrest-dbcheck` | Verificação de integridade da BD |
| `openrest-simctl` | Simula dispositivos para testes |
| `openrest-delog` | Descomprime logs |
| `openrest-verify-chain` | Verifica integridade da cadeia fiscal |
| `openrest-validate-saft` | Valida XML SAF-T contra XSD |
| `openrest-replay` | Replay de eventos para reconstrução |

## 14. Logs úteis

Em primeira linha:

- `log/openrest.log` — log principal
- `log/devices.log` — eventos de hardware
- `log/sync.log` — sincronização
- `log/fiscal.log` — operações fiscais
- `log/audit.log` — audit trail

Em problema reproduzível, activar log com classes detalhadas para a área afectada:

```yaml
classes:
  ClassDevice: 3
  ClassFile: 2
filename: "log/debug-(DATE).txt"
append: true
```

## 15. Contacto / Suporte

- **GitHub Issues** — bugs e features
- **Forum** — perguntas e discussão
- **Discord/Matrix** — chat em tempo real
- **Suporte Enterprise** — SLA contratual
- **Mailing list de segurança** — `security@openrest.org` (privada)
