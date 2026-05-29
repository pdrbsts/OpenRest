import { createMemo, createResource, createSignal, For, Show } from "solid-js";
import { api, Customer, CustomerInput } from "./api";

const PAISES = ["PT", "ES", "FR", "DE", "IT", "GB", "BR", "AO", "MZ"];

export function ClientesView() {
  const [customers, { refetch }] = createResource<Customer[]>(() => api.customers());
  const [search, setSearch] = createSignal("");
  const [editing, setEditing] = createSignal<Customer | "new" | null>(null);
  const [error, setError] = createSignal<string | null>(null);

  const filtered = createMemo(() => {
    const q = search().toLowerCase().trim();
    const list = customers() ?? [];
    if (!q) return list;
    return list.filter(
      (c) =>
        c.nome.toLowerCase().includes(q) ||
        (c.nif ?? "").includes(q) ||
        (c.telefone ?? "").includes(q)
    );
  });

  const forget = async (c: Customer) => {
    if (
      !confirm(
        `RGPD — anonimizar dados pessoais de "${c.nome}"?\n\nEsta operação não pode ser revertida. O cliente passa a [ESQUECIDO] e os contactos são apagados, mas o ID é preservado para integridade fiscal histórica.`
      )
    )
      return;
    setError(null);
    try {
      await api.forgetCustomer(c.id);
      await refetch();
    } catch (e: any) {
      setError(e.message ?? String(e));
    }
  };

  return (
    <div class="flex-1 flex overflow-hidden bg-zinc-950">
      <div class="w-1/2 border-r border-zinc-800 flex flex-col">
        <div class="p-3 border-b border-zinc-800 flex items-center gap-2">
          <input
            type="search"
            placeholder="Procurar nome, NIF ou telefone…"
            value={search()}
            onInput={(e) => setSearch(e.currentTarget.value)}
            class="flex-1 bg-zinc-900 border border-zinc-700 rounded-md px-3 py-2 text-sm"
          />
          <button
            onClick={() => setEditing("new")}
            class="px-3 py-2 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white text-sm font-semibold"
          >
            + Novo
          </button>
        </div>
        <Show when={error()}>
          <div class="m-3 px-3 py-2 bg-red-900/40 border border-red-700 text-red-200 text-sm rounded-md">
            {error()}
          </div>
        </Show>
        <div class="flex-1 overflow-y-auto">
          <Show
            when={!customers.loading}
            fallback={<div class="p-4 text-zinc-500">a carregar…</div>}
          >
            <For each={filtered()}>
              {(c) => (
                <button
                  onClick={() => setEditing(c)}
                  class={`w-full text-left px-3 py-2 border-b border-zinc-900 hover:bg-zinc-800 ${
                    typeof editing() === "object" &&
                    (editing() as Customer | null)?.id === c.id
                      ? "bg-zinc-800"
                      : ""
                  }`}
                >
                  <div class="flex items-center justify-between">
                    <span class="font-semibold text-zinc-100">{c.nome}</span>
                    <span class="text-xs text-zinc-500">{c.pais}</span>
                  </div>
                  <div class="text-xs text-zinc-400 flex gap-3">
                    <Show when={c.nif}>
                      <span>NIF {c.nif}</span>
                    </Show>
                    <Show when={c.telefone}>
                      <span>{c.telefone}</span>
                    </Show>
                    <Show when={c.esquecido_em}>
                      <span class="text-red-400 font-semibold">[esquecido]</span>
                    </Show>
                  </div>
                </button>
              )}
            </For>
            <Show when={filtered().length === 0}>
              <div class="p-6 text-center text-zinc-500 text-sm">
                Sem resultados.
              </div>
            </Show>
          </Show>
        </div>
      </div>

      <div class="flex-1 overflow-y-auto p-6">
        <Show
          when={editing()}
          fallback={
            <div class="text-zinc-500 text-sm italic">
              Selecciona um cliente para editar, ou cria um novo.
            </div>
          }
          keyed
        >
          {(ed) => (
            <CustomerForm
              initial={ed === "new" ? null : ed}
              onCancel={() => setEditing(null)}
              onSaved={async () => {
                await refetch();
                setEditing(null);
              }}
              onForget={ed === "new" ? undefined : () => forget(ed)}
            />
          )}
        </Show>
      </div>
    </div>
  );
}

