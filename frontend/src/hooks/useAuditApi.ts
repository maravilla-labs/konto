import { useQuery } from '@tanstack/react-query';
import { auditApi } from '@/api/audit';
import type { AuditLogParams } from '@/types/audit';

export function useAuditLogs(params?: AuditLogParams) {
  return useQuery({
    queryKey: ['audit-logs', params],
    queryFn: () => auditApi.list(params).then((r) => r.data),
  });
}
