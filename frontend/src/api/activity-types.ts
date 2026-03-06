import client from './client';
import type { ActivityType, CreateActivityType, UpdateActivityType } from '@/types/activity-type';

export const activityTypesApi = {
  list: () => client.get<ActivityType[]>('/activity-types'),
  create: (data: CreateActivityType) => client.post<ActivityType>('/activity-types', data),
  update: (id: string, data: UpdateActivityType) => client.put<ActivityType>(`/activity-types/${id}`, data),
  delete: (id: string) => client.delete(`/activity-types/${id}`),
};
