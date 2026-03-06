import client from './client';
import type {
  ProjectMember,
  AddProjectMemberRequest,
  UpdateProjectMemberRequest,
} from '@/types/project-member';

export const projectMembersApi = {
  list: (projectId: string) =>
    client.get<ProjectMember[]>(`/projects/${projectId}/members`),

  add: (projectId: string, data: AddProjectMemberRequest) =>
    client.post<ProjectMember>(`/projects/${projectId}/members`, data),

  update: (projectId: string, memberId: string, data: UpdateProjectMemberRequest) =>
    client.put<ProjectMember>(`/projects/${projectId}/members/${memberId}`, data),

  remove: (projectId: string, memberId: string) =>
    client.delete(`/projects/${projectId}/members/${memberId}`),
};
