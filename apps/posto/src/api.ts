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
  price: number;
  vat_rate: number;
  created_at: string;
  updated_at: string;
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
}

export interface Payment {
  id: string;
  document_id: string;
  payment_method_id: string;
  amount: number;
  created_at: string;
}

export interface DocumentResponse {
  document: DocumentDto;
  lines: DocumentDetail[];
  payments: Payment[];
}

export interface CatalogResponse {
  families: Family[];
  articles: Article[];
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

  closeDocument: (documentId: string, paymentMethodId: string | null) =>
    jsonReq<DocumentResponse>(`/api/documents/${documentId}/close`, {
      method: "POST",
      body: JSON.stringify({ payment_method_id: paymentMethodId }),
    }),

  printDocument: async (documentId: string): Promise<string> => {
    const res = await fetch(`${BASE}/api/documents/${documentId}/print`, {
      method: "POST",
    });
    if (!res.ok) throw new Error(`print failed: ${res.status}`);
    return res.text();
  },
};
