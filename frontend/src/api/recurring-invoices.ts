import client from './client';
import type {
  RecurringInvoice,
  CreateRecurringInvoice,
  UpdateRecurringInvoice,
  RecurringInvoiceListParams,
} from '@/types/recurring-invoice';
import type { PaginatedResponse } from '@/types/common';

export const recurringInvoicesApi = {
  list(params?: RecurringInvoiceListParams) {
    return client.get<PaginatedResponse<RecurringInvoice>>('/recurring-invoices', { params });
  },

  get(id: string) {
    return client.get<RecurringInvoice>(`/recurring-invoices/${id}`);
  },

  create(data: CreateRecurringInvoice) {
    return client.post<RecurringInvoice>('/recurring-invoices', data);
  },

  update(id: string, data: UpdateRecurringInvoice) {
    return client.put<RecurringInvoice>(`/recurring-invoices/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/recurring-invoices/${id}`);
  },

  trigger() {
    return client.post<{ generated: number }>('/recurring-invoices/trigger');
  },
};
