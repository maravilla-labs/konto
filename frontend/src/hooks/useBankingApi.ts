import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { bankingApi } from '@/api/banking';
import type { BankTransactionListParams, ManualMatchData, CreateJournalFromTx } from '@/types/bank-transaction';

export function useBankTransactions(params?: BankTransactionListParams) {
  return useQuery({
    queryKey: ['bank-transactions', params],
    queryFn: () => bankingApi.listTransactions(params).then((r) => r.data),
  });
}

export function useImportCamt053() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ bankAccountId, file }: { bankAccountId: string; file: File }) =>
      bankingApi.importCamt053(bankAccountId, file).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-transactions'] }),
  });
}

export function useAutoMatch() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (bankAccountId: string) =>
      bankingApi.autoMatch(bankAccountId).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-transactions'] }),
  });
}

export function useManualMatch() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: ManualMatchData }) =>
      bankingApi.manualMatch(id, data).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-transactions'] }),
  });
}

export function useCreateJournalFromTx() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: CreateJournalFromTx }) =>
      bankingApi.createJournal(id, data).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-transactions'] }),
  });
}

export function useIgnoreTransaction() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => bankingApi.ignore(id).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['bank-transactions'] }),
  });
}
