import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import { accountsApi } from '@/api/accounts';
import { contactsApi } from '@/api/contacts';
import { journalApi, type JournalListParams } from '@/api/journal';
import { projectsApi } from '@/api/projects';
import { importsApi } from '@/api/imports';
import { fiscalYearsApi } from '@/api/fiscal-years';
import { exchangeRatesApi } from '@/api/exchange-rates';
import { reportsApi, type TrialBalanceParams, type DateRangeParams, type AccountLedgerParams } from '@/api/reports';
import { dashboardApi } from '@/api/dashboard';
import { timeEntriesApi, type CreateTimeEntry, type UpdateTimeEntry } from '@/api/time-entries';
import type { ListParams } from '@/types/common';
import type { CreateAccount, UpdateAccount } from '@/types/accounts';
import type { CreateContact, UpdateContact } from '@/types/contacts';
import type { CreateJournalEntry } from '@/types/journal';
import type { CreateProject, UpdateProject } from '@/types/projects';
import type { ImportType } from '@/types/imports';
import type { CreateFiscalYear } from '@/types/fiscal-year';
import type { CreateExchangeRate } from '@/types/exchange-rate';
import { invoicesApi } from '@/api/invoices';
import type { CreateInvoice, UpdateInvoice, PayInvoiceData, InvoiceListParams } from '@/types/invoice';
import { usersApi } from '@/api/users';
import type { CreateUser, UpdateUser, ChangePassword } from '@/types/user';
import { dashboardChartsApi } from '@/api/dashboard-charts';
import { recurringInvoicesApi } from '@/api/recurring-invoices';
import type { CreateRecurringInvoice, UpdateRecurringInvoice, RecurringInvoiceListParams } from '@/types/recurring-invoice';
import { creditNotesApi } from '@/api/credit-notes';
import type { CreateCreditNote, UpdateCreditNote, CreditNoteListParams } from '@/types/credit-note';
import { expensesApi, expenseCategoriesApi } from '@/api/expenses';
import type { CreateExpense, UpdateExpense, PayExpenseData, ExpenseListParams, CreateExpenseCategory, UpdateExpenseCategory } from '@/types/expense';
import { dunningApi } from '@/api/dunning';
import type { UpdateDunningLevel, SendReminderData } from '@/types/dunning';
import { vatRatesApi } from '@/api/vat-rates';
import type { CreateVatRate, UpdateVatRate } from '@/types/vat-rate';
import { currenciesApi } from '@/api/currencies';
import type { CreateCurrency, UpdateCurrency } from '@/types/currency';
import { activityTypesApi } from '@/api/activity-types';
import type { CreateActivityType, UpdateActivityType } from '@/types/activity-type';
import { invoicePaymentsApi } from '@/api/invoice-payments';
import type { RecordPaymentData } from '@/types/invoice-payment';
import { contactTagsApi } from '@/api/contact-tags';
import { contactPersonsApi } from '@/api/contact-persons';
import type { CreateContactTag, CreateContactPerson, UpdateContactPerson } from '@/types/contacts';
import { rateFunctionsApi } from '@/api/rate-functions';
import type { CreateRateFunctionRequest, UpdateRateFunctionRequest } from '@/types/rate-function';
import { projectMembersApi } from '@/api/project-members';
import type { AddProjectMemberRequest, UpdateProjectMemberRequest } from '@/types/project-member';
import { projectItemsApi } from '@/api/project-items';
import type { CreateProjectItemRequest, UpdateProjectItemRequest } from '@/types/project-item';
import { projectMilestonesApi } from '@/api/project-milestones';
import type { CreateProjectMilestoneRequest, UpdateProjectMilestoneRequest } from '@/types/project-milestone';
import { projectDocumentsApi } from '@/api/project-documents';
import { projectActivityTypesApi } from '@/api/project-activity-types';
import type { AddProjectActivityTypeRequest, UpdateProjectActivityTypeRequest } from '@/types/project-activity-type';
import { contactRelationshipsApi } from '@/api/contact-relationships';
import type { CreateContactRelationshipRequest, UpdateContactRelationshipRequest } from '@/types/contact-relationship';
import { timesheetsApi } from '@/api/timesheets';
import type { CreateTimesheetRequest, UpdateTimesheetRequest, TimesheetListParams } from '@/types/timesheet';
import { defaultAccountsApi } from '@/api/default-accounts';
import { employeesApi } from '@/api/employees';
import type { CreateEmployee, UpdateEmployee } from '@/types/employee';
import { payrollSettingsApi } from '@/api/payroll-settings';
import type { UpdatePayrollSettings } from '@/types/payroll-settings';
import { payrollRunsApi } from '@/api/payroll-runs';
import type { CreatePayrollRun } from '@/types/payroll-run';
import { salaryCertificatesApi } from '@/api/salary-certificates';
import { payoutEntriesApi } from '@/api/payout-entries';
import { projectSubStatusesApi } from '@/api/project-sub-statuses';
import type { CreateProjectSubStatus, UpdateProjectSubStatus } from '@/types/project-sub-status';

