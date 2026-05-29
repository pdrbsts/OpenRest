import {
  createEffect,
  createMemo,
  createResource,
  createSignal,
  For,
  Show,
} from "solid-js";
import "./App.css";
import {
  api,
  Article,
  CatalogResponse,
  Customer,
  DocumentDetail,
  DocumentResponse,
  Employee,
  Family,
  Local,
  PaymentLineInput,
  PaymentMethod,
  SessaoEmpregado,
  Table,
  TipoPreco,
  pvpFor,
} from "./api";
import { AtSeriesView } from "./AtSeriesView";
import { ClientesView } from "./ClientesView";
import { ConfigView } from "./ConfigView";
import { CustomerPicker, DespachoView } from "./DeliveryView";

type View =
  | "tables"
  | "order"
  | "config"
  | "despacho"
  | "clientes"
  | "at-series";

const fmtMoney = (cents: number) => (cents / 100).toFixed(2) + "€";

// Formata qty_milli (1000 = 1 unidade). Inteiros saem como "3"; fraccionários
// como "0.5", "0.25" etc. (sem zeros à direita); negativos preservam o sinal
// para indicar linhas de compensação (modo Encaixar).
const fmtQtyMilli = (qm: number): string => {
  const sign = qm < 0 ? "-" : "";
  const abs = Math.abs(qm);
  const units = Math.floor(abs / 1000);
  const frac = abs % 1000;
  if (frac === 0) return `${sign}${units}`;
  const s =
    frac % 100 === 0
      ? `${frac / 100}`
      : frac % 10 === 0
        ? `${(frac / 10).toString().padStart(2, "0")}`
        : `${frac.toString().padStart(3, "0")}`;
  return `${sign}${units}.${s}`;
};

// Rótulo da linha: usa descricao se preenchida (linhas geradas pelo Encaixar),
// senão usa o nome do artigo. Centraliza a regra para reutilizar em todas as
// renderizações de linhas (consulta, pedido, modais).
const lineLabel = (
  line: { article_id: string; descricao: string | null },
  articleById: (id: string) => Article | undefined
): string =>
  line.descricao && line.descricao.trim().length > 0
    ? line.descricao
    : articleById(line.article_id)?.name ?? "Artigo";

function familySubtree(families: Family[], rootId: string): Set<string> {
  const childrenOf = new Map<string | null, Family[]>();
  for (const f of families) {
    const key = f.parent_id ?? null;
    const arr = childrenOf.get(key) ?? [];
    arr.push(f);
    childrenOf.set(key, arr);
  }
  const out = new Set<string>([rootId]);
  const stack = [rootId];
  while (stack.length > 0) {
    const cur = stack.pop()!;
    for (const child of childrenOf.get(cur) ?? []) {
      if (!out.has(child.id)) {
        out.add(child.id);
        stack.push(child.id);
      }
    }
  }
  return out;
}

