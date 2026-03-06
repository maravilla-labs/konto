export interface EmployeeSummary {
  id: string;
  first_name: string;
  last_name: string;
  ahv_number: string;
  date_of_birth: string;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  employment_start: string;
  employment_end: string | null;
  marital_status: string;
}

export interface SalaryCertificateData {
  employee: EmployeeSummary;
  year: number;
  months_worked: number;
  total_gross: number;
  total_ahv_employee: number;
  total_alv_employee: number;
  total_bvg_employee: number;
  total_nbu_employee: number;
  total_ktg_employee: number;
  total_quellensteuer: number;
  total_child_allowance: number;
  total_net: number;
  total_payout: number;
  total_social_deductions: number;
}
