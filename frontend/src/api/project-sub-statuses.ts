import client from './client';
import type { ProjectSubStatus, CreateProjectSubStatus, UpdateProjectSubStatus } from '@/types/project-sub-status';

export const projectSubStatusesApi = {
  list() {
    return client.get<ProjectSubStatus[]>('/project-sub-statuses').then(r => r.data);
  },

  create(data: CreateProjectSubStatus) {
    return client.post<ProjectSubStatus>('/project-sub-statuses', data).then(r => r.data);
  },

  update(id: string, data: UpdateProjectSubStatus) {
    return client.put<ProjectSubStatus>(`/project-sub-statuses/${id}`, data).then(r => r.data);
  },

  delete(id: string) {
    return client.delete(`/project-sub-statuses/${id}`);
  },
};
