# OpenRest — Bootstrap e Configuração Inicial

> Como instalar e configurar uma loja OpenRest da primeira vez. Inclui Wizard, ordem recomendada de configuração, e testes de verificação.

## 1. Pré-requisitos

### 1.1 Hardware mínimo

- Servidor: 4 cores, 8GB RAM, 128GB SSD, UPS
- Postos: depende (mini-CPC, mini-PC, tablet)
- LAN Gigabit
- Impressoras e periféricos conforme topologia

### 1.2 Software

- OS: Windows 10+ / Linux Ubuntu 22.04+ / macOS 13+ (apenas BackOffice)
- Runtime: nenhum (binário standalone)

### 1.3 Licenciamento

- Versão Community: open-source, sem limite (auto-licenciamento)
- Versão Enterprise: chave de licença + suporte
- Comunicação à AT (PT): NIF + certificado fiscal

## 2. Instalação

### 2.1 Windows

Executar instalador `.exe`:
1. Aceitar contrato
2. Escolher pasta (default: `C:\Program Files\OpenRest`)
3. Escolher modo: servidor / posto cliente / standalone
4. Configurar serviço Windows (auto-start)
5. Configurar firewall (portas)

### 2.2 Linux

```bash
# Debian/Ubuntu
sudo apt install ./openrest_1.0.0_amd64.deb

# RHEL/Fedora
sudo dnf install openrest-1.0.0-1.x86_64.rpm

# AppImage
chmod +x OpenRest-1.0.0.AppImage
./OpenRest-1.0.0.AppImage
```

Serviço systemd: `sudo systemctl start openrest-server`.

### 2.3 Docker

```bash
docker run -d \
  --name openrest \
  -p 8080:8080 \
  -v /data/openrest:/data \
  ghcr.io/openrest/server:latest
```

### 2.4 Tablet (Android)

APK ou Google Play (futura). Configurar URL do servidor da loja.

## 3. Primeira Execução / Wizard

No primeiro arranque, aparece o **Wizard de Configuração**.

```
[1] Língua → Português / Espanhol / Inglês / ...
[2] País → Portugal / Espanha / Brasil / ...
[3] Loja → Designação Social, NIF, Morada, Email, Telefone
[4] Logo → Upload (opcional)
[5] Modo → Standalone | Servidor | Cliente
[6] Licença → Carregar ficheiro / Modo Demo / Community
[7] Modelo de Catálogo → Restaurante / Café / Bar / Pizzaria / Take-Away / Vazio
[8] Postos → Adicionar postos conhecidos
[9] Impressoras → Adicionar
[10] Conclusão → Resumo + comece a configurar
```

Wizard pode ser saltado e revisitado depois.

## 4. Ordem Recomendada de Configuração

Para uma loja nova (sem Wizard ou em ajustes pós-Wizard):

### 4.1 Hardware do Posto (entrada)
1. Configurar touchscreen, teclado, leitor de cartões
2. Calibrar touchscreen
3. Testar entrada de cartão

### 4.2 Licenciamento
4. Activar licença (loja sem licença fica em modo demo: limites de artigos e empregados)

### 4.3 Hardware completo (dispositivos)
5. Adicionar impressoras
6. Gavetas
7. Displays de cliente
8. Balança
9. Câmaras (se videovigilância)
10. Testar cada dispositivo

### 4.4 Entidades Básicas
11. **Tipos de Preço** — definir nomes dos PVPs (PVP1=Mesa, PVP2=Take-Away, ...)
12. **Locais** — criar antes de configurar (Sala 1, Bar, Esplanada, Take-Away, …)
13. **Caixas** — uma por gaveta física
14. **Zonas de Impressão** — criar (Cozinha, Bar, D.Internos, D.Externos, ...) — sem mapear ainda

### 4.5 Configurações automáticas de Caixa
15. Caixas abrem automaticamente?
16. Sessões abrem com comando?
17. Apuramentos em automático no fecho?
18. Fecho directo no fim do dia?

### 4.6 Documentos
19. Configurar templates de Cabeçalho e Rodapé:
    - VD/FS — incluir `\ds`, `\nc`, ATCUD, QR
    - Factura — idem + cliente
    - Pedido — sem cabeçalho fiscal, só nome empregado e mesa
    - Consulta de Mesa
    - Recibo, Senha, etc.
