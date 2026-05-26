# OpenRest — Matriz de Permissões

Cada `nivel_acesso` tem associado um conjunto de flags booleanas. Esta matriz é a referência canónica. Heranças hierárquicas **não** são impostas (um nível 5 pode ter menos permissões que um nível 3) — segue convenção da casa.

## Operações vs flags

### Mesas

- `mesas.consultar` — abrir o ecrã de Mesas
- `mesas.totais` — consultar totais
- `mesas.reservas` — consultar reservas (requer módulo)
- `mesas.reservas.editar` — criar/alterar/apagar reservas
- `mesas.lista_negra.consultar` — ver lista negra
- `mesas.lista_negra.editar` — adicionar/remover mesas

### Plug-ins

- `plugins.acede` — entrar no menu Plug-ins
- `plugins.<chave>` — para cada plug-in, uma flag específica

### Pedidos

- `pedidos.acede` — abrir o ecrã de Pedidos
- `pedidos.mesas_outros` — entrar em mesas abertas por outros empregados
- `pedidos.faz_pedidos` — registar pedidos
- `pedidos.faz_pedidos.com_conta_impressa` — pedir após consulta de mesa impressa
- `pedidos.faz_pedidos.com_gaveta_aberta` — pedir com a gaveta aberta
- `pedidos.anula` — anular linhas pedidas
- `pedidos.anula.com_conta_impressa` — anular após consulta impressa
- `pedidos.recebe_mesas` — fechar mesa
- `pedidos.recebe_mesas.terminal_radio` — fechar via comando
- `pedidos.recebe_mesas.sem_conta_impressa` — fechar sem consulta impressa
- `pedidos.imprime_consulta_mesa`
- `pedidos.imprime_facturas` — controlar manualmente se imprime VD/factura
- `pedidos.faz_ofertas`
- `pedidos.consumo_proprio`
- `pedidos.cancelar` — cancelar pedido pendente
- `pedidos.escolhe_cliente`
- `pedidos.escolhe_cliente.lanca_conta_corrente`
- `pedidos.escolhe_cliente.lanca_cc.passa_limite_credito`
- `pedidos.transferencias`
- `pedidos.transferencias.com_conta_impressa`
- `pedidos.encomendar` — registar pedidos de delivery
- `pedidos.despachar` — atribuir entregadores
- `pedidos.entregar` — efectuar entregas
- `pedidos.outros_auto_consumos` — registar consumos para outros empregados

### Ficheiros

- `ficheiros.paginas_rapidas`
- `ficheiros.tabelas` — IVA, métodos pagamento, …
- `ficheiros.clientes`
- `ficheiros.clientes.edita_pontos`
- `ficheiros.empregados`
- `ficheiros.artigos`
- `ficheiros.familias`
- `ficheiros.armazens`
- `ficheiros.artigos_automatico`
- `ficheiros.happy_hour`
- `ficheiros.exclusoes`
- `ficheiros.promocoes`
- `ficheiros.pratos_do_dia`

### Caixa

- `caixa.abre_caixa`
- `caixa.abre_qualquer_sessao`
- `caixa.abre_sessao_propria`
- `caixa.pagamento_cc`
- `caixa.fundo_maneio`
- `caixa.emprestimo`
- `caixa.compra`
- `caixa.retirada`
- `caixa.vale`
- `caixa.transferencia_vendas_activas`
- `caixa.transferencia_turno`
- `caixa.fecha_qualquer_sessao`
- `caixa.fecha_sessao_propria`
- `caixa.fecha_caixa`
- `caixa.fecha_dia`
- `caixa.apura_qualquer_sessao`
- `caixa.apura_caixa`
- `caixa.apura_turno`
- `caixa.apura_dia`
- `caixa.consulta_registos`
- `caixa.consulta_registos.imprime_2via`
- `caixa.consulta_registos.estorna`
- `caixa.consulta_registos.imprime_listagem`
- `caixa.consulta_registos.visualiza_detalhes`
- `caixa.consulta_registos.edita_empregado`
- `caixa.consulta_registos.edita_metodo_pagamento`
- `caixa.estatisticas`
- `caixa.relatorios`

### Sistema

- `sistema.receptor` — consultar comunicações via rádio
- `sistema.redes_remotas`
- `sistema.redes_remotas.apaga_mensagens`
- `sistema.redes_remotas.envia_mensagens`
- `sistema.reimprime_documentos`
- `sistema.acerta_relogio`

### Hardware

- `gaveta.abre_avulsa` — abrir gaveta sem motivo de pagamento

### Manutenção (técnico)

- `manutencao.acede` — exige password técnica
- `manutencao.acessos`
- `manutencao.caixas`
- `manutencao.zonas_impressao`
- `manutencao.licenciamento`
- `manutencao.locais`
- `manutencao.documentos`
- `manutencao.terminais_radio`
- `manutencao.postos`
- `manutencao.dispositivos`
- `manutencao.teclas`
- `manutencao.definicoes_gerais`

## Modos especiais

- **Manutenção activa**: opcionalmente, em modo manutenção todos os acessos estão concedidos (configurável via `definicoes_gerais.manutencao_da_acesso_a_tudo`). Voltam ao normal no fecho do dia.
- **Acesso condicionado**: empregado de nível X só entra mediante autorização de empregado de nível Y (também configurável). Implementado por confirmação dupla (segunda autenticação superior).
- **Password Mestra**: dá acesso universal a qualquer prompt de palavra-passe — guardada em ficheiro de acessos, recomendada alteração periódica.
- **Password de Bloqueio de Posto**: senha global para desbloquear um posto bloqueado.
- **Password de Manutenção**: quando definida, a entrada em manutenção usa matriz alfabética (ver `01-architecture/05-security.md`).

## Modelo de armazenamento

```
nivel_acesso.permissoes = json["mesas.consultar": true, "caixa.fecha_dia": false, …]
```

Recomenda-se UI hierárquica em árvore com checkboxes, agrupada por secção.
