import { createSignal, For, Show, onMount, createMemo } from "solid-js";
import { api, DocumentTemplate } from "./api";

// Editor de documentos configuráveis (cabeçalho / linha de detalhe / rodapé)
// com flags (\xx) e a construção <! type="..." ... !>. Renderizados no momento
// da impressão pelo motor `devices::template` no backend.

// Referência rápida das flags mais usadas, agrupadas para o painel lateral.
const FLAG_REF: { grupo: string; flags: [string, string][] }[] = [
  {
    grupo: "Casa",
    flags: [
      ["\\no", "Nome comercial"],
      ["\\ds", "Designação social"],
      ["\\mo", "Morada"],
      ["\\lo", "Localidade"],
      ["\\cp", "Código postal"],
      ["\\nc", "NIF"],
      ["\\cs", "Capital social"],
    ],
  },
  {
    grupo: "Documento",
    flags: [
      ["\\nx", "Série/Número"],
      ["\\nd", "Nº documento"],
      ["\\ns", "Série"],
      ["\\atcud", "ATCUD"],
      ["\\hash", "Hash"],
      ["\\dt", "Data"],
      ["\\ho", "Hora HH:MM:SS"],
      ["\\hc", "Hora HH:MM"],
    ],
  },
  {
    grupo: "Cliente / Empregado / Mesa",
    flags: [
      ["\\ol", "Nome cliente"],
      ["\\cl", "NIF cliente"],
      ["\\oe", "Nome empregado"],
      ["\\om", "Nome mesa"],
      ["\\np", "Nº pessoas"],
    ],
  },
  {
    grupo: "Valores",
    flags: [
      ["\\vt", "Total"],
      ["\\st", "Sub-total"],
      ["\\sx", "Total sem IVA"],
      ["\\tx", "IVA total"],
      ["\\pp", "Valor por pessoa"],
      ["\\pg", "Valor pago"],
      ["\\tr", "Troco"],
      ["\\fp", "Forma pagamento"],
    ],
  },
  {
    grupo: "Blocos / Formatação",
    flags: [
      ["\\ti", "Tabela de IVA"],
      ["\\qr", "QR Code"],
      ["\\s7", "Centrar linha"],
      ["\\s8", "Alinhar à direita"],
      ["\\s9", "Alinhar à esquerda"],
    ],
  },
  {
    grupo: "Detalhe (linha)",
    flags: [
      ["fb_d_qtd", "Quantidade"],
      ["fb_d_design", "Designação"],
      ["fb_d_punit", "Preço unitário"],
      ["fb_d_iva_perc", "Taxa IVA"],
      ["fb_d_total_linha", "Total da linha"],
    ],
  },
];

