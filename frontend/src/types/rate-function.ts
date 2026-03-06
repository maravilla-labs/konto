export interface RateFunction {
  id: string;
  name: string;
  description?: string;
  hourly_rate: number;
  is_active: boolean;
  sort_order: number;
  created_at: string;
  updated_at: string;
}

export interface CreateRateFunctionRequest {
  name: string;
  description?: string;
  hourly_rate: number;
  is_active?: boolean;
  sort_order?: number;
}

export interface UpdateRateFunctionRequest {
  name?: string;
  description?: string;
  hourly_rate?: number;
  is_active?: boolean;
  sort_order?: number;
}
