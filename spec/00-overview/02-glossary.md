# OpenRest — Glossário Completo

Glossário consolidado de termos do domínio, herdados do WinREST e/ou introduzidos pela arquitectura OpenRest. Sempre que possível, mantém-se a designação histórica para minimizar fricção de reformação.

## A

- **Acerto (de máquina de café)** — Botão especial num dispositivo de produção de doses (máquina de café) que permite produzir uma quantidade "extra" sem nova autorização — usado tipicamente para acertar a quantidade de água num café. Só pode ser usado uma vez após uma produção válida.
- **Acesso condicionado** — Empregado de um determinado nível de acesso só consegue entrar na aplicação com autorização de outro de nível superior.
- **Activo (atributo de opção/posto)** — Indica se uma opção está visível e operacional num posto.
- **Aloca mesas dinamicamente** — Modo de local em que os pedidos entram sempre numa "primeira mesa" e o sistema redistribui-os por mesas livres do local. Útil em discotecas com cartões de consumo.
- **Alocação circular** — Variante do anterior que escolhe sempre a mesa livre menos utilizada, distribuindo o desgaste.
- **Anulação** — Remoção de um artigo já pedido (já impresso em cozinha). Pode ser *com desperdício* (gasta stock) ou *sem desperdício* (devolve ao stock).
- **Apuramento** — Mapa impresso com o fecho parcial ou total de uma unidade da hierarquia (sessão, caixa, turno, dia).
- **Área (de mapa de mesas)** — Imagem de planta de sala com pontos clicáveis associados a mesas. Independente do conceito de Local.
- **Área de Entrega** — Zona geográfica configurada para o serviço de entrega ao domicílio (Delivery), podendo ter associada uma taxa de entrega específica.
- **Artigo** — Linha de catálogo vendável. Tem código, designação, preço, IVA, zona de impressão, tipo, complementos, etc.
- **Artigo em Automático** — Artigo que é inserido automaticamente numa mesa em determinados eventos (abertura, fecho, início do dia, pedido de outro artigo associado).
- **Artigo Informativo** — Artigo sem preço; aparece no talão para cozinha mas não na conta do cliente.
- **Atalho** — Botão dedicado para uma operação frequente (ex: imprimir relatório X).
- **ATCUD** — Código Único de Documento (Portugal): `[ATCUD série]-[nº sequencial]` impresso em cada documento fiscal.
- **Atributo** — Etiqueta com até 3 dimensões usadas para classificar documentos para estatística (ex: ocasião, tipo, época).

## B

- **Backoffice / Store / Reports** — Aplicação externa que consome dados exportados do OpenRest. Fora de escopo do FrontOffice.
- **Base de Consumo** — Valor mensal/diário que um empregado tem direito a consumir gratuitamente; acima paga a diferença.
- **Base de Ofertas** — Valor de ofertas a clientes que o empregado tem permissão para fazer sem custo pessoal.
- **Bitmap** — Imagem que pode ser referenciada em documentos por `\b0..\b9`. Tipicamente o logo da casa.
- **Bolsa** — Acumulador de dinheiro associado a um empregado (não a uma gaveta). Permite ter empregados a vender com a caixa principal noutro turno.
- **Botão de Selecção** — Botão de UI que representa uma família, sub-família, artigo, mesa, empregado, cliente, etc.
- **Botoneira** — Dispositivo de hardware com contactos (série ou paralela) que envia teclas para o programa quando pressionado.

## C

