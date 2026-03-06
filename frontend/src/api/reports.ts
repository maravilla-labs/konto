import client from './client';
import type { TrialBalanceEntry, BalanceSheetResponse, ProfitLossResponse, AccountLedgerEntry, VatReportResponse, VatPaymentResponse, ExportVatXmlRequest } from '@/types/report';

export interface TrialBalanceParams {
  as_of?: string;
  fiscal_year_id?: string;
}

export interface DateRangeParams {
  from_date?: string;
  to_date?: string;
}

export interface AccountLedgerParams extends DateRangeParams {
  account_id: string;
}

export const reportsApi = {
  trialBalance(params?: TrialBalanceParams) {
    return client.get<TrialBalanceEntry[]>('/reports/trial-balance', { params });
  },

  balanceSheet(params?: { as_of?: string }) {
    return client.get<BalanceSheetResponse>('/reports/balance-sheet', { params });
  },

  profitLoss(params?: DateRangeParams) {
    return client.get<ProfitLossResponse>('/reports/profit-loss', { params });
  },

  accountLedger(params: AccountLedgerParams) {
    return client.get<AccountLedgerEntry[]>(`/reports/ledger/${params.account_id}`, {
      params: { from_date: params.from_date, to_date: params.to_date },
    });
  },

  vatReport(params?: DateRangeParams) {
    return client.get<VatReportResponse>('/reports/vat', { params });
  },

  createVatPayment(data: {
    from_date: string;
    to_date: string;
    payment_date: string;
    bank_account_id: string;
  }) {
    return client.post<VatPaymentResponse>('/reports/vat/payment', data);
  },

  async exportVatXml(data: ExportVatXmlRequest) {
    const resp = await client.post('/reports/vat/xml', data, {
      responseType: 'blob',
    });
    const blob = new Blob([resp.data as BlobPart], { type: 'application/xml' });
    const url = window.URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `mwst-abrechnung-${data.from_date}-${data.to_date}.xml`;
    document.body.appendChild(a);
    a.click();
    window.URL.revokeObjectURL(url);
    a.remove();
  },
};
