import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { emailTemplateApi } from '@/api/email-templates';
import type { UpdateEmailTemplate } from '@/types/email-template';

export function useEmailTemplates() {
  return useQuery({
    queryKey: ['email-templates'],
    queryFn: () => emailTemplateApi.list().then((r) => r.data),
  });
}

export function useEmailTemplate(id: string | null) {
  return useQuery({
    queryKey: ['email-templates', id],
    queryFn: () => emailTemplateApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useUpdateEmailTemplate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateEmailTemplate }) =>
      emailTemplateApi.update(id, data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['email-templates'] });
    },
  });
}

export function usePreviewEmailTemplate() {
  return useMutation({
    mutationFn: (id: string) =>
      emailTemplateApi.preview(id).then((r) => r.data),
  });
}