function CustomerForm(props: {
  initial: Customer | null;
  onCancel: () => void;
  onSaved: () => void;
  onForget?: () => void;
}) {
  const [nome, setNome] = createSignal(props.initial?.nome ?? "");
  const [nif, setNif] = createSignal(props.initial?.nif ?? "");
  const [pais, setPais] = createSignal(props.initial?.pais ?? "PT");
  const [telefone, setTelefone] = createSignal(props.initial?.telefone ?? "");
  const [morada, setMorada] = createSignal(props.initial?.morada ?? "");
  const [codPostal, setCodPostal] = createSignal(props.initial?.cod_postal ?? "");
  const [localidade, setLocalidade] = createSignal(props.initial?.localidade ?? "");
  const [email, setEmail] = createSignal(props.initial?.email ?? "");
  const [observacoes, setObservacoes] = createSignal(
    props.initial?.observacoes ?? ""
  );
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [warning, setWarning] = createSignal<string | null>(null);

  const isEsquecido = props.initial?.esquecido_em != null;

  const save = async () => {
    if (!nome().trim()) {
      setError("Nome é obrigatório.");
      return;
    }
    const body: CustomerInput = {
      nome: nome().trim(),
      nif: nif().trim() || null,
      pais: pais(),
      telefone: telefone().trim() || null,
      morada: morada().trim() || null,
      cod_postal: codPostal().trim() || null,
      localidade: localidade().trim() || null,
      email: email().trim() || null,
      observacoes: observacoes().trim() || null,
    };
    setBusy(true);
    setError(null);
    setWarning(null);
    try {
      const resp = props.initial
        ? await api.updateCustomer(props.initial.id, body)
        : await api.createCustomer({ ...body, nome: body.nome! });
      if (resp.nif_warning) {
        setWarning(resp.nif_warning + " — gravado na mesma.");
      }
      props.onSaved();
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  return (
    <div class="max-w-2xl">
      <h2 class="text-xl font-bold text-zinc-100 mb-4">
        {props.initial ? "Editar cliente" : "Novo cliente"}
        <Show when={isEsquecido}>
          <span class="ml-3 text-xs px-2 py-0.5 rounded-md bg-red-900/60 text-red-200">
            esquecido (RGPD)
          </span>
        </Show>
      </h2>

      <Show when={error()}>
        <div class="mb-3 px-3 py-2 bg-red-900/40 border border-red-700 text-red-200 text-sm rounded-md">
          {error()}
        </div>
      </Show>
      <Show when={warning()}>
        <div class="mb-3 px-3 py-2 bg-amber-900/40 border border-amber-700 text-amber-200 text-sm rounded-md">
          {warning()}
        </div>
      </Show>

      <div class="grid grid-cols-2 gap-3 text-sm">
        <Field label="Nome *" full>
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={nome()}
            onInput={(e) => setNome(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="NIF">
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50 font-mono"
            value={nif()}
            onInput={(e) => setNif(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="País">
          <select
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={pais()}
            onChange={(e) => setPais(e.currentTarget.value)}
            disabled={isEsquecido}
          >
            <For each={PAISES}>{(p) => <option value={p}>{p}</option>}</For>
          </select>
        </Field>
        <Field label="Telefone">
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={telefone()}
            onInput={(e) => setTelefone(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="Email">
          <input
            type="email"
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={email()}
            onInput={(e) => setEmail(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="Morada" full>
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={morada()}
            onInput={(e) => setMorada(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="Cód. postal">
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={codPostal()}
            onInput={(e) => setCodPostal(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="Localidade">
          <input
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50"
            value={localidade()}
            onInput={(e) => setLocalidade(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
        <Field label="Observações" full>
          <textarea
            rows={3}
            class="bg-zinc-800 border border-zinc-700 rounded-md px-3 py-2 disabled:opacity-50 resize-none"
            value={observacoes()}
            onInput={(e) => setObservacoes(e.currentTarget.value)}
            disabled={isEsquecido}
          />
        </Field>
      </div>

      <div class="flex justify-between items-center mt-6">
        <div>
          <Show when={props.onForget && !isEsquecido}>
            <button
              onClick={props.onForget}
              class="px-3 py-2 rounded-md bg-red-900 hover:bg-red-800 text-red-100 text-sm font-semibold"
            >
              Esquecer (RGPD)
            </button>
          </Show>
        </div>
        <div class="flex gap-2">
          <button
            onClick={props.onCancel}
            class="px-4 py-2 rounded-md bg-zinc-700 hover:bg-zinc-600 text-sm"
          >
            Cancelar
          </button>
          <button
            onClick={save}
            disabled={busy() || isEsquecido}
            class="px-5 py-2 rounded-md bg-blue-600 hover:bg-blue-500 text-white text-sm font-bold disabled:opacity-40"
          >
            {props.initial ? "Guardar" : "Criar"}
          </button>
        </div>
      </div>
    </div>
  );
}

function Field(props: { label: string; full?: boolean; children: any }) {
  return (
    <label class={`flex flex-col gap-1 ${props.full ? "col-span-2" : ""}`}>
      <span class="text-xs font-medium text-zinc-400">{props.label}</span>
      {props.children}
    </label>
  );
}
