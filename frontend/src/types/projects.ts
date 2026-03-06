export type ProjectStatus = 'active' | 'completed' | 'on_hold' | 'cancelled';

export interface Project {
  id: string;
  number?: string;
  name: string;
  contact_id?: string;
  contact_person_id?: string;
  language?: string;
  start_date?: string;
  end_date?: string;
  status: string;
  description?: string;
  project_type?: string;
  budget_hours?: string;
  budget_amount?: string;
  hourly_rate?: string;
  soft_budget_hours?: number;
  hard_budget_hours?: number;
  soft_budget_amount?: number;
  hard_budget_amount?: number;
  invoicing_method: string;
  currency: string;
  rounding_method: string | null;
  rounding_factor_minutes: number | null;
  flat_rate_total: number | null;
  owner_id?: string;
  sub_status_id: string | null;
}

export interface ProjectSummary extends Project {
  contact_name?: string;
  total_hours: string;
  billable_hours: string;
  budget_hours_remaining?: string;
  total_invoiced: string;
}

export interface CreateProject {
  name: string;
  number?: string;
  contact_id?: string;
  contact_person_id?: string;
  language?: string;
  start_date?: string;
  end_date?: string;
  description?: string;
  budget_hours?: string;
  budget_amount?: string;
  hourly_rate?: string;
  soft_budget_hours?: number;
  hard_budget_hours?: number;
  soft_budget_amount?: number;
  hard_budget_amount?: number;
  invoicing_method?: string;
  currency?: string;
  rounding_method?: string | null;
  rounding_factor_minutes?: number | null;
  flat_rate_total?: number | null;
  owner_id?: string;
}

export type UpdateProject = Partial<CreateProject> & { status?: string; sub_status_id?: string };

export interface TimeEntry {
  id: string;
  project_id?: string;
  contact_id?: string;
  user_id?: string;
  activity_type_id?: string;
  quantity?: number;
  date: string;
  actual_minutes: number;
  estimated_minutes?: number;
  description?: string;
  flat_amount?: string;
  travel_minutes?: number;
  status: string;
  billed: boolean;
  task_id?: string;
  task_name?: string;
  timesheet_id?: string;
  billable?: boolean;
  start_time?: string;
  end_time?: string;
}

export interface ProjectItem {
  id: string;
  project_id: string;
  parent_id?: string;
  item_type: string;
  name: string;
  description?: string;
  status: string;
  assignee_id?: string;
  start_date?: string;
  due_date?: string;
  estimated_hours?: string;
  budget_hours?: string;
  budget_amount?: string;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface ProjectItemTree extends ProjectItem {
  children: ProjectItemTree[];
}
