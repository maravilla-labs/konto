export interface FixedAsset {
  id: string;
  name: string;
  description: string | null;
  account_id: string;
  depreciation_account_id: string;
  acquisition_date: string;
  acquisition_cost: number;
  residual_value: number;
  useful_life_years: number;
  depreciation_method: string;
  declining_rate: number | null;
  status: string;
  disposed_date: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateFixedAsset {
  name: string;
  description?: string;
  account_id: string;
  depreciation_account_id: string;
  acquisition_date: string;
  acquisition_cost: number;
  residual_value: number;
  useful_life_years: number;
  depreciation_method: string;
  declining_rate?: number;
}

export interface UpdateFixedAsset {
  name: string;
  description?: string;
  account_id: string;
  depreciation_account_id: string;
  acquisition_date: string;
  acquisition_cost: number;
  residual_value: number;
  useful_life_years: number;
  depreciation_method: string;
  declining_rate?: number;
  status: string;
  disposed_date?: string;
}

export interface DepreciationEntry {
  id: string;
  fixed_asset_id: string;
  fiscal_year_id: string;
  journal_entry_id: string;
  amount: number;
  accumulated: number;
  book_value: number;
  period_date: string;
  created_at: string;
}

export interface RunDepreciationRequest {
  fiscal_year_id: string;
}
