import {
  LayoutDashboard,
  ReceiptText,
  RefreshCw,
  Receipt,
  FileStack,
  BookOpen,
  FileText,
  Wallet,
  Landmark,
  ArrowLeftRight,
  Users,
  FolderKanban,
  Clock,
  Scale,
  Building2,
  TrendingUp,
  Banknote,
  ClipboardList,
  FileOutput,
  BookMarked,
  FileBarChart,
  Settings,
  Upload,
  Plus,
  CalendarCheck,
  Briefcase,
  UserCheck,
  Tag,
  type LucideIcon,
} from 'lucide-react';
import type { Role } from '@/types/auth';

export type NavCategory = 'Overview' | 'Sales' | 'Finance' | 'CRM' | 'Reports' | 'Settings' | 'Data';

export interface NavItem {
  id: string;
  label: string;
  category: NavCategory;
  path: string;
  icon: LucideIcon;
  keywords: string[];
  roles: Role[];           // empty = all roles
  showInSidebar: boolean;  // false for settings sub-pages
  parent?: string;         // for sub-items
}

export interface QuickAction {
  id: string;
  label: string;
  path: string;
  icon: LucideIcon;
  keywords: string[];
  roles: Role[];
}

export const navItems: NavItem[] = [
  // Overview
  { id: 'dashboard', label: 'Dashboard', category: 'Overview', path: '/dashboard', icon: LayoutDashboard, keywords: ['home', 'übersicht', 'start'], roles: [], showInSidebar: true },

  // Sales
  { id: 'invoices', label: 'Invoices', category: 'Sales', path: '/invoices', icon: ReceiptText, keywords: ['rechnung', 'rechnungen', 'bills', 'faktura'], roles: [], showInSidebar: true },
  { id: 'recurring-invoices', label: 'Recurring Invoices', category: 'Sales', path: '/invoices/recurring', icon: RefreshCw, keywords: ['wiederkehrend', 'abo', 'subscription', 'recurring'], roles: [], showInSidebar: true, parent: 'invoices' },
  { id: 'credit-notes', label: 'Credit Notes', category: 'Sales', path: '/credit-notes', icon: Receipt, keywords: ['gutschrift', 'gutschriften', 'credit'], roles: [], showInSidebar: true },
  { id: 'documents', label: 'Quotes & Documents', category: 'Sales', path: '/documents', icon: FileStack, keywords: ['angebot', 'offerte', 'dokument', 'vertrag', 'quote', 'offer', 'contract', 'sow'], roles: [], showInSidebar: true },

  // Finance
  { id: 'accounts', label: 'Chart of Accounts', category: 'Finance', path: '/accounts', icon: BookOpen, keywords: ['kontenplan', 'konto', 'konten', 'chart'], roles: [], showInSidebar: true },
  { id: 'journal', label: 'Journal Entries', category: 'Finance', path: '/journal', icon: FileText, keywords: ['buchung', 'buchungen', 'journal', 'buchhaltung', 'booking'], roles: [], showInSidebar: true },
  { id: 'expenses', label: 'Expenses', category: 'Finance', path: '/expenses', icon: Wallet, keywords: ['ausgaben', 'spesen', 'kosten', 'expense'], roles: [], showInSidebar: true },
  { id: 'banking', label: 'Banking', category: 'Finance', path: '/banking', icon: Landmark, keywords: ['bank', 'konto', 'transaktionen', 'transactions'], roles: [], showInSidebar: true },
  { id: 'reconcile', label: 'Reconcile', category: 'Finance', path: '/banking/reconcile', icon: ArrowLeftRight, keywords: ['abstimmung', 'reconciliation', 'abgleich'], roles: [], showInSidebar: true, parent: 'banking' },

  // CRM
  { id: 'contacts', label: 'Contacts', category: 'CRM', path: '/contacts', icon: Users, keywords: ['kontakt', 'kontakte', 'kunde', 'kunden', 'lieferant', 'customer', 'vendor'], roles: [], showInSidebar: true },
  { id: 'projects', label: 'Projects', category: 'CRM', path: '/projects', icon: FolderKanban, keywords: ['projekt', 'projekte', 'project'], roles: [], showInSidebar: true },
  { id: 'time-tracking', label: 'Time Tracking', category: 'CRM', path: '/time-entries', icon: Clock, keywords: ['zeit', 'zeiterfassung', 'stunden', 'time', 'hours'], roles: [], showInSidebar: true },
  { id: 'timesheets', label: 'Timesheets', category: 'CRM', path: '/timesheets', icon: CalendarCheck, keywords: ['stundenzettel', 'timesheet', 'rapportzettel', 'abrechnung', 'approval', 'rapportierung', 'arbeitszeit'], roles: [], showInSidebar: true, parent: 'time-tracking' },
  { id: 'payroll', label: 'Payroll', category: 'CRM', path: '/payroll', icon: Banknote, keywords: ['lohn', 'gehalt', 'payroll', 'lohnlauf', 'salary', 'abrechnung'], roles: ['admin'], showInSidebar: true },

  // Reports (hub page lives under Finance in sidebar)
  { id: 'reports', label: 'Reports', category: 'Finance', path: '/reports', icon: Scale, keywords: ['berichte', 'report', 'auswertung'], roles: [], showInSidebar: true },
  { id: 'trial-balance', label: 'Trial Balance', category: 'Reports', path: '/reports/trial-balance', icon: Scale, keywords: ['saldenliste', 'probebilanz', 'trial'], roles: [], showInSidebar: false },
  { id: 'balance-sheet', label: 'Balance Sheet', category: 'Reports', path: '/reports/balance-sheet', icon: Building2, keywords: ['bilanz', 'balance'], roles: [], showInSidebar: false },
  { id: 'profit-loss', label: 'Profit & Loss', category: 'Reports', path: '/reports/profit-loss', icon: TrendingUp, keywords: ['erfolgsrechnung', 'gewinn', 'verlust', 'p&l', 'pnl'], roles: [], showInSidebar: false },
  { id: 'cash-flow', label: 'Cash Flow', category: 'Reports', path: '/reports/cash-flow', icon: Banknote, keywords: ['cashflow', 'geldfluss', 'liquidität'], roles: [], showInSidebar: false },
  { id: 'vat-report', label: 'VAT Report', category: 'Reports', path: '/reports/vat-report', icon: Receipt, keywords: ['mwst', 'mehrwertsteuer', 'vat', 'steuer'], roles: [], showInSidebar: false },
  { id: 'ar-aging', label: 'AR Aging', category: 'Reports', path: '/reports/ar-aging', icon: ClipboardList, keywords: ['debitoren', 'fällig', 'ausstehend', 'receivable'], roles: [], showInSidebar: false },
  { id: 'ap-aging', label: 'AP Aging', category: 'Reports', path: '/reports/ap-aging', icon: FileOutput, keywords: ['kreditoren', 'verbindlichkeiten', 'payable'], roles: [], showInSidebar: false },
  { id: 'account-ledger', label: 'Account Ledger', category: 'Reports', path: '/reports/account-ledger', icon: BookMarked, keywords: ['kontoblatt', 'ledger', 'hauptbuch'], roles: [], showInSidebar: false },
  { id: 'annual-report', label: 'Jahresrechnung', category: 'Reports', path: '/reports/annual-report', icon: FileBarChart, keywords: ['jahresrechnung', 'annual', 'geschäftsbericht', 'jahresabschluss'], roles: [], showInSidebar: false },
  { id: 'salary-certificates', label: 'Salary Certificates', category: 'Reports', path: '/reports/salary-certificates', icon: FileOutput, keywords: ['lohnausweis', 'salary certificate', 'formular 11', 'certificat de salaire', 'certificato di salario'], roles: ['admin'], showInSidebar: false },

  // Settings (all hidden from sidebar -- accessible via Settings Hub)
  { id: 'settings', label: 'Settings', category: 'Settings', path: '/settings', icon: Settings, keywords: ['einstellungen', 'settings', 'konfiguration'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-company', label: 'Company Settings', category: 'Settings', path: '/settings/company', icon: Settings, keywords: ['firma', 'unternehmen', 'company'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-bank-accounts', label: 'Bank Accounts', category: 'Settings', path: '/settings/bank-accounts', icon: Landmark, keywords: ['bankkonto', 'iban', 'bank'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-fiscal-years', label: 'Fiscal Years', category: 'Settings', path: '/settings/fiscal-years', icon: Scale, keywords: ['geschäftsjahr', 'fiscal', 'year'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-vat-rates', label: 'VAT Rates', category: 'Settings', path: '/settings/vat-rates', icon: Receipt, keywords: ['mwst', 'steuersatz', 'vat rate'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-currencies', label: 'Currencies', category: 'Settings', path: '/settings/currencies', icon: Banknote, keywords: ['währung', 'currency', 'chf', 'eur'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-exchange-rates', label: 'Exchange Rates', category: 'Settings', path: '/settings/exchange-rates', icon: ArrowLeftRight, keywords: ['wechselkurs', 'exchange', 'kurs'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-default-accounts', label: 'Default Accounts', category: 'Settings', path: '/settings/default-accounts', icon: BookOpen, keywords: ['standardkonten', 'default'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-templates', label: 'Templates', category: 'Settings', path: '/settings/templates', icon: FileStack, keywords: ['vorlage', 'template'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-letterhead', label: 'Letterhead', category: 'Settings', path: '/settings/letterhead', icon: FileText, keywords: ['briefkopf', 'letterhead'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-email', label: 'Email Settings', category: 'Settings', path: '/settings/email', icon: Settings, keywords: ['email', 'smtp', 'mail'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-email-templates', label: 'Email Templates', category: 'Settings', path: '/settings/email-templates', icon: FileText, keywords: ['email vorlage', 'mail template'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-expense-categories', label: 'Expense Categories', category: 'Settings', path: '/settings/expense-categories', icon: Wallet, keywords: ['ausgabenkategorie', 'category'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-activity-types', label: 'Activity Types', category: 'Settings', path: '/settings/activity-types', icon: Clock, keywords: ['aktivitätstyp', 'activity'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-rate-functions', label: 'Rate Functions', category: 'Settings', path: '/settings/rate-functions', icon: Briefcase, keywords: ['funktion', 'tarif', 'rate', 'stundensatz', 'billing'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-fixed-assets', label: 'Fixed Assets', category: 'Settings', path: '/settings/fixed-assets', icon: Landmark, keywords: ['anlagevermoegen', 'abschreibung', 'fixed asset', 'depreciation', 'anlage'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-dunning', label: 'Dunning', category: 'Settings', path: '/settings/dunning', icon: Receipt, keywords: ['mahnung', 'dunning', 'mahnwesen'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-shareholders', label: 'Gesellschafter', category: 'Settings', path: '/settings/shareholders', icon: Users, keywords: ['gesellschafter', 'shareholder', 'aktionär'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-users', label: 'Users', category: 'Settings', path: '/settings/users', icon: Users, keywords: ['benutzer', 'user', 'mitarbeiter'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-audit-log', label: 'Audit Log', category: 'Settings', path: '/settings/audit-log', icon: FileText, keywords: ['protokoll', 'audit', 'log'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-employees', label: 'Employees', category: 'Settings', path: '/settings/employees', icon: UserCheck, keywords: ['mitarbeiter', 'angestellte', 'personal', 'employee', 'staff'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-payroll-settings', label: 'Payroll Settings', category: 'Settings', path: '/settings/payroll-settings', icon: Banknote, keywords: ['lohn', 'gehalt', 'payroll', 'sozialversicherung', 'ahv', 'bvg'], roles: ['admin'], showInSidebar: false },
  { id: 'settings-project-sub-statuses', label: 'Project Sub-Statuses', category: 'Settings', path: '/settings/project-sub-statuses', icon: Tag, keywords: ['unterstatus', 'sub-status', 'projekt status', 'project status'], roles: ['admin'], showInSidebar: false },
  { id: 'import', label: 'Import', category: 'Data', path: '/import', icon: Upload, keywords: ['import', 'daten', 'csv', 'upload'], roles: ['admin', 'accountant'], showInSidebar: false },
];

export const quickActions: QuickAction[] = [
  { id: 'new-invoice', label: 'New Invoice', path: '/invoices/new', icon: Plus, keywords: ['neue rechnung', 'rechnung erstellen'], roles: [] },
  { id: 'new-journal', label: 'New Journal Entry', path: '/journal/new', icon: Plus, keywords: ['neue buchung', 'buchung erstellen'], roles: [] },
  { id: 'new-expense', label: 'New Expense', path: '/expenses/new', icon: Plus, keywords: ['neue ausgabe', 'ausgabe erstellen'], roles: [] },
  { id: 'new-credit-note', label: 'New Credit Note', path: '/credit-notes/new', icon: Plus, keywords: ['neue gutschrift'], roles: [] },
  { id: 'new-document', label: 'New Document', path: '/documents/new', icon: Plus, keywords: ['neues dokument', 'neues angebot'], roles: [] },
];
