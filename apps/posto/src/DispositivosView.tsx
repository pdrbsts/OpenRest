import { createSignal, For, Show, onMount, createMemo } from "solid-js";
import { api, ConexaoTipo, Dispositivo, DispositivoStatus } from "./api";

// Gestão de dispositivos (impressoras): ligação por fila do Windows, IP directo
// (TCP 9100) ou porta COM, mais ficheiro (mock) e nula. Edita a ligação, testa
// e mostra o estado reportado pela fila de impressão.

const CONEXOES: { tipo: ConexaoTipo; label: string }[] = [
  { tipo: "windows_spooler", label: "Impressora do Windows" },
  { tipo: "tcp", label: "IP directo (TCP 9100)" },
  { tipo: "serial", label: "Porta COM (série)" },
  { tipo: "file", label: "Ficheiro (mock)" },
  { tipo: "null", label: "Nula (desactivada)" },
];

const PARITIES = ["none", "odd", "even"];
const FLOWS = ["none", "software", "hardware"];

export function DispositivosView() {
  const [items, setItems] = createSignal<Dispositivo[]>([]);
  const [selectedId, setSelectedId] = createSignal<string | null>(null);
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [info, setInfo] = createSignal<string | null>(null);
  const [status, setStatus] = createSignal<DispositivoStatus | null>(null);

  const [nome, setNome] = createSignal("");
  const [conexao, setConexao] = createSignal<ConexaoTipo>("windows_spooler");
  // Campos de ligação (usados conforme o tipo).
  const [cfg, setCfg] = createSignal<Record<string, any>>({});

  const selected = createMemo<Dispositivo | undefined>(() =>
    items().find((d) => d.id === selectedId())
  );

  const setField = (k: string, v: any) => setCfg({ ...cfg(), [k]: v });

  const loadInto = (d: Dispositivo) => {
    setSelectedId(d.id);
    setNome(d.nome);
    setConexao(d.conexao_tipo);
    setCfg({ ...(d.conexao_config ?? {}) });
    setInfo(null);
    setError(null);
    setStatus(null);
    refreshStatus(d.id);
  };

  const refresh = async (keepId?: string) => {
    setError(null);
    try {
      const list = await api.dispositivos();
      setItems(list);
      const id = keepId ?? selectedId() ?? list[0]?.id ?? null;
      const d = list.find((x) => x.id === id) ?? list[0];
      if (d) loadInto(d);
    } catch (e: any) {
      setError(e.message ?? String(e));
    }
  };

  const refreshStatus = async (id: string) => {
    try {
      setStatus(await api.dispositivoStatus(id));
    } catch {
      setStatus(null);
    }
  };

  onMount(() => refresh());

  // Monta o conexao_config apenas com os campos relevantes ao tipo escolhido.
  const buildConfig = (): Record<string, unknown> => {
    const c = cfg();
    switch (conexao()) {
      case "file":
        return { path: c.path ?? "./receipts.txt" };
      case "tcp":
        return {
          host: c.host ?? "",
          port: Number(c.port ?? 9100),
          timeout_ms: Number(c.timeout_ms ?? 3000),
        };
      case "serial":
        return {
          port: c.port ?? "COM1",
          baud: Number(c.baud ?? 9600),
          data_bits: Number(c.data_bits ?? 8),
          parity: c.parity ?? "none",
          stop_bits: Number(c.stop_bits ?? 1),
          flow: c.flow ?? "none",
        };
      case "windows_spooler":
        return { printer_name: c.printer_name ?? "", data_type: c.data_type ?? "RAW" };
      default:
        return {};
    }
  };

  const wrap = async (fn: () => Promise<unknown>, ok?: string) => {
    setBusy(true);
    setError(null);
    setInfo(null);
    try {
      await fn();
      if (ok) setInfo(ok);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const createNew = () =>
    wrap(async () => {
      const d = await api.createDispositivo({
        nome: "Nova impressora",
        conexao_tipo: "windows_spooler",
        conexao_config: { printer_name: "", data_type: "RAW" },
      });
      await refresh(d.id);
    });

  const save = () => {
    const id = selectedId();
    if (!id) return;
    return wrap(async () => {
      await api.updateDispositivo(id, {
        nome: nome(),
        conexao_tipo: conexao(),
        conexao_config: buildConfig(),
      });
      await refresh(id);
    }, "Guardado.");
  };

  const testPrint = () => {
    const id = selectedId();
    if (!id) return;
    return wrap(async () => {
      await api.testDispositivo(id);
      // dá tempo ao worker e relê o estado
      setTimeout(() => refreshStatus(id), 600);
    }, "Talão de teste enviado.");
  };

  const inputClass =
    "w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-blue-500";
  const labelClass = "block text-xs font-semibold text-zinc-400 mb-1";

  return (
    <div class="flex-1 flex overflow-hidden bg-zinc-950 text-zinc-100">
      <div class="w-56 bg-zinc-900 border-r border-zinc-800 flex flex-col">
        <div class="p-3 border-b border-zinc-800 flex items-center justify-between">
          <span class="text-sm font-bold text-zinc-200">Dispositivos</span>
          <button
            onClick={createNew}
            class="px-2 py-1 rounded bg-blue-600 hover:bg-blue-500 text-xs font-semibold"
          >
            + Novo
          </button>
        </div>
        <div class="flex-1 overflow-y-auto">
          <For each={items()}>
            {(d) => (
              <button
                onClick={() => loadInto(d)}
                class={`w-full text-left px-3 py-2 text-sm border-b border-zinc-800/60 transition-colors ${
                  d.id === selectedId() ? "bg-blue-600/20 text-blue-200" : "hover:bg-zinc-800 text-zinc-300"
                }`}
              >
                <div class="font-semibold">{d.nome}</div>
                <div class="text-[11px] text-zinc-500">
                  {CONEXOES.find((c) => c.tipo === d.conexao_tipo)?.label ?? d.conexao_tipo}
                </div>
              </button>
            )}
          </For>
        </div>
      </div>

      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between bg-zinc-900">
          <div>
            <h1 class="text-2xl font-bold">Dispositivos</h1>
            <p class="text-sm text-zinc-400">Ligação de impressoras: Windows, IP ou COM.</p>
          </div>
          <div class="flex items-center gap-3">
            <Show when={info()}>
              <span class="text-sm text-emerald-400">{info()}</span>
            </Show>
            <Show when={error()}>
              <span class="text-sm text-red-400">{error()}</span>
            </Show>
            <Show when={selected()}>
              <button
                onClick={testPrint}
                disabled={busy()}
                class="px-3 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-sm font-semibold disabled:opacity-40"
              >
                Testar
              </button>
              <button
                onClick={save}
                disabled={busy()}
                class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-500 text-sm font-semibold disabled:opacity-40"
              >
                {busy() ? "..." : "Guardar"}
              </button>
            </Show>
          </div>
        </div>

        <Show
          when={selected()}
          fallback={
            <div class="flex-1 flex items-center justify-center text-zinc-500">
              Selecciona ou cria um dispositivo.
            </div>
          }
        >
          <div class="flex-1 overflow-y-auto p-6 space-y-4 max-w-xl">
            <div>
              <span class={labelClass}>Nome</span>
              <input value={nome()} onInput={(e) => setNome(e.currentTarget.value)} class={inputClass} />
            </div>

            <div>
              <span class={labelClass}>Tipo de ligação</span>
              <select
                value={conexao()}
                onChange={(e) => setConexao(e.currentTarget.value as ConexaoTipo)}
                class={inputClass}
              >
                <For each={CONEXOES}>
                  {(c) => <option value={c.tipo}>{c.label}</option>}
                </For>
              </select>
            </div>

            {/* Campos dinâmicos por tipo */}
            <Show when={conexao() === "windows_spooler"}>
              <div>
                <span class={labelClass}>Nome da fila no Windows</span>
                <input
                  value={cfg().printer_name ?? ""}
                  onInput={(e) => setField("printer_name", e.currentTarget.value)}
                  placeholder="EPSON TM-T20II"
                  class={inputClass}
                />
              </div>
              <div>
                <span class={labelClass}>Tipo de dados</span>
                <input
                  value={cfg().data_type ?? "RAW"}
                  onInput={(e) => setField("data_type", e.currentTarget.value)}
                  class={inputClass}
                />
              </div>
            </Show>

            <Show when={conexao() === "tcp"}>
              <div class="flex gap-3">
                <div class="flex-1">
                  <span class={labelClass}>Endereço IP / host</span>
                  <input
                    value={cfg().host ?? ""}
                    onInput={(e) => setField("host", e.currentTarget.value)}
                    placeholder="192.168.1.50"
                    class={inputClass}
                  />
                </div>
                <div class="w-28">
                  <span class={labelClass}>Porta</span>
                  <input
                    type="number"
                    value={cfg().port ?? 9100}
                    onInput={(e) => setField("port", e.currentTarget.value)}
                    class={inputClass}
                  />
                </div>
              </div>
            </Show>

            <Show when={conexao() === "serial"}>
              <div class="flex gap-3">
                <div class="flex-1">
                  <span class={labelClass}>Porta COM</span>
                  <input
                    value={cfg().port ?? "COM1"}
                    onInput={(e) => setField("port", e.currentTarget.value)}
                    class={inputClass}
                  />
                </div>
                <div class="w-32">
                  <span class={labelClass}>Baud</span>
                  <input
                    type="number"
                    value={cfg().baud ?? 9600}
                    onInput={(e) => setField("baud", e.currentTarget.value)}
                    class={inputClass}
                  />
                </div>
              </div>
              <div class="flex gap-3">
                <div class="flex-1">
                  <span class={labelClass}>Paridade</span>
                  <select
                    value={cfg().parity ?? "none"}
                    onChange={(e) => setField("parity", e.currentTarget.value)}
                    class={inputClass}
                  >
                    <For each={PARITIES}>{(p) => <option value={p}>{p}</option>}</For>
                  </select>
                </div>
                <div class="flex-1">
                  <span class={labelClass}>Controlo de fluxo</span>
                  <select
                    value={cfg().flow ?? "none"}
                    onChange={(e) => setField("flow", e.currentTarget.value)}
                    class={inputClass}
                  >
                    <For each={FLOWS}>{(f) => <option value={f}>{f}</option>}</For>
                  </select>
                </div>
              </div>
            </Show>

            <Show when={conexao() === "file"}>
              <div>
                <span class={labelClass}>Caminho do ficheiro</span>
                <input
                  value={cfg().path ?? "./receipts.txt"}
                  onInput={(e) => setField("path", e.currentTarget.value)}
                  class={inputClass}
                />
              </div>
            </Show>

            <Show when={status()}>
              {(s) => (
                <div class="mt-4 p-3 rounded-lg bg-zinc-900 border border-zinc-800 text-sm">
                  <div class="font-semibold text-zinc-300 mb-1">Estado da fila</div>
                  <div class="flex gap-4 text-zinc-400">
                    <span>
                      Saúde:{" "}
                      <span
                        class={
                          s().health === "ok"
                            ? "text-emerald-400"
                            : s().health === "failed"
                            ? "text-red-400"
                            : "text-zinc-400"
                        }
                      >
                        {s().health}
                      </span>
                    </span>
                    <span>Na fila: {s().queued}</span>
                    <span>Concluídos: {s().jobs_done}</span>
                  </div>
                  <Show when={s().last_error}>
                    <div class="text-red-400 text-xs mt-1">{s().last_error}</div>
                  </Show>
                </div>
              )}
            </Show>
          </div>
        </Show>
      </div>
    </div>
  );
}
