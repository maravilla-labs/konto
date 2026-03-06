import client from './client';
import type {
  ProjectItem,
  CreateProjectItemRequest,
  UpdateProjectItemRequest,
} from '@/types/project-item';

export const projectItemsApi = {
  list: (projectId: string) =>
    client.get<ProjectItem[]>(`/projects/${projectId}/items`),

  tree: (projectId: string) =>
    client.get<ProjectItem[]>(`/projects/${projectId}/items?tree=true`),

  get: (itemId: string) =>
    client.get<ProjectItem>(`/project-items/${itemId}`),

  create: (projectId: string, data: CreateProjectItemRequest) =>
    client.post<ProjectItem>(`/projects/${projectId}/items`, data),

  update: (itemId: string, data: UpdateProjectItemRequest) =>
    client.put<ProjectItem>(`/project-items/${itemId}`, data),

  delete: (itemId: string) =>
    client.delete(`/project-items/${itemId}`),

  reorder: (projectId: string, items: { id: string; sort_order: number }[]) =>
    client.put(`/projects/${projectId}/items/reorder`, { items }),
};
