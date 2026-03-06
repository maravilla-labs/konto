import client from './client';
import type { PayoutEntry } from '@/types/payout-entry';

export const payoutEntriesApi = {
  listByRun: (runId: string) =>
    client.get<PayoutEntry[]>(`/payroll-runs/${runId}/payout-entries`),

  generatePayouts: (runId: string) =>
    client.post<PayoutEntry[]>(`/payroll-runs/${runId}/generate-payouts`),

  exportPain001: (runId: string) =>
    client.post(`/payroll-runs/${runId}/export-pain001`, null, {
      responseType: 'blob',
    }),

  markPaid: (entryId: string) =>
    client.put<PayoutEntry>(`/payout-entries/${entryId}/mark-paid`),

  markAllPaid: (runId: string) =>
    client.post(`/payroll-runs/${runId}/mark-all-paid`),
};
