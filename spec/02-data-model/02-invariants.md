# OpenRest — Invariantes e Regras do Modelo

Este documento lista as regras de integridade que o domínio impõe. São **invariantes do agregado**: o sistema nunca deve atingir um estado que as viole. Algumas são impostas pela legislação (PT, BR, etc.), outras pela operação prática herdada do WinREST.

## Códigos curtos vs IDs

- Famílias têm código **múltiplo de 100**; sub-famílias têm os restantes códigos abaixo da próxima família. Esta convenção é mantida para compatibilidade conceptual.
- Códigos de tabela paramétrica (`nivel_acesso`, `grupo_comissao_*`, `qualidade_cliente`, `grupo_desconto_*`, `tipo_preco`, `metodo_pagamento`, `taxa_iva`, `tamanho`) são **1–9**.
- Empregados: código 1–999.999 (6 dígitos).
- Clientes: código 1–999.999 (6 dígitos), expansível.
- Artigos: código numérico (4 dígitos no legado; em OpenRest deixa de ser limitado mas mantém ordenação numérica para teclados).
- Códigos de pedido podem coincidir com código do artigo, mas são uma chave separada (vários artigos podem ter o mesmo código de barras lido como pedido com sintaxe `xxcccccpppppy`).
- Zonas (morada): 1–999.

## Hierarquia da caixa

```
Dia
└── Caixa (1..N)
    └── Turno (1..N por caixa)
        └── Sessão (1..N por empregado)
            └── Bolsa (0..1 por sessão se trabalha com bolsa)
```

**Invariantes:**

1. `Dia` só fecha quando todas as `Caixa`s estão fechadas (ou se `fecho_directo=true`).
2. `Caixa` só fecha quando todas as `Sessão`s nela abertas estão fechadas.
3. `Sessão` só fecha quando todas as mesas abertas pelo empregado estão (a) fechadas, (b) transferidas a outro empregado ou (c) marcadas como `permite_mesas_abertas_fim_do_dia` no local.
4. Um empregado pode ter **uma** sessão aberta por vez **por loja**.
5. Movimentos directos de venda imputam-se à `Caixa` da sessão; se `Sessão.com_bolsa=true`, imputam-se à `Bolsa` e só transitam para a `Caixa` no fecho da sessão.
6. Compras, vales, fundos, retiradas — imputam-se ao **turno actual da caixa**.
7. Caixa com `caixa_fixa` num posto: todos os registos desse posto vão para a `Caixa` indicada, independentemente da sessão do empregado.
8. O **fundo de maneio** transportado de um dia para o seguinte aparece como `abertura.fundo_maneio_transportado` e é igual ao `saldo_transporte` do dia anterior.
9. Em qualquer instante: `valor_actual_caixa = soma(movimento_caixa.valor) WHERE caixa_dia_id = X AND turno_id ≤ actual`.

## Mesas

1. Uma mesa só pode estar **aberta uma vez** simultaneamente (`mesa_sessao` com `fechada_em IS NULL`).
2. Uma mesa não pode receber pedidos se estiver na `lista_negra_mesa`.
3. Acesso à mesa controlado por (a) `empregado.mesas_atribuidas` e (b) `local.permite_acesso_outras`. Excepção: o gerente com acesso superior pode entrar em qualquer mesa.
4. Mesas dos locais `take_away` fecham automaticamente no momento do pedido (semelhante a VD imediata).
5. Mesas dos locais `take_away_seguro` aceitam pedidos parciais, fecham na 2ª confirmação.
6. Mesas dos locais `pub` recebem todos os pedidos na "primeira mesa"; ao transferir, o destino é uma mesa que pode ser identificada pelo nome.
7. Em `aloca_mesas_dinamicamente=true`, o sistema escolhe automaticamente a próxima mesa livre do local; em `alocacao_circular=true`, prefere a com menos utilizações.
8. Em `mesas_uma_vez_por_dia=true`, uma mesa fechada nesse dia não pode ser reaberta no mesmo dia.

## Pedidos e detalhes

1. Um pedido é submetido como **unidade atómica**: todas as linhas são impressas ou nenhuma.
2. Distribuição de pedidos pelas impressoras respeita: `artigo.zona_impressao_id` (ou herdado de família), depois `impressora_zona_local` em função do `local_id` e `origem_id` do posto.
3. Complementos saem sempre na **zona do principal**, excepto quando configurado de outra forma.
4. Pedidos secundários: quando uma zona é marcada `secundarios=true`, recebe um pedido "espelho" indicando que os artigos saem junto com os de outra zona.
5. Anulação de uma linha já pedida exige permissão `anula_pedidos`; anulação **após sub-total impresso** exige `anula_pedidos.com_conta_impressa`.
6. Anulação **com desperdício** marca a linha como `anulada_com_desperdicio=true` (gasta stock); sem desperdício devolve ao stock.
7. Quantidade máxima por linha: `definicoes_gerais.qt_maxima` (anti-engano).
8. Os preços base dos artigos residem exclusivamente nas colunas estáticas `pvp1..pvp5` do artigo/família (não existem tabelas relacionais de preços). Em artigos com `pvp_variavel=true`, o preço unitário pode ser sobrescrito no pedido.
9. Artigos do tipo `informativo` não contribuem para o total da mesa nem aparecem no documento de conta — saem apenas no talão de pedido.
10. Artigos do tipo `gorjeta` calculam-se como `arredondar(total_actual * percentagem, arredondamento) + base`.

