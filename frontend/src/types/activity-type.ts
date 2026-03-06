export interface ActivityType {
  id: string;
  name: string;
  is_active: boolean;
  unit_type: string;
  default_rate?: number;
}

export interface CreateActivityType {
  name: string;
  unit_type?: string;
  default_rate?: number;
}

export interface UpdateActivityType {
  name: string;
  is_active: boolean;
  unit_type?: string;
  default_rate?: number | null;
}
