import client from './client';
import type {
  Invoice,
  InvoiceDetail,
  CreateInvoice,
  UpdateInvoice,
  PayInvoiceData,
  InvoiceListParams,
} from '@/types/invoice';
import type { PaginatedResponse } from '@/types/common';

export const invoicesApi = {
  list(params?: InvoiceListParams) {
    return client.get<PaginatedResponse<Invoice>>('/invoices', { params });
  },

  get(id: string) {
    return client.get<InvoiceDetail>(`/invoices/${id}`);
  },

  create(data: CreateInvoice) {
    return client.post<Invoice>('/invoices', data);
  },

  update(id: string, data: UpdateInvoice) {
    return client.put<Invoice>(`/invoices/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/invoices/${id}`);
  },

  send(id: string) {
    return client.post<Invoice>(`/invoices/${id}/send`);
  },

  pay(id: string, data: PayInvoiceData) {
    return client.post<Invoice>(`/invoices/${id}/pay`, data);
  },

  cancel(id: string) {
    return client.post<Invoice>(`/invoices/${id}/cancel`);
  },

  downloadPdf(id: string) {
    return client.get(`/invoices/${id}/pdf`, { responseType: 'blob' });
  },

  duplicate(id: string) {
    return client.post<Invoice>(`/invoices/${id}/duplicate`);
  },

  createFromTimeEntries(data: {
    time_entry_ids: string[];
    contact_id: string;
    project_id?: string;
    language?: string;
    hourly_rate?: string;
    account_id: string;
  }) {
    return client.post<Invoice>('/invoices/from-time-entries', data);
  },
};
