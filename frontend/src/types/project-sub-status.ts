export interface ProjectSubStatus {
  id: string;
  name: string;
  sort_order: number;
  color: string;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateProjectSubStatus {
  name: string;
  sort_order?: number;
  color?: string;
}

export interface UpdateProjectSubStatus {
  name?: string;
  sort_order?: number;
  color?: string;
  is_active?: boolean;
}
