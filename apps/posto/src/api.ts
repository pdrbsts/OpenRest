export interface Family {
  id: string;
  parent_id: string | null;
  code: number;
  name: string;
}

export interface Article {
  id: string;
  family_id: string | null;
  code: number;
  name: string;
  pvp1: number;
  pvp2: number | null;
  pvp3: number | null;
  pvp4: number | null;
  pvp5: number | null;
  vat_rate: number;
  tipo_artigo: string;
  zona_impressao_id: string | null;
  created_at: string;
  updated_at: string;
}

export interface ZonaImpressao {
  id: string;
  codigo: number;
  designacao: string;
  secundarios: boolean;
  anulado_em: string | null;
}

export interface Dispositivo {
  id: string;
  nome: string;
  tipo: string;
  modelo: string | null;
  descricao: string | null;
  output_path: string | null;
  ativo: boolean;
  anulado_em: string | null;
}

export interface PrintMapping {
  id: string;
  zona_impressao_id: string;
  local_id: string;
  origem_id: string | null;
  dispositivo_id: string;
  agrupamento: string;
  numero_copias: number;
}

export function pvpFor(article: Article, codigo: number | null): number {
  if (codigo == null || codigo === 1) return article.pvp1;
  const map = [article.pvp2, article.pvp3, article.pvp4, article.pvp5];
  const v = map[codigo - 2];
  return v == null ? article.pvp1 : v;
}

export interface TipoPreco {
  id: string;
  codigo: number;
  designacao: string;
}

export interface Zona {
  id: string;
  codigo: number | null;
  designacao: string;
  taxa_entrega: number;
  rede_remota_associada_id: string | null;
  anulado_em: string | null;
}

export interface Entregador {
  id: string;
  nome: string;
  telefone: string | null;
  externo: boolean;
  ativo: boolean;
  anulado_em: string | null;
}

export type MesaEstadoKind =
  | "livre"
  | "aberta"
  | "em_espera"
  | "reservada"
  | "bloqueada";

export interface MesaEstado {
  mesa_id: string;
  estado: MesaEstadoKind;
  bloqueada_por_posto_id: string | null;
  bloqueada_motivo: string | null;
  cliente_associado_id: string | null;
  numero_pessoas: number | null;
  empregado_actual_id: string | null;
  aberta_em: string | null;
  subtotal_actual: number;
  reservada_ate: string | null;
  reserva_pessoas: number | null;
  reserva_cliente_id: string | null;
  reserva_observacoes: string | null;
}

export type LocalKind =
  | "normal"
  | "take_away"
  | "take_away_seguro"
  | "pub"
  | "delivery"
  | "consumo_proprio"
  | "restauracao_colectiva";

export interface Local {
  id: string;
  designacao: string;
  mesas_definicao: string | null;
  tipo: LocalKind;
  nome_generico_mesa: string;
  usa_desenho_mesas: boolean;
  imagem: string | null;
  largura: number | null;
  altura: number | null;
  permite_mesas_abertas_fim_do_dia: boolean;
  permite_zero_pessoas: boolean;
  tipo_preco_id: string | null;
}

export interface Table {
  id: string;
  local_id: string | null;
  code: number;
  name: string | null;
  nomeobjecto: string | null;
  posx: number | null;
  posy: number | null;
  imagem: string | null;
  fntname: string | null;
  fntsize: number | null;
  fntcolor: string | null;
  fontx: number | null;
  fonty: number | null;
  fontstyle: string | null;
  estadox: number | null;
  estadoy: number | null;
  reservax: number | null;
  reservay: number | null;
  altura: number | null;
  largura: number | null;
  criada_em: string | null;
  estado: MesaEstado;
}

export interface Employee {
  id: string;
  code: number;
  name: string;
  perc_consumo: number;
  base_consumo: number;
}

export interface Customer {
  id: string;
  codigo: number | null;
  nome: string;
  nif: string | null;
  pais: string;
  telefone: string | null;
  morada: string | null;
  cod_postal: string | null;
  localidade: string | null;
  email: string | null;
  observacoes: string | null;
  numero_cartao: string | null;
  limite_credito: number;
  zona_id: string | null;
  anulado_em: string | null;
  esquecido_em: string | null;
}

export interface CustomerResponse extends Customer {
  nif_warning: string | null;
}

