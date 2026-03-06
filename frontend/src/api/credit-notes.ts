import client from './client';
import type {
  CreditNote,
  CreditNoteDetail,
  CreateCreditNote,
  UpdateCreditNote,
  CreditNoteListParams,
} from '@/types/credit-note';
import type { PaginatedResponse } from '@/types/common';

export const creditNotesApi = {
  list(params?: CreditNoteListParams) {
    return client.get<PaginatedResponse<CreditNote>>('/credit-notes', { params });
  },

  get(id: string) {
    return client.get<CreditNoteDetail>(`/credit-notes/${id}`);
  },

  create(data: CreateCreditNote) {
    return client.post<CreditNote>('/credit-notes', data);
  },

  update(id: string, data: UpdateCreditNote) {
    return client.put<CreditNote>(`/credit-notes/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/credit-notes/${id}`);
  },

  issue(id: string) {
    return client.post<CreditNote>(`/credit-notes/${id}/issue`);
  },

  apply(id: string) {
    return client.post<CreditNote>(`/credit-notes/${id}/apply`);
  },

  cancel(id: string) {
    return client.post<CreditNote>(`/credit-notes/${id}/cancel`);
  },

  downloadPdf(id: string) {
    return client.get(`/credit-notes/${id}/pdf`, { responseType: 'blob' });
  },
};
