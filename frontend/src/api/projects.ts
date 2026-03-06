import client from './client';
import type { Project, ProjectSummary, CreateProject, UpdateProject, ProjectItemTree } from '@/types/projects';
import type { PaginatedResponse, ListParams } from '@/types/common';

export const projectsApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<Project>>('/projects', { params });
  },

  get(id: string) {
    return client.get<Project>(`/projects/${id}`);
  },

  summary(id: string) {
    return client.get<ProjectSummary>(`/projects/${id}/summary`);
  },

  create(data: CreateProject) {
    return client.post<Project>('/projects', data);
  },

  update(id: string, data: UpdateProject) {
    return client.put<Project>(`/projects/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/projects/${id}`);
  },

  listItems(projectId: string) {
    return client.get<ProjectItemTree[]>(`/projects/${projectId}/items`);
  },

  getBudgetAnalytics(projectId: string) {
    return client.get(`/projects/${projectId}/budget-analytics`).then(r => r.data);
  },
};
