export interface PayrollSettings {
  id: string;
  ahv_iv_eo_rate_employee: number;
  ahv_iv_eo_rate_employer: number;
  alv_rate_employee: number;
  alv_rate_employer: number;
  alv_salary_cap: number;
  bvg_coordination_deduction: number;
  bvg_entry_threshold: number;
  bvg_min_insured_salary: number;
  bvg_max_insured_salary: number;
  bvg_rate_25_34: number;
  bvg_rate_35_44: number;
  bvg_rate_45_54: number;
  bvg_rate_55_65: number;
  bvg_risk_rate: number;
  bvg_employer_share_pct: number;
  nbu_rate_employee: number;
  bu_rate_employer: number;
  ktg_rate_employee: number;
  ktg_rate_employer: number;
  fak_rate_employer: number;
  uvg_max_salary: number;
  payment_bank_account_id: string | null;
  company_clearing_number: string | null;
}

export interface UpdatePayrollSettings {
  ahv_iv_eo_rate_employee: number;
  ahv_iv_eo_rate_employer: number;
  alv_rate_employee: number;
  alv_rate_employer: number;
  alv_salary_cap: number;
  bvg_coordination_deduction: number;
  bvg_entry_threshold: number;
  bvg_min_insured_salary: number;
  bvg_max_insured_salary: number;
  bvg_rate_25_34: number;
  bvg_rate_35_44: number;
  bvg_rate_45_54: number;
  bvg_rate_55_65: number;
  bvg_risk_rate: number;
  bvg_employer_share_pct: number;
  nbu_rate_employee: number;
  bu_rate_employer: number;
  ktg_rate_employee: number;
  ktg_rate_employer: number;
  fak_rate_employer: number;
  uvg_max_salary: number;
  payment_bank_account_id?: string | null;
  company_clearing_number?: string | null;
}
