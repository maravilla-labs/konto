import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { settingsApi } from '@/api/settings';
import { bankAccountsApi } from '@/api/bank-accounts';
import type { UpdateCompanySettings } from '@/types/settings';
import type { CreateBankAccount, UpdateBankAccount } from '@/types/settings';

// Company Settings
export function useSettings() {
  return useQuery({
    queryKey: ['settings'],
    queryFn: () => settingsApi.get().then((r) => r.data),
  });
}

export function useUpdateSettings() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: UpdateCompanySettings) => settingsApi.update(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['settings'] }),
  });
}

export function useUploadLogo() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (file: File) => settingsApi.uploadLogo(file),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['settings'] }),
  });
}

// Bank Accounts
export function useBankAccounts() {
  return useQuery({
    queryKey: ['bank-accounts'],
    queryFn: () => bankAccountsApi.list().then((r) => r.data),
  });
}

export function useCreateBankAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateBankAccount) => bankAccountsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-accounts'] }),
  });
}

export function useUpdateBankAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateBankAccount }) =>
      bankAccountsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-accounts'] }),
  });
}

export function useDeleteBankAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => bankAccountsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-accounts'] }),
  });
}
