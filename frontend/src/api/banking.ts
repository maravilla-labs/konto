import client from './client';
import type {
  BankTransaction,
  ManualMatchData,
  CreateJournalFromTx,
  AutoMatchResult,
  ImportResult,
  BankTransactionListParams,
} from '@/types/bank-transaction';
import type { PaginatedResponse } from '@/types/common';

export const bankingApi = {
  listTransactions(params?: BankTransactionListParams) {
    return client.get<PaginatedResponse<BankTransaction>>('/bank-transactions', { params });
  },

  importCamt053(bankAccountId: string, file: File) {
    const formData = new FormData();
    formData.append('file', file);
    return client.post<ImportResult>(`/bank-transactions/import/${bankAccountId}`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  autoMatch(bankAccountId: string) {
    return client.post<AutoMatchResult>(`/bank-transactions/auto-match/${bankAccountId}`);
  },

  manualMatch(id: string, data: ManualMatchData) {
    return client.post<BankTransaction>(`/bank-transactions/${id}/match`, data);
  },

  createJournal(id: string, data: CreateJournalFromTx) {
    return client.post<BankTransaction>(`/bank-transactions/${id}/journal`, data);
  },

  ignore(id: string) {
    return client.post<BankTransaction>(`/bank-transactions/${id}/ignore`);
  },
};
