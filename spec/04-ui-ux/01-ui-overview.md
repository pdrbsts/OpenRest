# OpenRest — UI/UX (Visão Geral)

> Princípios de UX, conceitos visuais, padrões de interacção e layout dos ecrãs principais.

## 1. Princípios

1. **Toque primeiro** — interacções primárias com gestos amplos; teclas físicas e leitores são caminhos paralelos.
2. **Sempre acessível** — botões mínimos 48×48px (acessibilidade WCAG); contrastes AA.
3. **Errar é seguro** — operações destrutivas têm undo ou confirmação explícita.
4. **Feedback imediato** — cada acção tem confirmação visual (animação, som, vibração opcional).
5. **Modal mínimo** — janelas modais só quando estritamente necessárias.
6. **Configurabilidade visual** — cores, tamanhos de fonte, layouts ajustáveis por posto/utilizador.
7. **Familiaridade WinREST** — quem operou o sistema antigo deve conseguir operar este sem reformação radical.

## 2. Padrões de Componentes

### 2.1 Botão
- Texto grande, centrado por defeito
- Cor de fundo configurável (por família, empregado, etc.)
- Cor do texto configurável
- Imagem opcional (BMP/PNG/SVG, redimensionada)
- Tecla atalho mostrada no canto (canto superior direito)
- Estados: normal, hover, pressed, disabled
- Long-press: menu de contexto opcional

### 2.2 Caixa de texto / Campo
- Suporta teclado virtual (touchscreen)
- Suporta teclado físico
- Caixas especiais para Data, Hora, Moeda, Quantidade (com calculadora)
- Validação inline

### 2.3 Caixa de Lista
- Setas Up/Down
- Setas PageUp/PageDown
- Filtragem incremental (search-as-you-type)
- Barra de selecção bem visível (preto sobre claro)
- Suporte a multi-selecção (botões de transfer entre listas)

### 2.4 Caixa de Rolamento
- Mostra um valor de cada vez
- Cíclica
- Útil para datas, números, idiomas

### 2.5 Setas de Selecção (Anterior/Seguinte)
- Em listas paginadas (mesas, empregados, páginas de pedidos)
- Setas grandes, sempre visíveis quando há paginação

### 2.6 Caixa de Selecção (toggle)
- Pictograma + texto
- Estados: on / off
- Optional tri-state (indeterminate)

### 2.7 Caixa de Cor
- Paleta limitada de cores configurada
- Picker avançado opcional

## 3. Teclados Virtuais

### 3.1 Teclado Numérico

Aparece automaticamente em campos numéricos (touchscreen).

- 0–9, decimais
- 4 operações lógicas (+ − × ÷) para cálculo directo em campos de Qt e Moeda
- Backspace, Clear, Enter
- Toggle moeda base / moeda operador (quando aplicável)

### 3.2 Teclado Alfanumérico

Layout QWERTY.

- Shift, Ctrl, Alt, AltGr "pegajosos" (clicam sem arrastar)
- Cursor (Home/End, setas)
- Toggle entre dois layouts configurados (`tecla Swap`)
- Suporta línguas com caracteres especiais (ç, ã, ñ, ü, …)
- Layout configurável por instalação (PT, BR, ES, FR, DE, …)

### 3.3 Campo Data
Janela especial:
- Grelha de botões para o dia
- Caixa de rolamento para mês
- Caixa de rolamento para ano

### 3.4 Campo Hora
Janela especial:
- Grelha para horas
- Grelha para minutos (valores redondos por defeito; expandível para todos os minutos + segundos)

### 3.5 Campo Moeda
- Igual ao Numérico mas com símbolo monetário
- Suporta alternância de moeda em runtime

## 4. Ecrã Principal

```
+--------------------------------------------------+
| LOGO OpenRest                  [Hora]  [Rede][Antena]  
|  [Caps]  [Mensagens: HH em vigor]
+----------+---------------------------------------+
|          |                                       |
| Mesas    |                                       |
|          |        ZONA DE TRABALHO              |
| Pedidos  |        (janelas modais entram aqui)  |
|          |                                       |
| Ficheiros|                                       |
|          |                                       |
| Caixa    |                                       |
|          |                                       |
| Plug-ins |                                       |
|          |                                       |
| Sistema  |                                       |
|          |                                       |
+----------+---------------------------------------+
|        [LOGO / Zona de Retorno (toolbar)]        |
+--------------------------------------------------+
```

### 4.1 Zona de Selecção (esquerda)

Os 6 botões principais. Cada um pode ter:
- `Activo` / inactivo (esconde)
- `Pede Utilizador` — solicita login antes
- `Pede Código` — solicita password

### 4.2 Zona de Informação (topo)

- Hora e data (configurável: 12h/24h, formato data)
- Indicador de rede (verde/vermelho)
- Indicador de antena rádio (se hardware presente)
- Indicador de Caps Lock
- Janela de avisos: manutenção activa, erros pendentes, happy hour em curso, próxima reserva, …

### 4.3 Zona de Retorno (rodapé)

- Função "Escape"
- Botão LOGO abre **toolbar**:
  - Abrir gaveta (se acesso)
  - Alternar moeda base / operador
  - Calibrar touchscreen
  - Comutar entre consolas (versão Linux multi-consola)
  - Suspender posto
  - Ver acerca / versão

Se há só uma opção, o LOGO executa-a directamente.

## 5. Personalização

### 5.1 Cores

