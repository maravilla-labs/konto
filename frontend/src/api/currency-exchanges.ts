import client from './client';
import type { CurrencyExchange, RecordTransfer } from '@/types/currency-exchange';
import type { PaginatedResponse, ListParams } from '@/types/common';

export const currencyExchangesApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<CurrencyExchange>>('/currency-exchanges', { params });
  },

  create(data: RecordTransfer) {
    return client.post<CurrencyExchange>('/currency-exchanges', data);
  },
};
