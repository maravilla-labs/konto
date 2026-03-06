export type ExpenseStatus = 'pending' | 'approved' | 'paid' | 'cancelled' | 'submitted' | 'rejected';
export type ExpenseType = 'single' | 'report';

export interface Expense {
  id: string;
  expense_number: string | null;
  contact_id: string | null;
  category_id: string | null;
  description: string;
  amount: string;
  currency_id: string;
  vat_rate_id: string | null;
  vat_amount: string;
  total: string;
  expense_date: string;
  due_date: string | null;
  status: ExpenseStatus;
  payment_account_id: string | null;
  receipt_url: string | null;
  project_id: string | null;
  journal_entry_id: string | null;
  payment_journal_entry_id: string | null;
  created_by: string | null;
  expense_type: ExpenseType;
  purpose: string | null;
  employee_id: string | null;
  period_from: string | null;
  period_to: string | null;
  advances: string;
  total_reimbursement: string;
  approved_by: string | null;
  approved_at: string | null;
  rejected_reason: string | null;
}

export interface ExpenseDetail extends Expense {
  contact_name: string | null;
  category_name: string | null;
  project_name: string | null;
}

export interface CreateExpense {
  contact_id?: string;
  category_id?: string;
  description: string;
  amount: number;
  currency_id: string;
  vat_rate_id?: string;
  expense_date: string;
  due_date?: string;
  project_id?: string;
}

export type UpdateExpense = CreateExpense;

export interface PayExpenseData {
  payment_account_id: string;
}

export interface ExpenseListParams {
  page?: number;
  per_page?: number;
  status?: string;
  category_id?: string;
  contact_id?: string;
  date_from?: string;
  date_to?: string;
  search?: string;
}

export interface ExpenseCategory {
  id: string;
  name: string;
  account_id: string | null;
  is_active: boolean;
}

export interface CreateExpenseCategory {
  name: string;
  account_id?: string;
}

export interface UpdateExpenseCategory {
  name: string;
  account_id?: string;
  is_active: boolean;
}

export interface ExpenseReceipt {
  id: string;
  expense_id: string;
  line_id: string | null;
  file_name: string;
  file_size: number;
  mime_type: string;
  uploaded_at: string;
}
