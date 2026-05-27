import { createMemo, createResource, createSignal, For, Show } from "solid-js";
import "./App.css";
import {
  api,
  Article,
  CatalogResponse,
  DocumentResponse,
  Family,
  Local,
  PaymentMethod,
  Table,
} from "./api";
import { ConfigView } from "./ConfigView";

type View = "tables" | "order" | "config";

const fmtMoney = (cents: number) => (cents / 100).toFixed(2) + "€";

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
  const [locais, { refetch: refetchLocais }] = createResource<Local[]>(() =>
    api.locais()
  );
  const [selectedLocal, setSelectedLocal] = createSignal<string | null>(null);

  // pick first local by default once loaded
  createMemo(() => {
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

  const [view, setView] = createSignal<View>("tables");
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

  const openTable = async (t: Table) => {
    setError(null);
    setReceipt(null);
    setBusy(true);
    try {
      const d = await api.openTable(t.id, null);
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

  const total = () => doc()?.document.total ?? 0;

  return (
    <div class="flex h-full w-full bg-zinc-900 text-white select-none">
      <Sidebar
        view={view()}
        onTables={() => setView("tables")}
        canOrder={!!activeTable()}
        onOrder={() => setView("order")}
        onConfig={() => setView("config")}
      />

      <div class="flex-1 flex flex-col h-full bg-zinc-950">
        <TopBar
          terminal="Terminal 1"
          activeTable={activeTable()}
          onBack={view() === "order" ? backToTables : undefined}
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
              onPickLocal={(id) => setSelectedLocal(id)}
              tables={visibleTables()}
              loading={tables.loading || locais.loading}
              busy={busy()}
              onPick={openTable}
            />
          </Show>

          <Show when={view() === "order" && activeTable()}>
            <OrderColumn
              doc={doc()}
              receipt={receipt()}
              busy={busy()}
              articleById={articleById}
              paymentMethods={paymentMethods() ?? []}
              onClose={closeAndPrint}
            />
            <CatalogPane
              families={families()}
              visibleFamilies={visibleFamilies()}
              path={familyPath()}
              onEnter={(id) => setFamilyPath((p) => [...p, id])}
              onBack={() => setFamilyPath((p) => p.slice(0, -1))}
              onRoot={() => setFamilyPath([])}
              articles={visibleArticles()}
              loading={catalog.loading}
              disabled={!doc() || doc()?.document.is_closed || busy()}
              onPick={addToOrder}
            />
          </Show>
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
      <div class="flex-1" />
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
  onBack?: () => void;
}) {
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
      <span class="text-zinc-400 font-mono text-sm">
        {new Date().toLocaleDateString()}
      </span>
    </div>
  );
}

function TablesView(props: {
  locais: Local[];
  selectedLocal: string | null;
  onPickLocal: (id: string) => void;
  tables: Table[];
  loading: boolean;
  busy: boolean;
  onPick: (t: Table) => void;
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
        <h2 class="text-xl font-bold text-zinc-200 mb-4">Escolher mesa</h2>
        <Show when={!props.loading} fallback={<div class="text-zinc-400">A carregar…</div>}>
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
  onClose: (paymentMethodId: string | null) => void;
}) {
  const [selectedMethod, setSelectedMethod] = createSignal<string | "">("");
  return (
    <div class="w-80 bg-zinc-900 border-r border-zinc-700 flex flex-col relative">
      <div class="p-4 border-b border-zinc-800 bg-zinc-900 sticky top-0">
        <h2 class="text-xl font-bold text-zinc-200">Pedido Actual</h2>
      </div>

      <div class="flex-1 overflow-y-auto p-4 space-y-2">
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
            {(line) => (
              <div class="flex justify-between items-center py-2 px-3 bg-zinc-800/50 rounded-lg">
                <div class="flex items-center gap-3 min-w-0">
                  <span class="text-sm font-bold w-6 text-center text-blue-400 shrink-0">
                    {line.qty}x
                  </span>
                  <span class="text-sm text-zinc-200 truncate">
                    {props.articleById(line.article_id)?.name ?? "Artigo"}
                  </span>
                </div>
                <span class="text-sm font-mono text-zinc-300">
                  {fmtMoney(line.total)}
                </span>
              </div>
            )}
          </For>
        </Show>

        <Show when={props.receipt}>
          <pre class="mt-4 p-3 bg-black/50 text-emerald-300 text-xs whitespace-pre overflow-x-auto rounded-lg border border-emerald-900">
{props.receipt}
          </pre>
        </Show>
      </div>

      <div class="p-4 bg-zinc-800 border-t border-zinc-700 mt-auto">
        <div class="flex justify-between items-end mb-4">
          <span class="text-zinc-400 font-medium">Total</span>
          <span class="text-3xl font-bold tracking-tight text-white font-mono">
            {fmtMoney(props.doc?.document.total ?? 0)}
          </span>
        </div>
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
          {props.doc?.document.is_closed ? "FECHADA" : "FECHAR & IMPRIMIR"}
        </button>
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
                    {fmtMoney(article.price)}
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
