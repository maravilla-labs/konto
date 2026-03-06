import client from './client';
import type { VatRate, CreateVatRate, UpdateVatRate } from '@/types/vat-rate';

export const vatRatesApi = {
  list: () => client.get<VatRate[]>('/vat-rates'),
  create: (data: CreateVatRate) => client.post<VatRate>('/vat-rates', data),
  update: (id: string, data: UpdateVatRate) => client.put<VatRate>(`/vat-rates/${id}`, data),
  deactivate: (id: string) => client.delete(`/vat-rates/${id}`),
};