## Promoções

1. Uma promoção é um `artigo` (tipo `normal`) com `promocao` associada; o preço da promoção é o preço do artigo "cabeça".
2. Em cada nível da promoção, pelo menos um item deve ser escolhido (excepto níveis com `encaixe_permitido`).
3. Itens marcados `exclusivo=true` só são vendíveis dentro de uma promoção.
4. `delta_preco` de um item soma-se ao preço da promoção.
5. Promoção dentro de promoção é permitida (uma promoção pode ser item de outra), excepto auto-referência.
6. `permite_encaixe_promocoes` no local: itens podem ser pedidos depois e o sistema "encaixa-os" na promoção mais antiga compatível.
7. Em quantidades múltiplas, o sistema desmultiplica para optimizar: 10 menus podem ter 7 Cocas + 3 Fantas.
8. O sistema resolve ambiguidades **pelo menor custo total** (sempre que o mesmo item pode encaixar em vários níveis).

## Recebimentos

1. Soma dos `documento_pagamento.valor` deve ser ≥ `documento.total`; o excesso é troco.
2. Pagamentos parciais não fecham a mesa; geram documento de tipo `recibo_parcial`.
3. Divisão de conta gera N documentos novos, cada um com seu próprio método de pagamento.
4. Conta corrente do cliente: o pagamento para CC só pode exceder o `limite_credito` se o empregado tem permissão `pode_passar_limite_credito`.
5. Em clientes com `associacao_cliente_id`, o pagamento pode ser à associação, liquidando os associados (saldo da associação ≡ Σ saldos negativos dos associados + associação).
6. Em locais `obriga_indicar_valor_pago=true`, o utilizador tem que introduzir o valor pago para fechar a mesa.

## Fiscalidade (PT)

1. **Imutabilidade**: um documento fiscal emitido nunca é alterado. Para corrigir, emite-se uma `nota_credito` que referencia o original.
2. **Numeração**: estritamente sequencial dentro de série; gaps são proibidos.
3. **Hash de assinatura**: cada documento de venda inclui hash conforme Portaria 363/2010, dependendo do documento anterior (cadeia).
4. **ATCUD**: cada documento inclui `[CodigoValidacao]-[NumeroDoc]` desde 2023.
5. **QR Code**: payload obrigatório em D.Externos (factura, FR, VD, NC) — campos definidos pela Portaria 195/2020.
6. **NIF do cliente**: tem que ser válido (algoritmo de check digit PT). Em VD com total > limite legal (ex: € 1000 no PT), exige NIF preenchido.
7. **Designação social** (`\ds`) e **NIF da casa** (`\nc`) — obrigatórios em todos os documentos fiscais; o sistema **não imprime** o documento se faltarem.
8. **IVA na linha de detalhe**: o código IVA tem que aparecer em cada linha das facturas e VDs (regra PT).
9. **SAF-T PT**: exportação mensal exigida; estrutura conforme Portaria 302/2016 e actualizações.
10. **Data lógica**: `data_documento` ≤ data do relógio, com tolerância de horas (turnos pós-meia-noite). Não pode retroceder após fecho do dia.

## Empregados, comissões e consumo

1. Soma das percentagens em `produz_para` deve ser ≤ 100% (resto fica para o próprio empregado).
2. Em comissões de grupo, `recebe_de` indica quem produz para este empregado.
3. Comissão fixa por sessão é somada à variável.
4. Consumo próprio do empregado:
   - Total real = Σ detalhes pelo PVP normal do artigo.
   - Total a pagar = total real × `perc_consumo / 100` (após excluir `base_consumo`).
   - O empregado é debitado da diferença na sua CC.
5. Ofertas pelo empregado:
   - Acima de `base_ofertas`, o empregado paga (mesma lógica).
6. Comissões podem ser calculadas com ou sem IVA (configuração da `caixa`).

## Reservas

