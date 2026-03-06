export interface PayoutEntry {
  id: string;
  payroll_run_id: string;
  employee_id: string;
  amount: number;
  iban: string;
  bic: string | null;
  recipient_name: string;
  recipient_street: string;
  recipient_postal_code: string;
  recipient_city: string;
  recipient_country: string;
  status: string;
  paid_at: string | null;
  payment_reference: string;
  created_at: string;
  updated_at: string;
}
