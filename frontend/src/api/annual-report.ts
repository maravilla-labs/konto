import client from './client';
import type {
  Shareholder,
  CreateShareholder,
  UpdateShareholder,
  AnnualReportNote,
  UpdateNoteRequest,
  CreateNoteRequest,
  AnnualReport,
  SwissBalanceSheet,
  SwissIncomeStatement,
} from '@/types/annual-report';

export const shareholdersApi = {
  list() {
    return client.get<Shareholder[]>('/shareholders');
  },
  create(data: CreateShareholder) {
    return client.post<Shareholder>('/shareholders', data);
  },
  update(id: string, data: UpdateShareholder) {
    return client.put<Shareholder>(`/shareholders/${id}`, data);
  },
  delete(id: string) {
    return client.delete(`/shareholders/${id}`);
  },
};

export const annualReportNotesApi = {
  list(fiscalYearId: string) {
    return client.get<AnnualReportNote[]>(`/fiscal-years/${fiscalYearId}/notes`);
  },
  get(fiscalYearId: string, section: string) {
    return client.get<AnnualReportNote>(
      `/fiscal-years/${fiscalYearId}/notes/${section}`,
    );
  },
  update(fiscalYearId: string, section: string, data: UpdateNoteRequest) {
    return client.put<AnnualReportNote>(
      `/fiscal-years/${fiscalYearId}/notes/${section}`,
      data,
    );
  },
  create(fiscalYearId: string, data: CreateNoteRequest) {
    return client.post<AnnualReportNote>(
      `/fiscal-years/${fiscalYearId}/notes`,
      data,
    );
  },
  delete(fiscalYearId: string, sectionKey: string) {
    return client.delete(
      `/fiscal-years/${fiscalYearId}/notes/${sectionKey}`,
    );
  },
};

export const annualReportApi = {
  get(fiscalYearId: string) {
    return client.get<AnnualReport>(
      `/fiscal-years/${fiscalYearId}/annual-report`,
    );
  },
  generate(fiscalYearId: string) {
    return client.post<AnnualReport>(
      `/fiscal-years/${fiscalYearId}/annual-report/generate`,
    );
  },
  downloadPdf(fiscalYearId: string) {
    return client.get(`/fiscal-years/${fiscalYearId}/annual-report/pdf`, {
      responseType: 'blob',
    });
  },
  finalize(fiscalYearId: string) {
    return client.post<AnnualReport>(
      `/fiscal-years/${fiscalYearId}/annual-report/finalize`,
    );
  },
};

export const swissReportsApi = {
  balanceSheet(asOf: string) {
    return client.get<SwissBalanceSheet>('/reports/swiss-balance-sheet', {
      params: { as_of: asOf },
    });
  },
  incomeStatement(fromDate: string, toDate: string) {
    return client.get<SwissIncomeStatement>('/reports/swiss-income-statement', {
      params: { from_date: fromDate, to_date: toDate },
    });
  },
};
