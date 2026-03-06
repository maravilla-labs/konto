import client from './client';
import type { ExchangeRate, CreateExchangeRate } from '@/types/exchange-rate';
import type { PaginatedResponse, ListParams } from '@/types/common';

export const exchangeRatesApi = {
  list(params?: ListParams) {
    return client.get<PaginatedResponse<ExchangeRate>>('/exchange-rates', { params });
  },

  get(id: string) {
    return client.get<ExchangeRate>(`/exchange-rates/${id}`);
  },

  create(data: CreateExchangeRate) {
    return client.post<ExchangeRate>('/exchange-rates', data);
  },

  delete(id: string) {
    return client.delete(`/exchange-rates/${id}`);
  },
};
