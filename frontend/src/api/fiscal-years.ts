import client from './client';
import type { FiscalYear, CreateFiscalYear, UpdateFiscalYear } from '@/types/fiscal-year';
import type { PaginatedResponse, ListParams } from '@/types/common';

export const fiscalYearsApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<FiscalYear>>('/fiscal-years', { params });
  },

  get(id: string) {
    return client.get<FiscalYear>(`/fiscal-years/${id}`);
  },

  create(data: CreateFiscalYear) {
    return client.post<FiscalYear>('/fiscal-years', data);
  },

  update(id: string, data: UpdateFiscalYear) {
    return client.put<FiscalYear>(`/fiscal-years/${id}`, data);
  },

  close(id: string) {
    return client.post<FiscalYear>(`/fiscal-years/${id}/close`);
  },
};
