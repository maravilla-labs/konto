import client from './client';
import type { FixedAsset, CreateFixedAsset, UpdateFixedAsset, DepreciationEntry, RunDepreciationRequest } from '@/types/fixed-asset';

export const fixedAssetsApi = {
  list: () => client.get<FixedAsset[]>('/fixed-assets'),
  get: (id: string) => client.get<FixedAsset>(`/fixed-assets/${id}`),
  create: (data: CreateFixedAsset) => client.post<FixedAsset>('/fixed-assets', data),
  update: (id: string, data: UpdateFixedAsset) => client.put<FixedAsset>(`/fixed-assets/${id}`, data),
  delete: (id: string) => client.delete(`/fixed-assets/${id}`),
  getSchedule: (id: string) => client.get<DepreciationEntry[]>(`/fixed-assets/${id}/schedule`),
  runDepreciation: (data: RunDepreciationRequest) => client.post<DepreciationEntry[]>('/fixed-assets/run-depreciation', data),
};
