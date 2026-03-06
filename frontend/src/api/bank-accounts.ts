import client from './client';
import type {
  BankAccount,
  CreateBankAccount,
  UpdateBankAccount,
} from '@/types/settings';

export const bankAccountsApi = {
  list() {
    return client.get<BankAccount[]>('/bank-accounts');
  },

  create(data: CreateBankAccount) {
    return client.post<BankAccount>('/bank-accounts', data);
  },

  update(id: string, data: UpdateBankAccount) {
    return client.put<BankAccount>(`/bank-accounts/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/bank-accounts/${id}`);
  },
};
