import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { templatesApi } from '@/api/templates';
import type { CreateTemplate, UpdateTemplate, TemplateListParams } from '@/types/template';

export function useTemplates(params?: TemplateListParams) {
  return useQuery({
    queryKey: ['templates', params],
    queryFn: () => templatesApi.list(params).then((r) => r.data),
  });
}

export function useTemplate(id: string | undefined) {
  return useQuery({
    queryKey: ['templates', id],
    queryFn: () => templatesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateTemplate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateTemplate) => templatesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['templates'] }),
  });
}

export function useUpdateTemplate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateTemplate }) =>
      templatesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['templates'] }),
  });
}

export function useDeleteTemplate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => templatesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['templates'] }),
  });
}

export function useDuplicateTemplate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => templatesApi.duplicate(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['templates'] }),
  });
}
