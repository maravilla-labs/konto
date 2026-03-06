import { useQuery, useMutation } from '@tanstack/react-query';
import { setupApi, type SetupCompleteRequest } from '@/api/setup';

export function useSetupStatus() {
  return useQuery({
    queryKey: ['setup', 'status'],
    queryFn: () => setupApi.getStatus().then((r) => r.data),
    staleTime: 60_000,
    retry: 1,
  });
}

export function useCompleteSetup() {
  return useMutation({
    mutationFn: (data: SetupCompleteRequest) =>
      setupApi.complete(data).then((r) => r.data),
  });
}

export function useBranding() {
  return useQuery({
    queryKey: ['setup', 'branding'],
    queryFn: () => setupApi.getBranding().then((r) => r.data),
    staleTime: 300_000,
    retry: 1,
  });
}