type CashFlowDateParams = {
  from_date: string;
  to_date: string;
};

// Accounts
export function useAccountTree() {
  return useQuery({
    queryKey: ['accounts', 'tree'],
    queryFn: () => accountsApi.tree().then((r) => r.data),
  });
}

export function useAccountTreeWithBalances() {
  return useQuery({
    queryKey: ['accounts', 'tree-with-balances'],
    queryFn: () => accountsApi.treeWithBalances().then((r) => r.data),
  });
}

export function useAccounts(params?: ListParams) {
  return useQuery({
    queryKey: ['accounts', params],
    queryFn: () => accountsApi.list(params).then((r) => r.data),
  });
}

export function useCreateAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateAccount) => accountsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['accounts'] }),
  });
}

export function useUpdateAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateAccount }) =>
      accountsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['accounts'] }),
  });
}

// Contacts
export function useContacts(params?: ListParams) {
  return useQuery({
    queryKey: ['contacts', params],
    queryFn: () => contactsApi.list(params).then((r) => r.data),
  });
}

export function useCreateContact() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateContact) => contactsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contacts'] }),
  });
}

export function useUpdateContact() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateContact }) =>
      contactsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contacts'] }),
  });
}

// Journal
export function useJournalEntries(params?: JournalListParams) {
  return useQuery({
    queryKey: ['journal', params],
    queryFn: () => journalApi.list(params).then((r) => r.data),
  });
}

export function useCreateJournalEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateJournalEntry) => journalApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

export function usePostJournal() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => journalApi.post(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

export function useBulkPostJournal() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (params: { entryIds?: string[]; allDrafts?: boolean }) =>
      journalApi.bulkPost(params.entryIds, params.allDrafts).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

export function useReverseJournal() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => journalApi.reverse(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

export function useJournalDetail(id: string | undefined) {
  return useQuery({
    queryKey: ['journal', id],
    queryFn: () => journalApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useJournalAttachments(entryId: string | undefined) {
  return useQuery({
    queryKey: ['journal', entryId, 'attachments'],
    queryFn: () => journalApi.listAttachments(entryId!).then((r) => r.data),
    enabled: !!entryId,
  });
}

export function useUploadJournalAttachment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ entryId, file }: { entryId: string; file: File }) =>
      journalApi.uploadAttachment(entryId, file),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

export function useDeleteJournalAttachment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => journalApi.deleteAttachment(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['journal'] }),
  });
}

// Projects
export function useProjects(params?: ListParams) {
  return useQuery({
    queryKey: ['projects', params],
    queryFn: () => projectsApi.list(params).then((r) => r.data),
  });
}

export function useCreateProject() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateProject) => projectsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['projects'] }),
  });
}

export function useUpdateProject() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateProject }) =>
      projectsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['projects'] }),
  });
}

// Time Entries
export function useTimeEntries(params?: ListParams & { project_id?: string; billed?: boolean }) {
  return useQuery({
    queryKey: ['time-entries', params],
    queryFn: () => timeEntriesApi.list(params).then((r) => r.data),
  });
}

export function useCreateTimeEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateTimeEntry) => timeEntriesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['time-entries'] }),
  });
}

export function useUpdateTimeEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateTimeEntry }) =>
      timeEntriesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['time-entries'] }),
  });
}

export function useDeleteTimeEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => timeEntriesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['time-entries'] }),
  });
}

// Fiscal Years
export function useFiscalYears(params?: ListParams) {
  return useQuery({
    queryKey: ['fiscal-years', params],
    queryFn: () => fiscalYearsApi.list(params).then((r) => r.data),
  });
}

export function useCreateFiscalYear() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateFiscalYear) => fiscalYearsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fiscal-years'] }),
  });
}

export function useUpdateFiscalYear() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: { name?: string; start_date?: string; end_date?: string } }) =>
      fiscalYearsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fiscal-years'] }),
  });
}

export function useCloseFiscalYear() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => fiscalYearsApi.close(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['fiscal-years'] }),
  });
}