function App() {
  const [catalog] = createResource<CatalogResponse>(() => api.catalog());
  const [currentDay] = createResource(() => api.currentDay());
  const [locais, { refetch: refetchLocais }] = createResource<Local[]>(() =>
    api.locais()
  );
  const [selectedLocal, setSelectedLocal] = createSignal<string | null>(null);

  // pick first local by default once loaded
  createEffect(() => {
    if (selectedLocal() === null) {
      const list = locais();
      if (list && list.length > 0) setSelectedLocal(list[0].id);
    }
  });

  const [tables, { refetch: refetchTables }] = createResource<Table[]>(() => api.tables());
  const visibleTables = createMemo(() => {
    const all = tables() ?? [];
    const local = selectedLocal();
    return local ? all.filter((t) => t.local_id === local) : all;
  });
  const [paymentMethods] = createResource<PaymentMethod[]>(() => api.paymentMethods());
  const [employees] = createResource<Employee[]>(() => api.employees());
  const [tiposPreco] = createResource<TipoPreco[]>(() => api.tiposPreco());

  const currentTipoPrecoCodigo = (): number | null => {
    const local = locais()?.find((l) => l.id === selectedLocal());
    if (!local || !local.tipo_preco_id) return null;
    const tp = tiposPreco()?.find((t) => t.id === local.tipo_preco_id);
    return tp?.codigo ?? null;
  };

  const [view, setView] = createSignal<View>("tables");
  const [showCustomerPicker, setShowCustomerPicker] = createSignal(false);

  // Sessão de empregado (spec §4): nenhuma operação de sala sem sessão aberta.
  const [currentEmployee, setCurrentEmployee] = createSignal<Employee | null>(
    null
  );
  const [currentSessao, setCurrentSessao] = createSignal<SessaoEmpregado | null>(
    null
  );

  // Logout: volta ao portão sem fechar a sessão (spec §4.2 — a sessão fica
  // aberta no servidor e é retomada no próximo login).
  const logout = () => {
    setActiveTable(null);
    setDoc(null);
    setReceipt(null);
    setError(null);
    setView("tables");
    setCurrentEmployee(null);
    setCurrentSessao(null);
  };

  // Fecha a sessão no servidor (valida que não há mesas/documentos por fechar)
  // e regressa ao portão.
  const fecharSessao = async () => {
    const s = currentSessao();
    if (!s) return;
    setError(null);
    setBusy(true);
    try {
      await api.closeSessao(s.id, { fechada_por: currentEmployee()?.id ?? null });
      logout();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const currentLocal = createMemo(() =>
    (locais() ?? []).find((l) => l.id === selectedLocal())
  );
  const [activeTable, setActiveTable] = createSignal<Table | null>(null);
  const [doc, setDoc] = createSignal<DocumentResponse | null>(null);
  const [familyPath, setFamilyPath] = createSignal<string[]>([]);
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [receipt, setReceipt] = createSignal<string | null>(null);

  const families = (): Family[] => catalog()?.families ?? [];
  const articles = (): Article[] => catalog()?.articles ?? [];

  const currentParent = (): string | null => {
    const path = familyPath();
    return path.length > 0 ? path[path.length - 1] : null;
  };

  const visibleFamilies = createMemo(() =>
    families().filter((f) => (f.parent_id ?? null) === currentParent())
  );

  const visibleArticles = createMemo(() => {
    const parent = currentParent();
    if (parent === null) return articles().filter((a) => a.family_id === null);
    // articles attached directly to any descendant of current family
    const subtree = familySubtree(families(), parent);
    return articles().filter((a) => a.family_id !== null && subtree.has(a.family_id));
  });

  const articleById = (id: string) => articles().find((a) => a.id === id);

  const startDelivery = async (customer: Customer | null, observacoes: string) => {
    const local = currentLocal();
    if (!local) return;
    setError(null);
    setReceipt(null);
    setBusy(true);
    try {
      const d = await api.startLocalDocument(local.id, {
        customer_id: customer?.id ?? null,
        observacoes_pedido: observacoes || null,
      });
      setActiveTable({
        id: d.document.id,
        local_id: local.id,
        code: 0,
        name: customer ? `Delivery — ${customer.nome}` : "Delivery",
      } as Table);
      setDoc(d);
      setShowCustomerPicker(false);
      setView("order");
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const startConsumo = async (employee: Employee) => {
    const local = currentLocal();
    if (!local) return;
    setError(null);
    setReceipt(null);
    setBusy(true);
    try {
      const d = await api.openConsumo(local.id, employee.id);
      setActiveTable({
        id: d.document.table_id!,
        local_id: local.id,
        code: 9000 + employee.code,
        name: `Consumo — ${employee.name}`,
      } as Table);
      setDoc(d);
      setView("order");
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const openTable = async (t: Table) => {
    setError(null);
    setReceipt(null);
    setBusy(true);
    try {
      const d = await api.openTable(t.id, currentEmployee()?.id ?? null);
      setActiveTable(t);
      setDoc(d);
      setView("order");
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const backToTables = async () => {
    setActiveTable(null);
    setDoc(null);
    setReceipt(null);
    setView("tables");
    await refetchTables();
  };

  const addToOrder = async (article: Article) => {
    const d = doc();
    if (!d) return;
    setError(null);
    setBusy(true);
    try {
      const updated = await api.addLine(d.document.id, article.id, 1);
      setDoc(updated);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const [anularLineId, setAnularLineId] = createSignal<string | null>(null);

  const cancelLine = async (lineId: string) => {
    const d = doc();
    if (!d) return;
    setError(null);
    setBusy(true);
    try {
      const updated = await api.cancelLine(d.document.id, lineId, {
        employee_id: d.document.employee_id,
      });
      setDoc(updated);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const anularLine = async (
    lineId: string,
    com_desperdicio: boolean,
    motivo: string
  ) => {
    const d = doc();
    if (!d) return;
    setError(null);
    setBusy(true);
    try {
      const updated = await api.anularLine(d.document.id, lineId, {
        com_desperdicio,
        motivo: motivo || null,
        employee_id: d.document.employee_id,
      });
      setDoc(updated);
      setAnularLineId(null);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const [showTransferModal, setShowTransferModal] = createSignal(false);
  const [transferSelected, setTransferSelected] = createSignal<Set<string>>(
    new Set<string>()
  );

  const transferTo = async (targetTableId: string) => {
    const d = doc();
    if (!d) return;
    const ids = Array.from(transferSelected());
    setError(null);
    setBusy(true);
    try {
      const resp = await api.transferDocument(d.document.id, {
        target_table_id: targetTableId,
        line_ids: ids.length > 0 ? ids : null,
        employee_id: d.document.employee_id,
      });
      setDoc(resp.from_document);
      setShowTransferModal(false);
      setTransferSelected(new Set<string>());
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const pedir = async () => {
    const d = doc();
    if (!d) return;
    setError(null);
    setBusy(true);
    try {
      const updated = await api.pedirDocument(d.document.id);
      setDoc(updated);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const closeAndPrint = async (paymentMethodId: string | null) => {
    const d = doc();
    if (!d || d.lines.length === 0) return;
    setError(null);
    setBusy(true);
    try {
      const closed = await api.closeDocument(d.document.id, paymentMethodId);
      const printed = await api.printDocument(d.document.id);
      setDoc(closed);
      setReceipt(printed);
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  // Fecho com N rodapés de pagamento (janela Avançada). O servidor valida
  // soma >= total e calcula troco quando soma > total.
  const closeMultiAndPrint = async (payments: PaymentLineInput[]) => {
    const d = doc();
    if (!d || d.lines.length === 0 || payments.length === 0) return;
    setError(null);
    setBusy(true);
    try {
      const closed = await api.closeDocumentMulti(d.document.id, payments);
      const printed = await api.printDocument(d.document.id);
      setDoc(closed);
      setReceipt(printed);
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  // Pagamento parcial: move as linhas seleccionadas para um filho e fecha
  // fiscalmente esse filho. Pai mantém-se aberto com o resto das linhas; o
  // recibo impresso é o do filho (apenas linhas pagas).
  const partialCloseAndPrint = async (
    lineIds: string[],
    payments: PaymentLineInput[]
  ) => {
    const d = doc();
    if (!d || lineIds.length === 0 || payments.length === 0) return;
    setError(null);
    setBusy(true);
    try {
      const child = await api.partialCloseDocument(d.document.id, {
        line_ids: lineIds,
        payments,
      });
      const printed = await api.printDocument(child.document.id);
      // Recarrega o pai (linhas movidas / total actualizado).
      const parent = await api.document(d.document.id);
      setDoc(parent);
      setReceipt(printed);
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  // Divisão de conta: cria N filhos. Três modos:
  //   - "lines": linhas inteiras vão a uma só conta (totais podem diferir)
  //   - "quantidades": cada linha dividida fraccionariamente em N
  //   - "encaixar": linhas atribuídas + sistema gera compensações para
  //     igualar totais
  // Pai fica fechado (sem fiscal) quando ficar vazio. Filhos serão fechados
  // individualmente.
  type SplitPayload =
    | { mode: "lines"; assignments: Array<{ line_ids: string[] }> }
    | { mode: "quantidades"; num_accounts: number }
    | { mode: "encaixar"; assignments: Array<{ line_ids: string[] }> };
  const splitDocument = async (payload: SplitPayload) => {
    const d = doc();
    if (!d) return;
    setError(null);
    setBusy(true);
    try {
      if (payload.mode === "lines") {
        await api.splitDocumentLines(d.document.id, payload.assignments);
      } else if (payload.mode === "quantidades") {
        await api.splitDocumentQuantidades(d.document.id, payload.num_accounts);
      } else {
        await api.splitDocumentEncaixar(d.document.id, payload.assignments);
      }
      // O pai pode ter ficado split-closed; reavalia o estado.
      const parent = await api.document(d.document.id);
      setDoc(parent);
      await refetchTables();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <Show
      when={currentSessao()}
      fallback={
        <SessionGate
          employees={employees() ?? []}
          loading={employees.loading}
          onReady={(emp, sessao) => {
            setCurrentEmployee(emp);
            setCurrentSessao(sessao);
            setView("tables");
          }}
        />
      }
    >
    <div class="flex h-full w-full bg-zinc-900 text-white select-none">
      <Sidebar
        view={view()}
        onTables={() => setView("tables")}
        canOrder={!!activeTable()}
        onOrder={() => setView("order")}
        onConfig={() => setView("config")}
        onDespacho={() => setView("despacho")}
        onClientes={() => setView("clientes")}
        onAtSeries={() => setView("at-series")}
        showDespacho={(locais() ?? []).some((l) => l.tipo === "delivery")}
      />

      <div class="flex-1 flex flex-col h-full bg-zinc-950">
        <TopBar
          terminal="Terminal 1"
          activeTable={activeTable()}
          dataDia={currentDay()?.data_dia ?? null}
          onBack={view() === "order" ? backToTables : undefined}
          employeeName={currentEmployee()?.name ?? null}
          busy={busy()}
          onLogout={logout}
          onCloseSession={fecharSessao}
        />

        <Show when={error()}>
          <div class="bg-red-900/60 border-b border-red-700 px-4 py-2 text-sm text-red-100">
            {error()}
          </div>
        </Show>

        <div class="flex-1 flex overflow-hidden">
          <Show when={view() === "config"}>
            <ConfigView
              locais={locais() ?? []}
              tablesByLocal={Object.fromEntries(
                (locais() ?? []).map((l) => [
                  l.id,
                  (tables() ?? []).filter((t) => t.local_id === l.id),
                ])
              )}
              onChanged={async () => {
                await refetchLocais();
                await refetchTables();
              }}
            />
          </Show>

          <Show when={view() === "tables"}>
            <TablesView
              locais={locais() ?? []}
              selectedLocal={selectedLocal()}
              currentLocal={currentLocal()}
              onPickLocal={(id) => setSelectedLocal(id)}
              tables={visibleTables()}
              loading={tables.loading || locais.loading}
              busy={busy()}
              employees={employees() ?? []}
              onPick={openTable}
              onStartDelivery={() => setShowCustomerPicker(true)}
              onStartConsumo={startConsumo}
            />
          </Show>

          <Show when={view() === "despacho"}>
            <DespachoView onClose={() => setView("tables")} />
          </Show>

          <Show when={view() === "clientes"}>
            <ClientesView />
          </Show>

          <Show when={view() === "at-series"}>
            <AtSeriesView />
          </Show>

          <Show when={view() === "order" && activeTable()}>
            <OrderColumn
              doc={doc()}
              receipt={receipt()}
              busy={busy()}
              articleById={articleById}
              paymentMethods={paymentMethods() ?? []}
              localKind={currentLocal()?.tipo ?? "normal"}
              onPedir={pedir}
              onCancelLine={cancelLine}
              onAnularLine={(id) => setAnularLineId(id)}
              onTransfer={() => {
                setTransferSelected(
                  new Set<string>(
                    (doc()?.lines ?? [])
                      .filter((l) => !l.anulada)
                      .map((l) => l.id)
                  )
                );
                setShowTransferModal(true);
              }}
              onClose={closeAndPrint}
              onCloseMulti={closeMultiAndPrint}
              onPartialClose={partialCloseAndPrint}
              onSplit={splitDocument}
            />
            <CatalogPane
              families={families()}
              visibleFamilies={visibleFamilies()}
              path={familyPath()}
              onEnter={(id) => setFamilyPath((p) => [...p, id])}
              onBack={() => setFamilyPath((p) => p.slice(0, -1))}
              onRoot={() => setFamilyPath([])}
              articles={visibleArticles()}
              tipoPrecoCodigo={currentTipoPrecoCodigo()}
              loading={catalog.loading}
              disabled={!doc() || doc()?.document.is_closed || busy()}
              onPick={addToOrder}
            />
          </Show>
        </div>
      </div>

      <Show when={showCustomerPicker()}>
        <CustomerPicker
          onCancel={() => setShowCustomerPicker(false)}
          onConfirm={(c, obs) => startDelivery(c, obs)}
        />
      </Show>

      <Show when={anularLineId()}>
        <AnularDialog
          articleName={
            articleById(
              doc()?.lines.find((l) => l.id === anularLineId())?.article_id ?? ""
            )?.name ?? "Artigo"
          }
          qtyMilli={
            doc()?.lines.find((l) => l.id === anularLineId())?.qty_milli ?? 0
          }
          onCancel={() => setAnularLineId(null)}
          onConfirm={(com_desperdicio, motivo) =>
            anularLine(anularLineId()!, com_desperdicio, motivo)
          }
        />
      </Show>

      <Show when={showTransferModal() && doc()}>
        <TransferDialog
          lines={(doc()?.lines ?? []).filter((l) => !l.anulada)}
          articleById={articleById}
          tables={tables() ?? []}
          locais={locais() ?? []}
          currentTableId={doc()?.document.table_id ?? null}
          currentLocalKind={currentLocal()?.tipo ?? "normal"}
          selected={transferSelected()}
          onToggleLine={(id) => {
            const s = new Set<string>(transferSelected());
            if (s.has(id)) s.delete(id);
            else s.add(id);
            setTransferSelected(s);
          }}
          onCancel={() => {
            setShowTransferModal(false);
            setTransferSelected(new Set<string>());
          }}
          onConfirm={transferTo}
          busy={busy()}
        />
      </Show>
    </div>
    </Show>
  );
}

function AnularDialog(props: {
  articleName: string;
  qtyMilli: number;
  onCancel: () => void;
  onConfirm: (comDesperdicio: boolean, motivo: string) => void;
}) {
  const [comDesp, setComDesp] = createSignal(false);
  const [motivo, setMotivo] = createSignal("");
  return (
    <div class="fixed inset-0 bg-black/60 z-30 flex items-center justify-center p-6">
      <div class="bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl w-full max-w-md p-6 text-zinc-100">
        <h2 class="text-xl font-bold mb-1">Anular linha</h2>
        <p class="text-sm text-zinc-400 mb-4">
          {fmtQtyMilli(props.qtyMilli)}× {props.articleName}
        </p>
        <label class="flex items-center gap-2 mb-3 text-sm font-medium">
          <input
            type="checkbox"
            class="w-4 h-4"
            checked={comDesp()}
            onChange={(e) => setComDesp(e.currentTarget.checked)}
          />
          Com desperdício (artigo gasto, não volta ao stock)
        </label>
        <label class="block text-sm font-medium mb-1">Motivo (opcional)</label>
        <textarea
          rows={2}
          class="w-full bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 resize-none mb-4"
          placeholder="ex: queimada, cliente mudou de ideias…"
          value={motivo()}
          onInput={(e) => setMotivo(e.currentTarget.value)}
        />
        <div class="flex justify-end gap-2">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600"
          >
            Cancelar
          </button>
          <button
            onClick={() => props.onConfirm(comDesp(), motivo())}
            class="px-5 py-2 rounded-md bg-red-700 hover:bg-red-600 text-white font-bold"
          >
            Anular
          </button>
        </div>
      </div>
    </div>
  );
}

function TransferDialog(props: {
  lines: DocumentDetail[];
  articleById: (id: string) => Article | undefined;
  tables: Table[];
  locais: Local[];
  currentTableId: string | null;
  currentLocalKind: string;
  selected: Set<string>;
  onToggleLine: (id: string) => void;
  onCancel: () => void;
  onConfirm: (targetTableId: string) => void;
  busy: boolean;
}) {
  // Locais válidos como destino: mesma tipologia operacional, exclui consumo_proprio/delivery.
  const validKinds = ["normal", "pub", "take_away", "take_away_seguro"];
  const localById = (id: string | null) =>
    id ? props.locais.find((l) => l.id === id) : undefined;
  const targets = () =>
    props.tables.filter((t) => {
      if (t.id === props.currentTableId) return false;
      const loc = localById(t.local_id);
      if (!loc) return false;
      return validKinds.includes(loc.tipo);
    });
  const [target, setTarget] = createSignal<string | null>(null);

  return (
    <div class="fixed inset-0 bg-black/60 z-30 flex items-center justify-center p-6">
      <div class="bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl w-full max-w-3xl p-6 text-zinc-100 flex flex-col max-h-[85vh]">
        <h2 class="text-xl font-bold mb-1">Transferir linhas</h2>
        <p class="text-sm text-zinc-400 mb-4">
          Escolhe as linhas (todas seleccionadas por defeito) e a mesa destino.
        </p>

        <div class="grid grid-cols-2 gap-4 flex-1 min-h-0">
          <div class="flex flex-col min-h-0">
            <h3 class="text-sm font-semibold text-zinc-300 mb-2">
              Linhas ({props.selected.size} de {props.lines.length})
            </h3>
            <div class="overflow-y-auto flex-1 bg-zinc-950/50 rounded-md border border-zinc-800 p-2 space-y-1">
              <For each={props.lines}>
                {(line) => {
                  const checked = () => props.selected.has(line.id);
                  return (
                    <label
                      class={`flex items-center gap-2 px-2 py-1 rounded cursor-pointer text-sm ${
                        checked() ? "bg-indigo-900/40" : "hover:bg-zinc-800"
                      }`}
                    >
                      <input
                        type="checkbox"
                        class="w-4 h-4"
                        checked={checked()}
                        onChange={() => props.onToggleLine(line.id)}
                      />
                      <span class="font-bold text-blue-400 w-8">
                        {fmtQtyMilli(line.qty_milli)}x
                      </span>
                      <span class="truncate flex-1">
                        {props.articleById(line.article_id)?.name ?? "Artigo"}
                      </span>
                      <span class="text-zinc-400 font-mono text-xs">
                        {fmtMoney(line.total)}
                      </span>
                    </label>
                  );
                }}
              </For>
            </div>
          </div>

          <div class="flex flex-col min-h-0">
            <h3 class="text-sm font-semibold text-zinc-300 mb-2">
              Mesa destino
            </h3>
            <div class="overflow-y-auto flex-1 bg-zinc-950/50 rounded-md border border-zinc-800 p-2 grid grid-cols-3 gap-2 content-start">
              <For each={targets()}>
                {(t) => {
                  const loc = localById(t.local_id);
                  const label = t.name ?? `${loc?.nome_generico_mesa ?? "Mesa"} ${t.code}`;
                  return (
                    <button
                      onClick={() => setTarget(t.id)}
                      class={`px-2 py-3 rounded-md text-sm font-semibold border ${
                        target() === t.id
                          ? "bg-indigo-600 border-indigo-400 text-white"
                          : "bg-zinc-800 border-zinc-700 hover:bg-zinc-700 text-zinc-200"
                      }`}
                    >
                      <div class="text-xs text-zinc-400 truncate">
                        {loc?.designacao}
                      </div>
                      <div>{label}</div>
                    </button>
                  );
                }}
              </For>
            </div>
          </div>
        </div>

        <div class="flex justify-end gap-2 mt-4">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600"
          >
            Cancelar
          </button>
          <button
            onClick={() => target() && props.onConfirm(target()!)}
            disabled={!target() || props.selected.size === 0 || props.busy}
            class="px-5 py-2 rounded-md bg-indigo-600 hover:bg-indigo-500 text-white font-bold disabled:opacity-40"
          >
            Transferir
          </button>
        </div>
      </div>
    </div>
  );
}

function Sidebar(props: {
  view: View;
  canOrder: boolean;
  onTables: () => void;
  onOrder: () => void;
  onConfig: () => void;
  onDespacho: () => void;
  onClientes: () => void;
  onAtSeries: () => void;
  showDespacho: boolean;
}) {
  const active = "bg-blue-600 hover:bg-blue-500 text-white";
  const inactive = "bg-zinc-700 hover:bg-zinc-600 text-zinc-300";
  return (
    <div class="w-24 bg-zinc-800 flex flex-col items-center py-4 gap-4 border-r border-zinc-700 shadow-xl z-10">
      <button
        onClick={props.onTables}
        class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-sm ${
          props.view === "tables" ? active : inactive
        }`}
      >
        Mesas
      </button>
      <button
        onClick={props.onOrder}
        disabled={!props.canOrder}
        class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-sm ${
          props.view === "order" ? active : inactive
        } disabled:opacity-40 disabled:cursor-not-allowed`}
      >
        Pedido
      </button>
      <Show when={props.showDespacho}>
        <button
          onClick={props.onDespacho}
          class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-sm ${
            props.view === "despacho" ? active : inactive
          }`}
        >
          Despacho
        </button>
      </Show>
      <div class="flex-1" />
      <button
        onClick={props.onClientes}
        class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-sm ${
          props.view === "clientes" ? active : inactive
        }`}
      >
        Clientes
      </button>
      <button
        onClick={props.onAtSeries}
        class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-xs leading-tight ${
          props.view === "at-series" ? active : inactive
        }`}
      >
        Séries AT
      </button>
      <button
        onClick={props.onConfig}
        class={`w-16 h-16 rounded-xl transition-colors shadow-lg flex items-center justify-center font-semibold text-sm ${
          props.view === "config" ? active : inactive
        }`}
      >
        Config
      </button>
    </div>
  );
}

function TopBar(props: {
  terminal: string;
  activeTable: Table | null;
  dataDia: string | null;
  onBack?: () => void;
  employeeName?: string | null;
  busy?: boolean;
  onLogout?: () => void;
  onCloseSession?: () => void;
}) {
  const civilDate = () => new Date().toLocaleDateString();
  const dataDiaShifted = () =>
    props.dataDia && props.dataDia !== new Date().toISOString().slice(0, 10);
  return (
    <div class="h-12 bg-zinc-800 border-b border-zinc-700 flex items-center justify-between px-4">
      <div class="flex items-center gap-3">
        <Show when={props.onBack}>
          <button
            onClick={props.onBack}
            class="px-3 py-1 rounded-md bg-zinc-700 hover:bg-zinc-600 text-sm font-medium"
          >
            ← Mesas
          </button>
        </Show>
        <span class="font-medium text-zinc-400">
          {props.activeTable
            ? props.activeTable.name ?? `Mesa ${props.activeTable.code}`
            : "Sem mesa"}
          {" — "}
          {props.terminal}
        </span>
      </div>
      <div class="flex items-center gap-3 font-mono text-sm">
        <Show when={props.dataDia}>
          <span
            class={`px-2 py-0.5 rounded ${
              dataDiaShifted()
                ? "bg-amber-900/40 text-amber-200 border border-amber-700/50"
                : "text-zinc-400"
            }`}
            title="Dia operacional (data lógica de caixa)"
          >
            Dia {props.dataDia}
          </span>
        </Show>
        <span class="text-zinc-500">{civilDate()}</span>
        <Show when={props.employeeName}>
          <span
            class="px-2 py-0.5 rounded bg-blue-900/40 text-blue-200 border border-blue-700/50"
            title="Sessão de empregado aberta"
          >
            {props.employeeName}
          </span>
          <button
            onClick={props.onLogout}
            disabled={props.busy}
            class="px-2 py-0.5 rounded-md bg-zinc-700 hover:bg-zinc-600 text-xs font-sans disabled:opacity-50"
            title="Trocar de empregado (mantém a sessão aberta)"
          >
            Trocar
          </button>
          <button
            onClick={props.onCloseSession}
            disabled={props.busy}
            class="px-2 py-0.5 rounded-md bg-red-800 hover:bg-red-700 text-xs font-sans disabled:opacity-50"
            title="Fechar sessão"
          >
            Fechar sessão
          </button>
        </Show>
      </div>
    </div>
  );
}

// Portão de sessão (spec §4): ecrã inicial obrigatório. Sem sessão aberta o
// empregado não acede à zona de operação. Identifica-se escolhendo o
// empregado; retoma sessão já aberta ou abre uma nova (com bolsa opcional).
function SessionGate(props: {
  employees: Employee[];
  loading: boolean;
  onReady: (employee: Employee, sessao: SessaoEmpregado) => void;
}) {
  const [picked, setPicked] = createSignal<Employee | null>(null);
  const [comBolsa, setComBolsa] = createSignal(false);
  const [fundo, setFundo] = createSignal("0.00");
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);

  const pick = async (emp: Employee) => {
    setError(null);
    setBusy(true);
    try {
      // Retoma sessão já aberta (ex.: após recarregar a aplicação).
      const existing = await api.openSessaoForEmployee(emp.id);
      if (existing) {
        props.onReady(emp, existing);
        return;
      }
      setPicked(emp);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const abrir = async () => {
    const emp = picked();
    if (!emp) return;
    setError(null);
    setBusy(true);
    try {
      const sessao = await api.openSessao({
        empregado_id: emp.id,
        com_bolsa: comBolsa(),
        fundo_bolsa: comBolsa()
          ? Math.round(parseFloat(fundo() || "0") * 100)
          : 0,
      });
      props.onReady(emp, sessao);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div class="flex h-full w-full items-center justify-center bg-zinc-900 text-white select-none">
      <div class="w-full max-w-2xl p-8">
        <h1 class="text-3xl font-bold text-center mb-1">OpenRest</h1>
        <p class="text-center text-zinc-400 mb-6">
          {picked()
            ? `Abrir sessão — ${picked()!.name}`
            : "Escolhe o empregado para abrir sessão"}
        </p>

        <Show when={error()}>
          <div class="mb-4 px-3 py-2 bg-red-900/40 border border-red-700 text-red-200 text-sm rounded-md">
            {error()}
          </div>
        </Show>

        <Show
          when={picked()}
          fallback={
            <Show
              when={!props.loading}
              fallback={<div class="text-zinc-400 text-center">A carregar…</div>}
            >
              <Show
                when={props.employees.length > 0}
                fallback={
                  <div class="text-zinc-500 text-center italic">
                    Sem empregados configurados.
                  </div>
                }
              >
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                  <For each={props.employees}>
                    {(emp) => (
                      <button
                        onClick={() => pick(emp)}
                        disabled={busy()}
                        class="aspect-square rounded-2xl bg-zinc-800 border border-zinc-700 hover:border-blue-500 hover:bg-zinc-700 p-4 flex flex-col justify-between items-start text-left shadow-md transition-all active:scale-95 disabled:opacity-50"
                      >
                        <span class="text-lg font-bold leading-tight">
                          {emp.name}
                        </span>
                        <span class="text-xs font-mono text-zinc-400">
                          #{emp.code}
                        </span>
                      </button>
                    )}
                  </For>
                </div>
              </Show>
            </Show>
          }
        >
          <div class="bg-zinc-800 border border-zinc-700 rounded-xl p-6 space-y-4">
            <label class="flex items-center gap-2 text-sm font-medium">
              <input
                type="checkbox"
                class="w-4 h-4"
                checked={comBolsa()}
                onChange={(e) => setComBolsa(e.currentTarget.checked)}
              />
              Abrir com bolsa (fundo de caixa)
            </label>
            <Show when={comBolsa()}>
              <label class="flex flex-col gap-1 text-sm font-medium">
                Fundo inicial (€)
                <input
                  type="number"
                  inputmode="decimal"
                  step="0.01"
                  min="0"
                  value={fundo()}
                  onInput={(e) => setFundo(e.currentTarget.value)}
                  class="bg-zinc-900 rounded-md px-3 py-2 border border-zinc-700 font-mono w-40"
                />
              </label>
            </Show>
            <div class="flex justify-end gap-2 pt-2">
              <button
                onClick={() => setPicked(null)}
                disabled={busy()}
                class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600 text-sm font-semibold disabled:opacity-50"
              >
                Voltar
              </button>
              <button
                onClick={abrir}
                disabled={busy()}
                class="px-5 py-2 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white font-bold disabled:opacity-50"
              >
                Abrir sessão
              </button>
            </div>
          </div>
        </Show>
      </div>
    </div>
  );
}

function TablesView(props: {
  locais: Local[];
  selectedLocal: string | null;
  currentLocal: Local | undefined;
  onPickLocal: (id: string) => void;
  tables: Table[];
  loading: boolean;
  busy: boolean;
  employees: Employee[];
  onPick: (t: Table) => void;
  onStartDelivery: () => void;
  onStartConsumo: (employee: Employee) => void;
}) {
  const stateColors: Record<string, string> = {
    livre: "bg-zinc-800 border-zinc-700 hover:border-blue-500 hover:bg-zinc-700",
    aberta: "bg-amber-600/30 border-amber-500 hover:bg-amber-600/50",
    em_espera: "bg-purple-700/30 border-purple-500 hover:bg-purple-700/50",
    reservada: "bg-sky-700/30 border-sky-500 hover:bg-sky-700/50",
    bloqueada: "bg-red-700/30 border-red-500 hover:bg-red-700/50",
  };
  const stateLabel: Record<string, string> = {
    livre: "Livre",
    aberta: "Aberta",
    em_espera: "Em espera",
    reservada: "Reservada",
    bloqueada: "Bloqueada",
  };
  return (
    <div class="flex-1 flex flex-col overflow-hidden bg-zinc-950">
      <div class="flex gap-2 p-3 border-b border-zinc-800 overflow-x-auto bg-zinc-900">
        <For each={props.locais}>
          {(l) => (
            <button
              onClick={() => props.onPickLocal(l.id)}
              class={`px-4 py-2 rounded-lg text-sm font-medium whitespace-nowrap transition-colors ${
                props.selectedLocal === l.id
                  ? "bg-blue-600 text-white shadow"
                  : "bg-zinc-800 text-zinc-300 hover:bg-zinc-700"
              }`}
            >
              <span>{l.designacao}</span>
              <span class="ml-2 text-xs uppercase tracking-wider text-zinc-400">
                {l.tipo.replace("_", " ")}
              </span>
            </button>
          )}
        </For>
      </div>
      <div class="flex-1 p-6 overflow-y-auto">
        <Show when={!props.loading} fallback={<div class="text-zinc-400">A carregar…</div>}>
          <Show
            when={props.currentLocal?.tipo === "delivery"}
            fallback={
              <Show when={props.currentLocal?.tipo === "consumo_proprio"} fallback={
                <>
                  <h2 class="text-xl font-bold text-zinc-200 mb-4">Escolher mesa</h2>
                  <div class="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 gap-4">
                    <For each={props.tables}>
                      {(t) => {
                        const estado = t.estado.estado;
                        return (
                          <button
                            onClick={() => props.onPick(t)}
                            disabled={props.busy || estado === "bloqueada"}
                            class={`aspect-square rounded-2xl border p-4 flex flex-col justify-between items-start text-left shadow-md transition-all active:scale-95 disabled:opacity-50 disabled:pointer-events-none ${
                              stateColors[estado] ?? stateColors.livre
                            }`}
                          >
                            <span class="text-2xl font-bold text-zinc-100 leading-tight">
                              {t.name ?? `Mesa ${t.code}`}
                            </span>
                            <div class="w-full flex justify-between items-baseline">
                              <span class="text-xs font-mono uppercase tracking-wider text-zinc-300">
                                {stateLabel[estado] ?? estado}
                              </span>
                              <Show when={t.estado.subtotal_actual > 0}>
                                <span class="text-xs font-mono text-zinc-200">
                                  {fmtMoney(t.estado.subtotal_actual)}
                                </span>
                              </Show>
                            </div>
                          </button>
                        );
                      }}
                    </For>
                  </div>
                </>
              }>
                <h2 class="text-xl font-bold text-zinc-200 mb-4">Consumo Próprio — escolher empregado</h2>
                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-4">
                  <For each={props.employees}>
                    {(emp) => (
                      <button
                        onClick={() => props.onStartConsumo(emp)}
                        disabled={props.busy}
                        class="aspect-square rounded-2xl bg-purple-900/30 border border-purple-600 hover:bg-purple-800/50 p-4 flex flex-col justify-between items-start text-left shadow-md disabled:opacity-50"
                      >
                        <span class="text-xl font-bold">{emp.name}</span>
                        <span class="text-xs font-mono text-purple-300">
                          paga {emp.perc_consumo / 100}% PVP
                        </span>
                      </button>
                    )}
                  </For>
                </div>
              </Show>
            }
          >
            <div class="space-y-4">
              <h2 class="text-xl font-bold text-zinc-200">Delivery</h2>
              <button
                onClick={props.onStartDelivery}
                disabled={props.busy}
                class="px-6 py-4 rounded-xl bg-emerald-600 hover:bg-emerald-500 text-white text-lg font-bold shadow-lg disabled:opacity-50"
              >
                + Novo pedido (identificar cliente)
              </button>
              <p class="text-sm text-zinc-400">
                Usa o botão Despacho na sidebar para gerir os pedidos pendentes.
              </p>
            </div>
          </Show>
        </Show>
      </div>
    </div>
  );
}

function OrderColumn(props: {
  doc: DocumentResponse | null;
  receipt: string | null;
  busy: boolean;
  articleById: (id: string) => Article | undefined;
  paymentMethods: PaymentMethod[];
  localKind: string;
  onPedir: () => void;
  onCancelLine: (lineId: string) => void;
  onAnularLine: (lineId: string) => void;
  onTransfer: () => void;
  onClose: (paymentMethodId: string | null) => void;
  onCloseMulti: (payments: PaymentLineInput[]) => void;
  onPartialClose: (lineIds: string[], payments: PaymentLineInput[]) => void;
  onSplit: (
    payload:
      | { mode: "lines"; assignments: Array<{ line_ids: string[] }> }
      | { mode: "quantidades"; num_accounts: number }
      | { mode: "encaixar"; assignments: Array<{ line_ids: string[] }> }
  ) => void;
}) {
  const [selectedMethod, setSelectedMethod] = createSignal<string | "">("");
  const [receivedCents, setReceivedCents] = createSignal<number>(0);
  const [showAdvanced, setShowAdvanced] = createSignal(false);
  const [showPartial, setShowPartial] = createSignal(false);
  const [showSplit, setShowSplit] = createSignal(false);
  const total = () => props.doc?.document.total ?? 0;
  const change = () => Math.max(0, receivedCents() - total());
  const remaining = () => Math.max(0, total() - receivedCents());

  const quickCents = [500, 1000, 2000, 5000];
  const addReceived = (cents: number) => setReceivedCents((r) => r + cents);
  const setExact = () => setReceivedCents(total());

  return (
    <div class="w-80 bg-zinc-900 border-r border-zinc-700 flex flex-col relative h-full overflow-hidden">
      <div class="p-4 border-b border-zinc-800 bg-zinc-900 shrink-0">
        <h2 class="text-xl font-bold text-zinc-200">Pedido Actual</h2>
      </div>

      <div class="flex-1 min-h-0 overflow-y-auto p-4 space-y-2">
        <Show
          when={(props.doc?.lines.length ?? 0) > 0}
          fallback={
            <div class="text-zinc-500 text-sm italic">
              {props.doc
                ? "Toque num artigo para o adicionar."
                : "Sem mesa aberta."}
            </div>
          }
        >
          <For each={props.doc?.lines ?? []}>
            {(line) => {
              const pending = line.pedida_em === null;
              const anulada = line.anulada;
              const bg = anulada
                ? "bg-red-950/30 border border-red-900/50"
                : pending
                  ? "bg-amber-900/30 border border-amber-700/50"
                  : "bg-zinc-800/50";
              return (
                <div class={`py-2 px-3 rounded-lg ${bg}`}>
                  <div class="flex justify-between items-center">
                    <div class="flex items-center gap-3 min-w-0">
                      <span
                        class={`text-sm font-bold text-center shrink-0 ${
                          anulada ? "text-red-400 line-through" : "text-blue-400"
                        }`}
                      >
                        {fmtQtyMilli(line.qty_milli)}x
                      </span>
                      <span
                        class={`text-sm truncate ${
                          anulada ? "text-zinc-500 line-through" : "text-zinc-200"
                        }`}
                      >
                        {lineLabel(line, props.articleById)}
                      </span>
                      <Show when={pending && !anulada}>
                        <span class="text-[10px] uppercase tracking-wider text-amber-300 font-bold">
                          novo
                        </span>
                      </Show>
                      <Show when={anulada}>
                        <span class="text-[10px] uppercase tracking-wider text-red-300 font-bold">
                          anulada{line.anulada_com_desperdicio ? " (desp.)" : ""}
                        </span>
                      </Show>
                    </div>
                    <span
                      class={`text-sm font-mono ${
                        anulada ? "text-zinc-500 line-through" : "text-zinc-300"
                      }`}
                    >
                      {fmtMoney(line.total)}
                    </span>
                  </div>
                  <Show when={!anulada && !props.doc?.document.is_closed}>
                    <div class="flex gap-2 mt-1 ml-9">
                      <Show
                        when={pending}
                        fallback={
                          <button
                            onClick={() => props.onAnularLine(line.id)}
                            disabled={props.busy}
                            class="text-[10px] uppercase font-bold px-2 py-0.5 rounded-md bg-red-800 hover:bg-red-700 text-white disabled:opacity-40"
                          >
                            Anular
                          </button>
                        }
                      >
                        <button
                          onClick={() => props.onCancelLine(line.id)}
                          disabled={props.busy}
                          class="text-[10px] uppercase font-bold px-2 py-0.5 rounded-md bg-zinc-700 hover:bg-zinc-600 text-zinc-200 disabled:opacity-40"
                        >
                          Cancelar
                        </button>
                      </Show>
                    </div>
                  </Show>
                  <Show when={anulada && line.anulada_motivo}>
                    <div class="text-xs text-zinc-500 ml-9 italic">
                      Motivo: {line.anulada_motivo}
                    </div>
                  </Show>
                </div>
              );
            }}
          </For>
        </Show>

        <Show when={props.receipt}>
          <pre class="mt-4 p-3 bg-black/50 text-emerald-300 text-xs whitespace-pre overflow-x-auto rounded-lg border border-emerald-900">{props.receipt}</pre>
        </Show>
      </div>

      <div class="p-4 bg-zinc-800 border-t border-zinc-700 shrink-0">
        <div class="flex justify-between items-end mb-3">
          <span class="text-zinc-400 font-medium">Total</span>
          <span class="text-3xl font-bold tracking-tight text-white font-mono">
            {fmtMoney(total())}
          </span>
        </div>

        <Show when={props.localKind === "take_away"}>
          <div class="mb-3 grid grid-cols-3 gap-2">
            <For each={quickCents}>
              {(c) => (
                <button
                  onClick={() => addReceived(c)}
                  disabled={props.doc?.document.is_closed || props.busy}
                  class="px-2 py-2 rounded-lg text-sm font-bold bg-amber-700 hover:bg-amber-600 text-white disabled:opacity-50"
                >
                  + {fmtMoney(c)}
                </button>
              )}
            </For>
            <button
              onClick={setExact}
              class="px-2 py-2 rounded-lg text-sm font-bold bg-amber-900 hover:bg-amber-800 text-white"
            >
              Exacto
            </button>
            <button
              onClick={() => setReceivedCents(0)}
              class="px-2 py-2 rounded-lg text-sm bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
            >
              Limpar
            </button>
          </div>
          <div class="mb-3 text-sm font-mono bg-zinc-900 rounded-md p-2 space-y-1">
            <div class="flex justify-between">
              <span class="text-zinc-400">Recebido</span>
              <span class="text-zinc-100">{fmtMoney(receivedCents())}</span>
            </div>
            <Show when={remaining() > 0}>
              <div class="flex justify-between text-amber-300">
                <span>Em Falta</span>
                <span>{fmtMoney(remaining())}</span>
              </div>
            </Show>
            <Show when={change() > 0}>
              <div class="flex justify-between text-emerald-300 font-bold">
                <span>Troco</span>
                <span>{fmtMoney(change())}</span>
              </div>
            </Show>
          </div>
        </Show>

        <button
          onClick={props.onPedir}
          disabled={
            !props.doc ||
            props.doc.document.is_closed ||
            !(props.doc.lines ?? []).some((l) => l.pedida_em === null) ||
            props.busy
          }
          class="w-full mb-2 py-2 rounded-lg font-bold text-sm bg-amber-600 hover:bg-amber-500 text-white shadow disabled:opacity-50 disabled:pointer-events-none"
        >
          PEDIR (imprime cozinha/bar)
        </button>

        <Show
          when={
            (props.localKind === "normal" || props.localKind === "pub") &&
            props.doc?.document.table_id &&
            !props.doc?.document.is_closed
          }
        >
          <button
            onClick={props.onTransfer}
            disabled={
              !props.doc ||
              (props.doc.lines ?? []).filter((l) => !l.anulada).length === 0 ||
              props.busy
            }
            class="w-full mb-3 py-2 rounded-lg font-bold text-sm bg-indigo-700 hover:bg-indigo-600 text-white shadow disabled:opacity-50 disabled:pointer-events-none"
          >
            TRANSFERIR
          </button>
        </Show>

        <div class="grid grid-cols-2 gap-2 mb-3">
          <For each={props.paymentMethods}>
            {(pm) => (
              <button
                onClick={() => setSelectedMethod(pm.id)}
                disabled={props.doc?.document.is_closed || props.busy}
                class={`px-3 py-2 rounded-lg text-sm font-semibold transition-colors disabled:opacity-50 disabled:pointer-events-none ${
                  selectedMethod() === pm.id
                    ? "bg-blue-600 text-white"
                    : "bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
                }`}
              >
                {pm.name}
              </button>
            )}
          </For>
        </div>
        <button
          onClick={() => props.onClose(selectedMethod() || null)}
          disabled={
            !props.doc ||
            props.doc.document.is_closed ||
            props.doc.lines.length === 0 ||
            !selectedMethod() ||
            props.busy
          }
          class="w-full py-4 rounded-xl font-bold text-lg transition-colors shadow-lg active:scale-[0.98]
                 bg-emerald-500 hover:bg-emerald-400 text-zinc-950 disabled:opacity-50 disabled:pointer-events-none"
        >
          {props.doc?.document.is_closed
            ? "FECHADA"
            : props.localKind === "take_away"
              ? "COBRAR & IMPRIMIR"
              : "FECHAR & IMPRIMIR"}
        </button>

        <div class="grid grid-cols-3 gap-2 mt-2">
          <button
            onClick={() => setShowAdvanced(true)}
            disabled={
              !props.doc ||
              props.doc.document.is_closed ||
              props.doc.lines.length === 0 ||
              props.busy
            }
            class="py-2 rounded-lg font-semibold text-xs bg-zinc-700 hover:bg-zinc-600 text-zinc-100 disabled:opacity-50 disabled:pointer-events-none"
          >
            Múltiplos métodos
          </button>
          <button
            onClick={() => setShowPartial(true)}
            disabled={
              !props.doc ||
              props.doc.document.is_closed ||
              (props.doc.lines ?? []).filter(
                (l) => l.pedida_em !== null && !l.anulada
              ).length === 0 ||
              props.busy
            }
            class="py-2 rounded-lg font-semibold text-xs bg-zinc-700 hover:bg-zinc-600 text-zinc-100 disabled:opacity-50 disabled:pointer-events-none"
          >
            Pagamento parcial
          </button>
          <button
            onClick={() => setShowSplit(true)}
            disabled={
              !props.doc ||
              props.doc.document.is_closed ||
              (props.doc.lines ?? []).filter(
                (l) => l.pedida_em !== null && !l.anulada
              ).length < 2 ||
              props.busy
            }
            class="py-2 rounded-lg font-semibold text-xs bg-zinc-700 hover:bg-zinc-600 text-zinc-100 disabled:opacity-50 disabled:pointer-events-none"
          >
            Dividir conta
          </button>
        </div>
      </div>

      <Show when={showAdvanced() && props.doc && !props.doc.document.is_closed}>
        <AdvancedPaymentModal
          total={total()}
          paymentMethods={props.paymentMethods}
          busy={props.busy}
          onCancel={() => setShowAdvanced(false)}
          onConfirm={(payments) => {
            setShowAdvanced(false);
            props.onCloseMulti(payments);
          }}
        />
      </Show>

      <Show when={showPartial() && props.doc && !props.doc.document.is_closed}>
        <PartialPaymentModal
          lines={(props.doc?.lines ?? []).filter(
            (l) => l.pedida_em !== null && !l.anulada
          )}
          articleById={props.articleById}
          paymentMethods={props.paymentMethods}
          busy={props.busy}
          onCancel={() => setShowPartial(false)}
          onConfirm={(lineIds, payments) => {
            setShowPartial(false);
            props.onPartialClose(lineIds, payments);
          }}
        />
      </Show>

      <Show when={showSplit() && props.doc && !props.doc.document.is_closed}>
        <SplitDocumentModal
          documentId={props.doc!.document.id}
          lines={(props.doc?.lines ?? []).filter(
            (l) => l.pedida_em !== null && !l.anulada
          )}
          articleById={props.articleById}
          busy={props.busy}
          onCancel={() => setShowSplit(false)}
          onConfirm={(payload) => {
            setShowSplit(false);
            props.onSplit(payload);
          }}
        />
      </Show>
    </div>
  );
}

type AdvancedRow = {
  payment_method_id: string;
  amount: number; // cêntimos
  descricao: string;
};

function AdvancedPaymentModal(props: {
  total: number;
  paymentMethods: PaymentMethod[];
  busy: boolean;
  onCancel: () => void;
  onConfirm: (payments: PaymentLineInput[]) => void;
}) {
  // Linha inicial: nenhum método pré-seleccionado, valor = total (atalho 1-método).
  const [rows, setRows] = createSignal<AdvancedRow[]>([
    { payment_method_id: "", amount: props.total, descricao: "" },
  ]);
  const [amountInput, setAmountInput] = createSignal<string>(
    (props.total / 100).toFixed(2)
  );
  const [descInput, setDescInput] = createSignal<string>("");

  const sumCents = () =>
    rows().reduce(
      (s, r) => (r.payment_method_id ? s + Math.max(0, r.amount) : s),
      0
    );
  const remaining = () => Math.max(0, props.total - sumCents());
  const change = () => Math.max(0, sumCents() - props.total);
  const canConfirm = () =>
    sumCents() >= props.total &&
    rows().some((r) => r.payment_method_id && r.amount > 0);

  // Atribui o método à 1ª linha sem método, ou cria uma nova com o valor em
  // falta. Permite o fluxo do spec §8.2: "introduzir valor → premir método 1
  // → introduzir resto → premir método 2".
  const applyMethod = (methodId: string) => {
    const valueCents = Math.round(parseFloat(amountInput() || "0") * 100);
    if (!Number.isFinite(valueCents) || valueCents <= 0) return;
    const desc = descInput().trim();
    setRows((curr) => {
      const idx = curr.findIndex((r) => !r.payment_method_id);
      if (idx >= 0) {
        const next = curr.slice();
        next[idx] = {
          payment_method_id: methodId,
          amount: valueCents,
          descricao: desc,
        };
        return next;
      }
      return [
        ...curr,
        { payment_method_id: methodId, amount: valueCents, descricao: desc },
      ];
    });
    // Pré-preenche próxima linha com o valor em falta. setRows já aplicou
    // este valueCents, por isso sumCents() já o inclui — não somar de novo.
    const next = Math.max(0, props.total - sumCents());
    setAmountInput((next / 100).toFixed(2));
    setDescInput("");
    setRows((curr) => {
      if (curr.every((r) => r.payment_method_id)) {
        return [
          ...curr,
          { payment_method_id: "", amount: next, descricao: "" },
        ];
      }
      return curr;
    });
  };

  const removeRow = (idx: number) =>
    setRows((curr) => curr.filter((_, i) => i !== idx));

  const methodName = (id: string) =>
    props.paymentMethods.find((m) => m.id === id)?.name ?? "?";

  const submit = () => {
    const payments: PaymentLineInput[] = rows()
      .filter((r) => r.payment_method_id && r.amount > 0)
      .map((r) => ({
        payment_method_id: r.payment_method_id,
        amount: r.amount,
        descricao: r.descricao.trim() || null,
      }));
    props.onConfirm(payments);
  };

  return (
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
      <div class="bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl w-full max-w-xl flex flex-col max-h-[90vh]">
        <div class="px-5 py-4 border-b border-zinc-800 flex justify-between items-center">
          <h2 class="text-xl font-bold text-white">Recebimento — Avançado</h2>
          <button
            onClick={props.onCancel}
            class="text-zinc-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <div class="flex-1 overflow-y-auto p-5 space-y-4">
          <div class="grid grid-cols-3 gap-2 text-sm font-mono">
            <div class="bg-zinc-800 rounded-lg p-3">
              <div class="text-zinc-400 text-xs uppercase">Total</div>
              <div class="text-white text-lg">{fmtMoney(props.total)}</div>
            </div>
            <div class="bg-zinc-800 rounded-lg p-3">
              <div class="text-zinc-400 text-xs uppercase">Recebido</div>
              <div class="text-white text-lg">{fmtMoney(sumCents())}</div>
            </div>
            <div
              class={`rounded-lg p-3 ${
                remaining() > 0
                  ? "bg-amber-900/40"
                  : change() > 0
                    ? "bg-emerald-900/40"
                    : "bg-zinc-800"
              }`}
            >
              <div class="text-zinc-300 text-xs uppercase">
                {remaining() > 0 ? "Em Falta" : change() > 0 ? "Troco" : "OK"}
              </div>
              <div class="text-white text-lg">
                {fmtMoney(remaining() > 0 ? remaining() : change())}
              </div>
            </div>
          </div>

          <div>
            <div class="text-xs uppercase text-zinc-400 mb-1">
              Valor da próxima linha
            </div>
            <div class="grid grid-cols-3 gap-2">
              <input
                type="number"
                inputmode="decimal"
                step="0.01"
                min="0"
                value={amountInput()}
                onInput={(e) => setAmountInput(e.currentTarget.value)}
                class="col-span-1 px-3 py-2 rounded-lg bg-zinc-800 text-white font-mono border border-zinc-700"
              />
              <input
                type="text"
                placeholder="Descrição (opcional)"
                value={descInput()}
                onInput={(e) => setDescInput(e.currentTarget.value)}
                class="col-span-2 px-3 py-2 rounded-lg bg-zinc-800 text-white text-sm border border-zinc-700"
              />
            </div>
          </div>

          <div>
            <div class="text-xs uppercase text-zinc-400 mb-1">
              Aplicar método
            </div>
            <div class="grid grid-cols-2 gap-2">
              <For each={props.paymentMethods}>
                {(pm) => (
                  <button
                    onClick={() => applyMethod(pm.id)}
                    disabled={props.busy}
                    class="px-3 py-2 rounded-lg text-sm font-semibold bg-blue-700 hover:bg-blue-600 text-white disabled:opacity-50"
                  >
                    {pm.name}
                  </button>
                )}
              </For>
            </div>
          </div>

          <div>
            <div class="text-xs uppercase text-zinc-400 mb-1">
              Rodapés aplicados
            </div>
            <div class="space-y-1">
              <For
                each={rows().filter((r) => r.payment_method_id)}
                fallback={
                  <div class="text-zinc-500 italic text-sm py-2">
                    Sem rodapés. Indique valor + método.
                  </div>
                }
              >
                {(row) => (
                  <div class="flex items-center justify-between bg-zinc-800 rounded-md px-3 py-2">
                    <div class="flex flex-col">
                      <span class="text-sm text-white font-semibold">
                        {methodName(row.payment_method_id)}
                      </span>
                      <Show when={row.descricao}>
                        <span class="text-xs text-zinc-400">
                          {row.descricao}
                        </span>
                      </Show>
                    </div>
                    <div class="flex items-center gap-3">
                      <span class="text-sm font-mono text-zinc-200">
                        {fmtMoney(row.amount)}
                      </span>
                      <button
                        onClick={() =>
                          removeRow(
                            rows().findIndex(
                              (r) =>
                                r.payment_method_id === row.payment_method_id &&
                                r.amount === row.amount &&
                                r.descricao === row.descricao
                            )
                          )
                        }
                        class="text-red-400 hover:text-red-300 text-lg leading-none px-2"
                        title="Remover rodapé"
                      >
                        ×
                      </button>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </div>
        </div>

        <div class="px-5 py-4 border-t border-zinc-800 flex gap-2 justify-end">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-zinc-100 text-sm font-semibold"
          >
            Cancelar
          </button>
          <button
            onClick={submit}
            disabled={!canConfirm() || props.busy}
            class="px-4 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-zinc-950 text-sm font-bold disabled:opacity-50 disabled:pointer-events-none"
          >
            Fechar & Imprimir
          </button>
        </div>
      </div>
    </div>
  );
}

// Pagamento Parcial: o operador selecciona N linhas pedidas (não anuladas),
// escolhe um método de pagamento mono-método (mantém o fluxo simples para o
// caso comum — pagar uma rodada para um cliente), e o servidor cria um filho
// com essas linhas e fecha-o fiscalmente. O pai mantém-se aberto.
function PartialPaymentModal(props: {
  lines: DocumentDetail[];
  articleById: (id: string) => Article | undefined;
  paymentMethods: PaymentMethod[];
  busy: boolean;
  onCancel: () => void;
  onConfirm: (lineIds: string[], payments: PaymentLineInput[]) => void;
}) {
  const [selected, setSelected] = createSignal<Set<string>>(new Set());
  const [methodId, setMethodId] = createSignal<string>("");

  const toggle = (id: string) =>
    setSelected((curr) => {
      const next = new Set(curr);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });

  const selectedTotal = () =>
    props.lines
      .filter((l) => selected().has(l.id))
      .reduce((s, l) => s + l.total, 0);

  const canConfirm = () => selected().size > 0 && methodId() !== "";

  const submit = () => {
    const ids = props.lines.filter((l) => selected().has(l.id)).map((l) => l.id);
    props.onConfirm(ids, [
      { payment_method_id: methodId(), amount: selectedTotal() },
    ]);
  };

  return (
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
      <div class="bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl w-full max-w-xl flex flex-col max-h-[90vh]">
        <div class="px-5 py-4 border-b border-zinc-800 flex justify-between items-center">
          <h2 class="text-xl font-bold text-white">Pagamento Parcial</h2>
          <button
            onClick={props.onCancel}
            class="text-zinc-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <div class="flex-1 overflow-y-auto p-5 space-y-4">
          <div class="text-sm text-zinc-300">
            Selecciona as linhas a pagar. Vai ser gerada uma factura
            independente para essas linhas. A mesa mantém-se aberta com o
            resto do pedido.
          </div>

          <div class="space-y-1">
            <For each={props.lines}>
              {(line) => {
                const checked = () => selected().has(line.id);
                return (
                  <label
                    class={`flex items-center justify-between gap-3 px-3 py-2 rounded-md border cursor-pointer ${
                      checked()
                        ? "bg-blue-900/30 border-blue-700"
                        : "bg-zinc-800 border-zinc-700"
                    }`}
                  >
                    <div class="flex items-center gap-3 min-w-0">
                      <input
                        type="checkbox"
                        checked={checked()}
                        onChange={() => toggle(line.id)}
                        class="w-5 h-5"
                      />
                      <span class="text-sm font-bold text-blue-400">
                        {fmtQtyMilli(line.qty_milli)}x
                      </span>
                      <span class="text-sm text-zinc-100 truncate">
                        {lineLabel(line, props.articleById)}
                      </span>
                    </div>
                    <span class="text-sm font-mono text-zinc-200">
                      {fmtMoney(line.total)}
                    </span>
                  </label>
                );
              }}
            </For>
          </div>

          <div class="bg-zinc-800 rounded-lg p-3 flex justify-between items-center">
            <span class="text-zinc-400 text-sm">Total seleccionado</span>
            <span class="text-white font-mono text-lg">
              {fmtMoney(selectedTotal())}
            </span>
          </div>

          <div>
            <div class="text-xs uppercase text-zinc-400 mb-1">
              Método de pagamento
            </div>
            <div class="grid grid-cols-2 gap-2">
              <For each={props.paymentMethods}>
                {(pm) => (
                  <button
                    onClick={() => setMethodId(pm.id)}
                    class={`px-3 py-2 rounded-lg text-sm font-semibold ${
                      methodId() === pm.id
                        ? "bg-blue-600 text-white"
                        : "bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
                    }`}
                  >
                    {pm.name}
                  </button>
                )}
              </For>
            </div>
          </div>
        </div>

        <div class="px-5 py-4 border-t border-zinc-800 flex gap-2 justify-end">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-zinc-100 text-sm font-semibold"
          >
            Cancelar
          </button>
          <button
            onClick={submit}
            disabled={!canConfirm() || props.busy}
            class="px-4 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-zinc-950 text-sm font-bold disabled:opacity-50 disabled:pointer-events-none"
          >
            Cobrar parcial & imprimir
          </button>
        </div>
      </div>
    </div>
  );
}

type SplitMode = "lines" | "quantidades" | "encaixar";
type SplitConfirmPayload =
  | { mode: "lines"; assignments: Array<{ line_ids: string[] }> }
  | { mode: "quantidades"; num_accounts: number }
  | { mode: "encaixar"; assignments: Array<{ line_ids: string[] }> };

// Divisão de Conta com três modos:
//   * Linhas: cada linha vai inteira para uma só conta (totais podem diferir)
//   * Quantidades: cada linha é dividida fraccionariamente em N partes
//   * Encaixar: operador atribui linhas a contas primárias; sistema gera
//     compensações para igualar totais
// Ao confirmar, o servidor cria N filhos. O pai fica split-closed quando todas
// as linhas elegíveis tiverem sido processadas.
function SplitDocumentModal(props: {
  documentId: string;
  lines: DocumentDetail[];
  articleById: (id: string) => Article | undefined;
  busy: boolean;
  onCancel: () => void;
  onConfirm: (payload: SplitConfirmPayload) => void;
}) {
  const [mode, setMode] = createSignal<SplitMode>("lines");
  const [numAccounts, setNumAccounts] = createSignal<number>(2);
  // Mapa lineId -> accountIndex (0-based). Usado em modos "lines" e "encaixar".
  const [assignment, setAssignment] = createSignal<Record<string, number>>(
    Object.fromEntries(props.lines.map((l) => [l.id, 0]))
  );

  const accountIdxs = () =>
    Array.from({ length: numAccounts() }, (_, i) => i);

  const eligibleTotal = () => props.lines.reduce((s, l) => s + l.total, 0);

  // Total de cada conta segundo o modo seleccionado.
  const accountTotal = (idx: number): number => {
    const m = mode();
    if (m === "quantidades") {
      // Cada conta paga floor(total/N); cêntimos residuais ficam no pai.
      return Math.floor(eligibleTotal() / numAccounts());
    }
    if (m === "encaixar") {
      // Cada conta paga sempre o target, independentemente da atribuição.
      return Math.floor(eligibleTotal() / numAccounts());
    }
    // Modo "lines": soma das linhas atribuídas à conta.
    return props.lines
      .filter((l) => assignment()[l.id] === idx)
      .reduce((s, l) => s + l.total, 0);
  };

  const cycle = (lineId: string) => {
    setAssignment((curr) => {
      const cur = curr[lineId] ?? 0;
      return { ...curr, [lineId]: (cur + 1) % numAccounts() };
    });
  };

  // Ao mudar N, normaliza atribuições fora de range para a conta 0.
  const setN = (n: number) => {
    setNumAccounts(Math.max(2, Math.min(10, n)));
    setAssignment((curr) => {
      const max = numAccounts();
      const next: Record<string, number> = {};
      for (const [k, v] of Object.entries(curr)) {
        next[k] = v < max ? v : 0;
      }
      return next;
    });
  };

  const autoDistribute = async () => {
    try {
      const plan = await api.autoSplitPlan(props.documentId, numAccounts());
      const next: Record<string, number> = {};
      plan.assignments.forEach((acc, idx) => {
        for (const lineId of acc.line_ids) next[lineId] = idx;
      });
      for (const l of props.lines) if (!(l.id in next)) next[l.id] = 0;
      setAssignment(next);
    } catch {
      /* mantém estado actual */
    }
  };

  const needsAssignments = () => mode() === "lines" || mode() === "encaixar";

  const canConfirm = () => {
    if (mode() === "quantidades") return numAccounts() >= 2 && props.lines.length > 0;
    // lines/encaixar: cada conta tem pelo menos uma linha atribuída.
    if (mode() === "lines") {
      return accountIdxs().every((idx) =>
        props.lines.some((l) => assignment()[l.id] === idx)
      );
    }
    // encaixar: cada conta tem pelo menos uma linha (a "primária"), senão o
    // recibo de uma conta fica vazio (só compensações).
    return accountIdxs().every((idx) =>
      props.lines.some((l) => assignment()[l.id] === idx)
    );
  };

  const submit = () => {
    if (mode() === "quantidades") {
      props.onConfirm({ mode: "quantidades", num_accounts: numAccounts() });
      return;
    }
    const assignments = accountIdxs().map((idx) => ({
      line_ids: props.lines
        .filter((l) => assignment()[l.id] === idx)
        .map((l) => l.id),
    }));
    props.onConfirm({ mode: mode() as "lines" | "encaixar", assignments });
  };

  return (
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
      <div class="bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl w-full max-w-3xl flex flex-col max-h-[90vh]">
        <div class="px-5 py-4 border-b border-zinc-800 flex justify-between items-center">
          <h2 class="text-xl font-bold text-white">Dividir Conta</h2>
          <button
            onClick={props.onCancel}
            class="text-zinc-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>

        <div class="flex-1 overflow-y-auto p-5 space-y-4">
          <div class="grid grid-cols-3 gap-2">
            <button
              onClick={() => setMode("lines")}
              class={`py-2 px-3 rounded-lg text-sm font-semibold ${
                mode() === "lines"
                  ? "bg-blue-600 text-white"
                  : "bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              }`}
            >
              Linhas
            </button>
            <button
              onClick={() => setMode("quantidades")}
              class={`py-2 px-3 rounded-lg text-sm font-semibold ${
                mode() === "quantidades"
                  ? "bg-blue-600 text-white"
                  : "bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              }`}
            >
              Quantidades
            </button>
            <button
              onClick={() => setMode("encaixar")}
              class={`py-2 px-3 rounded-lg text-sm font-semibold ${
                mode() === "encaixar"
                  ? "bg-blue-600 text-white"
                  : "bg-zinc-700 hover:bg-zinc-600 text-zinc-100"
              }`}
            >
              Encaixar
            </button>
          </div>

          <div class="text-xs text-zinc-400">
            <Show when={mode() === "lines"}>
              Cada linha vai inteira para uma só conta. Toca na linha para a
              passar à próxima conta. Totais por conta podem diferir.
            </Show>
            <Show when={mode() === "quantidades"}>
              Cada linha é dividida fraccionariamente em N partes iguais.
              Cada conta paga exactamente o mesmo (cêntimo residual é
              absorvido pelo pai).
            </Show>
            <Show when={mode() === "encaixar"}>
              Atribui cada linha a uma conta "primária"; sistema gera linhas
              de compensação (positivas e negativas) para igualar totais.
              Cada conta paga o mesmo, mas o recibo da primária mostra o
              artigo completo + compensação.
            </Show>
          </div>

          <div class="flex items-center gap-3">
            <span class="text-zinc-300 text-sm">Nº contas</span>
            <button
              onClick={() => setN(numAccounts() - 1)}
              disabled={numAccounts() <= 2}
              class="w-10 h-10 rounded-md bg-zinc-700 text-white font-bold text-lg disabled:opacity-40"
            >
              −
            </button>
            <span class="text-white font-mono w-8 text-center text-xl">
              {numAccounts()}
            </span>
            <button
              onClick={() => setN(numAccounts() + 1)}
              disabled={numAccounts() >= 10}
              class="w-10 h-10 rounded-md bg-zinc-700 text-white font-bold text-lg disabled:opacity-40"
            >
              +
            </button>
            <Show when={needsAssignments()}>
              <button
                onClick={autoDistribute}
                disabled={props.busy}
                class="ml-auto px-3 py-2 rounded-lg bg-indigo-700 hover:bg-indigo-600 text-white text-sm font-semibold"
              >
                Distribuição Automática
              </button>
            </Show>
          </div>

          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
            <For each={accountIdxs()}>
              {(idx) => (
                <div
                  class={`rounded-lg p-2 border ${
                    accountTotal(idx) > 0
                      ? "border-emerald-700 bg-emerald-900/20"
                      : "border-amber-700 bg-amber-900/20"
                  }`}
                >
                  <div class="text-xs uppercase text-zinc-300">
                    Conta {idx + 1}
                  </div>
                  <div class="text-white font-mono text-lg">
                    {fmtMoney(accountTotal(idx))}
                  </div>
                </div>
              )}
            </For>
          </div>

          <Show when={needsAssignments()}>
            <div class="text-xs text-zinc-400">
              Toca numa linha para passá-la à próxima conta (1 → 2 → … → N → 1).
            </div>

            <div class="space-y-1">
              <For each={props.lines}>
                {(line) => {
                  const idx = () => assignment()[line.id] ?? 0;
                  return (
                    <button
                      onClick={() => cycle(line.id)}
                      class="w-full flex items-center justify-between gap-3 px-3 py-2 rounded-md bg-zinc-800 hover:bg-zinc-700 border border-zinc-700"
                    >
                      <div class="flex items-center gap-3 min-w-0">
                        <span class="text-xs uppercase font-bold text-indigo-300 w-16 text-left">
                          Conta {idx() + 1}
                        </span>
                        <span class="text-sm font-bold text-blue-400">
                          {fmtQtyMilli(line.qty_milli)}x
                        </span>
                        <span class="text-sm text-zinc-100 truncate">
                          {lineLabel(line, props.articleById)}
                        </span>
                      </div>
                      <span class="text-sm font-mono text-zinc-200">
                        {fmtMoney(line.total)}
                      </span>
                    </button>
                  );
                }}
              </For>
            </div>
          </Show>
        </div>

        <div class="px-5 py-4 border-t border-zinc-800 flex gap-2 justify-end items-center">
          <Show when={!canConfirm() && needsAssignments()}>
            <span class="text-xs text-amber-300 mr-auto">
              Cada conta tem de ter pelo menos uma linha.
            </span>
          </Show>
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-zinc-100 text-sm font-semibold"
          >
            Cancelar
          </button>
          <button
            onClick={submit}
            disabled={!canConfirm() || props.busy}
            class="px-4 py-2 rounded-lg bg-emerald-500 hover:bg-emerald-400 text-zinc-950 text-sm font-bold disabled:opacity-50 disabled:pointer-events-none"
          >
            Confirmar Divisão
          </button>
        </div>
      </div>
    </div>
  );
}

function CatalogPane(props: {
  families: Family[];
  visibleFamilies: Family[];
  path: string[];
  onEnter: (id: string) => void;
  onBack: () => void;
  onRoot: () => void;
  articles: Article[];
  tipoPrecoCodigo: number | null;
  loading: boolean;
  disabled: boolean;
  onPick: (a: Article) => void;
}) {
  const familyById = (id: string) => props.families.find((f) => f.id === id);
  const breadcrumb = () =>
    props.path
      .map((id) => familyById(id)?.name ?? "?")
      .join(" › ");
  return (
    <div class="flex-1 flex flex-col overflow-hidden bg-zinc-950">
      <div class="flex items-center gap-2 p-3 border-b border-zinc-800 bg-zinc-900">
        <button
          onClick={props.onRoot}
          class="px-3 py-2 rounded-lg text-sm font-medium bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
        >
          Topo
        </button>
        <Show when={props.path.length > 0}>
          <button
            onClick={props.onBack}
            class="px-3 py-2 rounded-lg text-sm font-medium bg-zinc-700 hover:bg-zinc-600 text-zinc-200"
          >
            ← Voltar
          </button>
          <span class="text-sm text-zinc-400 truncate">{breadcrumb()}</span>
        </Show>
      </div>
      <div class="flex-1 overflow-y-auto p-6">
        <Show
          when={!props.loading}
          fallback={<div class="text-zinc-400">A carregar catálogo…</div>}
        >
          <Show when={props.visibleFamilies.length > 0}>
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4 mb-6">
              <For each={props.visibleFamilies}>
                {(f) => (
                  <button
                    onClick={() => props.onEnter(f.id)}
                    class="aspect-square rounded-2xl bg-indigo-900/40 border border-indigo-600 hover:border-indigo-400 hover:bg-indigo-800/60
                           transition-all p-4 flex flex-col justify-between items-start text-left shadow-md active:scale-95"
                  >
                    <span class="text-lg font-bold text-zinc-100 leading-tight">{f.name}</span>
                    <span class="text-xs font-mono text-indigo-300">Família</span>
                  </button>
                )}
              </For>
            </div>
          </Show>
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
            <For each={props.articles}>
              {(article) => (
                <button
                  onClick={() => props.onPick(article)}
                  disabled={props.disabled}
                  class="aspect-square rounded-2xl bg-zinc-800 border border-zinc-700 hover:border-blue-500 hover:bg-zinc-700
                         transition-all p-4 flex flex-col justify-between items-start text-left shadow-md group relative overflow-hidden active:scale-95
                         disabled:opacity-50 disabled:pointer-events-none"
                >
                  <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                  <span class="text-lg font-bold text-zinc-100 leading-tight relative z-10">
                    {article.name}
                  </span>
                  <span class="text-sm font-mono text-blue-400 relative z-10">
                    {(() => {
                      const p = pvpFor(article, props.tipoPrecoCodigo);
                      return p === 0 ? "Grátis" : fmtMoney(p);
                    })()}
                  </span>
                </button>
              )}
            </For>
          </div>
        </Show>
      </div>
    </div>
  );
}

export default App;