- **C/R (Cabeçalho/Rodapé)** — Template usado por um tipo de documento. Para vendas a dinheiro, facturas, pedidos e consultas de mesa existem até 9 templates distintos numeráveis 1–9. Outros tipos têm 1.
- **Caixa** — Acumulador físico/lógico de dinheiro. Cada gaveta física ≈ uma caixa. Pode haver várias caixas em simultâneo.
- **Caixa Fixa** — Configuração de posto que força todos os registos desse posto a contabilizar para uma caixa específica, ignorando onde o empregado abriu sessão.
- **Carregamento rápido de mesas** — Optimização que carrega só mesas com consumo no arranque (típico em discotecas com 10.000 cartões).
- **Centro de Custos** — Armazém lógico ao qual é abatido o stock. Associado a uma zona de impressão.
- **Cliente Associado / Associação** — Cliente cuja conta corrente é paga por outro cliente (ex: empresa, família). Permite agrupar contas.
- **Codepage** — Tabela de caracteres da impressora (ex: 437, 850, 858, 1252).
- **Código de Pedido** — Código numérico curto pelo qual um artigo é pedido via teclado/comando.
- **Comando (terminal rádio)** — Dispositivo portátil rádio para fazer pedidos à mesa. Tem identificador único e configuração própria.
- **Comissão Fixa** — Valor monetário absoluto que o empregado recebe por sessão (não depende das vendas).
- **Comissão Variável** — Percentagem cruzada Grupo-de-Comissão-Empregado × Grupo-de-Comissão-Artigo.
- **Complemento** — Artigo que acompanha um principal (ex: "Bem passado", "Sem cebola"). Imprime na mesma zona do principal por defeito.
- **Compras (movimento de caixa)** — Saída de dinheiro da caixa para aquisição de bens/serviços.
- **Conta Corrente (CC)** — Saldo a crédito/débito de um cliente ou empregado.
- **Consumo Próprio** — Operação em que um empregado regista o seu próprio consumo (tipicamente consumo dos funcionários). Normalmente há produtos gratuitos e outros pagos a preço reduzido. Este local utiliza habitualmente uma tabela de preços (PVP) diferente dos restantes locais.
- **Consulta de Mesa** — Documento informativo com o consumo actual da mesa. Não tem valor fiscal.
- **Cronograma de Reservas** — Vista temporal das reservas (mensal/diária).

## D

- **D.Externos** — Documentos com valor fiscal: factura, venda a dinheiro, factura-recibo, recibo, nota de crédito.
- **Data Lógica de Caixa** — Data atribuída pelo programa ao "dia de operação". Pode diferir da data do relógio do sistema (ex: trabalho que se prolonga após meia-noite).
- **Despacho (Delivery)** — Janela que atribui encomendas pendentes a entregadores.
- **Desperdício (anulação com)** — Indica que o artigo anulado já foi consumido/produzido; não volta para stock.
- **Display de Cliente** — Mostrador secundário virado para o cliente, mostra item registado, total e troco.

## E

- **Empregado** — Utilizador do sistema. Tem código, password, cartão, nível de acesso, comissões, mesas atribuídas.
- **Empréstimo (movimento de caixa)** — Entrada de dinheiro do empregado para a caixa, criando crédito na CC do empregado.
- **Encaixe de promoções** — Modo de local em que se pode pedir uma promoção sem todos os seus itens, completando-a depois ("o café fica para depois").
- **Entregador / Motoboy** — Responsável por efectuar entregas ao domicílio (Delivery). Não necessita ser um empregado da empresa, podendo ser um prestador de serviços externo.
- **Envelope / Depósito** — Operação informativa que regista o ensacamento de dinheiro retirado da caixa, mas continua contabilizado como em caixa até ao fecho do dia.
- **Estorno** — Anulação de documento já fechado. Existe estorno com e sem desperdício.
- **Exclusão** — Configuração que esconde famílias, artigos ou empregados num determinado posto ou local; também pode esconder operações (transferir, fechar).
- **Exclusivo (artigo)** — Artigo que só pode ser vendido como parte de uma promoção.
- **Export / Modem** — Directórios de exportação de registos diários, para Store/Reports ou envio remoto.

## F

- **Factor de Conversão** — Multiplicador associado a um método de pagamento para converter para moeda base.
- **Factor de Multiplicação por Pessoa** — Usado em artigos automáticos cuja quantidade depende do nº de pessoas na mesa.
- **Factura** — Documento fiscal nominativo (com NIF do cliente).
- **Factura-Recibo** — Factura com recibo de quitação no mesmo documento.
- **Família** — Grupo de cabeçalho de catálogo (códigos múltiplos de 100, ex: 600 Bebidas). Define defaults para sub-famílias e artigos.
- **Família Superior** — Atributo de uma família ou sub-família que indica a sua "pai".
- **Fecho do Dia** — Operação final do dia que congela todos os movimentos, exporta dados e avança para o dia seguinte.
- **Fecho Financeiro** — Módulo adicional que produz mapa económico e financeiro detalhado (recibos, métodos de pagamento, depósitos, despesas, IVA).
- **Filtro** — Sequência de transformação aplicada a leituras de leitor de cartões/códigos para extrair o ID útil.
- **Folga Semanal** — Configuração que avança automaticamente a data no fecho do dia anterior à folga, saltando o(s) dia(s) fechado(s).
- **Forma de Pagamento (Default por Local)** — Método de pagamento sugerido na janela de recebimento, configurável por local.
- **Fundo de Maneio** — Dinheiro disponível em caixa no início do dia para dar troco.

