import client from './client';
import type { PayrollRun, PayrollRunDetail, CreatePayrollRun } from '@/types/payroll-run';

export const payrollRunsApi = {
  list: () => client.get<PayrollRun[]>('/payroll-runs'),
  get: (id: string) => client.get<PayrollRunDetail>(`/payroll-runs/${id}`),
  create: (data: CreatePayrollRun) => client.post<PayrollRun>('/payroll-runs', data),
  calculate: (id: string) => client.post<PayrollRunDetail>(`/payroll-runs/${id}/calculate`),
  approve: (id: string) => client.post<PayrollRun>(`/payroll-runs/${id}/approve`),
  markPaid: (id: string) => client.post<PayrollRun>(`/payroll-runs/${id}/pay`),
  delete: (id: string) => client.delete(`/payroll-runs/${id}`),
  downloadPayslip: (id: string, employeeId: string) =>
    client.get(`/payroll-runs/${id}/payslip/${employeeId}`, { responseType: 'blob' }),
  downloadPayslips: (id: string) =>
    client.get(`/payroll-runs/${id}/payslips`, { responseType: 'blob' }),
};
