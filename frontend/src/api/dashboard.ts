import client from './client';
import type { DashboardStats } from '@/types/dashboard';

export const dashboardApi = {
  stats() {
    return client.get<DashboardStats>('/dashboard');
  },
};
