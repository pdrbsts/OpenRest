# OpenRest — Topologias de Implementação

> Cenários típicos de deployment e a sua especificação.

## 1. Cenários

### 1.1 Single-Post (mini-restaurante / café)

```
┌──────────────────┐
│  PC POS          │
│  ┌──────────┐    │
│  │ Server   │    │
│  │ + Posto  │    │
│  │ + SQLite │    │
│  └──────────┘    │
└──┬───────┬───────┘
   │       │
[Impr.] [Gaveta]
```

- 1 PC com servidor + posto na mesma máquina
- 1-2 impressoras (talão + cozinha opcional)
- 1 gaveta
- 1 display cliente opcional

**Características**:
- Setup mais simples
- Backup → pen USB ou cloud
- Recuperação por restore de BD

### 1.2 Multi-Post (restaurante de média dimensão)

```
                ┌────────────────┐
                │   Servidor     │
                │  (PC fixo)     │
                │  SQLite/Postgres│
                └───┬──────┬─────┘
                    │      │
        ┌───────────┘      └───────────┐
        │                              │
┌───────▼──────┐                ┌──────▼───────┐
│ Posto Caixa  │                │ Posto Bar    │
│ + Impr. talão│                │ + Impr. bar  │
│ + Gaveta     │                │              │
│ + Display    │                │              │
└──────────────┘                └──────────────┘
        │                              │
┌───────▼─────────────────────────────┐
│  Impr. Cozinha (partilhada)         │
│  (acedida pela rede)                │
└─────────────────────────────────────┘
```

- Servidor central (pode ser um dos POS ou máquina dedicada)
- N postos (caixa, bar, balcão)
- Impressoras partilhadas em rede
- Possível Monitor de Pedidos (KDS) na cozinha

**Características**:
- LAN Gigabit recomendada
- Servidor com UPS
- Sync entre postos em milissegundos

### 1.3 Multi-Post + Comandos Rádio (restaurante grande)

Igual ao anterior + terminais rádio (legado) ou tablets móveis (moderno).

```
                ┌────────────────┐
                │   Servidor     │
                │   + Antena/WiFi│
                └───┬────────────┘
                    │
        ┌───────────┼───────────┬────────────┐
        │           │           │            │
   [Posto Caixa] [Bar] [Tablet 1] [Tablet 2] [Comando 1]
```

- Tablets/comandos para empregados de sala
- Pedidos directamente para cozinha sem ir ao posto

### 1.4 Multi-Loja (Cadeia / Franchising)

```
                 ┌─────────────────────┐
                 │  Servidor Central   │
                 │  (cloud ou DC)      │
                 │  PostgreSQL         │
                 │  BackOffice         │
                 └──┬──────┬──────┬────┘
                    │      │      │
       ┌────────────┘      │      └─────────────┐
       │                   │                    │
┌──────▼─────┐      ┌──────▼─────┐      ┌───────▼────┐
│  Loja A    │      │  Loja B    │      │  Loja C    │
│  Servidor  │      │  Servidor  │      │  Servidor  │
│  + Postos  │      │  + Postos  │      │  + Postos  │
└────────────┘      └────────────┘      └────────────┘
```

- Sincronização periódica (push em alterações de catálogo, pull em vendas)
- Cada loja autónoma offline (modo degradado se sem internet)
- Possível menu/preços comuns gerenciados centralmente
- Reports consolidados no Cloud

**Características**:
- Cada loja com seu servidor local (mesma stack)
- Sync via gRPC com retry/queue
- Encriptação TLS em todas as comunicações
- BackOffice (web) para gestão central
- Possível autenticação SSO

### 1.5 Cloud-Native (lojas pequenas com boa internet)

```
┌─────────────────────────────────────────┐
│   Cloud OpenRest (managed service)      │
│   (responsibilidade do fornecedor SaaS) │
└────────────────────────┬────────────────┘
                         │ (Internet)
        ┌────────────────┼────────────────┐
        │                │                │
┌───────▼────┐   ┌───────▼────┐   ┌───────▼────┐
│  Loja A    │   │  Loja B    │   │  Loja C    │
│  Postos só │   │  Postos só │   │  Postos só │
│ (thin clients)│   │ ...    │   │ ...    │
└────────────┘   └────────────┘   └────────────┘
```

- Sem servidor local; postos vão directamente à cloud
- Latência crítica → exige rede de baixa latência
- **Risco**: sem internet, postos param. Mitigação: modo offline com cache local.

Por defeito, OpenRest favorece a topologia 1.4 (cloud + local) sobre 1.5 (puramente cloud) — robustez vs. simplicidade.

### 1.6 Edge POS Android (Take-Away / Food Truck)

```
┌─────────────────────────┐
│  Tablet Android         │
│  OpenRest Posto Mobile  │
│  + SQLite local         │
└────┬──────────┬─────────┘
     │          │
  [Impr.    [Pinpad
   USB]     Bluetooth]
```

- Sem servidor "fixo"; tablet é tudo
- Sync com cloud quando há internet
- Cartão de pagamento via Bluetooth
- Bateria UPS embutida