// Exchange Rates
export function useExchangeRates(params?: ListParams) {
  return useQuery({
    queryKey: ['exchange-rates', params],
    queryFn: () => exchangeRatesApi.list(params).then((r) => r.data),
  });
}

export function useCreateExchangeRate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateExchangeRate) => exchangeRatesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['exchange-rates'] }),
  });
}

export function useDeleteExchangeRate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => exchangeRatesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['exchange-rates'] }),
  });
}

// Reports
export function useTrialBalance(params?: TrialBalanceParams) {
  return useQuery({
    queryKey: ['reports', 'trial-balance', params],
    queryFn: () => reportsApi.trialBalance(params).then((r) => r.data),
    enabled: false,
  });
}

export function useBalanceSheet(params?: { as_of?: string }) {
  return useQuery({
    queryKey: ['reports', 'balance-sheet', params],
    queryFn: () => reportsApi.balanceSheet(params).then((r) => r.data),
    enabled: false,
  });
}

export function useProfitLoss(params?: DateRangeParams) {
  return useQuery({
    queryKey: ['reports', 'profit-loss', params],
    queryFn: () => reportsApi.profitLoss(params).then((r) => r.data),
    enabled: false,
  });
}

export function useAccountLedger(params: AccountLedgerParams | null) {
  return useQuery({
    queryKey: ['reports', 'account-ledger', params],
    queryFn: () => reportsApi.accountLedger(params!).then((r) => r.data),
    enabled: !!params?.account_id,
  });
}

export function useVatReport(params?: DateRangeParams) {
  return useQuery({
    queryKey: ['reports', 'vat-report', params],
    queryFn: () => reportsApi.vatReport(params).then((r) => r.data),
    enabled: false,
  });
}

export function useCreateVatPayment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: { from_date: string; to_date: string; payment_date: string; bank_account_id: string }) =>
      reportsApi.createVatPayment(data).then((r) => r.data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['reports', 'vat-report'] }),
  });
}

// Dashboard
export function useDashboardStats() {
  return useQuery({
    queryKey: ['dashboard', 'stats'],
    queryFn: () => dashboardApi.stats().then((r) => r.data),
  });
}

// Imports
export function useUploadImport() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ file, importType }: { file: File; importType: ImportType }) =>
      importsApi.upload(file, importType),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['imports'] }),
  });
}

export function useImportPreview(batchId: string | null) {
  return useQuery({
    queryKey: ['imports', batchId, 'preview'],
    queryFn: () => importsApi.preview(batchId!).then((r) => r.data),
    enabled: !!batchId,
    retry: false,
  });
}

export function useExecuteImport() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (batchId: string) => importsApi.execute(batchId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['imports'] }),
  });
}

// Invoices
export function useInvoices(params?: InvoiceListParams) {
  return useQuery({
    queryKey: ['invoices', params],
    queryFn: () => invoicesApi.list(params).then((r) => r.data),
  });
}

export function useInvoice(id: string | undefined) {
  return useQuery({
    queryKey: ['invoices', id],
    queryFn: () => invoicesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateInvoice) => invoicesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

export function useUpdateInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateInvoice }) =>
      invoicesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

export function useDeleteInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => invoicesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

export function useSendInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => invoicesApi.send(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

export function usePayInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: PayInvoiceData }) =>
      invoicesApi.pay(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

export function useCancelInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => invoicesApi.cancel(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['invoices'] }),
  });
}

// Users
export function useUsers() {
  return useQuery({
    queryKey: ['users'],
    queryFn: () => usersApi.list().then((r) => r.data),
  });
}

export function useRoles() {
  return useQuery({
    queryKey: ['roles'],
    queryFn: () => usersApi.listRoles().then((r) => r.data),
  });
}

export function useCreateUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateUser) => usersApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['users'] }),
  });
}

export function useUpdateUser() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateUser }) =>
      usersApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['users'] }),
  });
}

export function useChangePassword() {
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: ChangePassword }) =>
      usersApi.changePassword(id, data),
  });
}

// Recurring Invoices
export function useRecurringInvoices(params?: RecurringInvoiceListParams) {
  return useQuery({
    queryKey: ['recurring-invoices', params],
    queryFn: () => recurringInvoicesApi.list(params).then((r) => r.data),
  });
}

export function useRecurringInvoice(id: string | undefined) {
  return useQuery({
    queryKey: ['recurring-invoices', id],
    queryFn: () => recurringInvoicesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateRecurringInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateRecurringInvoice) => recurringInvoicesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['recurring-invoices'] }),
  });
}

