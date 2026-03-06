import client from './client';
import type {
  Document,
  DocumentDetail,
  CreateDocument,
  UpdateDocument,
  DocumentListParams,
} from '@/types/document';
import type { PaginatedResponse } from '@/types/common';

export const documentsApi = {
  list(params?: DocumentListParams) {
    return client.get<PaginatedResponse<Document>>('/documents', { params });
  },

  get(id: string) {
    return client.get<DocumentDetail>(`/documents/${id}`);
  },

  create(data: CreateDocument) {
    return client.post<Document>('/documents', data);
  },

  update(id: string, data: UpdateDocument) {
    return client.put<Document>(`/documents/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/documents/${id}`);
  },

  send(id: string) {
    return client.post<Document>(`/documents/${id}/send`);
  },

  accept(id: string) {
    return client.post<Document>(`/documents/${id}/accept`);
  },

  reject(id: string) {
    return client.post<Document>(`/documents/${id}/reject`);
  },

  convert(id: string, data: { target_type: string }) {
    return client.post<Document>(`/documents/${id}/convert`, data);
  },
};
