import client from './client';
import type { InvoicePayment, RecordPaymentData } from '@/types/invoice-payment';

export const invoicePaymentsApi = {
  list: (invoiceId: string) =>
    client.get<InvoicePayment[]>(`/invoices/${invoiceId}/payments`),
  record: (invoiceId: string, data: RecordPaymentData) =>
    client.post<InvoicePayment>(`/invoices/${invoiceId}/payments`, data),
};