export function useUpdateRecurringInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateRecurringInvoice }) =>
      recurringInvoicesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['recurring-invoices'] }),
  });
}

export function useDeleteRecurringInvoice() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => recurringInvoicesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['recurring-invoices'] }),
  });
}

export function useTriggerRecurringInvoices() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => recurringInvoicesApi.trigger(),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['recurring-invoices'] });
      qc.invalidateQueries({ queryKey: ['invoices'] });
    },
  });
}

// Credit Notes
export function useCreditNotes(params?: CreditNoteListParams) {
  return useQuery({
    queryKey: ['credit-notes', params],
    queryFn: () => creditNotesApi.list(params).then((r) => r.data),
  });
}

export function useCreditNote(id: string | undefined) {
  return useQuery({
    queryKey: ['credit-notes', id],
    queryFn: () => creditNotesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateCreditNote) => creditNotesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

export function useUpdateCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateCreditNote }) =>
      creditNotesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

export function useDeleteCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => creditNotesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

export function useIssueCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => creditNotesApi.issue(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

export function useApplyCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => creditNotesApi.apply(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

export function useCancelCreditNote() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => creditNotesApi.cancel(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['credit-notes'] }),
  });
}

// Dashboard Charts
export function useMonthlyRevenue(months = 12) {
  return useQuery({
    queryKey: ['dashboard', 'monthly-revenue', months],
    queryFn: () => dashboardChartsApi.monthlyRevenue(months).then((r) => r.data),
  });
}

export function useMonthlyExpenses(months = 12) {
  return useQuery({
    queryKey: ['dashboard', 'monthly-expenses', months],
    queryFn: () => dashboardChartsApi.monthlyExpenses(months).then((r) => r.data),
  });
}

export function useInvoiceAging() {
  return useQuery({
    queryKey: ['dashboard', 'invoice-aging'],
    queryFn: () => dashboardChartsApi.invoiceAging().then((r) => r.data),
  });
}

export function useTopOutstanding(limit = 5) {
  return useQuery({
    queryKey: ['dashboard', 'top-outstanding', limit],
    queryFn: () => dashboardChartsApi.topOutstanding(limit).then((r) => r.data),
  });
}

export function useDashboardOverview(year?: number) {
  return useQuery({
    queryKey: ['dashboard', 'overview', year],
    queryFn: () => dashboardChartsApi.overview(year).then((r) => r.data),
  });
}

export function useCashFlow(params: CashFlowDateParams) {
  return useQuery({
    queryKey: ['reports', 'cash-flow', params],
    queryFn: () => dashboardChartsApi.cashFlow(params).then((r) => r.data),
    enabled: false,
  });
}

export function useMonthlyCashFlow(params: CashFlowDateParams) {
  return useQuery({
    queryKey: ['reports', 'cash-flow', 'monthly', params],
    queryFn: () => dashboardChartsApi.monthlyCashFlow(params).then((r) => r.data),
    enabled: false,
  });
}

export function useArAging() {
  return useQuery({
    queryKey: ['reports', 'ar-aging'],
    queryFn: () => dashboardChartsApi.arAging().then((r) => r.data),
    enabled: false,
  });
}

export function useApAging() {
  return useQuery({
    queryKey: ['reports', 'ap-aging'],
    queryFn: () => dashboardChartsApi.apAging().then((r) => r.data),
    enabled: false,
  });
}

// Expenses
export function useExpenses(params?: ExpenseListParams) {
  return useQuery({
    queryKey: ['expenses', params],
    queryFn: () => expensesApi.list(params).then((r) => r.data),
  });
}

export function useExpense(id: string | undefined) {
  return useQuery({
    queryKey: ['expenses', id],
    queryFn: () => expensesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateExpense) => expensesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function useUpdateExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateExpense }) =>
      expensesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function useDeleteExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => expensesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function useApproveExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => expensesApi.approve(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function usePayExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: PayExpenseData }) =>
      expensesApi.pay(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function useCancelExpense() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => expensesApi.cancel(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

export function useUploadReceipt() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, file }: { id: string; file: File }) =>
      expensesApi.uploadReceipt(id, file),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expenses'] }),
  });
}

// Expense Categories
export function useExpenseCategories() {
  return useQuery({
    queryKey: ['expense-categories'],
    queryFn: () => expenseCategoriesApi.list().then((r) => r.data),
  });
}

