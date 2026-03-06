import client from './client';
import type {
  ProjectActivityType,
  AddProjectActivityTypeRequest,
  UpdateProjectActivityTypeRequest,
} from '@/types/project-activity-type';

export const projectActivityTypesApi = {
  list: (projectId: string) =>
    client.get<ProjectActivityType[]>(`/projects/${projectId}/activity-types`),
  add: (projectId: string, data: AddProjectActivityTypeRequest) =>
    client.post<ProjectActivityType>(`/projects/${projectId}/activity-types`, data),
  update: (projectId: string, patId: string, data: UpdateProjectActivityTypeRequest) =>
    client.put<ProjectActivityType>(`/projects/${projectId}/activity-types/${patId}`, data),
  remove: (projectId: string, patId: string) =>
    client.delete(`/projects/${projectId}/activity-types/${patId}`),
};
