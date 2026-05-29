import { createResource, createSignal, For, Show } from "solid-js";
import { api, Customer, Entregador, PedidoDelivery, Zona } from "./api";

interface CustomerPickerProps {
  onCancel: () => void;
  onConfirm: (customer: Customer | null, observacoes: string) => void;
}

export function CustomerPicker(props: CustomerPickerProps) {
  const [phone, setPhone] = createSignal("");
  const [results, setResults] = createSignal<Customer[]>([]);
  const [selected, setSelected] = createSignal<Customer | null>(null);
  const [creatingNew, setCreatingNew] = createSignal(false);
  const [newNome, setNewNome] = createSignal("");
  const [newMorada, setNewMorada] = createSignal("");
  const [newZonaId, setNewZonaId] = createSignal<string>("");
  const [observacoes, setObservacoes] = createSignal("");
  const [error, setError] = createSignal<string | null>(null);
  const [busy, setBusy] = createSignal(false);

  const [zonas] = createResource<Zona[]>(() => api.zonas());
  const zonaById = (id: string | null) =>
    id ? zonas()?.find((z) => z.id === id) ?? null : null;

  const search = async () => {
    if (!phone()) return;
    setBusy(true);
    setError(null);
    try {
      const r = await api.searchCustomers({ phone: phone() });
      setResults(r);
      setSelected(r[0] ?? null);
      setCreatingNew(r.length === 0);
      if (r.length === 0) setNewNome("");
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const confirmExisting = () => props.onConfirm(selected(), observacoes());

  const createAndConfirm = async () => {
    if (!newNome()) {
      setError("nome obrigatório");
      return;
    }
    setBusy(true);
    setError(null);
    try {
      const c = await api.createCustomer({
        nome: newNome(),
        telefone: phone() || null,
        morada: newMorada() || null,
        zona_id: newZonaId() || null,
      });
      props.onConfirm(c, observacoes());
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div class="fixed inset-0 bg-black/60 z-30 flex items-center justify-center p-6">
      <div class="bg-zinc-900 border border-zinc-700 rounded-xl shadow-2xl w-full max-w-2xl p-6 text-zinc-100">
        <div class="flex items-center justify-between mb-4">
          <h2 class="text-xl font-bold">Identificar cliente</h2>
          <button
            onClick={props.onCancel}
            class="text-zinc-400 hover:text-white text-2xl"
          >
            ✕
          </button>
        </div>

        <Show when={error()}>
          <div class="bg-red-900/60 border border-red-700 px-3 py-2 rounded text-sm mb-3 text-red-100">
            {error()}
          </div>
        </Show>

        <div class="flex gap-2 mb-4">
          <input
            type="tel"
            autofocus
            placeholder="Telefone (sufixo)"
            class="flex-1 bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 focus:border-blue-500 outline-none font-mono"
            value={phone()}
            onInput={(e) => setPhone(e.currentTarget.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter") search();
            }}
          />
          <button
            onClick={search}
            disabled={busy() || !phone()}
            class="px-4 py-2 rounded-md bg-blue-600 hover:bg-blue-500 text-white font-medium disabled:opacity-50"
          >
            Procurar
          </button>
        </div>

        <Show when={results().length > 0 && !creatingNew()}>
          <div class="space-y-1 mb-4 max-h-48 overflow-y-auto">
            <For each={results()}>
              {(c) => {
                const zona = zonaById(c.zona_id);
                return (
                  <button
                    onClick={() => setSelected(c)}
                    class={`w-full text-left px-3 py-2 rounded-md border ${
                      selected()?.id === c.id
                        ? "bg-blue-700/40 border-blue-500"
                        : "bg-zinc-800 border-zinc-700 hover:bg-zinc-700"
                    }`}
                  >
                    <div class="font-medium">{c.nome}</div>
                    <div class="text-xs text-zinc-400 font-mono">
                      {c.telefone ?? "—"} · {c.morada ?? "sem morada"}
                    </div>
                    <Show when={zona}>
                      <div class="text-xs text-amber-300">
                        Zona: {zona!.designacao} — taxa {(zona!.taxa_entrega / 100).toFixed(2)}€
                      </div>
                    </Show>
                  </button>
                );
              }}
            </For>
          </div>
        </Show>

        <Show when={creatingNew()}>
          <div class="space-y-2 mb-4">
            <div class="text-sm text-amber-300">
              Cliente não encontrado — preenche para criar:
            </div>
            <input
              placeholder="Nome do cliente"
              class="w-full bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
              value={newNome()}
              onInput={(e) => setNewNome(e.currentTarget.value)}
            />
            <input
              placeholder="Morada"
              class="w-full bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
              value={newMorada()}
              onInput={(e) => setNewMorada(e.currentTarget.value)}
            />
            <select
              class="w-full bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
              value={newZonaId()}
              onChange={(e) => setNewZonaId(e.currentTarget.value)}
            >
              <option value="">— sem zona —</option>
              <For each={zonas() ?? []}>
                {(z) => (
                  <option value={z.id}>
                    {z.designacao} (taxa {(z.taxa_entrega / 100).toFixed(2)}€)
                  </option>
                )}
              </For>
            </select>
          </div>
        </Show>

        <Show when={results().length > 0 || creatingNew()}>
          <label class="block text-sm font-medium mb-1">Observações</label>
          <textarea
            class="w-full bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 resize-none mb-4"
            rows={2}
            placeholder="bem passado, frente ao restaurante…"
            value={observacoes()}
            onInput={(e) => setObservacoes(e.currentTarget.value)}
          />
        </Show>

        <div class="flex justify-end gap-2">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600"
          >
            Cancelar
          </button>
          <Show when={results().length > 0 && !creatingNew()}>
            <button
              onClick={() => setCreatingNew(true)}
              class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600"
            >
              Novo cliente
            </button>
            <button
              onClick={confirmExisting}
              disabled={!selected() || busy()}
              class="px-5 py-2 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white font-bold disabled:opacity-50"
            >
              Confirmar
            </button>
          </Show>
          <Show when={creatingNew()}>
            <button
              onClick={createAndConfirm}
              disabled={!newNome() || busy()}
              class="px-5 py-2 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white font-bold disabled:opacity-50"
            >
              Criar e abrir pedido
            </button>
          </Show>
        </div>
      </div>
    </div>
  );
}

interface DespachoProps {
  onClose: () => void;
}

export function DespachoView(_props: DespachoProps) {
  const [deliveries, { refetch }] = createResource<PedidoDelivery[]>(() =>
    api.activeDeliveries()
  );
  const [entregadores] = createResource<Entregador[]>(() => api.entregadores());

  const entregadorNome = (id: string | null) =>
    id ? entregadores()?.find((e) => e.id === id)?.nome ?? "?" : null;

  const transition = async (
    id: string,
    estado:
      | "em_preparacao"
      | "pronto"
      | "despachado"
      | "entregue"
      | "cancelado",
    entregadorId?: string | null
  ) => {
    await api.updateDeliveryState(id, estado, entregadorId ?? null);
    await refetch();
  };

  const stateColors: Record<string, string> = {
    recebido: "bg-zinc-700",
    em_preparacao: "bg-amber-700",
    pronto: "bg-blue-700",
    despachado: "bg-purple-700",
  };
  const stateLabel: Record<string, string> = {
    recebido: "Recebido",
    em_preparacao: "Em preparação",
    pronto: "Pronto",
    despachado: "Despachado",
  };

  return (
    <div class="flex-1 flex flex-col bg-zinc-950 overflow-hidden">
      <div class="px-6 py-3 border-b border-zinc-800 flex items-center justify-between bg-zinc-900">
        <h1 class="text-xl font-bold">Despacho — pedidos pendentes</h1>
        <button
          onClick={() => refetch()}
          class="text-sm px-3 py-1.5 rounded-md bg-zinc-700 hover:bg-zinc-600"
        >
          Refrescar
        </button>
      </div>
      <div class="flex-1 overflow-y-auto p-6">
        <Show
          when={!deliveries.loading}
          fallback={<div class="text-zinc-400">A carregar…</div>}
        >
          <Show
            when={(deliveries() ?? []).length > 0}
            fallback={
              <div class="text-zinc-500 italic">Sem pedidos pendentes.</div>
            }
          >
            <div class="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-4">
              <For each={deliveries()!}>
                {(d) => (
                  <div class="bg-zinc-900 border border-zinc-800 rounded-xl p-4 shadow-md">
                    <div class="flex justify-between items-start mb-2">
                      <div>
                        <div class="font-mono text-xs text-zinc-400">
                          {new Date(d.recebido_em).toLocaleTimeString()}
                        </div>
                        <div class="font-bold text-lg">
                          {d.morada_snapshot ?? "Sem morada"}
                        </div>
                        <div class="text-sm text-zinc-300 font-mono">
                          {d.telefone_snapshot ?? "—"}
                        </div>
                      </div>
                      <span
                        class={`px-2 py-1 rounded-md text-xs font-bold uppercase tracking-wider text-white ${
                          stateColors[d.estado] ?? "bg-zinc-700"
                        }`}
                      >
                        {stateLabel[d.estado] ?? d.estado}
                      </span>
                    </div>
                    <Show when={d.entregador_id}>
                      <div class="text-xs text-zinc-400 mb-2">
                        Entregador: {entregadorNome(d.entregador_id)}
                      </div>
                    </Show>
                    <Show when={d.taxa_entrega_cents > 0}>
                      <div class="text-xs text-amber-300 mb-2">
                        Taxa de entrega: {(d.taxa_entrega_cents / 100).toFixed(2)}€
                      </div>
                    </Show>
                    <div class="flex flex-wrap gap-2 mt-3 border-t border-zinc-800 pt-3">
                      <Show when={d.estado === "recebido"}>
                        <button
                          onClick={() => transition(d.id, "em_preparacao")}
                          class="px-3 py-1.5 rounded-md text-xs font-bold bg-amber-700 hover:bg-amber-600 text-white"
                        >
                          Em preparação
                        </button>
                      </Show>
                      <Show when={d.estado === "em_preparacao"}>
                        <button
                          onClick={() => transition(d.id, "pronto")}
                          class="px-3 py-1.5 rounded-md text-xs font-bold bg-blue-700 hover:bg-blue-600 text-white"
                        >
                          Marcar pronto
                        </button>
                      </Show>
                      <Show when={d.estado === "pronto"}>
                        <select
                          class="bg-zinc-800 rounded-md px-2 py-1 text-xs border border-zinc-700"
                          onChange={(e) => {
                            if (!e.currentTarget.value) return;
                            transition(d.id, "despachado", e.currentTarget.value);
                          }}
                        >
                          <option value="">— atribuir entregador —</option>
                          <For each={entregadores() ?? []}>
                            {(e) => (
                              <option value={e.id}>
                                {e.nome}
                                {e.externo ? " (externo)" : ""}
                              </option>
                            )}
                          </For>
                        </select>
                      </Show>
                      <Show when={d.estado === "despachado"}>
                        <button
                          onClick={() => transition(d.id, "entregue")}
                          class="px-3 py-1.5 rounded-md text-xs font-bold bg-emerald-600 hover:bg-emerald-500 text-white"
                        >
                          Marcar entregue
                        </button>
                      </Show>
                      <button
                        onClick={() => transition(d.id, "cancelado")}
                        class="px-3 py-1.5 rounded-md text-xs bg-red-800 hover:bg-red-700 text-white"
                      >
                        Cancelar
                      </button>
                    </div>
                  </div>
                )}
              </For>
            </div>
          </Show>
        </Show>
      </div>
    </div>
  );
}
