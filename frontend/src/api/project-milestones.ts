import client from './client';
import type {
  ProjectMilestone,
  CreateProjectMilestoneRequest,
  UpdateProjectMilestoneRequest,
} from '@/types/project-milestone';

export const projectMilestonesApi = {
  list: (projectId: string) =>
    client.get<ProjectMilestone[]>(`/projects/${projectId}/milestones`),

  get: (milestoneId: string) =>
    client.get<ProjectMilestone>(`/project-milestones/${milestoneId}`),

  create: (projectId: string, data: CreateProjectMilestoneRequest) =>
    client.post<ProjectMilestone>(`/projects/${projectId}/milestones`, data),

  update: (milestoneId: string, data: UpdateProjectMilestoneRequest) =>
    client.put<ProjectMilestone>(`/project-milestones/${milestoneId}`, data),

  delete: (milestoneId: string) =>
    client.delete(`/project-milestones/${milestoneId}`),

  reach: (milestoneId: string) =>
    client.post<ProjectMilestone>(`/project-milestones/${milestoneId}/reach`),
};
