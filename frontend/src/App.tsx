import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { Toaster } from '@/components/ui/sonner';
import { TooltipProvider } from '@/components/ui/tooltip';
import { AuthGuard } from '@/components/auth/AuthGuard';
import { RoleGuard } from '@/components/auth/RoleGuard';
import { AppLayout } from '@/components/layout/AppLayout';
import { LoginPage } from '@/pages/LoginPage';
import { DashboardPage } from '@/pages/DashboardPage';
import { AccountsPage } from '@/pages/AccountsPage';
import { ContactsPage } from '@/pages/ContactsPage';
import { JournalPage } from '@/pages/JournalPage';
import { JournalCreatePage } from '@/pages/JournalCreatePage';
import { JournalDetailPage } from '@/pages/JournalDetailPage';
import { ProjectsPage } from '@/pages/ProjectsPage';
import { TimeEntriesPage } from '@/pages/TimeEntriesPage';
import { ImportPage } from '@/pages/ImportPage';
import { ReportsPage } from '@/pages/ReportsPage';
import { TrialBalancePage } from '@/pages/TrialBalancePage';
import { BalanceSheetPage } from '@/pages/BalanceSheetPage';
import { ProfitLossPage } from '@/pages/ProfitLossPage';
import { AccountLedgerPage } from '@/pages/AccountLedgerPage';
import { VatReportPage } from '@/pages/VatReportPage';
import { FiscalYearsPage } from '@/pages/FiscalYearsPage';
import { ExchangeRatesPage } from '@/pages/ExchangeRatesPage';
import { InvoicesPage } from '@/pages/InvoicesPage';
import { InvoiceCreatePage } from '@/pages/InvoiceCreatePage';
import { InvoiceDetailPage } from '@/pages/InvoiceDetailPage';
import { InvoiceEditPage } from '@/pages/InvoiceEditPage';
import { CompanySettingsPage } from '@/pages/settings/CompanySettingsPage';
import { BankAccountsPage } from '@/pages/settings/BankAccountsPage';
import { TemplatesPage } from '@/pages/settings/TemplatesPage';
import { TemplateEditorPage } from '@/pages/settings/TemplateEditorPage';
import { LetterheadPage } from '@/pages/settings/LetterheadPage';
import { UsersPage } from '@/pages/settings/UsersPage';
import { EmailSettingsPage } from '@/pages/settings/EmailSettingsPage';
import { AuditLogPage } from '@/pages/settings/AuditLogPage';
import { DefaultAccountsPage } from '@/pages/settings/DefaultAccountsPage';
import { EmailTemplatesPage } from '@/pages/settings/EmailTemplatesPage';
import { DocumentsPage } from '@/pages/DocumentsPage';
import { DocumentCreatePage } from '@/pages/DocumentCreatePage';
import { DocumentDetailPage } from '@/pages/DocumentDetailPage';
import { DocumentEditPage } from '@/pages/DocumentEditPage';
import { ExpensesPage } from '@/pages/ExpensesPage';
import { ExpenseCreatePage } from '@/pages/ExpenseCreatePage';
import { ExpenseDetailPage } from '@/pages/ExpenseDetailPage';
import { ExpenseEditPage } from '@/pages/ExpenseEditPage';
import { ExpenseCategoriesPage } from '@/pages/settings/ExpenseCategoriesPage';
import { DunningSettingsPage } from '@/pages/settings/DunningSettingsPage';
import { VatRatesPage } from '@/pages/settings/VatRatesPage';
import { CurrenciesPage } from '@/pages/settings/CurrenciesPage';
import { ActivityTypesPage } from '@/pages/settings/ActivityTypesPage';
import { RateFunctionsPage } from '@/pages/settings/RateFunctionsPage';
import { RecurringInvoicesPage } from '@/pages/RecurringInvoicesPage';
import { CreditNotesPage } from '@/pages/CreditNotesPage';
import { CreditNoteCreatePage } from '@/pages/CreditNoteCreatePage';
import { CreditNoteDetailPage } from '@/pages/CreditNoteDetailPage';
import { CreditNoteEditPage } from '@/pages/CreditNoteEditPage';
import { CashFlowPage } from '@/pages/CashFlowPage';
import { ArAgingPage } from '@/pages/ArAgingPage';
import { ApAgingPage } from '@/pages/ApAgingPage';
import { ContactDetailPage } from '@/pages/ContactDetailPage';
import { ProjectDetailPage } from '@/pages/ProjectDetailPage';
import { BankingPage } from '@/pages/BankingPage';
import { ReconciliationPage } from '@/pages/ReconciliationPage';
import { AnnualReportPage } from '@/pages/AnnualReportPage';
import { ShareholdersPage } from '@/pages/settings/ShareholdersPage';
import { SettingsHubPage } from '@/pages/settings/SettingsHubPage';
import { SalesHubPage, FinanceHubPage, CrmHubPage } from '@/pages/CategoryHubPage';
import { ProfilePage } from '@/pages/ProfilePage';
import { TimesheetsPage } from '@/pages/TimesheetsPage';
import { TimesheetDetailPage } from '@/pages/TimesheetDetailPage';
import { FixedAssetsPage } from '@/pages/settings/FixedAssetsPage';
import { EmployeesPage } from '@/pages/settings/EmployeesPage';
import { PayrollSettingsPage } from '@/pages/settings/PayrollSettingsPage';
import { ProjectSubStatusesPage } from '@/pages/settings/ProjectSubStatusesPage';
import { PayrollRunsPage } from '@/pages/PayrollRunsPage';
import { PayrollRunDetailPage } from '@/pages/PayrollRunDetailPage';
import { PayoutSchedulePage } from '@/pages/PayoutSchedulePage';
import { SalaryCertificatesPage } from '@/pages/SalaryCertificatesPage';
import { useEffect } from 'react';
import { useAuthStore } from '@/stores/authStore';
import { useFullscreenSync } from '@/components/layout/WindowControls';
import { SetupPage } from '@/pages/SetupPage';
import { PdfViewPage } from '@/pages/PdfViewPage';
import { I18nProvider } from '@/i18n';
import { ErrorBoundary } from '@/components/ui/error-boundary';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      retry: 1,
      staleTime: 30_000,
      refetchOnWindowFocus: false,
    },
  },
});

