export type TimesheetStatus = 'draft' | 'submitted' | 'approved' | 'locked';

export interface Timesheet {
  id: string;
  user_id: string;
  user_name?: string;
  period_start: string;
  period_end: string;
  status: TimesheetStatus;
  submitted_at?: string;
  approved_by?: string;
  approved_at?: string;
  notes?: string;
  total_hours?: number;
  entry_count?: number;
  created_at: string;
  updated_at: string;
}

export interface CreateTimesheetRequest {
  period_start: string;
  period_end: string;
  notes?: string;
}

export interface UpdateTimesheetRequest {
  period_start?: string;
  period_end?: string;
  notes?: string;
}

export interface TimesheetListParams {
  page?: number;
  per_page?: number;
  search?: string;
  status?: string;
  user_id?: string;
}