export function useCreateExpenseCategory() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateExpenseCategory) => expenseCategoriesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expense-categories'] }),
  });
}

export function useUpdateExpenseCategory() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateExpenseCategory }) =>
      expenseCategoriesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expense-categories'] }),
  });
}

export function useDeleteExpenseCategory() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => expenseCategoriesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['expense-categories'] }),
  });
}

// Contact Tags
export function useContactTags() {
  return useQuery({
    queryKey: ['contact-tags'],
    queryFn: () => contactTagsApi.list().then((r) => r.data),
  });
}

export function useCreateContactTag() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateContactTag) => contactTagsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-tags'] }),
  });
}

export function useDeleteContactTag() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => contactTagsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-tags'] }),
  });
}

export function useAssignContactTag() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, tagId }: { contactId: string; tagId: string }) =>
      contactTagsApi.assign(contactId, tagId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-tags'] }),
  });
}

export function useRemoveContactTag() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, tagId }: { contactId: string; tagId: string }) =>
      contactTagsApi.remove(contactId, tagId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-tags'] }),
  });
}

// Contact Persons
export function useContactPersons(contactId: string | undefined) {
  return useQuery({
    queryKey: ['contact-persons', contactId],
    queryFn: () => contactPersonsApi.list(contactId!).then((r) => r.data),
    enabled: !!contactId,
  });
}

export function useCreateContactPerson() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, data }: { contactId: string; data: CreateContactPerson }) =>
      contactPersonsApi.create(contactId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-persons'] }),
  });
}

export function useUpdateContactPerson() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, personId, data }: { contactId: string; personId: string; data: UpdateContactPerson }) =>
      contactPersonsApi.update(contactId, personId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-persons'] }),
  });
}

export function useDeleteContactPerson() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, personId }: { contactId: string; personId: string }) =>
      contactPersonsApi.delete(contactId, personId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-persons'] }),
  });
}

// Contact Sub-resources
export function useContactInvoices(contactId: string | undefined, params?: ListParams) {
  return useQuery({
    queryKey: ['contacts', contactId, 'invoices', params],
    queryFn: () => contactsApi.invoices(contactId!, params).then((r) => r.data),
    enabled: !!contactId,
  });
}

export function useContactDocuments(contactId: string | undefined, params?: ListParams) {
  return useQuery({
    queryKey: ['contacts', contactId, 'documents', params],
    queryFn: () => contactsApi.documents(contactId!, params).then((r) => r.data),
    enabled: !!contactId,
  });
}

export function useContactTimeEntries(contactId: string | undefined, params?: ListParams) {
  return useQuery({
    queryKey: ['contacts', contactId, 'time-entries', params],
    queryFn: () => contactsApi.timeEntries(contactId!, params).then((r) => r.data),
    enabled: !!contactId,
  });
}

// Project Summary
export function useProjectSummary(id: string | undefined) {
  return useQuery({
    queryKey: ['projects', id, 'summary'],
    queryFn: () => projectsApi.summary(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useDeleteProject() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => projectsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['projects'] }),
  });
}

// Create Invoice from Time Entries
export function useCreateInvoiceFromTimeEntries() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: {
      time_entry_ids: string[];
      contact_id: string;
      project_id?: string;
      language?: string;
      hourly_rate?: string;
      account_id: string;
    }) => invoicesApi.createFromTimeEntries(data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['invoices'] });
      qc.invalidateQueries({ queryKey: ['time-entries'] });
    },
  });
}

// Contact Detail (single contact)
export function useContact(id: string | undefined) {
  return useQuery({
    queryKey: ['contacts', id],
    queryFn: () => contactsApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

// Dunning
export function useDunningLevels() {
  return useQuery({
    queryKey: ['dunning-levels'],
    queryFn: () => dunningApi.listLevels().then((r) => r.data),
  });
}

export function useUpdateDunningLevel() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateDunningLevel }) =>
      dunningApi.updateLevel(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['dunning-levels'] }),
  });
}

export function useInvoiceDunning(invoiceId: string | undefined) {
  return useQuery({
    queryKey: ['invoices', invoiceId, 'dunning'],
    queryFn: () => dunningApi.getInvoiceDunning(invoiceId!).then((r) => r.data),
    enabled: !!invoiceId,
  });
}

export function useSendReminder() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ invoiceId, data }: { invoiceId: string; data: SendReminderData }) =>
      dunningApi.sendReminder(invoiceId, data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['invoices'] });
      qc.invalidateQueries({ queryKey: ['dunning-levels'] });
    },
  });
}

