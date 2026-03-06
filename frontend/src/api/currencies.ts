import client from './client';
import type { Currency, CreateCurrency, UpdateCurrency } from '@/types/currency';

export const currenciesApi = {
  list: () => client.get<Currency[]>('/currencies'),
  create: (data: CreateCurrency) => client.post<Currency>('/currencies', data),
  update: (id: string, data: UpdateCurrency) => client.put<Currency>(`/currencies/${id}`, data),
};