20. Configurar detalhes (quais campos saem em cada documento e como)

### 4.7 Zonas de Impressão (mapping)
21. Para cada local, cada zona, cada origem → escolher impressora
22. Configurar agrupamentos
23. Configurar pedidos secundários se aplicável
24. Configurar complementos

### 4.8 Tabelas Paramétricas
25. Níveis de acesso (1–9)
26. Métodos de pagamento (1=Numerário, 2=MB, 3=Visa, ..., 9=CC)
27. Taxas de IVA conforme país
28. Unidades de movimento (Un, Kg, L, dose)
29. Tamanhos (Pequeno, Médio, Grande)
30. Atributos (opcional)
31. Grupos de comissão (empregado e artigo)
32. Grupos de desconto (cliente e artigo)
33. Tabelas matriz (comissões, descontos)
34. Comissões fixas
35. Qualidade de cliente
36. Zonas de morada

### 4.9 Catálogo
37. **Famílias** primeiro (com defaults bons: PVP, IVA, zona impressão, cores)
38. **Sub-famílias** (herdam família)
39. **Artigos** (herdam sub-família — só preencher exceções)
40. Configurar promoções, happy hour, pratos do dia se aplicável
41. Configurar artigos automáticos
42. Configurar páginas rápidas

### 4.10 Empregados
43. Adicionar todos (com mesas atribuídas, comissões, etc.)
44. Atribuir cartões / Led IDs

### 4.11 Postos
45. Confirmar propriedades de cada posto
46. Mesa por defeito
47. Mesas acessíveis
48. Opções activas (Mesas, Pedidos, Caixa, Sistema)
49. Impressora de sistema
50. Display de cliente
51. Gaveta
52. Caixa fixa (se aplicável)
53. Teclas customizadas

### 4.12 Clientes (opcional)
54. Importar lista de clientes existente (CSV)
55. Ou começar do zero e criar conforme necessário

## 5. Testes de Verificação

Antes de ir para produção, executar checklist:

### 5.1 Pedidos
- [ ] Fazer pedidos com **todos** os artigos
- [ ] Verificar que cada um sai na impressora correcta
- [ ] Confirmar formato do pedido (cabeçalho, campos, agrupamento)
- [ ] Testar complementos
- [ ] Testar pedidos secundários (se configurados)
- [ ] Testar promoções (cada nível)
- [ ] Testar artigos com preço variável
- [ ] Testar artigos a peso (balança)
- [ ] Testar artigos com código de barras
- [ ] Testar pedidos pelo teclado físico (se usado)
- [ ] Testar pedidos pelo comando rádio (se usado)

### 5.2 Recebimento
- [ ] Pagamento simples com Numerário
- [ ] Pagamento com troco
- [ ] Pagamento múltiplo (várias formas)
- [ ] Pagamento parcial
- [ ] Divisão de conta
- [ ] Conta corrente de cliente
- [ ] Identificação eventual com NIF
- [ ] Impressão da factura/VD/FS
- [ ] Reimpressão (2ª via)

### 5.3 Cozinha
- [ ] Pedidos aparecem no KDS (se há)
- [ ] Tempos de transição funcionam
- [ ] Bumps funcionam (apagar pedido)
- [ ] Falha de impressora → redireccionamento

### 5.4 Caixa
- [ ] Abrir caixa
- [ ] Abrir sessão
- [ ] Vendas registadas correctamente
- [ ] Movimentos (compras, vales) registados
- [ ] Fechar sessão (apuramento sai)
- [ ] Fechar caixa
- [ ] Fechar dia
- [ ] Exportações geradas

### 5.5 Fiscal (PT)
- [ ] ATCUD impresso
- [ ] QR Code lê com app móvel oficial AT
- [ ] Hash de assinatura no documento
- [ ] SAF-T gerado e valida contra XSD
- [ ] Cadeia íntegra (`openrest-verify-chain`)

### 5.6 Limpeza pré-Go-Live

Após testes, apagar dados de teste:

```bash
# CLI tool
openrest-cli reset-test-data \
  --keep-config \
  --keep-master-data \
  --confirm
```

Ou manualmente:
- Apagar ficheiros em `files/export/`
- Apagar `files/data/wrsttser.000` (contadores)
- Apagar documentos do dia (com cuidado!)
- Resetar `wrstcaix.000` e recriar caixas
- Acertar data lógica

