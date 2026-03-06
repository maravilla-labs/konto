import client from './client';
import type {
  DunningLevel, UpdateDunningLevel,
  DunningEntry, SendReminderData, DunningRunResult,
} from '@/types/dunning';

export const dunningApi = {
  listLevels: () =>
    client.get<DunningLevel[]>('/dunning/levels'),

  updateLevel: (id: string, data: UpdateDunningLevel) =>
    client.put<DunningLevel>(`/dunning/levels/${id}`, data),

  getInvoiceDunning: (invoiceId: string) =>
    client.get<DunningEntry[]>(`/invoices/${invoiceId}/dunning`),

  sendReminder: (invoiceId: string, data: SendReminderData) =>
    client.post<DunningEntry>(`/invoices/${invoiceId}/dunning`, data),

  runDunning: () =>
    client.post<DunningRunResult>('/dunning/run'),
};