1. Uma reserva existe até `data_fim+hora_fim`; depois é `expirada` e some do cronograma.
2. Reservas no passado não geram histórico (são purgadas no fecho do dia).
3. Uma reserva pode estar em várias mesas (lista) mas cada mesa em "estado de reserva" apenas durante a janela `aviso_reserva_min` antes da hora.
4. O sistema **não** restringe o nº de pessoas por mesa.
5. Reservas inter-loja são possíveis via `rede_remota`; a senha gerada na loja A vale na loja B.

## Senhas de refeição (cantinas)

1. UID é gerado por: `encrypt(loja || artigo || data || refeicao || contador, codigo_secreto)`, codificado em EAN-13.
2. Contador por loja, com `offset` configurável para evitar colisões.
3. Quando lido e validado, o UID é marcado como **usado** (não reutilizável).
4. Validação remota: a senha emitida pela loja A para consumo na loja B usa o `codigo_secreto` partilhado.
5. Probabilidade de colisão aleatória ≈ 1/30M (limite estatístico aceite).

## Lista negra

1. Mesa em `lista_negra_mesa` é bloqueada imediatamente, mesmo que tenha consumo pendente — o empregado não pode acrescentar pedidos nem fechá-la pelos canais normais.
2. Desbloqueio requer permissão específica.
3. Inclusão pode ser individual (uma mesa) ou em bloco via intervalo `De..Até`.

## Documentos automáticos

1. **Artigo automático na abertura**: ao abrir mesa, os artigos configurados são adicionados (couvert, gorjeta…).
2. **Artigo automático no fecho**: ao fechar, perguntam-se quantidades (se `Qt=0`) ou aplicam-se directamente.
3. **Artigo automático no início do dia**: aplicado a mesas que ficaram abertas (hotel).
4. **Artigo encaixado**: quando o artigo automático é uma promoção e o local permite encaixe, o sistema agrupa os artigos já pedidos que pertencem à promoção.
5. **Transferência automática**: artigos com `quantidade_em_automatico` podem ser transferidos de uma "mesa origem" (slice) — usado para vendas por fatias.
6. **Quantidade por nº de pessoas**: `quantidade = factor × nº_pessoas + base`.

## Macros

1. Cada artigo registado num pedido tem associado um `nivel_macro` (default 0).
2. Artigos `identificadores_de_macro` aumentam o nível do bloco que se segue.
3. Artigos `execucao_de_macro` reimprimem na cozinha os pedidos anteriores desse nível.
4. Macros respeitam `mesma_zona_impressao` se assim configurado.

## Identificação e fidelidade

1. Identificação de cliente pode ser por: cartão (banda/RFID/código barras), código numérico, telefone (delivery), nome (parcial).
2. Pesquisa por telefone aceita sintaxe `225551234/5/6,22555321` com separadores `/` (sufixos) e `,` (vários números).
3. Pesquisa por nome aceita iniciais: "Jos Alex" matches "José Alexandre".
4. Pontos: `pontos += floor(total / valor_por_ponto)` no fecho do documento.
5. Pontos usados como pagamento: `valor_pago_pontos = pontos_usados × valor_por_ponto_venda`.

## Documentos de pedido (cozinha/bar)

1. Devem sair na zona configurada.
2. Devem incluir: empregado pedido, nº mesa, hora, lista de artigos.
3. `agrupamento` controla como linhas iguais são consolidadas.
4. Quando a impressora primária da zona está em erro, deve ser possível **redireccionar** em tempo real para outra impressora (sem perder pedidos).
5. Documentos guardados após impressão (configurável, default 2) permitem reimpressão.

## Listas e ordenação

1. Listas que exibem artigos respeitam `ordem_impressao` (1–9) — não a ordem alfabética.
2. Listas que exibem empregados respeitam a ordem em que foram criados (com possibilidade de manual reordering).
3. Botões de mesas respeitam o código da mesa.

## Multi-loja / sincronização

1. Cada loja tem o seu próprio sequencial de documentos; séries são prefixadas pela loja para evitar colisões.
2. Mestres (artigos, famílias, clientes) podem ser **partilhados** entre lojas (gerido no BackOffice) ou **locais**.
3. Conflitos de edição concorrente resolvem-se com `last-write-wins` por entidade, com excepção de fichas críticas (ver `02-data-model/05-sync.md`).
4. Mensagens remotas são guardadas localmente até confirmação de entrega.

## Robustez transaccional

1. Toda a operação que afecta mais que um ficheiro/tabela é **atómica** (commit/rollback). Não é admissível, por ex., incrementar o contador de facturas e ter a venda a falhar.
2. Em caso de falha de energia, ao reiniciar o sistema reaplica/cancela transacções pendentes a partir do log.
3. Cache de disco é forçada a *flush* nos pontos críticos (`fsync`) por defeito; desligável com perda de garantias (modo "rápido" do WinREST).
4. Backups automáticos diários (mínimo) com retenção configurável.
