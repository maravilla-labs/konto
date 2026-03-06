import client from './client';
import type { Account, AccountTree, AccountTreeWithBalance, CreateAccount, UpdateAccount } from '@/types/accounts';
import type { PaginatedResponse, ListParams } from '@/types/common';

export const accountsApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<Account>>('/accounts', { params });
  },

  tree() {
    return client.get<AccountTree[]>('/accounts/tree');
  },

  get(id: string) {
    return client.get<Account>(`/accounts/${id}`);
  },

  create(data: CreateAccount) {
    return client.post<Account>('/accounts', data);
  },

  update(id: string, data: UpdateAccount) {
    return client.put<Account>(`/accounts/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/accounts/${id}`);
  },

  treeWithBalances() {
    return client.get<AccountTreeWithBalance[]>('/accounts/tree-with-balances');
  },
};
