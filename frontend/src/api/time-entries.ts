import client from './client';
import type { TimeEntry } from '@/types/projects';
import type { PaginatedResponse, ListParams } from '@/types/common';

export interface CreateTimeEntry {
  project_id?: string;
  task_id?: string;
  user_id?: string;
  activity_type_id?: string;
  date: string;
  actual_minutes: number;
  description?: string;
  quantity?: number;
}

export interface UpdateTimeEntry {
  project_id?: string;
  task_id?: string;
  activity_type_id?: string;
  date?: string;
  actual_minutes?: number;
  description?: string;
  quantity?: number | null;
}

export const timeEntriesApi = {
  list(params?: ListParams & { project_id?: string; billed?: boolean }) {
    return client.get<PaginatedResponse<TimeEntry>>('/time-entries', { params });
  },

  create(data: CreateTimeEntry) {
    return client.post<TimeEntry>('/time-entries', data);
  },

  update(id: string, data: UpdateTimeEntry) {
    return client.put<TimeEntry>(`/time-entries/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/time-entries/${id}`);
  },

  transitionTimeEntry(id: string, status: string) {
    return client.put(`/time-entries/${id}/transition`, { status }).then(r => r.data);
  },
};
