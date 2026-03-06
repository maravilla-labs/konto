import client from './client';
import type { PayrollSettings, UpdatePayrollSettings } from '@/types/payroll-settings';

export const payrollSettingsApi = {
  get() {
    return client.get<PayrollSettings>('/payroll-settings');
  },
  update(data: UpdatePayrollSettings) {
    return client.put<PayrollSettings>('/payroll-settings', data);
  },
};
