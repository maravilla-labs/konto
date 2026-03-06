export interface Employee {
  id: string;
  number: string | null;
  user_id: string | null;
  first_name: string;
  last_name: string;
  email: string | null;
  phone: string | null;
  ahv_number: string;
  date_of_birth: string;
  nationality: string;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  iban: string;
  bic: string | null;
  bank_name: string | null;
  employment_start: string;
  employment_end: string | null;
  position: string | null;
  department: string | null;
  employment_percentage: number;
  gross_monthly_salary: number;
  annual_salary_13th: boolean;
  has_children: boolean;
  number_of_children: number;
  child_allowance_amount: number;
  education_allowance_amount: number;
  bvg_insured: boolean;
  uvg_insured: boolean;
  ktg_insured: boolean;
  is_quellensteuer: boolean;
  quellensteuer_tariff: string | null;
  quellensteuer_rate: number | null;
  marital_status: string;
  canton: string;
  status: string;
  notes: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateEmployee {
  first_name: string;
  last_name: string;
  email?: string;
  phone?: string;
  ahv_number: string;
  date_of_birth: string;
  nationality?: string;
  street: string;
  postal_code: string;
  city: string;
  country?: string;
  iban: string;
  bic?: string;
  bank_name?: string;
  employment_start: string;
  employment_end?: string;
  position?: string;
  department?: string;
  employment_percentage?: number;
  gross_monthly_salary: number;
  annual_salary_13th?: boolean;
  has_children?: boolean;
  number_of_children?: number;
  child_allowance_amount?: number;
  education_allowance_amount?: number;
  bvg_insured?: boolean;
  uvg_insured?: boolean;
  ktg_insured?: boolean;
  is_quellensteuer?: boolean;
  quellensteuer_tariff?: string;
  quellensteuer_rate?: number;
  marital_status?: string;
  canton?: string;
  user_id?: string;
  notes?: string;
  create_user?: boolean;
  user_role_id?: string;
}

export interface CreateEmployeeResponse {
  employee: Employee;
  provisioned_user?: {
    user_id: string;
    temp_password: string;
  };
}

export interface UpdateEmployee {
  first_name: string;
  last_name: string;
  email?: string | null;
  phone?: string | null;
  ahv_number: string;
  date_of_birth: string;
  nationality: string;
  street: string;
  postal_code: string;
  city: string;
  country: string;
  iban: string;
  bic?: string | null;
  bank_name?: string | null;
  employment_start: string;
  employment_end?: string | null;
  position?: string | null;
  department?: string | null;
  employment_percentage: number;
  gross_monthly_salary: number;
  annual_salary_13th: boolean;
  has_children: boolean;
  number_of_children: number;
  child_allowance_amount: number;
  education_allowance_amount: number;
  bvg_insured: boolean;
  uvg_insured: boolean;
  ktg_insured: boolean;
  is_quellensteuer: boolean;
  quellensteuer_tariff?: string | null;
  quellensteuer_rate?: number | null;
  marital_status: string;
  canton: string;
  status: string;
  user_id?: string | null;
  notes?: string | null;
}
