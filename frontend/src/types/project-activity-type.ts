export interface ProjectActivityType {
  id: string;
  project_id: string;
  activity_type_id: string;
  activity_type_name?: string;
  unit_type?: string;
  default_rate?: number;
  rate?: number;
  effective_rate?: number;
  budget_hours: number | null;
  chargeable: boolean;
  created_at: string;
  updated_at: string;
}

export interface AddProjectActivityTypeRequest {
  activity_type_id: string;
  rate?: number;
}

export interface UpdateProjectActivityTypeRequest {
  rate?: number | null;
}
