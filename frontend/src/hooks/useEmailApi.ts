import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { emailApi } from '@/api/email';
import type { UpdateEmailSettings, TestEmailRequest } from '@/types/email';

export function useEmailSettings() {
  return useQuery({
    queryKey: ['email-settings'],
    queryFn: () => emailApi.getSettings().then((r) => r.data),
  });
}

export function useUpdateEmailSettings() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: UpdateEmailSettings) => emailApi.updateSettings(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['email-settings'] }),
  });
}

export function useSendTestEmail() {
  return useMutation({
    mutationFn: (data?: TestEmailRequest) => emailApi.sendTestEmail(data),
  });
}

export function useEmailInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (invoiceId: string) => emailApi.emailInvoice(invoiceId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}
