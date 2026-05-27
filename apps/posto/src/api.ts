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

export interface Table {
  id: string;
  code: number;
  name: string | null;
  is_open: boolean;
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