Por posto:
- Cor dos botões (fundo)
- Cor das janelas
- Imagem de fundo do ecrã principal
- Texturas (xadrez, gradiente, sólido)

Por elemento (família, artigo, empregado):
- Cor do botão
- Cor do texto

### 5.2 Tamanho da Fonte

- Letras grandes vs pequenas na janela de pedidos (`letras_pequenas`)
- Lista grande de pedidos (`lista_grande_pedidos`)
- Tamanho global por posto

### 5.3 Layout

- Botões na janela de pedidos (máximo antes de paginar)
- Resolução: 320×240, 640×480, 800×600, 1024×768, 1280×800, FullHD
- Rotação: 0, 90, 180, 270
- Multi-monitor (escolher monitor)
- Multi-consola (mesma máquina, vários postos virtuais — Linux)

### 5.4 Modo Cego / Daltónico

Toggles especiais:
- Modo daltonismo (cores adaptadas)
- Modo alto contraste
- Modo escuro (UI moderna)
- Tamanho de fonte aumentado
- Anúncios sonoros (reader)

## 6. Imagens e Recursos

### 6.1 Imagens de botões

- Localização: `files/images/`
- Formatos: BMP/TGA 24bit (legado), PNG/SVG (moderno)
- Redimensionamento automático ao tamanho do botão
- Compressão GZ para poupar memória (mantém extensão original)
- Nomes ≤ 8 chars (legado), longos suportados em OpenRest

### 6.2 Mapas de mesas

- Localização: `files/timages/`
- BMP 24bit, 580×560 px (recomendado)
- Suporta múltiplas áreas
- Até 10 pontos clicáveis por mesa

### 6.3 Fontes

- Ficheiros em `config/fonts/`
- Suporte a TrueType/OpenType moderno
- Múltiplos pesos (regular, bold)
- Fontes monoespaçadas para documentos (replica thermal printers)

### 6.4 Icons

- `config/icons/`
- Vector preferred (SVG)
- Suporte a temas (light/dark)

### 6.5 Texturas

- `config/textures/`
- Aplicadas a botões, janelas, fundo

## 7. Internacionalização

### 7.1 Sistema

- Dicionário em `config/dictionary.yml` (todos os textos)
- Multi-locale: pt-PT, pt-BR, es-ES, en-US, fr-FR, de-DE, ru-RU, …
- Fallback em cascata: locale específico → genérico → en-US

### 7.2 Locale do Empregado

Cada empregado tem `lingua`. UI muda quando empregado identifica-se.

### 7.3 Documentos

Templates podem ter variantes por locale (multilingue).

### 7.4 Formato de Números, Datas, Moeda

Inerente ao locale, com overrides em `definicoes_gerais`.

## 8. Acessibilidade

- Cumprir WCAG 2.1 AA
- Atalhos de teclado para todas as operações de UI
- Suporte a leitor de ecrã (em postos administrativos)
- Modo "operador inexperiente" com wizard step-by-step

## 9. Animações e Som

### 9.1 Animações

- Botão pressed: 100ms feedback
- Transição entre ecrãs: 200ms (configurável; off)
- Long-press detectado a 500ms
- Indicador de "a processar" para operações > 200ms

### 9.2 Som

- Bip ao premir botão (opcional)
- Som de erro
- Som de novo pedido (cozinha)
- Som de chamada (delivery)
- Som de reserva próxima

## 10. Modos especiais

### 10.1 Modo VNC / Remoto

Posto pode ser servido a clientes VNC (PDA, tablet). Configuração via:
- `VNCServer=1`
- `VNCExclusive=1` (não mostra UI local)

Em OpenRest moderno: substituir VNC por web (browser-based) ou WebRTC.

### 10.2 Modo Display Cliente

Postos podem operar como **display de cliente** (mostram detalhes ao cliente, não ao operador). Configuração separada.

### 10.3 Modo Demo

Sem licença:
- Limite de artigos (230)
- Limite de empregados (4)
- Mensagem "Versão de Demonstração" em todos os documentos
- Numeração zerada todos os dias

### 10.4 Modo Manutenção

- Símbolo de ferramenta intermitente
- Acessos ampliados (configurável)
- Algumas acções habituais bloqueadas

### 10.5 Modo Bloqueado

Posto bloqueado por inactividade ou comando. Mostra apenas tela de desbloqueio. Útil para deixar terminal seguro durante pausas.

## 11. Notificações no Sistema

### 11.1 Toasts

Mensagens não-modais:
- Verde: sucesso
- Azul: informativa
- Amarelo: aviso
- Vermelho: erro

Duração configurável; persistem até dismiss em erros graves.

### 11.2 Banner persistente

Para condições de longo prazo:
- "Modo manutenção activo"
- "Servidor central offline — em modo local"
- "Happy Hour activo até 23:00"

### 11.3 Modal alarm

Para situações que exigem acção:
- Mesa com pedido pronto há > 10 min
- Falta de papel em impressora crítica
- Conexão perdida com servidor

## 12. UX por modo de operação

Cada modo (`local.tipo`) gera variantes do ecrã de pedidos:
- **Normal** — fluxo completo de mesa
- **Take-Away** — botões de troco rápido
- **Take-Away Seguro** — mostra total só na confirmação
- **PUB** — botões transferência com identificação
- **Delivery** — botão chamada + janela cliente
- **Consumo Próprio** — restrições visíveis
- **Restauração Colectiva** — botão identificação cliente em vez de Qt
