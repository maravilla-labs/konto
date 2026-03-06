import client from './client';
import type {
  Timesheet,
  CreateTimesheetRequest,
  UpdateTimesheetRequest,
  TimesheetListParams,
} from '@/types/timesheet';
import type { PaginatedResponse } from '@/types/common';

export const timesheetsApi = {
  list(params?: TimesheetListParams) {
    return client.get<PaginatedResponse<Timesheet>>('/timesheets', { params });
  },

  get(id: string) {
    return client.get<Timesheet>(`/timesheets/${id}`);
  },

  create(data: CreateTimesheetRequest) {
    return client.post<Timesheet>('/timesheets', data);
  },

  update(id: string, data: UpdateTimesheetRequest) {
    return client.put<Timesheet>(`/timesheets/${id}`, data);
  },

  submit(id: string) {
    return client.post<Timesheet>(`/timesheets/${id}/submit`);
  },

  approve(id: string) {
    return client.post<Timesheet>(`/timesheets/${id}/approve`);
  },

  reject(id: string) {
    return client.post<Timesheet>(`/timesheets/${id}/reject`);
  },

  delete(id: string) {
    return client.delete(`/timesheets/${id}`);
  },
};
