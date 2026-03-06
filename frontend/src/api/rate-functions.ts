import client from './client';
import type {
  RateFunction,
  CreateRateFunctionRequest,
  UpdateRateFunctionRequest,
} from '@/types/rate-function';

export const rateFunctionsApi = {
  list: () => client.get<RateFunction[]>('/rate-functions'),

  get: (id: string) => client.get<RateFunction>(`/rate-functions/${id}`),

  create: (data: CreateRateFunctionRequest) =>
    client.post<RateFunction>('/rate-functions', data),

  update: (id: string, data: UpdateRateFunctionRequest) =>
    client.put<RateFunction>(`/rate-functions/${id}`, data),

  delete: (id: string) => client.delete(`/rate-functions/${id}`),
};