export function DocumentTemplatesView() {
  const [items, setItems] = createSignal<DocumentTemplate[]>([]);
  const [selectedTipo, setSelectedTipo] = createSignal<string | null>(null);
  const [busy, setBusy] = createSignal(false);
  const [error, setError] = createSignal<string | null>(null);
  const [info, setInfo] = createSignal<string | null>(null);

  // Cópia editável dos campos.
  const [designacao, setDesignacao] = createSignal("");
  const [cabecalho, setCabecalho] = createSignal("");
  const [linhaDetalhe, setLinhaDetalhe] = createSignal("");
  const [rodape, setRodape] = createSignal("");
  const [naoImprimeDetalhes, setNaoImprimeDetalhes] = createSignal(false);
  const [largura, setLargura] = createSignal(48);

  const selected = createMemo<DocumentTemplate | undefined>(() =>
    items().find((t) => t.tipo_documento === selectedTipo())
  );

  const loadInto = (t: DocumentTemplate) => {
    setSelectedTipo(t.tipo_documento);
    setDesignacao(t.designacao);
    setCabecalho(t.cabecalho);
    setLinhaDetalhe(t.linha_detalhe);
    setRodape(t.rodape);
    setNaoImprimeDetalhes(t.nao_imprime_detalhes);
    setLargura(t.largura);
    setInfo(null);
    setError(null);
  };

  const refresh = async (keepTipo?: string) => {
    setError(null);
    try {
      const list = await api.documentTemplates();
      setItems(list);
      const tipo = keepTipo ?? selectedTipo() ?? list[0]?.tipo_documento ?? null;
      const t = list.find((x) => x.tipo_documento === tipo) ?? list[0];
      if (t) loadInto(t);
    } catch (e: any) {
      setError(e.message ?? String(e));
    }
  };

  onMount(() => refresh());

  const save = async () => {
    const tipo = selectedTipo();
    if (!tipo) return;
    setBusy(true);
    setError(null);
    setInfo(null);
    try {
      await api.updateDocumentTemplate(tipo, {
        designacao: designacao(),
        cabecalho: cabecalho(),
        linha_detalhe: linhaDetalhe(),
        rodape: rodape(),
        nao_imprime_detalhes: naoImprimeDetalhes(),
        largura: largura(),
      });
      setInfo("Template guardado.");
      await refresh(tipo);
    } catch (e: any) {
      setError(e.message ?? String(e));
    } finally {
      setBusy(false);
    }
  };

  const dirty = createMemo(() => {
    const t = selected();
    if (!t) return false;
    return (
      t.designacao !== designacao() ||
      t.cabecalho !== cabecalho() ||
      t.linha_detalhe !== linhaDetalhe() ||
      t.rodape !== rodape() ||
      t.nao_imprime_detalhes !== naoImprimeDetalhes() ||
      t.largura !== largura()
    );
  });

  const taClass =
    "w-full bg-zinc-950 border border-zinc-700 rounded-lg p-3 font-mono text-xs text-zinc-100 resize-y focus:outline-none focus:border-blue-500";

  return (
    <div class="flex-1 flex overflow-hidden bg-zinc-950 text-zinc-100">
      {/* Lista de templates */}
      <div class="w-56 bg-zinc-900 border-r border-zinc-800 flex flex-col">
        <div class="p-3 border-b border-zinc-800">
          <span class="text-sm font-bold text-zinc-200">Documentos</span>
        </div>
        <div class="flex-1 overflow-y-auto">
          <For each={items()}>
            {(t) => (
              <button
                onClick={() => loadInto(t)}
                class={`w-full text-left px-3 py-2 text-sm border-b border-zinc-800/60 transition-colors ${
                  t.tipo_documento === selectedTipo()
                    ? "bg-blue-600/20 text-blue-200"
                    : "hover:bg-zinc-800 text-zinc-300"
                }`}
              >
                <div class="font-semibold">{t.designacao}</div>
                <div class="text-[11px] text-zinc-500">{t.tipo_documento}</div>
              </button>
            )}
          </For>
        </div>
      </div>

      {/* Editor */}
      <div class="flex-1 flex flex-col overflow-hidden">
        <div class="px-6 py-4 border-b border-zinc-800 flex items-center justify-between bg-zinc-900">
          <div>
            <h1 class="text-2xl font-bold">Documentos configuráveis</h1>
            <p class="text-sm text-zinc-400">
              Cabeçalho, linha de detalhe e rodapé com flags. Largura em colunas.
            </p>
          </div>
          <div class="flex items-center gap-3">
            <Show when={info()}>
              <span class="text-sm text-emerald-400">{info()}</span>
            </Show>
            <Show when={error()}>
              <span class="text-sm text-red-400">{error()}</span>
            </Show>
            <button
              onClick={save}
              disabled={busy() || !dirty()}
              class="px-4 py-2 rounded-lg bg-blue-600 hover:bg-blue-500 text-sm font-semibold disabled:opacity-40 disabled:cursor-not-allowed"
            >
              {busy() ? "A guardar..." : "Guardar"}
            </button>
          </div>
        </div>

        <Show
          when={selected()}
          fallback={
            <div class="flex-1 flex items-center justify-center text-zinc-500">
              Selecciona um documento.
            </div>
          }
        >
          <div class="flex-1 flex overflow-hidden">
            <div class="flex-1 overflow-y-auto p-6 space-y-4">
              <div class="flex gap-4">
                <label class="flex-1">
                  <span class="block text-xs font-semibold text-zinc-400 mb-1">
                    Designação
                  </span>
                  <input
                    value={designacao()}
                    onInput={(e) => setDesignacao(e.currentTarget.value)}
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
                  />
                </label>
                <label class="w-32">
                  <span class="block text-xs font-semibold text-zinc-400 mb-1">
                    Largura
                  </span>
                  <input
                    type="number"
                    min="20"
                    max="120"
                    value={largura()}
                    onInput={(e) =>
                      setLargura(parseInt(e.currentTarget.value || "48", 10))
                    }
                    class="w-full bg-zinc-950 border border-zinc-700 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-blue-500"
                  />
                </label>
                <label class="flex items-end gap-2 pb-2">
                  <input
                    type="checkbox"
                    checked={naoImprimeDetalhes()}
                    onChange={(e) =>
                      setNaoImprimeDetalhes(e.currentTarget.checked)
                    }
                    class="w-4 h-4 accent-blue-600"
                  />
                  <span class="text-sm text-zinc-300">Só total</span>
                </label>
              </div>

              <div>
                <span class="block text-xs font-semibold text-zinc-400 mb-1">
                  Cabeçalho
                </span>
                <textarea
                  rows="9"
                  value={cabecalho()}
                  onInput={(e) => setCabecalho(e.currentTarget.value)}
                  class={taClass}
                />
              </div>

              <div>
                <span class="block text-xs font-semibold text-zinc-400 mb-1">
                  Linha de detalhe{" "}
                  <span class="text-zinc-600">
                    (renderizada uma vez por artigo)
                  </span>
                </span>
                <textarea
                  rows="3"
                  value={linhaDetalhe()}
                  onInput={(e) => setLinhaDetalhe(e.currentTarget.value)}
                  class={`${taClass} ${naoImprimeDetalhes() ? "opacity-40" : ""}`}
                  disabled={naoImprimeDetalhes()}
                />
              </div>

              <div>
                <span class="block text-xs font-semibold text-zinc-400 mb-1">
                  Rodapé
                </span>
                <textarea
                  rows="9"
                  value={rodape()}
                  onInput={(e) => setRodape(e.currentTarget.value)}
                  class={taClass}
                />
              </div>
            </div>

            {/* Referência de flags */}
            <div class="w-64 border-l border-zinc-800 overflow-y-auto p-4 bg-zinc-900/40">
              <div class="text-xs font-bold text-zinc-300 mb-3 uppercase tracking-wide">
                Flags
              </div>
              <For each={FLAG_REF}>
                {(g) => (
                  <div class="mb-4">
                    <div class="text-[11px] font-semibold text-zinc-500 mb-1">
                      {g.grupo}
                    </div>
                    <For each={g.flags}>
                      {([flag, desc]) => (
                        <div class="flex justify-between gap-2 text-[11px] py-0.5">
                          <code class="text-blue-300">{flag}</code>
                          <span class="text-zinc-400 text-right">{desc}</span>
                        </div>
                      )}
                    </For>
                  </div>
                )}
              </For>
              <div class="text-[11px] text-zinc-500 mt-2 leading-relaxed">
                Campos de detalhe via{" "}
                <code class="text-blue-300">
                  {'<! type="field" id="fb_d_qtd" mask="###" align="right" !>'}
                </code>
                .
              </div>
            </div>
          </div>
        </Show>
      </div>
    </div>
  );
}
