import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { dunningApi } from '@/api/dunning';
import type { UpdateDunningLevel, SendReminderData } from '@/types/dunning';

export function useDunningLevels() {
  return useQuery({
    queryKey: ['dunning-levels'],
    queryFn: () => dunningApi.listLevels().then((r) => r.data),
  });
}

export function useUpdateDunningLevel() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateDunningLevel }) =>
      dunningApi.updateLevel(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['dunning-levels'] }),
  });
}

export function useInvoiceDunning(invoiceId?: string) {
  return useQuery({
    queryKey: ['dunning-entries', invoiceId],
    queryFn: () => dunningApi.getInvoiceDunning(invoiceId!).then((r) => r.data),
    enabled: !!invoiceId,
  });
}

export function useSendReminder() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ invoiceId, data }: { invoiceId: string; data: SendReminderData }) =>
      dunningApi.sendReminder(invoiceId, data),
    onSuccess: (_, vars) => {
      qc.invalidateQueries({ queryKey: ['dunning-entries', vars.invoiceId] });
      qc.invalidateQueries({ queryKey: ['invoices'] });
    },
  });
}

export function useRunDunning() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => dunningApi.runDunning(),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['dunning-entries'] });
      qc.invalidateQueries({ queryKey: ['invoices'] });
    },
  });
}
