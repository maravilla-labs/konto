import client from './client';
import type {
  Expense,
  ExpenseDetail,
  CreateExpense,
  UpdateExpense,
  PayExpenseData,
  ExpenseListParams,
  ExpenseCategory,
  CreateExpenseCategory,
  UpdateExpenseCategory,
  ExpenseReceipt,
} from '@/types/expense';
import type { PaginatedResponse } from '@/types/common';

export const expensesApi = {
  list(params?: ExpenseListParams) {
    return client.get<PaginatedResponse<Expense>>('/expenses', { params });
  },

  get(id: string) {
    return client.get<ExpenseDetail>(`/expenses/${id}`);
  },

  create(data: CreateExpense) {
    return client.post<Expense>('/expenses', data);
  },

  update(id: string, data: UpdateExpense) {
    return client.put<Expense>(`/expenses/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/expenses/${id}`);
  },

  approve(id: string) {
    return client.post<Expense>(`/expenses/${id}/approve`);
  },

  pay(id: string, data: PayExpenseData) {
    return client.post<Expense>(`/expenses/${id}/pay`, data);
  },

  cancel(id: string) {
    return client.post<Expense>(`/expenses/${id}/cancel`);
  },

  uploadReceipt(id: string, file: File) {
    const formData = new FormData();
    formData.append('file', file);
    return client.post<Expense>(`/expenses/${id}/receipt`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  listReceipts(id: string) {
    return client.get<ExpenseReceipt[]>(`/expenses/${id}/receipts`);
  },

  uploadReceiptNew(id: string, file: File) {
    const formData = new FormData();
    formData.append('file', file);
    return client.post<ExpenseReceipt>(`/expenses/${id}/receipts`, formData, {
      headers: { 'Content-Type': 'multipart/form-data' },
    });
  },

  deleteReceipt(id: string) {
    return client.delete(`/expenses/receipts/${id}`);
  },
};

export const expenseCategoriesApi = {
  list() {
    return client.get<ExpenseCategory[]>('/expense-categories');
  },

  create(data: CreateExpenseCategory) {
    return client.post<ExpenseCategory>('/expense-categories', data);
  },

  update(id: string, data: UpdateExpenseCategory) {
    return client.put<ExpenseCategory>(`/expense-categories/${id}`, data);
  },

  delete(id: string) {
    return client.delete(`/expense-categories/${id}`);
  },
};
