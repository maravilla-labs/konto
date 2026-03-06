import client from './client';
import type {
  DocumentTemplate,
  CreateTemplate,
  UpdateTemplate,
  TemplateListParams,
} from '@/types/template';
import type { PaginatedResponse } from '@/types/common';

export const templatesApi = {
  list(params?: TemplateListParams) {
    return client.get<PaginatedResponse<DocumentTemplate>>('/templates', { params });
  },

  get(id: string) {
    return client.get<DocumentTemplate>(`/templates/${id}`);
  },

  create(data: CreateTemplate) {
    return client.post<DocumentTemplate>('/templates', data);
  },

  update(id: string, data: UpdateTemplate) {
    return client.put<DocumentTemplate>(`/templates/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/templates/${id}`);
  },

  duplicate(id: string) {
    return client.post<DocumentTemplate>(`/templates/${id}/duplicate`);
  },
};
