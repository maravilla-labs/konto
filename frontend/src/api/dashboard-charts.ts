import client from './client';
import type {
  MonthlyAmount,
  InvoiceAgingBucket,
  OutstandingContact,
  CashFlowReport,
  CashFlowMonthlyReport,
  AgingBucket,
  OverviewData,
} from '@/types/dashboard-charts';

export const dashboardChartsApi = {
  monthlyRevenue(months = 12) {
    return client.get<MonthlyAmount[]>('/dashboard/monthly-revenue', {
      params: { months },
    });
  },

  monthlyExpenses(months = 12) {
    return client.get<MonthlyAmount[]>('/dashboard/monthly-expenses', {
      params: { months },
    });
  },

  invoiceAging() {
    return client.get<InvoiceAgingBucket[]>('/dashboard/invoice-aging');
  },

  topOutstanding(limit = 5) {
    return client.get<OutstandingContact[]>('/dashboard/top-outstanding', {
      params: { limit },
    });
  },

  overview(year?: number) {
    return client.get<OverviewData>('/dashboard/overview', {
      params: year ? { year } : undefined,
    });
  },

  cashFlow(params: { from_date: string; to_date: string }) {
    return client.get<CashFlowReport>('/reports/cash-flow', { params });
  },

  monthlyCashFlow(params: { from_date: string; to_date: string }) {
    return client.get<CashFlowMonthlyReport>('/reports/cash-flow/monthly', { params });
  },

  arAging() {
    return client.get<AgingBucket[]>('/reports/ar-aging');
  },

  apAging() {
    return client.get<AgingBucket[]>('/reports/ap-aging');
  },
};
