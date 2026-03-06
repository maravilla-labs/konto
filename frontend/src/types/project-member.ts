export interface ProjectMember {
  id: string;
  project_id: string;
  user_id: string;
  user_name?: string;
  rate_function_id?: string;
  rate_function_name?: string;
  hourly_rate?: number;
  resolved_rate?: number;
  role_label?: string;
  budget_hours: number | null;
  joined_at: string;
  left_at?: string;
  created_at: string;
  updated_at: string;
}

export interface AddProjectMemberRequest {
  user_id: string;
  rate_function_id?: string;
  hourly_rate?: number;
  role_label?: string;
}

export interface UpdateProjectMemberRequest {
  rate_function_id?: string;
  hourly_rate?: number;
  role_label?: string;
  left_at?: string;
}
