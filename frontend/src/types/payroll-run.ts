export interface PayrollRun {
  id: string;
  period_month: number;
  period_year: number;
  status: 'draft' | 'calculated' | 'approved' | 'paid';
  run_date: string;
  approved_by: string | null;
  approved_at: string | null;
  paid_at: string | null;
  journal_entry_id: string | null;
  payment_file_generated: boolean;
  total_gross: number;
  total_net: number;
  total_employer_cost: number;
  created_at: string;
  updated_at: string;
}

export interface PayrollRunLine {
  id: string;
  payroll_run_id: string;
  employee_id: string;
  employee_name?: string | null;
  gross_salary: number;
  ahv_employee: number;
  ahv_employer: number;
  alv_employee: number;
  alv_employer: number;
  bvg_employee: number;
  bvg_employer: number;
  nbu_employee: number;
  bu_employer: number;
  ktg_employee: number;
  ktg_employer: number;
  fak_employer: number;
  quellensteuer: number;
  child_allowance: number;
  net_salary: number;
  payout_amount: number;
  total_employer_cost: number;
}

export interface PayrollRunDetail {
  run: PayrollRun;
  lines: PayrollRunLine[];
}

export interface CreatePayrollRun {
  month: number;
  year: number;
}
