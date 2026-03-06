import client from './client';
import type { ProjectDocument } from '@/types/project-document';

export const projectDocumentsApi = {
  list: (projectId: string) =>
    client.get<ProjectDocument[]>(`/projects/${projectId}/files`),

  upload: (projectId: string, file: File, projectItemId?: string) => {
    const formData = new FormData();
    formData.append('file', file);
    if (projectItemId) {
      formData.append('project_item_id', projectItemId);
    }
    return client.post<ProjectDocument>(`/projects/${projectId}/files`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  download: (fileId: string) =>
    client.get(`/project-files/${fileId}/download`, { responseType: 'blob' }),

  delete: (fileId: string) =>
    client.delete(`/project-files/${fileId}`),
};
