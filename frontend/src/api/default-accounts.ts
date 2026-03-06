import client from './client';
import type { DefaultAccount, DefaultAccountUpdate } from '@/types/default-account';

export const defaultAccountsApi = {
  list() {
    return client.get<DefaultAccount[]>('/settings/default-accounts');
  },

  update(settings: DefaultAccountUpdate[]) {
    return client.put<DefaultAccount[]>('/settings/default-accounts', { settings });
  },
};
