import { createSignal, For, Show } from "solid-js";
import { api, AtSeriesInfo } from "./api";

// Painel de gestão de comunicação de séries fiscais à AT (web-service
// SeriesWS). Cobre as 4 operações da WSDL: registar, consultar, finalizar,
// anular. Cada acção é independente; a consulta serve de pano de fundo
// (mostra o estado actual conforme reportado pela AT).
export function AtSeriesView() {
  const [items, setItems] = createSignal<AtSeriesInfo[]>([]);
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [info, setInfo] = createSignal<string | null>(null);
  const [openModal, setOpenModal] = createSignal<
    "registar" | "finalizar" | "anular" | null
  >(null);
  const [activeSerie, setActiveSerie] = createSignal<AtSeriesInfo | null>(null);

  const wrap = async (fn: () => Promise<unknown>, ok?: string) => {
    setError(null);
    setInfo(null);
    setBusy(true);
    try {
      await fn();
      if (ok) setInfo(ok);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const consultar = () =>
    wrap(async () => {
      const r = await api.atConsultarSeries({});
      setItems(r.items);
    });

  return (
    <div class="flex-1 flex flex-col bg-zinc-950 text-zinc-100 overflow-hidden">
      <div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between bg-zinc-900">
        <div>
          <h1 class="text-2xl font-bold">Séries AT</h1>
          <p class="text-sm text-zinc-400">
            Comunicação de séries de facturação ao web-service SeriesWS.
          </p>
        </div>
        <div class="flex gap-2">
          <button
            onClick={consultar}
            disabled={busy()}
            class="px-3 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-sm font-semibold disabled:opacity-50"
          >
            {busy() ? "..." : "Consultar AT"}
          </button>
          <button
            onClick={() => setOpenModal("registar")}
            disabled={busy()}
            class="px-3 py-2 rounded-lg bg-emerald-600 hover:bg-emerald-500 text-white text-sm font-semibold disabled:opacity-50"
          >
            Registar nova série
          </button>
        </div>
      </div>

      <Show when={error()}>
        <div class="bg-red-900/60 border-b border-red-700 px-6 py-2 text-sm text-red-100">
          {error()}
        </div>
      </Show>
      <Show when={info()}>
        <div class="bg-emerald-900/40 border-b border-emerald-700 px-6 py-2 text-sm text-emerald-100">
          {info()}
        </div>
      </Show>

      <div class="flex-1 overflow-auto p-6">
        <Show
          when={items().length > 0}
          fallback={
            <div class="text-zinc-500 italic">
              Clica em "Consultar AT" para carregar as séries comunicadas.
            </div>
          }
        >
          <div class="overflow-x-auto rounded-lg border border-zinc-800">
            <table class="w-full text-sm">
              <thead class="bg-zinc-800 text-zinc-300 uppercase text-xs">
                <tr>
                  <th class="text-left px-3 py-2">Série</th>
                  <th class="text-left px-3 py-2">Classe/Tipo</th>
                  <th class="text-left px-3 py-2">Início seq.</th>
                  <th class="text-left px-3 py-2">Cód. validação</th>
                  <th class="text-left px-3 py-2">Estado</th>
                  <th class="text-left px-3 py-2">Data registo</th>
                  <th class="text-left px-3 py-2">Acções</th>
                </tr>
              </thead>
              <tbody>
                <For each={items()}>
                  {(s) => (
                    <tr class="border-t border-zinc-800 hover:bg-zinc-900">
                      <td class="px-3 py-2 font-mono">
                        {s.serie}{" "}
                        <span class="text-xs text-zinc-400">({s.tipo_serie})</span>
                      </td>
                      <td class="px-3 py-2 font-mono">
                        {s.classe_doc}/{s.tipo_doc}
                      </td>
                      <td class="px-3 py-2 font-mono">{s.num_inicial_seq}</td>
                      <td class="px-3 py-2 font-mono">
                        {s.cod_validacao_serie}
                      </td>
                      <td class="px-3 py-2">
                        <EstadoBadge estado={s.estado} />
                      </td>
                      <td class="px-3 py-2 text-zinc-400">{s.data_registo}</td>
                      <td class="px-3 py-2 flex gap-1">
                        <Show when={s.estado === "A"}>
                          <button
                            onClick={() => {
                              setActiveSerie(s);
                              setOpenModal("finalizar");
                            }}
                            class="px-2 py-1 rounded-md bg-amber-700 hover:bg-amber-600 text-xs"
                          >
                            Finalizar
                          </button>
                          <button
                            onClick={() => {
                              setActiveSerie(s);
                              setOpenModal("anular");
                            }}
                            class="px-2 py-1 rounded-md bg-red-700 hover:bg-red-600 text-xs"
                          >
                            Anular
                          </button>
                        </Show>
                      </td>
                    </tr>
                  )}
                </For>
              </tbody>
            </table>
          </div>
        </Show>
      </div>

      <Show when={openModal() === "registar"}>
        <RegistarSerieModal
          busy={busy()}
          onCancel={() => setOpenModal(null)}
          onConfirm={(body) =>
            wrap(async () => {
              const r = await api.atRegistarSerie(body);
              setOpenModal(null);
              setItems((curr) => [r.info, ...curr]);
            }, "Série registada na AT com sucesso.")
          }
        />
      </Show>

      <Show when={openModal() === "finalizar" && activeSerie()}>
        <FinalizarSerieModal
          serie={activeSerie()!}
          busy={busy()}
          onCancel={() => setOpenModal(null)}
          onConfirm={(body) =>
            wrap(async () => {
              const r = await api.atFinalizarSerie(body);
              setOpenModal(null);
              setItems((curr) =>
                curr.map((x) =>
                  x.cod_validacao_serie === r.info.cod_validacao_serie
                    ? r.info
                    : x
                )
              );
            }, "Série finalizada na AT.")
          }
        />
      </Show>

      <Show when={openModal() === "anular" && activeSerie()}>
        <AnularSerieModal
          serie={activeSerie()!}
          busy={busy()}
          onCancel={() => setOpenModal(null)}
          onConfirm={(body) =>
            wrap(async () => {
              const r = await api.atAnularSerie(body);
              setOpenModal(null);
              setItems((curr) =>
                curr.map((x) =>
                  x.cod_validacao_serie === r.info.cod_validacao_serie
                    ? r.info
                    : x
                )
              );
            }, "Comunicação da série anulada na AT.")
          }
        />
      </Show>
    </div>
  );
}

function EstadoBadge(props: { estado: string }) {
  const labels: Record<string, { label: string; cls: string }> = {
    A: { label: "Activa", cls: "bg-emerald-900/40 text-emerald-300" },
    F: { label: "Finalizada", cls: "bg-amber-900/40 text-amber-300" },
    N: { label: "Anulada", cls: "bg-red-900/40 text-red-300" },
  };
  const v = labels[props.estado] ?? {
    label: props.estado,
    cls: "bg-zinc-800 text-zinc-300",
  };
  return (
    <span class={`px-2 py-0.5 rounded text-xs font-bold ${v.cls}`}>
      {v.label}
    </span>
  );
}

function RegistarSerieModal(props: {
  busy: boolean;
  onCancel: () => void;
  onConfirm: (body: {
    serie: string;
    tipo_serie?: "N" | "T";
    classe_doc: string;
    tipo_doc: string;
    num_inicial_seq: number;
    data_inicio_prev_utiliz: string;
    meio_processamento: string;
  }) => void;
}) {
  const today = new Date().toISOString().slice(0, 10);
  const [serie, setSerie] = createSignal("A");
  const [tipoSerie, setTipoSerie] = createSignal<"N" | "T">("N");
  const [classeDoc, setClasseDoc] = createSignal("SI");
  const [tipoDoc, setTipoDoc] = createSignal("FS");
  const [numInicial, setNumInicial] = createSignal(1);
  const [dataInicio, setDataInicio] = createSignal(today);
  const [meio, setMeio] = createSignal("PF");

  const submit = () =>
    props.onConfirm({
      serie: serie(),
      tipo_serie: tipoSerie(),
      classe_doc: classeDoc(),
      tipo_doc: tipoDoc(),
      num_inicial_seq: numInicial(),
      data_inicio_prev_utiliz: dataInicio(),
      meio_processamento: meio(),
    });

  return (
    <ModalShell title="Registar nova série na AT" onCancel={props.onCancel}>
      <div class="grid grid-cols-2 gap-3">
        <Field label="Série">
          <Input value={serie()} onInput={setSerie} />
        </Field>
        <Field label="Tipo">
          <Select
            value={tipoSerie()}
            options={[
              ["N", "N — Normal"],
              ["T", "T — Substituição"],
            ]}
            onChange={(v) => setTipoSerie(v as "N" | "T")}
          />
        </Field>
        <Field label="Classe (2)">
          <Input value={classeDoc()} onInput={setClasseDoc} maxLength={2} />
        </Field>
        <Field label="Tipo doc. (2)">
          <Input value={tipoDoc()} onInput={setTipoDoc} maxLength={2} />
        </Field>
        <Field label="Nº inicial">
          <Input
            value={String(numInicial())}
            onInput={(v) => setNumInicial(parseInt(v || "1") || 1)}
            type="number"
          />
        </Field>
        <Field label="Início previsto">
          <Input
            value={dataInicio()}
            onInput={setDataInicio}
            type="date"
          />
        </Field>
        <Field label="Meio processamento (2)">
          <Select
            value={meio()}
            options={[
              ["PF", "PF — Programa de facturação"],
              ["OO", "OO — Outras aplicações"],
              ["MD", "MD — Manual"],
            ]}
            onChange={setMeio}
          />
        </Field>
      </div>
      <ModalActions
        busy={props.busy}
        onCancel={props.onCancel}
        onConfirm={submit}
        confirmLabel="Registar"
      />
    </ModalShell>
  );
}

function FinalizarSerieModal(props: {
  serie: AtSeriesInfo;
  busy: boolean;
  onCancel: () => void;
  onConfirm: (body: {
    serie: string;
    classe_doc: string;
    tipo_doc: string;
    cod_validacao_serie: string;
    seq_ultimo_doc_emitido: number;
    justificacao?: string;
  }) => void;
}) {
  const [seqUltimo, setSeqUltimo] = createSignal(
    props.serie.seq_ultimo_doc_emitido ?? props.serie.num_inicial_seq
  );
  const [justificacao, setJustificacao] = createSignal("");
  return (
    <ModalShell
      title={`Finalizar série ${props.serie.serie}`}
      onCancel={props.onCancel}
    >
      <p class="text-sm text-zinc-300 mb-3">
        Confirma que esta série não será mais usada após o documento indicado.
      </p>
      <div class="grid grid-cols-1 gap-3">
        <Field label="Nº do último doc. emitido">
          <Input
            type="number"
            value={String(seqUltimo())}
            onInput={(v) => setSeqUltimo(parseInt(v || "0") || 0)}
          />
        </Field>
        <Field label="Justificação (opcional)">
          <textarea
            class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 outline-none focus:border-blue-500"
            rows={3}
            value={justificacao()}
            onInput={(e) => setJustificacao(e.currentTarget.value)}
          />
        </Field>
      </div>
      <ModalActions
        busy={props.busy}
        onCancel={props.onCancel}
        onConfirm={() =>
          props.onConfirm({
            serie: props.serie.serie,
            classe_doc: props.serie.classe_doc,
            tipo_doc: props.serie.tipo_doc,
            cod_validacao_serie: props.serie.cod_validacao_serie,
            seq_ultimo_doc_emitido: seqUltimo(),
            justificacao: justificacao() || undefined,
          })
        }
        confirmLabel="Finalizar"
        confirmClass="bg-amber-600 hover:bg-amber-500"
      />
    </ModalShell>
  );
}

function AnularSerieModal(props: {
  serie: AtSeriesInfo;
  busy: boolean;
  onCancel: () => void;
  onConfirm: (body: {
    serie: string;
    classe_doc: string;
    tipo_doc: string;
    cod_validacao_serie: string;
    motivo: string;
    declaracao_nao_emissao: boolean;
  }) => void;
}) {
  const [motivo, setMotivo] = createSignal("E1");
  const [declaracao, setDeclaracao] = createSignal(false);
  return (
    <ModalShell
      title={`Anular comunicação da série ${props.serie.serie}`}
      onCancel={props.onCancel}
    >
      <p class="text-sm text-zinc-300 mb-3">
        Apenas é possível anular se ainda não tiver emitido documentos com
        esta série. Caso contrário a AT recusa.
      </p>
      <div class="grid grid-cols-1 gap-3">
        <Field label="Motivo (2 chars)">
          <Input value={motivo()} onInput={setMotivo} maxLength={2} />
        </Field>
        <label class="flex items-start gap-2 text-sm text-zinc-200">
          <input
            type="checkbox"
            class="mt-1 w-5 h-5"
            checked={declaracao()}
            onChange={(e) => setDeclaracao(e.currentTarget.checked)}
          />
          Declaro que não emiti documentos com esta série.
        </label>
      </div>
      <ModalActions
        busy={props.busy}
        onCancel={props.onCancel}
        onConfirm={() =>
          props.onConfirm({
            serie: props.serie.serie,
            classe_doc: props.serie.classe_doc,
            tipo_doc: props.serie.tipo_doc,
            cod_validacao_serie: props.serie.cod_validacao_serie,
            motivo: motivo(),
            declaracao_nao_emissao: declaracao(),
          })
        }
        confirmLabel="Anular"
        confirmClass="bg-red-600 hover:bg-red-500"
        confirmDisabled={!declaracao()}
      />
    </ModalShell>
  );
}

function ModalShell(props: {
  title: string;
  onCancel: () => void;
  children: any;
}) {
  return (
    <div class="fixed inset-0 bg-black/70 flex items-center justify-center z-50 p-4">
      <div class="bg-zinc-900 border border-zinc-700 rounded-2xl shadow-2xl w-full max-w-xl flex flex-col max-h-[90vh]">
        <div class="px-5 py-4 border-b border-zinc-800 flex justify-between items-center">
          <h2 class="text-xl font-bold text-white">{props.title}</h2>
          <button
            onClick={props.onCancel}
            class="text-zinc-400 hover:text-white text-2xl leading-none"
          >
            ×
          </button>
        </div>
        <div class="flex-1 overflow-y-auto p-5">{props.children}</div>
      </div>
    </div>
  );
}

function ModalActions(props: {
  busy: boolean;
  onCancel: () => void;
  onConfirm: () => void;
  confirmLabel: string;
  confirmClass?: string;
  confirmDisabled?: boolean;
}) {
  return (
    <div class="mt-5 flex gap-2 justify-end border-t border-zinc-800 pt-4">
      <button
        onClick={props.onCancel}
        class="px-4 py-2 rounded-lg bg-zinc-700 hover:bg-zinc-600 text-zinc-100 text-sm font-semibold"
      >
        Cancelar
      </button>
      <button
        onClick={props.onConfirm}
        disabled={props.busy || props.confirmDisabled}
        class={`px-4 py-2 rounded-lg text-white text-sm font-bold disabled:opacity-50 disabled:pointer-events-none ${
          props.confirmClass ?? "bg-emerald-600 hover:bg-emerald-500"
        }`}
      >
        {props.confirmLabel}
      </button>
    </div>
  );
}

function Field(props: { label: string; children: any }) {
  return (
    <label class="flex flex-col text-sm font-medium gap-1 text-zinc-200">
      {props.label}
      {props.children}
    </label>
  );
}

function Input(props: {
  value: string;
  onInput: (v: string) => void;
  type?: string;
  maxLength?: number;
}) {
  return (
    <input
      type={props.type ?? "text"}
      maxLength={props.maxLength}
      class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 outline-none focus:border-blue-500"
      value={props.value}
      onInput={(e) => props.onInput(e.currentTarget.value)}
    />
  );
}

function Select(props: {
  value: string;
  options: Array<[string, string]>;
  onChange: (v: string) => void;
}) {
  return (
    <select
      class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
      value={props.value}
      onChange={(e) => props.onChange(e.currentTarget.value)}
    >
      <For each={props.options}>
        {([v, label]) => <option value={v}>{label}</option>}
      </For>
    </select>
  );
}
