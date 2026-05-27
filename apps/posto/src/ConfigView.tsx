import { createMemo, createSignal, For, Show } from "solid-js";
import { api, Local, LocalKind, Table } from "./api";

const LOCAL_KINDS: LocalKind[] = [
  "normal",
  "take_away",
  "take_away_seguro",
  "pub",
  "delivery",
  "consumo_proprio",
  "restauracao_colectiva",
];

interface Props {
  locais: Local[];
  tablesByLocal: Record<string, Table[]>;
  onChanged: () => void;
}

export function ConfigView(props: Props) {
  const [selectedId, setSelectedId] = createSignal<string | null>(
    props.locais[0]?.id ?? null
  );
  const [editing, setEditing] = createSignal<"local" | "mesas">("local");
  const [error, setError] = createSignal<string | null>(null);

  const selected = createMemo<Local | undefined>(() =>
    props.locais.find((l) => l.id === selectedId())
  );
  const tables = () =>
    (selectedId() ? props.tablesByLocal[selectedId()!] : []) ?? [];

  const wrap = async (fn: () => Promise<unknown>) => {
    setError(null);
    try {
      await fn();
      props.onChanged();
    } catch (e: any) {
      setError(e.message ?? String(e));
    }
  };

  const createNew = () =>
    wrap(async () => {
      const local = await api.createLocal({
        designacao: "Novo local",
        tipo: "normal",
      });
      setSelectedId(local.id);
    });

  return (
    <div class="flex-1 flex overflow-hidden bg-zinc-950">
      <div class="w-64 bg-zinc-900 border-r border-zinc-800 flex flex-col">
        <div class="p-3 border-b border-zinc-800 flex items-center justify-between">
          <span class="text-sm font-bold text-zinc-200">Locais</span>
          <button
            onClick={createNew}
            class="text-xs px-2 py-1 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white"
          >
            + Novo
          </button>
        </div>
        <div class="flex-1 overflow-y-auto">
          <For each={props.locais}>
            {(l) => (
              <button
                onClick={() => setSelectedId(l.id)}
                class={`w-full text-left px-3 py-2 border-b border-zinc-800 transition-colors ${
                  selectedId() === l.id
                    ? "bg-blue-700 text-white"
                    : "hover:bg-zinc-800 text-zinc-300"
                }`}
              >
                <div class="font-medium">{l.designacao}</div>
                <div class="text-xs text-zinc-400">{l.tipo}</div>
              </button>
            )}
          </For>
        </div>
      </div>

      <div class="flex-1 flex flex-col overflow-hidden">
        <Show when={error()}>
          <div class="bg-red-900/60 border-b border-red-700 px-4 py-2 text-sm text-red-100">
            {error()}
          </div>
        </Show>

        <div class="flex gap-2 border-b border-zinc-800 bg-zinc-900 px-3 py-2">
          <button
            onClick={() => setEditing("local")}
            class={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors ${
              editing() === "local"
                ? "bg-blue-600 text-white"
                : "bg-zinc-800 text-zinc-300 hover:bg-zinc-700"
            }`}
          >
            Local
          </button>
          <button
            onClick={() => setEditing("mesas")}
            disabled={!selected()}
            class={`px-3 py-1.5 rounded-md text-sm font-medium transition-colors disabled:opacity-50 ${
              editing() === "mesas"
                ? "bg-blue-600 text-white"
                : "bg-zinc-800 text-zinc-300 hover:bg-zinc-700"
            }`}
          >
            Mesas
          </button>
        </div>

        <div class="flex-1 overflow-auto p-4">
          <Show when={selected()} fallback={<div class="text-zinc-500">Sem local seleccionado.</div>}>
            <Show when={editing() === "local"}>
              <LocalForm local={selected()!} onChange={(b) => wrap(() => api.updateLocal(selected()!.id, b))} onDelete={() => wrap(() => api.deleteLocal(selected()!.id).then(() => setSelectedId(null)))} />
            </Show>
            <Show when={editing() === "mesas"}>
              <MesasEditor
                local={selected()!}
                tables={tables()}
                onCreate={(b) => wrap(() => api.createLocalTable(selected()!.id, b))}
                onUpdate={(id, b) => wrap(() => api.updateTable(id, b))}
                onDelete={(id) => wrap(() => api.deleteTable(id))}
              />
            </Show>
          </Show>
        </div>
      </div>
    </div>
  );
}

function LocalForm(props: {
  local: Local;
  onChange: (b: Partial<Local>) => void;
  onDelete: () => void;
}) {
  const [designacao, setDesignacao] = createSignal(props.local.designacao);
  const [tipo, setTipo] = createSignal<LocalKind>(props.local.tipo);
  const [largura, setLargura] = createSignal<string>(
    props.local.largura?.toString() ?? ""
  );
  const [altura, setAltura] = createSignal<string>(
    props.local.altura?.toString() ?? ""
  );
  const [usaDesenho, setUsaDesenho] = createSignal(props.local.usa_desenho_mesas);
  const [imagem, setImagem] = createSignal<string | null>(props.local.imagem);

  const handleFile = async (file: File) => {
    const reader = new FileReader();
    reader.onload = () => {
      const result = reader.result;
      if (typeof result === "string") {
        setImagem(result);
        props.onChange({ imagem: result });
      }
    };
    reader.readAsDataURL(file);
  };

  return (
    <div class="max-w-3xl space-y-4 text-zinc-200">
      <div class="grid grid-cols-2 gap-4">
        <label class="flex flex-col text-sm font-medium gap-1">
          Designação
          <input
            class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700 focus:border-blue-500 outline-none"
            value={designacao()}
            onInput={(e) => setDesignacao(e.currentTarget.value)}
            onBlur={() => props.onChange({ designacao: designacao() })}
          />
        </label>
        <label class="flex flex-col text-sm font-medium gap-1">
          Tipo
          <select
            class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
            value={tipo()}
            onChange={(e) => {
              const v = e.currentTarget.value as LocalKind;
              setTipo(v);
              props.onChange({ tipo: v });
            }}
          >
            <For each={LOCAL_KINDS}>{(k) => <option value={k}>{k}</option>}</For>
          </select>
        </label>
      </div>

      <div class="grid grid-cols-3 gap-4 items-end">
        <label class="flex flex-col text-sm font-medium gap-1">
          Largura desenho (px)
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
            value={largura()}
            onInput={(e) => setLargura(e.currentTarget.value)}
            onBlur={() =>
              props.onChange({
                largura: largura() ? parseInt(largura(), 10) : null,
              })
            }
          />
        </label>
        <label class="flex flex-col text-sm font-medium gap-1">
          Altura desenho (px)
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-3 py-2 border border-zinc-700"
            value={altura()}
            onInput={(e) => setAltura(e.currentTarget.value)}
            onBlur={() =>
              props.onChange({
                altura: altura() ? parseInt(altura(), 10) : null,
              })
            }
          />
        </label>
        <label class="flex items-center gap-2 text-sm font-medium">
          <input
            type="checkbox"
            checked={usaDesenho()}
            onChange={(e) => {
              const v = e.currentTarget.checked;
              setUsaDesenho(v);
              props.onChange({ usa_desenho_mesas: v });
            }}
            class="w-4 h-4"
          />
          Usa desenho de mesas
        </label>
      </div>

      <div>
        <label class="flex flex-col text-sm font-medium gap-1">
          Imagem de fundo
          <input
            type="file"
            accept="image/*"
            class="text-sm text-zinc-300"
            onChange={(e) => {
              const f = e.currentTarget.files?.[0];
              if (f) handleFile(f);
            }}
          />
        </label>
        <Show when={imagem()}>
          <div class="mt-2 inline-block border border-zinc-700 rounded-md p-2 bg-zinc-900">
            <img
              src={imagem()!}
              alt="fundo"
              style={{ "max-width": "320px", "max-height": "240px" }}
            />
            <button
              class="mt-2 block text-xs text-red-400 hover:text-red-300"
              onClick={() => {
                setImagem(null);
                props.onChange({ imagem: null });
              }}
            >
              Remover imagem
            </button>
          </div>
        </Show>
      </div>

      <div class="pt-4 border-t border-zinc-800">
        <button
          onClick={props.onDelete}
          class="px-4 py-2 rounded-md bg-red-700 hover:bg-red-600 text-white text-sm font-medium"
        >
          Eliminar local
        </button>
      </div>
    </div>
  );
}

function MesasEditor(props: {
  local: Local;
  tables: Table[];
  onCreate: (b: Partial<Table> & { code: number }) => void;
  onUpdate: (id: string, b: Partial<Table>) => void;
  onDelete: (id: string) => void;
}) {
  const [pickedId, setPickedId] = createSignal<string | null>(null);
  const [dragging, setDragging] = createSignal<{
    id: string;
    offsetX: number;
    offsetY: number;
  } | null>(null);

  const nextCode = () =>
    Math.max(0, ...props.tables.map((t) => t.code)) + 1;

  const addMesa = () =>
    props.onCreate({
      code: nextCode(),
      name: `Mesa ${nextCode()}`,
      posx: 20,
      posy: 20,
      altura: 60,
      largura: 80,
    });

  const widthPx = props.local.largura ?? 800;
  const heightPx = props.local.altura ?? 600;

  const onMouseDown = (e: MouseEvent, t: Table) => {
    setPickedId(t.id);
    const target = e.currentTarget as HTMLElement;
    const rect = target.getBoundingClientRect();
    setDragging({
      id: t.id,
      offsetX: e.clientX - rect.left,
      offsetY: e.clientY - rect.top,
    });
  };

  const onMouseMove = (e: MouseEvent) => {
    const drag = dragging();
    if (!drag) return;
    const container = (e.currentTarget as HTMLElement).getBoundingClientRect();
    const x = Math.max(0, e.clientX - container.left - drag.offsetX);
    const y = Math.max(0, e.clientY - container.top - drag.offsetY);
    // optimistic visual update via DOM
    const el = document.getElementById(`mesa-${drag.id}`);
    if (el) {
      el.style.left = `${x}px`;
      el.style.top = `${y}px`;
    }
  };

  const onMouseUp = () => {
    const drag = dragging();
    if (!drag) return;
    const el = document.getElementById(`mesa-${drag.id}`);
    if (el) {
      const x = parseInt(el.style.left, 10) || 0;
      const y = parseInt(el.style.top, 10) || 0;
      props.onUpdate(drag.id, { posx: x, posy: y });
    }
    setDragging(null);
  };

  return (
    <div class="space-y-4">
      <div class="flex items-center gap-3">
        <button
          onClick={addMesa}
          class="px-3 py-1.5 rounded-md bg-emerald-600 hover:bg-emerald-500 text-white text-sm font-medium"
        >
          + Nova mesa
        </button>
        <Show when={props.local.usa_desenho_mesas}>
          <span class="text-xs text-zinc-400">
            Arrasta as mesas sobre o fundo do local para as posicionar.
          </span>
        </Show>
      </div>

      <Show
        when={props.local.usa_desenho_mesas}
        fallback={
          <TableList
            tables={props.tables}
            pickedId={pickedId()}
            setPicked={setPickedId}
            onUpdate={props.onUpdate}
            onDelete={props.onDelete}
          />
        }
      >
        <div class="flex gap-4">
          <div
            class="relative bg-zinc-800 border-2 border-zinc-700 overflow-hidden"
            style={{
              width: `${widthPx}px`,
              height: `${heightPx}px`,
              "background-image": props.local.imagem
                ? `url(${props.local.imagem})`
                : undefined,
              "background-size": "cover",
              "background-position": "center",
            }}
            onMouseMove={onMouseMove}
            onMouseUp={onMouseUp}
            onMouseLeave={onMouseUp}
          >
            <For each={props.tables}>
              {(t) => (
                <div
                  id={`mesa-${t.id}`}
                  onMouseDown={(e) => onMouseDown(e, t)}
                  class={`absolute select-none cursor-move rounded-lg border-2 flex items-center justify-center text-sm font-bold shadow-md ${
                    pickedId() === t.id
                      ? "bg-blue-600/80 border-blue-300 text-white"
                      : "bg-zinc-700/90 border-zinc-500 text-zinc-100"
                  }`}
                  style={{
                    left: `${t.posx ?? 0}px`,
                    top: `${t.posy ?? 0}px`,
                    width: `${t.largura ?? 80}px`,
                    height: `${t.altura ?? 60}px`,
                  }}
                >
                  {t.name ?? `M${t.code}`}
                </div>
              )}
            </For>
          </div>

          <Show when={pickedId() && props.tables.find((t) => t.id === pickedId())}>
            <MesaForm
              key={pickedId()!}
              table={props.tables.find((t) => t.id === pickedId())!}
              onUpdate={(b) => props.onUpdate(pickedId()!, b)}
              onDelete={() => {
                props.onDelete(pickedId()!);
                setPickedId(null);
              }}
            />
          </Show>
        </div>
      </Show>
    </div>
  );
}

function TableList(props: {
  tables: Table[];
  pickedId: string | null;
  setPicked: (id: string | null) => void;
  onUpdate: (id: string, b: Partial<Table>) => void;
  onDelete: (id: string) => void;
}) {
  return (
    <div class="flex gap-4">
      <div class="w-64 max-h-[600px] overflow-y-auto border border-zinc-800 rounded-md">
        <For each={props.tables}>
          {(t) => (
            <button
              onClick={() => props.setPicked(t.id)}
              class={`w-full text-left px-3 py-2 border-b border-zinc-800 ${
                props.pickedId === t.id
                  ? "bg-blue-700 text-white"
                  : "hover:bg-zinc-800 text-zinc-300"
              }`}
            >
              {t.name ?? `Mesa ${t.code}`}
              <span class="text-xs text-zinc-400 ml-2">#{t.code}</span>
            </button>
          )}
        </For>
      </div>
      <Show when={props.pickedId && props.tables.find((t) => t.id === props.pickedId)}>
        <MesaForm
          key={props.pickedId!}
          table={props.tables.find((t) => t.id === props.pickedId)!}
          onUpdate={(b) => props.onUpdate(props.pickedId!, b)}
          onDelete={() => {
            props.onDelete(props.pickedId!);
            props.setPicked(null);
          }}
        />
      </Show>
    </div>
  );
}

function MesaForm(props: {
  key: string;
  table: Table;
  onUpdate: (b: Partial<Table>) => void;
  onDelete: () => void;
}) {
  const [name, setName] = createSignal(props.table.name ?? "");
  const [code, setCode] = createSignal(props.table.code.toString());
  const [posx, setPosx] = createSignal(props.table.posx?.toString() ?? "");
  const [posy, setPosy] = createSignal(props.table.posy?.toString() ?? "");
  const [largura, setLargura] = createSignal(
    props.table.largura?.toString() ?? ""
  );
  const [altura, setAltura] = createSignal(props.table.altura?.toString() ?? "");

  const parseNum = (s: string): number | null =>
    s === "" ? null : parseInt(s, 10);

  return (
    <div class="w-72 bg-zinc-900 border border-zinc-800 rounded-md p-3 space-y-3 text-zinc-200">
      <h3 class="font-bold text-zinc-100 text-sm border-b border-zinc-800 pb-2">
        Mesa #{props.table.code}
      </h3>
      <label class="flex flex-col gap-1 text-xs font-medium">
        Nome
        <input
          class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
          value={name()}
          onInput={(e) => setName(e.currentTarget.value)}
          onBlur={() => props.onUpdate({ name: name() || null })}
        />
      </label>
      <label class="flex flex-col gap-1 text-xs font-medium">
        Código
        <input
          type="number"
          class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
          value={code()}
          onInput={(e) => setCode(e.currentTarget.value)}
          onBlur={() => props.onUpdate({ code: parseInt(code(), 10) || 0 })}
        />
      </label>
      <div class="grid grid-cols-2 gap-2">
        <label class="flex flex-col gap-1 text-xs font-medium">
          x
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
            value={posx()}
            onInput={(e) => setPosx(e.currentTarget.value)}
            onBlur={() => props.onUpdate({ posx: parseNum(posx()) })}
          />
        </label>
        <label class="flex flex-col gap-1 text-xs font-medium">
          y
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
            value={posy()}
            onInput={(e) => setPosy(e.currentTarget.value)}
            onBlur={() => props.onUpdate({ posy: parseNum(posy()) })}
          />
        </label>
        <label class="flex flex-col gap-1 text-xs font-medium">
          largura
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
            value={largura()}
            onInput={(e) => setLargura(e.currentTarget.value)}
            onBlur={() => props.onUpdate({ largura: parseNum(largura()) })}
          />
        </label>
        <label class="flex flex-col gap-1 text-xs font-medium">
          altura
          <input
            type="number"
            class="bg-zinc-800 rounded-md px-2 py-1 border border-zinc-700"
            value={altura()}
            onInput={(e) => setAltura(e.currentTarget.value)}
            onBlur={() => props.onUpdate({ altura: parseNum(altura()) })}
          />
        </label>
      </div>
      <button
        onClick={props.onDelete}
        class="w-full px-3 py-1.5 rounded-md bg-red-700 hover:bg-red-600 text-white text-xs font-medium"
      >
        Eliminar mesa
      </button>
    </div>
  );
}