export interface CustomerInput {
  nome?: string;
  nif?: string | null;
  pais?: string;
  telefone?: string | null;
  morada?: string | null;
  cod_postal?: string | null;
  localidade?: string | null;
  email?: string | null;
  observacoes?: string | null;
  zona_id?: string | null;
}


export type DeliveryEstado =
  | "recebido"
  | "em_preparacao"
  | "pronto"
  | "despachado"
  | "entregue"
  | "cancelado";

export interface PedidoDelivery {
  id: string;
  document_id: string;
  cliente_id: string | null;
  morada_snapshot: string | null;
  telefone_snapshot: string | null;
  recebido_em: string;
  recebido_via: string;
  entregador_id: string | null;
  pronto_em: string | null;
  despachado_em: string | null;
  entregue_em: string | null;
  estado: DeliveryEstado;
  zona_id: string | null;
  taxa_entrega_cents: number;
}

export interface PaymentMethod {
  id: string;
  code: number;
  name: string;
}

export interface DocumentDetail {
  id: string;
  document_id: string;
  article_id: string;
  qty: number;
  unit_price: number;
  total: number;
  pedida_em: string | null;
  anulada: boolean;
  anulada_com_desperdicio: boolean;
  anulada_em: string | null;
  anulada_por: string | null;
  anulada_motivo: string | null;
}

export interface DocumentDto {
  id: string;
  table_id: string | null;
  employee_id: string | null;
  total: number;
  is_closed: boolean;
  created_at: string;
  series_id: string | null;
  document_type: string | null;
  document_number: number | null;
  atcud: string | null;
  hash: string | null;
  hash_short: string | null;
  previous_hash: string | null;
  issued_at: string | null;
  qr_payload: string | null;
  customer_id: string | null;
  local_id: string | null;
  observacoes_pedido: string | null;
  observacoes_factura: string | null;
  observacoes_cliente: string | null;
  observacoes_morada: string | null;
  delivery_morada: string | null;
  delivery_telefone: string | null;
  subtotal_impresso_em?: string | null;
  data_dia: string | null;
  troco_cents: number;
}

export interface Payment {
  id: string;
  document_id: string;
  payment_method_id: string;
  amount: number;
  descricao: string | null;
  created_at: string;
}

export interface PaymentLineInput {
  payment_method_id: string;
  amount: number;
  descricao?: string | null;
}

export interface DocumentResponse {
  document: DocumentDto;
  lines: DocumentDetail[];
  payments: Payment[];
}

export interface CurrentDayResponse {
  data_dia: string;
  server_now: string;
  cutoff_minutes: number;
  tz_offset_minutes: number;
}

export interface CatalogResponse {
  families: Family[];
  articles: Article[];
}

export interface Transferencia {
  id: string;
  from_document_id: string;
  to_document_id: string;
  line_id: string;
  article_id: string;
  qty: number;
  employee_id: string | null;
  transferida_em: string;
}

export interface TransferResponse {
  from_document: DocumentResponse;
  to_document: DocumentResponse;
  transferencias: Transferencia[];
}

const BASE = "http://localhost:3000";

async function jsonReq<T>(path: string, init?: RequestInit): Promise<T> {
  const res = await fetch(`${BASE}${path}`, {
    headers: { "Content-Type": "application/json" },
    ...init,
  });
  if (!res.ok) {
    let msg = res.statusText;
    try {
      const body = await res.json();
      if (body?.error) msg = body.error;
    } catch (_) {}
    throw new Error(`${res.status} ${msg}`);
  }
  if (res.status === 204) return undefined as unknown as T;
  return res.json() as Promise<T>;
}

