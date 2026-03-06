import client from './client';
import type { AuditLog, AuditLogParams } from '@/types/audit';
import type { PaginatedResponse } from '@/types/common';

export const auditApi = {
  list(params?: AuditLogParams) {
    return client.get<PaginatedResponse<AuditLog>>('/audit-log', { params });
  },
};
