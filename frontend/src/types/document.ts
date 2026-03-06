export interface Document {
  id: string;
  doc_type: string;
  doc_number: string | null;
  title: string;
  status: string;
  contact_id: string;
  project_id: string | null;
  template_id: string | null;
  content_json: string;
  language: string | null;
  currency_id: string | null;
  subtotal: string;
  vat_rate: string;
  vat_amount: string;
  total: string;
  valid_until: string | null;
  issued_at: string | null;
  signed_at: string | null;
  converted_from: string | null;
  created_by: string | null;
  created_at: string;
  updated_at: string;
}

export interface DocumentLineItem {
  id: string;
  document_id: string;
  position: number;
  description: string;
  quantity: string;
  unit: string | null;
  unit_price: string;
  discount_pct: string;
  total: string;
}

export interface DocumentDetail extends Document {
  lines: DocumentLineItem[];
  contact_name: string | null;
  project_name: string | null;
}

export interface CreateDocument {
  doc_type: string;
  title: string;
  contact_id: string;
  project_id?: string | null;
  template_id?: string | null;
  content_json: string;
  language?: string | null;
  currency_id?: string | null;
  valid_until?: string | null;
  lines: CreateDocumentLine[];
}

export interface CreateDocumentLine {
  description: string;
  quantity: string;
  unit?: string | null;
  unit_price: string;
  discount_pct?: string;
}

export type UpdateDocument = CreateDocument;

export interface DocumentListParams {
  page?: number;
  per_page?: number;
  doc_type?: string;
  status?: string;
  contact_id?: string;
  search?: string;
}
