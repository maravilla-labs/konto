export interface AuditLog {
  id: string;
  user_id: string | null;
  action: string;
  entity_type: string;
  entity_id: string | null;
  old_values: string | null;
  new_values: string | null;
  created_at: string;
}

export interface AuditLogParams {
  page?: number;
  per_page?: number;
  entity_type?: string;
  action?: string;
  user_id?: string;
  from?: string;
  to?: string;
}
