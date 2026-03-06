import client from './client';
import type { EmailSettings, UpdateEmailSettings, TestEmailRequest } from '@/types/email';

export const emailApi = {
  getSettings() {
    return client.get<EmailSettings | null>('/settings/email');
  },

  updateSettings(data: UpdateEmailSettings) {
    return client.put<EmailSettings>('/settings/email', data);
  },

  sendTestEmail(data?: TestEmailRequest) {
    return client.post<{ message: string }>('/settings/email/test', data ?? {});
  },

  emailInvoice(invoiceId: string) {
    return client.post<{ message: string }>(`/invoices/${invoiceId}/email`);
  },
};