function AppRoutes() {
  const initialize = useAuthStore((s) => s.initialize);

  useEffect(() => {
    initialize();
  }, [initialize]);

  // Sync fullscreen class on <html> for Tauri desktop
  useFullscreenSync();

  return (
    <Routes>
      <Route path="/login" element={<LoginPage />} />
      <Route path="/setup" element={<SetupPage />} />
      <Route element={<AuthGuard />}>
        {/* Standalone pages (no AppLayout chrome) — used by Tauri child windows */}
        <Route path="/pdf-view/:id" element={<PdfViewPage />} />
        <Route element={<ErrorBoundary><AppLayout /></ErrorBoundary>}>
          <Route path="/dashboard" element={<DashboardPage />} />
          <Route path="/profile" element={<ProfilePage />} />
          <Route path="/sales" element={<SalesHubPage />} />
          <Route path="/finance" element={<FinanceHubPage />} />
          <Route path="/crm" element={<CrmHubPage />} />
          <Route path="/accounts" element={<AccountsPage />} />
          <Route path="/contacts" element={<ContactsPage />} />
          <Route path="/contacts/:id" element={<ContactDetailPage />} />
          <Route path="/journal" element={<JournalPage />} />
          <Route path="/journal/new" element={<JournalCreatePage />} />
          <Route path="/journal/:id" element={<JournalDetailPage />} />
          <Route path="/projects" element={<ProjectsPage />} />
          <Route path="/projects/:id" element={<ProjectDetailPage />} />
          <Route path="/projects/:id/:tab" element={<ProjectDetailPage />} />
          <Route path="/time-entries" element={<TimeEntriesPage />} />
          <Route path="/timesheets" element={<TimesheetsPage />} />
          <Route path="/timesheets/:id" element={<TimesheetDetailPage />} />
          <Route path="/payroll" element={<RoleGuard roles={['admin']}><PayrollRunsPage /></RoleGuard>} />
          <Route path="/payroll/:id" element={<RoleGuard roles={['admin']}><PayrollRunDetailPage /></RoleGuard>} />
          <Route path="/payroll/:id/payouts" element={<RoleGuard roles={['admin']}><PayoutSchedulePage /></RoleGuard>} />
          <Route path="/invoices" element={<InvoicesPage />} />
          <Route path="/invoices/new" element={<InvoiceCreatePage />} />
          <Route path="/invoices/:id" element={<InvoiceDetailPage />} />
          <Route path="/invoices/recurring" element={<RecurringInvoicesPage />} />
          <Route path="/invoices/:id/edit" element={<InvoiceEditPage />} />
          <Route path="/credit-notes" element={<CreditNotesPage />} />
          <Route path="/credit-notes/new" element={<CreditNoteCreatePage />} />
          <Route path="/credit-notes/:id" element={<CreditNoteDetailPage />} />
          <Route path="/credit-notes/:id/edit" element={<CreditNoteEditPage />} />
          <Route path="/expenses" element={<ExpensesPage />} />
          <Route path="/expenses/new" element={<ExpenseCreatePage />} />
          <Route path="/expenses/:id" element={<ExpenseDetailPage />} />
          <Route path="/expenses/:id/edit" element={<ExpenseEditPage />} />
          <Route path="/documents" element={<DocumentsPage />} />
          <Route path="/documents/new" element={<DocumentCreatePage />} />
          <Route path="/documents/:id" element={<DocumentDetailPage />} />
          <Route path="/documents/:id/edit" element={<DocumentEditPage />} />
          <Route path="/banking" element={<BankingPage />} />
          <Route path="/banking/reconcile" element={<ReconciliationPage />} />
          <Route path="/import" element={<RoleGuard roles={['admin']}><ImportPage /></RoleGuard>} />
          <Route path="/reports" element={<ReportsPage />} />
          <Route path="/reports/trial-balance" element={<TrialBalancePage />} />
          <Route path="/reports/balance-sheet" element={<BalanceSheetPage />} />
          <Route path="/reports/profit-loss" element={<ProfitLossPage />} />
          <Route path="/reports/account-ledger" element={<AccountLedgerPage />} />
          <Route path="/reports/vat-report" element={<VatReportPage />} />
          <Route path="/reports/cash-flow" element={<CashFlowPage />} />
          <Route path="/reports/ar-aging" element={<ArAgingPage />} />
          <Route path="/reports/ap-aging" element={<ApAgingPage />} />
          <Route path="/reports/annual-report" element={<AnnualReportPage />} />
          <Route path="/reports/salary-certificates" element={<RoleGuard roles={['admin']}><SalaryCertificatesPage /></RoleGuard>} />
          <Route path="/settings" element={<RoleGuard roles={['admin']}><SettingsHubPage /></RoleGuard>} />
          <Route path="/settings/company" element={<RoleGuard roles={['admin']}><CompanySettingsPage /></RoleGuard>} />
          <Route path="/settings/bank-accounts" element={<RoleGuard roles={['admin']}><BankAccountsPage /></RoleGuard>} />
          <Route path="/settings/templates" element={<RoleGuard roles={['admin']}><TemplatesPage /></RoleGuard>} />
          <Route path="/settings/templates/new" element={<RoleGuard roles={['admin']}><TemplateEditorPage /></RoleGuard>} />
          <Route path="/settings/templates/:id" element={<RoleGuard roles={['admin']}><TemplateEditorPage /></RoleGuard>} />
          <Route path="/settings/letterhead" element={<RoleGuard roles={['admin']}><LetterheadPage /></RoleGuard>} />
          <Route path="/settings/fiscal-years" element={<RoleGuard roles={['admin']}><FiscalYearsPage /></RoleGuard>} />
          <Route path="/settings/exchange-rates" element={<RoleGuard roles={['admin']}><ExchangeRatesPage /></RoleGuard>} />
          <Route path="/settings/users" element={<RoleGuard roles={['admin']}><UsersPage /></RoleGuard>} />
          <Route path="/settings/email" element={<RoleGuard roles={['admin']}><EmailSettingsPage /></RoleGuard>} />
          <Route path="/settings/email-templates" element={<RoleGuard roles={['admin']}><EmailTemplatesPage /></RoleGuard>} />
          <Route path="/settings/expense-categories" element={<RoleGuard roles={['admin']}><ExpenseCategoriesPage /></RoleGuard>} />
          <Route path="/settings/dunning" element={<RoleGuard roles={['admin']}><DunningSettingsPage /></RoleGuard>} />
          <Route path="/settings/vat-rates" element={<RoleGuard roles={['admin']}><VatRatesPage /></RoleGuard>} />
          <Route path="/settings/currencies" element={<RoleGuard roles={['admin']}><CurrenciesPage /></RoleGuard>} />
          <Route path="/settings/activity-types" element={<RoleGuard roles={['admin']}><ActivityTypesPage /></RoleGuard>} />
          <Route path="/settings/rate-functions" element={<RoleGuard roles={['admin']}><RateFunctionsPage /></RoleGuard>} />
          <Route path="/settings/default-accounts" element={<RoleGuard roles={['admin']}><DefaultAccountsPage /></RoleGuard>} />
          <Route path="/settings/shareholders" element={<RoleGuard roles={['admin']}><ShareholdersPage /></RoleGuard>} />
          <Route path="/settings/fixed-assets" element={<RoleGuard roles={['admin']}><FixedAssetsPage /></RoleGuard>} />
          <Route path="/settings/audit-log" element={<RoleGuard roles={['admin']}><AuditLogPage /></RoleGuard>} />
          <Route path="/settings/employees" element={<RoleGuard roles={['admin']}><EmployeesPage /></RoleGuard>} />
          <Route path="/settings/payroll-settings" element={<RoleGuard roles={['admin']}><PayrollSettingsPage /></RoleGuard>} />
          <Route path="/settings/project-sub-statuses" element={<RoleGuard roles={['admin']}><ProjectSubStatusesPage /></RoleGuard>} />
        </Route>
      </Route>
      <Route path="*" element={<Navigate to="/dashboard" replace />} />
    </Routes>
  );
}

export default function App() {
  return (
    <I18nProvider>
      <QueryClientProvider client={queryClient}>
        <TooltipProvider>
          <BrowserRouter>
            <AppRoutes />
            <Toaster position="bottom-right" richColors />
          </BrowserRouter>
        </TooltipProvider>
      </QueryClientProvider>
    </I18nProvider>
  );
}