export const api = {
  catalog: () => jsonReq<CatalogResponse>("/api/catalog"),
  currentDay: () => jsonReq<CurrentDayResponse>("/api/system/current-day"),
  locais: () => jsonReq<Local[]>("/api/locais"),
  createLocal: (body: Partial<Local> & { designacao: string; tipo: LocalKind }) =>
    jsonReq<Local>("/api/locais", { method: "POST", body: JSON.stringify(body) }),
  updateLocal: (id: string, body: Partial<Local>) =>
    jsonReq<Local>(`/api/locais/${id}`, { method: "PUT", body: JSON.stringify(body) }),
  deleteLocal: (id: string) =>
    fetch(`http://localhost:3000/api/locais/${id}`, { method: "DELETE" }).then(
      (r) => {
        if (!r.ok) throw new Error(`delete failed: ${r.status}`);
      }
    ),
  localTables: (localId: string) =>
    jsonReq<Table[]>(`/api/locais/${localId}/tables`),
  createLocalTable: (localId: string, body: Partial<Table> & { code: number }) =>
    jsonReq<Table>(`/api/locais/${localId}/tables`, {
      method: "POST",
      body: JSON.stringify(body),
    }),
  updateTable: (id: string, body: Partial<Table>) =>
    jsonReq<Table>(`/api/tables/${id}`, { method: "PUT", body: JSON.stringify(body) }),
  deleteTable: (id: string) =>
    fetch(`http://localhost:3000/api/tables/${id}`, { method: "DELETE" }).then(
      (r) => {
        if (!r.ok) throw new Error(`delete failed: ${r.status}`);
      }
    ),
  tables: () => jsonReq<Table[]>("/api/tables"),
  employees: () => jsonReq<Employee[]>("/api/employees"),
  paymentMethods: () => jsonReq<PaymentMethod[]>("/api/payment-methods"),

  customers: () => jsonReq<Customer[]>("/api/customers"),
  searchCustomers: (params: { phone?: string; name?: string }) => {
    const qs = new URLSearchParams();
    if (params.phone) qs.set("phone", params.phone);
    if (params.name) qs.set("name", params.name);
    return jsonReq<Customer[]>(`/api/customers/search?${qs.toString()}`);
  },
  createCustomer: (body: CustomerInput & { nome: string }) =>
    jsonReq<CustomerResponse>("/api/customers", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  updateCustomer: (id: string, body: CustomerInput) =>
    jsonReq<CustomerResponse>(`/api/customers/${id}`, {
      method: "PUT",
      body: JSON.stringify(body),
    }),
  forgetCustomer: (id: string) =>
    jsonReq<Customer>(`/api/customers/${id}/forget`, { method: "POST" }),

  startLocalDocument: (
    localId: string,
    body: {
      employee_id?: string | null;
      customer_id?: string | null;
      observacoes_pedido?: string | null;
    }
  ) =>
    jsonReq<DocumentResponse>(`/api/locais/${localId}/start-document`, {
      method: "POST",
      body: JSON.stringify(body),
    }),

  openConsumo: (localId: string, employeeId: string) =>
    jsonReq<DocumentResponse>(`/api/locais/${localId}/consumo`, {
      method: "POST",
      body: JSON.stringify({ employee_id: employeeId }),
    }),

  setDocumentContext: (
    documentId: string,
    body: Partial<Pick<DocumentDto, "customer_id" | "observacoes_pedido" | "observacoes_factura" | "observacoes_cliente" | "observacoes_morada" | "delivery_morada" | "delivery_telefone">>
  ) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/context`, {
      method: "POST",
      body: JSON.stringify(body),
    }),

  activeDeliveries: () => jsonReq<PedidoDelivery[]>("/api/deliveries"),
  updateDeliveryState: (
    id: string,
    estado: DeliveryEstado,
    entregadorId?: string | null
  ) =>
    jsonReq<PedidoDelivery>(`/api/deliveries/${id}/state`, {
      method: "POST",
      body: JSON.stringify({ estado, entregador_id: entregadorId ?? null }),
    }),

  tiposPreco: () => jsonReq<TipoPreco[]>("/api/tipos-preco"),
  zonas: () => jsonReq<Zona[]>("/api/zonas"),
  createZona: (body: { designacao: string; codigo?: number | null; taxa_entrega?: number }) =>
    jsonReq<Zona>("/api/zonas", { method: "POST", body: JSON.stringify(body) }),
  updateZona: (id: string, body: Partial<Zona>) =>
    jsonReq<Zona>(`/api/zonas/${id}`, { method: "PUT", body: JSON.stringify(body) }),
  deleteZona: (id: string) =>
    fetch(`http://localhost:3000/api/zonas/${id}`, { method: "DELETE" }).then(
      (r) => {
        if (!r.ok) throw new Error(`delete failed: ${r.status}`);
      }
    ),
  entregadores: () => jsonReq<Entregador[]>("/api/entregadores"),
  createEntregador: (body: { nome: string; telefone?: string | null; externo?: boolean }) =>
    jsonReq<Entregador>("/api/entregadores", { method: "POST", body: JSON.stringify(body) }),
  updateEntregador: (id: string, body: Partial<Entregador>) =>
    jsonReq<Entregador>(`/api/entregadores/${id}`, {
      method: "PUT",
      body: JSON.stringify(body),
    }),
  deleteEntregador: (id: string) =>
    fetch(`http://localhost:3000/api/entregadores/${id}`, { method: "DELETE" }).then(
      (r) => {
        if (!r.ok) throw new Error(`delete failed: ${r.status}`);
      }
    ),

  openTable: (tableId: string, employeeId: string | null) =>
    jsonReq<DocumentResponse>(`/api/tables/${tableId}/open`, {
      method: "POST",
      body: JSON.stringify({ employee_id: employeeId }),
    }),

  tableDocument: (tableId: string) =>
    jsonReq<DocumentResponse | null>(`/api/tables/${tableId}/document`),

  document: (documentId: string) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}`),

  addLine: (documentId: string, articleId: string, qty: number) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/lines`, {
      method: "POST",
      body: JSON.stringify({ article_id: articleId, qty }),
    }),

  cancelLine: (
    documentId: string,
    lineId: string,
    body?: { motivo?: string | null; employee_id?: string | null }
  ) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/lines/${lineId}`, {
      method: "DELETE",
      body: JSON.stringify(body ?? {}),
    }),

  anularLine: (
    documentId: string,
    lineId: string,
    body: {
      com_desperdicio: boolean;
      motivo?: string | null;
      employee_id?: string | null;
    }
  ) =>
    jsonReq<DocumentResponse>(
      `/api/documents/${documentId}/lines/${lineId}/anular`,
      {
        method: "POST",
        body: JSON.stringify(body),
      }
    ),

  transferDocument: (
    documentId: string,
    body: {
      target_table_id: string;
      line_ids?: string[] | null;
      employee_id?: string | null;
    }
  ) =>
    jsonReq<TransferResponse>(`/api/documents/${documentId}/transfer`, {
      method: "POST",
      body: JSON.stringify(body),
    }),

  closeDocument: (documentId: string, paymentMethodId: string | null) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/close`, {
      method: "POST",
      body: JSON.stringify({ payment_method_id: paymentMethodId }),
    }),

  /**
   * Fecha um documento com 1..N rodapés de pagamento (janela Avançada).
   * Servidor valida soma >= total e regista troco em `Document.troco_cents`
   * quando soma > total. Útil para pagamento misto (e.g., metade dinheiro,
   * metade Visa).
   */
  closeDocumentMulti: (documentId: string, payments: PaymentLineInput[]) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/close`, {
      method: "POST",
      body: JSON.stringify({ payments }),
    }),

  printDocument: async (documentId: string): Promise<string> => {
    const res = await fetch(`${BASE}/api/documents/${documentId}/print`, {
      method: "POST",
    });
    if (!res.ok) throw new Error(`print failed: ${res.status}`);
    return res.text();
  },

  pedirDocument: (documentId: string) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/pedir`, {
      method: "POST",
    }),

  zonasImpressao: () => jsonReq<ZonaImpressao[]>("/api/zonas-impressao"),
  createZonaImpressao: (body: { codigo: number; designacao: string; secundarios?: boolean }) =>
    jsonReq<ZonaImpressao>("/api/zonas-impressao", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  deleteZonaImpressao: (id: string) =>
    fetch(`${BASE}/api/zonas-impressao/${id}`, { method: "DELETE" }).then((r) => {
      if (!r.ok) throw new Error(`delete failed: ${r.status}`);
    }),

  dispositivos: () => jsonReq<Dispositivo[]>("/api/dispositivos"),
  createDispositivo: (body: { nome: string; tipo?: string; output_path?: string | null }) =>
    jsonReq<Dispositivo>("/api/dispositivos", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  deleteDispositivo: (id: string) =>
    fetch(`${BASE}/api/dispositivos/${id}`, { method: "DELETE" }).then((r) => {
      if (!r.ok) throw new Error(`delete failed: ${r.status}`);
    }),

  printMappings: () => jsonReq<PrintMapping[]>("/api/print-mappings"),
  createPrintMapping: (body: {
    zona_impressao_id: string;
    local_id: string;
    dispositivo_id: string;
    agrupamento?: string;
    numero_copias?: number;
  }) =>
    jsonReq<PrintMapping>("/api/print-mappings", {
      method: "POST",
      body: JSON.stringify(body),
    }),
  deletePrintMapping: (id: string) =>
    fetch(`${BASE}/api/print-mappings/${id}`, { method: "DELETE" }).then((r) => {
      if (!r.ok) throw new Error(`delete failed: ${r.status}`);
    }),
};