## 6. Múltiplos postos

### 6.1 Setup do servidor

1. Configurar IP fixo
2. Abrir portas firewall (8080, 8443, gRPC)
3. Configurar mDNS para descoberta automática
4. Instalar certificado TLS (Let's Encrypt ou auto-assinado)

### 6.2 Setup de cada posto cliente

1. Instalar OpenRest Posto
2. Configurar `posto.toml`:
   ```toml
   posto_id = 2
   server_url = "https://servidor.local:8443"
   net_id = 134
   ```
3. Aceitar certificado do servidor
4. Login com credenciais de configuração
5. Confirmar dispositivos detectados
6. Calibrar touchscreen

### 6.3 Confirmação de Inter-operação

- Posto 1 abre mesa → Posto 2 vê em tempo real
- Posto 2 anula linha → Posto 1 vê
- Sincronização < 1 segundo

## 7. Multi-loja

### 7.1 Setup do central

1. Instalar OpenRest Central
2. Configurar PostgreSQL
3. Configurar autenticação
4. Adicionar lojas (cada com NIF, designação, certificado)

### 7.2 Setup de cada loja

1. Instalar servidor local
2. Configurar conexão ao central:
   ```toml
   central.url = "https://central.empresa.com"
   central.api_key = "..."
   loja_id = "uuid"
   ```
3. Sync inicial: pull do catálogo central
4. Confirmar bidireccional

## 8. Backup desde dia 1

```bash
# Cron job exemplo
0 2 * * * openrest-backup --output /backup/openrest-$(date +%Y%m%d).tar.gz

# Cloud sync (S3)
0 3 * * * aws s3 sync /backup s3://meu-bucket/backup/ --delete
```

## 9. Monitoring desde dia 1

1. Instalar Prometheus + Grafana
2. Apontar Prometheus a `http://servidor:9090/metrics`
3. Importar dashboards OpenRest
4. Configurar alertas:
   - Caixa não abre há > 1h após hora esperada
   - Impressora offline
   - Sync atrasada
   - Disco cheio
   - SSL próximo da expiração
   - Backup falhou
   - Crash detectado

## 10. Estrutura de directórios pós-instalação

```
/var/lib/openrest/         (Linux)
├── data/
│   ├── openrest.db        SQLite local
│   ├── chain/             Hash chain history
│   ├── event_log/         Event sourcing
│   └── backups/           Local backups
├── config/
│   ├── server.toml
│   ├── posto.toml
│   ├── log.toml
│   ├── posto/
│   ├── plugins/
│   └── license.json
├── files/
│   ├── export/
│   ├── import/
│   ├── archive/
│   ├── images/
│   ├── timages/
│   ├── ticket/
│   └── tmp/
├── plugins/
└── log/
```

## 11. Comandos CLI úteis

```bash
openrest-cli status                  # Estado geral
openrest-cli posto list              # Lista postos
openrest-cli printer test --id 1     # Teste impressora
openrest-cli backup                  # Backup manual
openrest-cli restore --from <file>   # Restore
openrest-cli license info            # Info licença
openrest-cli saft export --month 2024-01  # Export SAF-T
openrest-cli sync force              # Forçar sync
openrest-cli logs tail               # Tail logs
openrest-cli user create -u joao -n "João" --level 5
openrest-cli reset-test-data         # Limpar dados de teste
openrest-cli verify-chain            # Verificar cadeia fiscal
openrest-cli dbcheck                 # Integridade BD
```

## 12. Pós-Go-Live

### Primeira semana
- Monitorizar logs diariamente
- Verificar exportações
- Confirmar comunicações à AT (se PT)
- Recolher feedback dos operadores

### Primeiro mês
- Treino adicional para operadores em casos de uso menos frequentes
- Optimizar layouts (páginas rápidas, atalhos)
- Ajustar relatórios automáticos
- Documentar processos da casa

### Trimestralmente
- Update minor (segurança + bug fixes)
- Review de relatórios
- Auditoria de dados (consistência)
- Limpeza de logs antigos

### Anualmente
- Update major (com janela de manutenção)
- Renovação de certificados
- Revisão de licenças
- Backup de longo prazo (off-site)