export function useRunDunning() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: () => dunningApi.runDunning(),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['invoices'] });
      qc.invalidateQueries({ queryKey: ['dunning-levels'] });
    },
  });
}

// VAT Rates
export function useVatRates() {
  return useQuery({
    queryKey: ['vat-rates'],
    queryFn: () => vatRatesApi.list().then((r) => r.data),
  });
}

export function useCreateVatRate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateVatRate) => vatRatesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['vat-rates'] }),
  });
}

export function useUpdateVatRate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateVatRate }) => vatRatesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['vat-rates'] }),
  });
}

export function useDeactivateVatRate() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => vatRatesApi.deactivate(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['vat-rates'] }),
  });
}

// Currencies
export function useCurrencies() {
  return useQuery({
    queryKey: ['currencies'],
    queryFn: () => currenciesApi.list().then((r) => r.data),
  });
}

export function useCreateCurrency() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateCurrency) => currenciesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['currencies'] }),
  });
}

export function useUpdateCurrency() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateCurrency }) => currenciesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['currencies'] }),
  });
}

// Activity Types
export function useActivityTypes() {
  return useQuery({
    queryKey: ['activity-types'],
    queryFn: () => activityTypesApi.list().then((r) => r.data),
  });
}

export function useCreateActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateActivityType) => activityTypesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['activity-types'] }),
  });
}

export function useUpdateActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateActivityType }) => activityTypesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['activity-types'] }),
  });
}

export function useDeleteActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => activityTypesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['activity-types'] }),
  });
}

// Invoice Payments
export function useInvoicePayments(invoiceId: string | undefined) {
  return useQuery({
    queryKey: ['invoice-payments', invoiceId],
    queryFn: () => invoicePaymentsApi.list(invoiceId!).then((r) => r.data),
    enabled: !!invoiceId,
  });
}

export function useRecordPayment() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ invoiceId, data }: { invoiceId: string; data: RecordPaymentData }) =>
      invoicePaymentsApi.record(invoiceId, data),
    onSuccess: () => {
      qc.invalidateQueries({ queryKey: ['invoices'] });
      qc.invalidateQueries({ queryKey: ['invoice-payments'] });
    },
  });
}

// Rate Functions
export function useRateFunctions() {
  return useQuery({
    queryKey: ['rate-functions'],
    queryFn: () => rateFunctionsApi.list().then((r) => r.data),
  });
}

export function useCreateRateFunction() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateRateFunctionRequest) => rateFunctionsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['rate-functions'] }),
  });
}

export function useUpdateRateFunction() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateRateFunctionRequest }) =>
      rateFunctionsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['rate-functions'] }),
  });
}

export function useDeleteRateFunction() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => rateFunctionsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['rate-functions'] }),
  });
}

// Contact Relationships
export function useContactRelationships(contactId: string | undefined) {
  return useQuery({
    queryKey: ['contact-relationships', contactId],
    queryFn: () => contactRelationshipsApi.list(contactId!).then((r) => r.data),
    enabled: !!contactId,
  });
}

export function useCreateContactRelationship() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ contactId, data }: { contactId: string; data: CreateContactRelationshipRequest }) => contactRelationshipsApi.create(contactId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-relationships'] }),
  });
}

export function useUpdateContactRelationship() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateContactRelationshipRequest }) =>
      contactRelationshipsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-relationships'] }),
  });
}

export function useDeleteContactRelationship() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => contactRelationshipsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['contact-relationships'] }),
  });
}

// Project Members
export function useProjectMembers(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-members', projectId],
    queryFn: () => projectMembersApi.list(projectId!).then((r) => r.data),
    enabled: !!projectId,
  });
}

export function useAddProjectMember() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, data }: { projectId: string; data: AddProjectMemberRequest }) =>
      projectMembersApi.add(projectId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-members'] }),
  });
}

export function useUpdateProjectMember() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, memberId, data }: { projectId: string; memberId: string; data: UpdateProjectMemberRequest }) =>
      projectMembersApi.update(projectId, memberId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-members'] }),
  });
}

export function useRemoveProjectMember() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, memberId }: { projectId: string; memberId: string }) =>
      projectMembersApi.remove(projectId, memberId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-members'] }),
  });
}

// Project Items (WBS)
export function useProjectItems(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-items', projectId],
    queryFn: () => projectItemsApi.list(projectId!).then((r) => r.data),
    enabled: !!projectId,
  });
}

export function useCreateProjectItem() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, data }: { projectId: string; data: CreateProjectItemRequest }) =>
      projectItemsApi.create(projectId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-items'] }),
  });
}

