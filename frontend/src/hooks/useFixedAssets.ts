import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { fixedAssetsApi } from '@/api/fixed-assets';
import type { CreateFixedAsset, UpdateFixedAsset, RunDepreciationRequest } from '@/types/fixed-asset';

// Fixed Assets
export function useFixedAssets() {
  return useQuery({
    queryKey: ['fixed-assets'],
    queryFn: () => fixedAssetsApi.list().then((r) => r.data),
  });
}

export function useFixedAsset(id: string | undefined) {
  return useQuery({
    queryKey: ['fixed-assets', id],
    queryFn: () => fixedAssetsApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateFixedAsset() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateFixedAsset) => fixedAssetsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fixed-assets'] }),
  });
}

export function useUpdateFixedAsset() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateFixedAsset }) =>
      fixedAssetsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fixed-assets'] }),
  });
}

export function useDeleteFixedAsset() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => fixedAssetsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fixed-assets'] }),
  });
}

export function useDepreciationSchedule(assetId: string) {
  return useQuery({
    queryKey: ['fixed-assets', assetId, 'schedule'],
    queryFn: () => fixedAssetsApi.getSchedule(assetId).then((r) => r.data),
    enabled: !!assetId,
  });
}

export function useRunDepreciation() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: RunDepreciationRequest) => fixedAssetsApi.runDepreciation(data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['fixed-assets'] });
      qc.invalidateQueries({ queryKey: ['journal'] });
    },
  });
}
