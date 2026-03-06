export interface ProjectMilestone {
  id: string;
  project_id: string;
  project_item_id?: string;
  name: string;
  description?: string;
  target_date: string;
  status: string;
  reached_at?: string;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectMilestoneRequest {
  project_item_id?: string;
  name: string;
  description?: string;
  target_date: string;
}

export interface UpdateProjectMilestoneRequest {
  project_item_id?: string;
  name?: string;
  description?: string;
  target_date?: string;
  status?: string;
}