export function useUpdateProjectItem() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ itemId, data }: { itemId: string; data: UpdateProjectItemRequest }) =>
      projectItemsApi.update(itemId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-items'] }),
  });
}

export function useDeleteProjectItem() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (itemId: string) =>
      projectItemsApi.delete(itemId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-items'] }),
  });
}

// Project Milestones
export function useProjectMilestones(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-milestones', projectId],
    queryFn: () => projectMilestonesApi.list(projectId!).then((r) => r.data),
    enabled: !!projectId,
  });
}

export function useCreateProjectMilestone() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, data }: { projectId: string; data: CreateProjectMilestoneRequest }) =>
      projectMilestonesApi.create(projectId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-milestones'] }),
  });
}

export function useUpdateProjectMilestone() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ milestoneId, data }: { milestoneId: string; data: UpdateProjectMilestoneRequest }) =>
      projectMilestonesApi.update(milestoneId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-milestones'] }),
  });
}

export function useReachProjectMilestone() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (milestoneId: string) =>
      projectMilestonesApi.reach(milestoneId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-milestones'] }),
  });
}

export function useDeleteProjectMilestone() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (milestoneId: string) =>
      projectMilestonesApi.delete(milestoneId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-milestones'] }),
  });
}

// Project Documents
export function useProjectDocuments(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-documents', projectId],
    queryFn: () => projectDocumentsApi.list(projectId!).then((r) => r.data),
    enabled: !!projectId,
  });
}

export function useUploadProjectDocument() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, file, title }: { projectId: string; file: File; title?: string }) =>
      projectDocumentsApi.upload(projectId, file, title),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-documents'] }),
  });
}

export function useDeleteProjectDocument() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (fileId: string) =>
      projectDocumentsApi.delete(fileId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-documents'] }),
  });
}

// Project Activity Types
export function useProjectActivityTypes(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-activity-types', projectId],
    queryFn: () => projectActivityTypesApi.list(projectId!).then((r) => r.data),
    enabled: !!projectId,
  });
}

export function useAddProjectActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, data }: { projectId: string; data: AddProjectActivityTypeRequest }) =>
      projectActivityTypesApi.add(projectId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-activity-types'] }),
  });
}

export function useUpdateProjectActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, patId, data }: { projectId: string; patId: string; data: UpdateProjectActivityTypeRequest }) =>
      projectActivityTypesApi.update(projectId, patId, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-activity-types'] }),
  });
}

export function useRemoveProjectActivityType() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ projectId, patId }: { projectId: string; patId: string }) =>
      projectActivityTypesApi.remove(projectId, patId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-activity-types'] }),
  });
}

// Default Accounts
export function useDefaultAccounts() {
  return useQuery({
    queryKey: ['default-accounts'],
    queryFn: () => defaultAccountsApi.list().then((r) => r.data),
  });
}

export function useUpdateDefaultAccount() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (settings: { setting_key: string; account_id: string | null }[]) =>
      defaultAccountsApi.update(settings),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['default-accounts'] }),
  });
}

// Timesheets
export function useTimesheets(params?: TimesheetListParams) {
  return useQuery({
    queryKey: ['timesheets', params],
    queryFn: () => timesheetsApi.list(params).then((r) => r.data),
  });
}

export function useTimesheet(id: string | undefined) {
  return useQuery({
    queryKey: ['timesheets', id],
    queryFn: () => timesheetsApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateTimesheetRequest) => timesheetsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

export function useUpdateTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateTimesheetRequest }) =>
      timesheetsApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

export function useSubmitTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => timesheetsApi.submit(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

export function useApproveTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => timesheetsApi.approve(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

export function useRejectTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => timesheetsApi.reject(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

export function useDeleteTimesheet() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => timesheetsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['timesheets'] }),
  });
}

// Employees
export function useEmployees() {
  return useQuery({
    queryKey: ['employees'],
    queryFn: () => employeesApi.list().then((r) => r.data),
  });
}

export function useEmployee(id: string | undefined) {
  return useQuery({
    queryKey: ['employees', id],
    queryFn: () => employeesApi.get(id!).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreateEmployee() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateEmployee) => employeesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['employees'] }),
  });
}

export function useUpdateEmployee() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateEmployee }) =>
      employeesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['employees'] }),
  });
}

export function useDeleteEmployee() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => employeesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['employees'] }),
  });
}

// Payroll Settings
export function usePayrollSettings() {
  return useQuery({
    queryKey: ['payroll-settings'],
    queryFn: () => payrollSettingsApi.get().then((r) => r.data),
  });
}

