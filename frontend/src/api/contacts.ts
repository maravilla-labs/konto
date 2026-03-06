import client from './client';
import type { Contact, CreateContact, UpdateContact, DocumentSummary } from '@/types/contacts';
import type { PaginatedResponse, ListParams } from '@/types/common';
import type { TimeEntry } from '@/types/projects';
import type { VatInfo } from '@/types/vat-info';

export const contactsApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<Contact>>('/contacts', { params });
  },

  get(id: string) {
    return client.get<Contact>(`/contacts/${id}`);
  },

  create(data: CreateContact) {
    return client.post<Contact>('/contacts', data);
  },

  update(id: string, data: UpdateContact) {
    return client.put<Contact>(`/contacts/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/contacts/${id}`);
  },

  invoices(id: string, params?: ListParams) {
    return client.get<PaginatedResponse<unknown>>(`/contacts/${id}/invoices`, { params });
  },

  documents(id: string, params?: ListParams) {
    return client.get<PaginatedResponse<DocumentSummary>>(`/contacts/${id}/documents`, { params });
  },

  timeEntries(id: string, params?: ListParams) {
    return client.get<PaginatedResponse<TimeEntry>>(`/contacts/${id}/time-entries`, { params });
  },

  persons(id: string) {
    return client.get<Contact[]>(`/contacts/${id}/persons`);
  },

  vatInfo(id: string) {
    return client.get<VatInfo>(`/contacts/${id}/vat-info`);
  },
};