## G

- **Gaveta** — Dispositivo de dinheiro físico. Aberta por sinal eléctrico vindo da impressora ou directamente.
- **Gorjeta / Taxa de Serviço** — Artigo automático calculado por percentagem do consumo da mesa.
- **Grupo de Comissão** — Categoria de empregado ou artigo usada para cruzar e calcular comissões.
- **Grupo de Desconto** — Categoria de cliente ou artigo usada para cruzar e calcular descontos.

## H

- **Happy Hour** — Período (dia, hora, local) em que determinados artigos são vendidos com tipo de preço diferente ou em "2 pelo preço de 1".
- **Hardlock** — Dongle de licenciamento (USB ou série) do WinREST histórico. Em OpenRest, substituído por modelo de licenciamento open ou opcionalmente cloud-key.
- **HardServer** — Serviço de rede que valida hardlocks centralmente. Em OpenRest, equivalente conceptual ao serviço de licenciamento.

## I

- **Imprime conta acima de** — Configuração por local que força impressão de factura quando o consumo ultrapassa um limite.
- **Imprime Sub-Total** — Configuração por local de em que eventos (pedido, anulação, pagamento parcial, fecho) imprime sub-total.
- **Importação Automática** — Polling periódico que verifica ficheiros vindos do Store/Reports e os importa.
- **Imprime no Fecho de…** — Atalhos de configuração de relatórios para impressão automática em eventos.
- **Inclui desconto nos preços** — Configuração por local que decide se os descontos aparecem dentro do preço de cada linha ou como linha separada.
- **Inicialização de Display / Modem** — Sequência de bytes enviada uma vez ao dispositivo no arranque.
- **Item de Promoção** — Componente individual de uma promoção (cada "nível" da promoção).

## L

- **Lay Out** — Linguagem da UI por defeito.
- **Led ID** — Dispositivo identificador do empregado por proximidade.
- **Limite de Consumo** — Valor máximo que uma mesa pode atingir (típico em cartões pré-pagos de discoteca).
- **Limite de Crédito** — Valor máximo da CC negativa de um cliente.
- **Lista Negra** — Conjunto de mesas/cartões bloqueados (perdidos, suspeitos).
- **Local** — Conjunto lógico de mesas com configuração própria (tipo, PVP, IVA, comportamentos).
- **Local com Facturação Externa** — Local cujos documentos finais são emitidos por sistema externo (ex: PMS de hotel).
- **Log.ini** — Ficheiro de configuração de logging.

## M

- **Macro** — Mecanismo que associa um "nível" a artigos pedidos e permite invocá-los depois (`Pode sair`).
- **Manutenção** — Modo técnico do programa onde se fazem configurações de baixo nível. Acedido por senha.
- **Mapa de Mesas** — Imagem da planta da sala onde se identifica visualmente cada mesa.
- **Margem (no mapa económico)** — Vendas menos custos.
- **Master / Mestre (ficheiro)** — Tabela principal de uma entidade (artigos, clientes, empregados, famílias, armazéns).
- **Meia-Dose** — Atributo de um artigo que aponta para um "homólogo" mais pequeno (em pizzas, complementos por tamanho, etc.).
- **Mesa por Defeito** — Mesa pré-seleccionada ao entrar no ecrã de pedidos.
- **Mesa Excepcional** — Mesa com nome diferente do nome genérico do local.
- **Mesas Abertas no fim do dia** — Configuração que permite fechar o dia com mesas abertas (típico em hotéis com dormidas).
- **Método de Pagamento** — Numerário, Multibanco, Visa, Cheque, Vale, Conta Corrente, etc. O método 1 é tipicamente Numerário e o 9 Conta Corrente.
- **Mestre de Conhecimento (SmartChoice)** — BD interna que regista as últimas decisões do utilizador para sugerir defaults.
- **Modelo de Movimento** — Indica se um artigo se move por unidade ou peso.
- **Modo Demonstração** — Modo sem licença com limitações (poucos artigos, poucos empregados, sem numeração sequencial real).
- **Moeda Base** — Moeda interna em que todos os valores são armazenados.
- **Moeda do Operador** — Moeda em que o utilizador introduz e vê valores. Pode diferir da Base na transição cambial.

