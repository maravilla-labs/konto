export interface CompanySettings {
  id: string;
  legal_name: string;
  trade_name: string | null;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  email: string | null;
  phone: string | null;
  website: string | null;
  vat_number: string | null;
  vat_method: string;
  flat_rate_percentage: string | null;
  register_number: string | null;
  logo_url: string | null;
  default_currency_id: string | null;
  date_format: string;
  number_format: string;
  ui_language: string;
  fiscal_year_start_month: number;
  tax_id_label: string;
  audit_optout: boolean;
  project_number_auto: boolean;
  project_number_prefix: string;
  project_number_restart_yearly: boolean;
  project_number_start: number;
  project_number_min_length: number;
  customer_number_auto: boolean;
  customer_number_prefix: string;
  customer_number_restart_yearly: boolean;
  customer_number_start: number;
  customer_number_min_length: number;
  employee_number_auto: boolean;
  employee_number_prefix: string;
  employee_number_restart_yearly: boolean;
  employee_number_start: number;
  employee_number_min_length: number;
  created_at: string;
  updated_at: string;
}

export interface UpdateCompanySettings {
  legal_name: string;
  trade_name?: string | null;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  email?: string | null;
  phone?: string | null;
  website?: string | null;
  vat_number?: string | null;
  vat_method: string;
  flat_rate_percentage?: number | null;
  register_number?: string | null;
  default_currency_id?: string | null;
  date_format?: string;
  number_format?: string;
  ui_language?: string;
  fiscal_year_start_month?: number;
  tax_id_label?: string;
  audit_optout?: boolean;
  project_number_auto?: boolean;
  project_number_prefix?: string;
  project_number_restart_yearly?: boolean;
  project_number_start?: number;
  project_number_min_length?: number;
  customer_number_auto?: boolean;
  customer_number_prefix?: string;
  customer_number_restart_yearly?: boolean;
  customer_number_start?: number;
  customer_number_min_length?: number;
  employee_number_auto?: boolean;
  employee_number_prefix?: string;
  employee_number_restart_yearly?: boolean;
  employee_number_start?: number;
  employee_number_min_length?: number;
}

export interface BankAccount {
  id: string;
  name: string;
  bank_name: string;
  iban: string;
  bic: string | null;
  currency_id: string | null;
  account_id: string | null;
  qr_iban: string | null;
  is_default: boolean;
  created_at: string;
  updated_at: string;
}

export interface CreateBankAccount {
  name: string;
  bank_name: string;
  iban: string;
  bic?: string | null;
  currency_id?: string | null;
  account_id?: string | null;
  qr_iban?: string | null;
  is_default?: boolean;
}

export type UpdateBankAccount = CreateBankAccount;