## 2. Requisitos por topologia

### 2.1 Servidor

| Topologia | Especificação mínima |
|---|---|
| Single-post | 2 cores, 4GB RAM, 64GB SSD |
| Multi-post (até 5) | 4 cores, 8GB RAM, 128GB SSD, UPS |
| Multi-post (até 20) | 8 cores, 16GB RAM, 256GB SSD, RAID, UPS |
| Cadeia (BackOffice) | Cloud VM ou dedicada |
| Tablet único | tablet Android 10+, 3GB RAM |

### 2.2 Rede

- LAN Gigabit (POS multi-post)
- WiFi 5/6 com cobertura (tablets, comandos)
- Internet de banda larga (≥10Mbps) para cadeia
- VPN segura entre lojas e BackOffice (opcional)

### 2.3 Backup

- Local: snapshot diário em disco interno
- Externo: pen USB / NAS / cloud
- Recuperação testada periodicamente

## 3. Configuração de Posto

Por posto, definir:
- **Número** (identificador único)
- **Tipo**:
  - `fundamental` — sem este, o sistema não opera
  - `importante` — sem este, opera mas com aviso
  - `secundario` — opera sem
- **Resolução** e **rotação** do ecrã
- **Hardware** ligado
- **Mesas acessíveis**
- **Opções activas**
- **Impressora de sistema**
- **Display de cliente**
- **Gaveta**
- **Caixa fixa** (opcional)
- **Teclas** customizadas

## 4. Roles dos Postos

### 4.1 Caixa
- Tem gaveta
- Imprime VD/factura
- Pode fechar dia
- Tipicamente: 1 ou 2 por loja

### 4.2 Bar / Balcão
- Imprime pedidos para bar
- Pode ter gaveta
- Display cliente

### 4.3 Sala / Take-away
- Display cliente
- Tablet ou POS fixo
- Pode emitir pedidos para cozinha

### 4.4 Cozinha
- KDS (Monitor de Pedidos)
- Ou impressora térmica
- Sem operação completa (só visualização + bump)

### 4.5 Delivery / Despacho
- Sem gaveta principal
- Imprime pedidos
- Listagem de entregas

### 4.6 PDA / Tablet (móvel)
- Operação completa via touch
- Bateria

## 5. Funcionamento em rede Windows (legado)

Atalho com argumentos:
```
WINRESTW.EXE [nº posto] [nº rede] [IP servidor]
```

OpenRest desktop usa configuração em `config/posto/` em vez de argumentos.

Cada PC tem ficheiro `posto.toml`:
```toml
posto_id = 2
net_id = 134
server_url = "http://servidor.local:8080"
```

## 6. Multi-consola (Linux)

OpenRest mantém capacidade de executar múltiplos postos virtuais na mesma máquina:
- Consolas virtuais (Ctrl+Alt+F1..F9 no Linux)
- Útil para casos de "trocar de operador" sem encerrar a operação anterior
- Cada consola = um posto independente

Toolbar tem botão de troca de consola se houver mais que uma configurada.

Dispositivos partilhados:
- Impressoras (em rede) — não duplicar
- Displays, leitores, balanças, botoneiras, multiplexers — configurar em todos os postos virtuais
- O posto activo é que controla o dispositivo

## 7. Modos especiais

### 7.1 Modo "thin client" (VNC/RDP)

Posto sem aplicação local; mostra UI do servidor.
Útil para PDAs, tablets simples.

Em OpenRest, substituído por:
- PWA no tablet (mais leve)
- Browser apontando ao servidor
- Tauri configurado para "puxar" UI remota

### 7.2 Modo "ecrã exclusivo"

Posto configurado para servir só via VNC/Web; UI local não aparece.
Útil em servidores headless.

### 7.3 Modo "display cliente"

Posto inteiro mostra "vista cliente": detalhes do pedido em curso, animações.
Usado em fast-food com display grande.

## 8. Sincronização entre lojas

Documentada exaustivamente em [03-sync.md](./03-sync.md).

Resumo:
- Catálogo: push central → lojas
- Operação: pull central de lojas
- Mensagens remotas: bidireccional
- Reservas inter-loja: bidireccional
- Conflict resolution: last-write-wins por entidade, com excepções configuráveis

## 9. Resiliência

### 9.1 Falhas de rede

- Posto sem ligação ao servidor: avisa, mantém operação local (modo "degradado")
- Reconciliação ao restabelecer ligação

### 9.2 Falhas do servidor

- Postos detectam timeout
- Pode-se eleger um posto como "servidor de emergência" (failover)
- Restauração rápida de backup

### 9.3 Falhas de impressora

- Redireccionamento em tempo real
- Fila guarda até reactivação

### 9.4 Falhas de hardware (gaveta, balança, leitor)

- Desactivam-se com aviso
- Operação continua manualmente

### 9.5 Falha de energia

- UPS no servidor (mínimo 30 min)
- Transacções atómicas garantem consistência
- Reaplica transacções pendentes ao restart