## N

- **Net** — Identificador numérico de uma rede de FrontOffice na mesma LAN, usado quando há várias instâncias.
- **NetPay** — Plug-in histórico para pagamentos electrónicos via BPN.
- **NIF / N.º Contribuinte** — Número fiscal do cliente / casa. Validado por dígito de controlo em PT.
- **Nível de Acesso** — 1 a 9; define que operações um empregado pode efectuar.
- **Nível de Macro** — Inteiro associado a cada pedido para o sistema de macros.
- **Nome Curto** — Designação reduzida usada em botões e listagens.
- **Nome Excepcional** — Nome dado individualmente a uma mesa, em vez do padrão `Mesa 7`.
- **Nome Genérico (de mesa)** — Padrão para o nome das mesas do local, com `\nm` substituído pelo número.

## O

- **Oferta** — Desconto pontual feito em alguns artigos de uma mesa.
- **Opção por Defeito** — Acção tomada ao premir uma área neutra do ecrã (ex: ir directamente para Pedidos).
- **Origem (de zona de impressão)** — Conjunto de postos/comandos que partilham a mesma matriz de zonas.

## P

- **Página Rápida** — Ecrã configurável com botões de artigos de várias famílias misturados, optimizado para artigos mais vendidos.
- **Pagamento Parcial** — Fechar parte do consumo de uma mesa sem fechar a mesa toda.
- **Parente Mononível** — Campo de cliente com referência a outro cliente (relacionamento livre, sem CC consolidada).
- **PDA** — Personal Digital Assistant. Suportado historicamente via cliente VNC.
- **Pedido** — Submissão de um conjunto de artigos para impressão em cozinha/bar e adição à mesa.
- **Pedido por Código** — Introdução de artigo via teclado pelo seu código de pedido, sem navegar famílias.
- **Pedido Secundário** — Pedido espelho impresso noutra zona, com indicação que sai *junto com* o original.
- **Permite Encaixe de Promoções** — Ver "Encaixe".
- **Peso Unitário** — Peso médio de um artigo vendido à unidade, mas verificável por balança (ex: contar drops).
- **Plug-in** — Módulo opcional que estende o FrontOffice (Primavera, MiliStore, Ticket, Videovigilância, etc.).
- **Pontos** — Sistema de fidelidade. Cada X euros gastos = 1 ponto. Pontos podem pagar parte da conta.
- **Posto** — Terminal físico. Pode ser Fundamental, Importante ou Secundário.
- **Prato do Dia** — Artigo "ementa do dia" com preço fixo. Mapeado para o artigo real consumido (Prato Carne, Prato Peixe).
- **Preço Variável (PVP Variável)** — Atributo que permite alterar o preço no momento do pedido.
- **Promoção (Menu)** — Conjunto de artigos vendidos como pacote a preço fechado. Cada nível tem várias escolhas.
- **Propriedades do Posto** — Configurações individuais de um posto.
- **PVP** — Preço de Venda ao Público. Há até 5 tipos: PVP1..PVP5.

## Q

- **Qualidade de Cliente** — Categoria descritiva de cliente (ex: VIP, Empresa, Particular).
- **Quantidade Máxima** — Limite para prevenir erros (introduzir código a pensar que é quantidade).

## R

- **Receita** — Termo equivalente a Venda na linguagem do utilizador final.
- **Recebe de / Produz para** — Configuração de comissões em grupo (distribuição entre empregados).
- **Recibo** — Documento de quitação de uma conta corrente.
- **Recuperar** — Operação inversa de Anular: traz de volta uma ficha anulada.
- **Redes Remotas** — Lojas remotas com quem se troca mensagens, reservas, pedidos delivery.
- **Reimpressão** — Emissão de 2ª via de documento (carimba "Duplicado").
- **Relógio de Ponto / Timeclock** — Módulo de marcação de entrada/saída de empregados.
- **Reserva** — Marcação prévia de mesas para um cliente em hora/dia.
- **Restauração Colectiva** — Modo de cantinas: pratos do dia + senhas + reservas com UID + torniquete.
- **Retirada** — Saída de dinheiro da caixa para cofre/depósito.

## S