export function useUpdatePayrollSettings() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: UpdatePayrollSettings) => payrollSettingsApi.update(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-settings'] }),
  });
}

// Payroll Runs
export function usePayrollRuns() {
  return useQuery({
    queryKey: ['payroll-runs'],
    queryFn: () => payrollRunsApi.list().then((r) => r.data),
  });
}

export function usePayrollRun(id: string) {
  return useQuery({
    queryKey: ['payroll-runs', id],
    queryFn: () => payrollRunsApi.get(id).then((r) => r.data),
    enabled: !!id,
  });
}

export function useCreatePayrollRun() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreatePayrollRun) => payrollRunsApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-runs'] }),
  });
}

export function useCalculatePayrollRun() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => payrollRunsApi.calculate(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-runs'] }),
  });
}

export function useApprovePayrollRun() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => payrollRunsApi.approve(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-runs'] }),
  });
}

export function useMarkPayrollRunPaid() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => payrollRunsApi.markPaid(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-runs'] }),
  });
}

export function useDeletePayrollRun() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => payrollRunsApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payroll-runs'] }),
  });
}

// Salary Certificates
export function useSalaryCertificates(year: number) {
  return useQuery({
    queryKey: ['salary-certificates', year],
    queryFn: () => salaryCertificatesApi.list(year).then((r) => r.data),
  });
}

// Payout Entries
export function usePayoutEntries(runId: string) {
  return useQuery({
    queryKey: ['payout-entries', runId],
    queryFn: () => payoutEntriesApi.listByRun(runId).then((r) => r.data),
    enabled: !!runId,
  });
}

export function useGeneratePayouts() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (runId: string) => payoutEntriesApi.generatePayouts(runId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payout-entries'] }),
  });
}

export function useExportPain001() {
  return useMutation({
    mutationFn: (runId: string) => payoutEntriesApi.exportPain001(runId),
  });
}

export function useMarkPayoutPaid() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (entryId: string) => payoutEntriesApi.markPaid(entryId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payout-entries'] }),
  });
}

export function useMarkAllPayoutsPaid() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (runId: string) => payoutEntriesApi.markAllPaid(runId),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['payout-entries'] }),
  });
}

// Salary Certificates (with enabled guard)
export function useSalaryCertificatesList(year: number) {
  return useQuery({
    queryKey: ['salary-certificates', year],
    queryFn: () => salaryCertificatesApi.list(year).then((r) => r.data),
    enabled: year > 0,
  });
}

// Project Budget Analytics
export function useProjectBudgetAnalytics(projectId: string | undefined) {
  return useQuery({
    queryKey: ['project-budget-analytics', projectId],
    queryFn: () => projectsApi.getBudgetAnalytics(projectId!),
    enabled: !!projectId,
  });
}

// Project Sub-Statuses
export function useProjectSubStatuses() {
  return useQuery({
    queryKey: ['project-sub-statuses'],
    queryFn: () => projectSubStatusesApi.list(),
  });
}

export function useCreateProjectSubStatus() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (data: CreateProjectSubStatus) => projectSubStatusesApi.create(data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-sub-statuses'] }),
  });
}

export function useUpdateProjectSubStatus() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, data }: { id: string; data: UpdateProjectSubStatus }) =>
      projectSubStatusesApi.update(id, data),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-sub-statuses'] }),
  });
}

export function useDeleteProjectSubStatus() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: (id: string) => projectSubStatusesApi.delete(id),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['project-sub-statuses'] }),
  });
}

// Contact Persons via Relationships (new /persons endpoint)
export function useContactPersonsViaRelationships(contactId?: string) {
  return useQuery({
    queryKey: ['contact-persons-via-relationships', contactId],
    queryFn: () => contactsApi.persons(contactId!).then((r) => r.data),
    enabled: !!contactId,
  });
}

// Contact VAT Info
export function useContactVatInfo(contactId?: string) {
  return useQuery({
    queryKey: ['contact-vat-info', contactId],
    queryFn: () => contactsApi.vatInfo(contactId!).then((r) => r.data),
    enabled: !!contactId,
  });
}

// Time Entry Transition
export function useTransitionTimeEntry() {
  const qc = useQueryClient();
  return useMutation({
    mutationFn: ({ id, status }: { id: string; status: string }) =>
      timeEntriesApi.transitionTimeEntry(id, status),
    onSuccess: () => qc.invalidateQueries({ queryKey: ['time-entries'] }),
  });
}
