import { createSignal, onMount, For } from "solid-js";
import "./App.css";

interface Article {
  id: string;
  name: string;
  price: number;
}

interface OrderLine {
  article: Article;
  qty: number;
}

function App() {
  const [articles, setArticles] = createSignal<Article[]>([]);
  const [order, setOrder] = createSignal<OrderLine[]>([]);

  onMount(async () => {
    // Phase 1 Mock fetch from API
    try {
      const res = await fetch("http://localhost:3000/api/catalog");
      const data = await res.json();
      setArticles(data.articles);
    } catch (e) {
      console.warn("API not reachable yet, using fallback data.");
      setArticles([
        { id: "1", name: "Café Expresso", price: 80 },
        { id: "2", name: "Galão", price: 150 },
        { id: "3", name: "Tosta Mista", price: 300 },
      ]);
    }
  });

  const addToOrder = (article: Article) => {
    setOrder((prev) => {
      const existing = prev.find((line) => line.article.id === article.id);
      if (existing) {
        return prev.map((line) =>
          line.article.id === article.id
            ? { ...line, qty: line.qty + 1 }
            : line
        );
      }
      return [...prev, { article, qty: 1 }];
    });
  };

  const submitOrder = async () => {
    const lines = order().map(l => ({
      article_id: l.article.id,
      qty: l.qty,
      unit_price: l.article.price,
    }));
    
    try {
      const res = await fetch("http://localhost:3000/api/orders", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          table_id: "00000000-0000-0000-0000-000000000000",
          employee_id: "00000000-0000-0000-0000-000000000000",
          lines,
        }),
      });
      if (res.ok) {
        alert("Pedido registado (Fase 1)!");
        setOrder([]);
      }
    } catch (e) {
      alert("Falha ao comunicar com o servidor.");
    }
  };

  const total = () => order().reduce((acc, curr) => acc + curr.article.price * curr.qty, 0);

  return (
    <div class="flex h-full w-full bg-zinc-900 text-white select-none">
      {/* Sidebar */}
      <div class="w-24 bg-zinc-800 flex flex-col items-center py-4 gap-4 border-r border-zinc-700 shadow-xl z-10">
        <button class="w-16 h-16 rounded-xl bg-blue-600 hover:bg-blue-500 transition-colors shadow-lg flex items-center justify-center font-semibold text-sm">
          Pedidos
        </button>
        <button class="w-16 h-16 rounded-xl bg-zinc-700 hover:bg-zinc-600 transition-colors flex items-center justify-center font-semibold text-sm text-zinc-300">
          Mesas
        </button>
        <button class="w-16 h-16 rounded-xl bg-zinc-700 hover:bg-zinc-600 transition-colors flex items-center justify-center font-semibold text-sm text-zinc-300">
          Caixa
        </button>
      </div>

      {/* Main Content Area */}
      <div class="flex-1 flex flex-col h-full bg-zinc-950">
        {/* Top Info Bar */}
        <div class="h-12 bg-zinc-800 border-b border-zinc-700 flex items-center justify-between px-6">
          <div class="font-medium text-zinc-400">Empregado 1 - Terminal 1</div>
          <div class="text-zinc-400 font-mono text-sm">{new Date().toLocaleTimeString()}</div>
        </div>

        <div class="flex-1 flex overflow-hidden">
          {/* Order / Receipt Column */}
          <div class="w-80 bg-zinc-900 border-r border-zinc-700 flex flex-col relative">
            <div class="p-4 border-b border-zinc-800 bg-zinc-900 sticky top-0">
              <h2 class="text-xl font-bold text-zinc-200">Pedido Actual</h2>
            </div>
            
            <div class="flex-1 overflow-y-auto p-4 space-y-2">
              <For each={order()}>
                {(line) => (
                  <div class="flex justify-between items-center py-2 px-3 bg-zinc-800/50 rounded-lg">
                    <div class="flex items-center gap-3">
                      <span class="text-sm font-bold w-6 text-center text-blue-400">{line.qty}x</span>
                      <span class="text-sm text-zinc-200 truncate">{line.article.name}</span>
                    </div>
                    <span class="text-sm font-mono text-zinc-300">
                      {((line.article.price * line.qty) / 100).toFixed(2)}€
                    </span>
                  </div>
                )}
              </For>
            </div>

            {/* Total & Action */}
            <div class="p-4 bg-zinc-800 border-t border-zinc-700 mt-auto">
              <div class="flex justify-between items-end mb-4">
                <span class="text-zinc-400 font-medium">Total</span>
                <span class="text-3xl font-bold tracking-tight text-white font-mono">
                  {(total() / 100).toFixed(2)}€
                </span>
              </div>
              <button
                onClick={submitOrder}
                disabled={order().length === 0}
                class="w-full py-4 rounded-xl font-bold text-lg transition-colors shadow-lg active:scale-[0.98]
                       bg-emerald-500 hover:bg-emerald-400 text-zinc-950 disabled:opacity-50 disabled:pointer-events-none"
              >
                PAGAR
              </button>
            </div>
          </div>

          {/* Catalog Grid */}
          <div class="flex-1 p-6 overflow-y-auto bg-zinc-950">
            <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-4">
              <For each={articles()}>
                {(article) => (
                  <button
                    onClick={() => addToOrder(article)}
                    class="aspect-square rounded-2xl bg-zinc-800 border border-zinc-700 hover:border-blue-500 hover:bg-zinc-700 
                           transition-all p-4 flex flex-col justify-between items-start text-left shadow-md group relative overflow-hidden active:scale-95"
                  >
                    <div class="absolute inset-0 bg-gradient-to-br from-blue-500/10 to-transparent opacity-0 group-hover:opacity-100 transition-opacity"></div>
                    <span class="text-lg font-bold text-zinc-100 leading-tight relative z-10">{article.name}</span>
                    <span class="text-sm font-mono text-blue-400 relative z-10">
                      {(article.price / 100).toFixed(2)}€
                    </span>
                  </button>
                )}
              </For>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