- **SAF-T PT** — Ficheiro XML padrão da fiscalidade portuguesa (Standard Audit File for Tax).
- **Senha (de refeição)** — Documento que titula uma reserva de refeição, com código de barras UID validável em torniquete.
- **Sequência Especial (flag)** — Marcador no template de documento (`\nd`, `\vt`, `\dt`, ...) substituído por dados em tempo de impressão.
- **Sessão** — Período de actividade autenticada de um empregado.
- **Sistema** — Ecrã raiz do programa que dá acesso a Mesas, Pedidos, Ficheiros, Caixa, Plug-ins, Sistema técnico.
- **SmartChoice** — Mecanismo que regista as últimas opções escolhidas e as pré-selecciona em interacções futuras.
- **Sub-família** — Subdivisão de família (ex: Vinhos → Tintos, Brancos, Verdes).
- **Sub-total** — Mostra/imprime o estado actual da mesa sem fechar.

## T

- **Tabela de Comissões** — Matriz `Grupo Empregado × Grupo Artigo` de percentagens.
- **Tabela de Descontos** — Matriz `Grupo Cliente × Grupo Artigo` de percentagens.
- **Take-Away** — Local de venda directa. Mesa fecha no acto do pedido.
- **Take-Away Seguro** — Variante: total só é mostrado ao premir confirmação; ideal para evitar fraudes.
- **Tamanho (de artigo)** — Atributo (pequeno/médio/grande) usado para escolher complemento compatível.
- **Tara** — Peso a subtrair em artigos pesados em balança.
- **Taxa de Entrega** — Valor cobrado pelo serviço de Delivery, que pode variar dependendo da Área de Entrega.
- **Taxa de IVA** — Há 9 códigos (1..9). Cada artigo tem 2 atribuições: IVA na Mesa e IVA Venda Directa.
- **Tempo Extra (entre mensagens)** — Atraso aplicado a comunicação com antena rádio para tolerar USB→Serial.
- **Terminal Adapter (TA)** — Adaptador RDIS para identificação de chamadas (delivery).
- **Tipo de Movimento** — Por unidade ou por peso.
- **Tipo de Pedido** — Configuração de documento usada para pedidos numa zona.
- **Tipo de Preço** — PVP1..PVP5, com nomes configuráveis.
- **Tipos de Família** — Normal, Complemento, Informativa, Consumo.
- **Tipos de Artigo** — Normal, Complemento, Informativo, Consumo, Gorjeta/Taxa de Serviço.
- **Transferência (de mesa)** — Mover artigos de uma mesa para outra.
- **Transferência (de turno)** — Fechar turno actual e reabrir, mantendo sessões abertas.
- **Transferência de Vendas Activas** — Passar mesas abertas de um empregado para outro no fecho de sessão.
- **Troco Rápido** — Botões pré-configurados com valores típicos para acelerar cálculo de troco no Take-Away.
- **Turno** — Subdivisão da caixa. A caixa pode rodar por turno sem fechar.

## U

- **UID (Unique ID)** — Código encriptado impresso em senhas de refeição. Validável por leitor associado a torniquete. Garante autenticidade.
- **Unidade de Movimento** — Unidade de medida do artigo (Un, Kg, L, dose, etc.).
- **Utilizador** — Pessoa que opera o programa (sinónimo de Empregado em contextos de UI).

## V

- **Vale (entrada/saída)** — Movimento de caixa que dá ou recebe dinheiro de empregado.
- **Validação de Cliente** — Documento de confirmação assinada pelo cliente (CC, validação fiscal).
- **Venda a Dinheiro (VD)** — Documento fiscal de venda com NIF "Consumidor Final" (ou outro).
- **Venda Activa** — Mesa aberta com pedidos pendentes.
- **Vendas Negativas** — Mapa de operações que não contribuem para facturação (anulações, ofertas, estornos).
- **VNC** — Servidor virtual de écran para acesso remoto a um posto (modernizado em OpenRest para WebRTC/Browser).

## W

- **W4 (winrest world wide web)** — Browser embebido controlado para acesso a parceiros.
- **WServer** — Endereço do servidor onde corre o FrontOffice.
- **Wizard** — Assistente de configuração inicial.
- **WQL** — Linguagem de query interna (semelhante a SQL).
- **WRSTSC00.\*** — Ficheiros de configuração históricos. Em OpenRest, equivalente em formato moderno (YAML/JSON).

## Z

- **Zona (de morada)** — Subdivisão geográfica usada em fichas de cliente (típica para delivery).
- **Zona de Impressão** — Sítio lógico (Bar, Cozinha, Grelhador, D.Externos) para onde os artigos são distribuídos.
