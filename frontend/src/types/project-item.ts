export type ProjectItemType = 'phase' | 'work_package' | 'task';
export type ProjectItemStatus = 'pending' | 'in_progress' | 'completed' | 'cancelled';

export interface ProjectItem {
  id: string;
  project_id: string;
  parent_id?: string;
  item_type: ProjectItemType;
  name: string;
  description?: string;
  status: ProjectItemStatus;
  assignee_id?: string;
  start_date?: string;
  due_date?: string;
  estimated_hours?: number;
  budget_hours?: number;
  budget_amount?: number;
  sort_order: number;
  children?: ProjectItem[];
  created_at: string;
  updated_at: string;
}

export interface CreateProjectItemRequest {
  parent_id?: string;
  item_type: ProjectItemType;
  name: string;
  description?: string;
  status?: string;
  assignee_id?: string;
  start_date?: string;
  due_date?: string;
  estimated_hours?: number;
  budget_hours?: number;
  budget_amount?: number;
  sort_order?: number;
}

export interface UpdateProjectItemRequest {
  parent_id?: string;
  name?: string;
  description?: string;
  status?: string;
  assignee_id?: string;
  start_date?: string;
  due_date?: string;
  estimated_hours?: number;
  budget_hours?: number;
  budget_amount?: number;
  sort_order?: number;
}
