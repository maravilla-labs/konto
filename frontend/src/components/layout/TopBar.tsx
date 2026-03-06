import { useMemo } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Check, ChevronDown, MessageCircle, Search } from 'lucide-react';
import { SidebarTrigger, useSidebar } from '@/components/ui/sidebar';
import { Separator } from '@/components/ui/separator';
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from '@/components/ui/popover';
import { isTauri } from '@/lib/platform';
import { WinControls, useIsMac } from './WindowControls';
import { useNavigation } from '@/hooks/useNavigation';
import type { NavCategory, NavItem } from '@/lib/navigation';
import { SUPPORTED_LANGUAGES, type SupportedLanguage } from '@/lib/language';
import { DISCORD_URL, GITHUB_ISSUES_URL } from '@/lib/constants';
import { useI18n } from '@/i18n';
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';

const isDesktop = isTauri();
const CATEGORY_HUB_PATHS: Partial<Record<NavCategory, string>> = {
  Overview: '/dashboard',
  Sales: '/sales',
  Finance: '/finance',
  CRM: '/crm',
  Reports: '/reports',
  Settings: '/settings',
  Data: '/import',
};

interface BreadcrumbEntry {
  category?: string;
  title: string;
}

const routes: Record<string, BreadcrumbEntry> = {
  '/dashboard': { title: 'Dashboard' },
  '/sales': { category: 'Sales', title: 'Sales' },
  '/finance': { category: 'Finance', title: 'Finance' },
  '/crm': { category: 'CRM', title: 'CRM' },
  // Sales
  '/invoices': { category: 'Sales', title: 'Invoices' },
  '/invoices/new': { category: 'Sales', title: 'New Invoice' },
  '/invoices/recurring': { category: 'Sales', title: 'Recurring Invoices' },
  '/credit-notes': { category: 'Sales', title: 'Credit Notes' },
  '/credit-notes/new': { category: 'Sales', title: 'New Credit Note' },
  '/documents': { category: 'Sales', title: 'Quotes & Documents' },
  '/documents/new': { category: 'Sales', title: 'New Document' },
  // Finance
  '/accounts': { category: 'Finance', title: 'Chart of Accounts' },
  '/journal': { category: 'Finance', title: 'Journal Entries' },
  '/journal/new': { category: 'Finance', title: 'New Journal Entry' },
  '/expenses': { category: 'Finance', title: 'Expenses' },
  '/expenses/new': { category: 'Finance', title: 'New Expense' },
  '/banking': { category: 'Finance', title: 'Banking' },
  '/banking/reconcile': { category: 'Finance', title: 'Reconciliation' },
  // CRM
  '/contacts': { category: 'CRM', title: 'Contacts' },
  '/projects': { category: 'CRM', title: 'Projects' },
  '/time-entries': { category: 'CRM', title: 'Time Entries' },
  // Reports
  '/reports': { category: 'Reports', title: 'Reports' },
  '/reports/trial-balance': { category: 'Reports', title: 'Trial Balance' },
  '/reports/balance-sheet': { category: 'Reports', title: 'Balance Sheet' },
  '/reports/profit-loss': { category: 'Reports', title: 'Profit & Loss' },
  '/reports/cash-flow': { category: 'Reports', title: 'Cash Flow' },
  '/reports/vat-report': { category: 'Reports', title: 'VAT Report' },
  '/reports/ar-aging': { category: 'Reports', title: 'AR Aging' },
  '/reports/ap-aging': { category: 'Reports', title: 'AP Aging' },
  '/reports/account-ledger': { category: 'Reports', title: 'Account Ledger' },
  // Settings
  '/settings/company': { category: 'Settings', title: 'Company' },
  '/settings/bank-accounts': { category: 'Settings', title: 'Bank Accounts' },
  '/settings/fiscal-years': { category: 'Settings', title: 'Fiscal Years' },
  '/settings/vat-rates': { category: 'Settings', title: 'VAT Rates' },
  '/settings/currencies': { category: 'Settings', title: 'Currencies' },
  '/settings/exchange-rates': { category: 'Settings', title: 'Exchange Rates' },
  '/settings/activity-types': { category: 'Settings', title: 'Activity Types' },
  '/settings/expense-categories': { category: 'Settings', title: 'Expense Categories' },
  '/settings/dunning': { category: 'Settings', title: 'Dunning' },
  '/settings/email': { category: 'Settings', title: 'Email Settings' },
  '/settings/email-templates': { category: 'Settings', title: 'Email Templates' },
  '/settings/templates': { category: 'Settings', title: 'Templates' },
  '/settings/letterhead': { category: 'Settings', title: 'Letterhead' },
  '/settings/users': { category: 'Settings', title: 'Users' },
  '/settings/audit-log': { category: 'Settings', title: 'Audit Log' },
  // Data
  '/import': { category: 'Data', title: 'Import' },
};

function resolveRoute(pathname: string): BreadcrumbEntry {
  if (routes[pathname]) return routes[pathname];

  const segments = pathname.split('/').filter(Boolean);
  if (segments.length >= 2) {
    const base = segments[0];
    const last = segments[segments.length - 1];

    const categoryMap: Record<string, string> = {
      invoices: 'Sales',
      'credit-notes': 'Sales',
      documents: 'Sales',
      expenses: 'Finance',
      journal: 'Finance',
      contacts: 'CRM',
      projects: 'CRM',
      settings: 'Settings',
    };
    const category = categoryMap[base];

    const nameMap: Record<string, string> = {
      invoices: 'Invoice',
      'credit-notes': 'Credit Note',
      documents: 'Document',
      expenses: 'Expense',
      journal: 'Journal Entry',
      contacts: 'Contact',
      projects: 'Project',
    };

    if (last === 'edit') {
      return { category, title: `Edit ${nameMap[base] ?? base}` };
    }
    if (nameMap[base]) {
      return { category, title: `${nameMap[base]} Details` };
    }
  }

  return { title: 'Maravilla Konto' };
}

function getMatchedNavItem(pathname: string, items: NavItem[]): NavItem | undefined {
  const exact = items.find((item) => item.path === pathname);
  if (exact) return exact;

  return items
    .filter((item) => pathname.startsWith(`${item.path}/`))
    .sort((a, b) => b.path.length - a.path.length)[0];
}

function isPathActive(pathname: string, targetPath: string): boolean {
  return pathname === targetPath || pathname.startsWith(`${targetPath}/`);
}

export function TopBar() {
  const location = useLocation();
  const navigate = useNavigate();
  const { filteredItems } = useNavigation();
  const { language, setLanguage, t } = useI18n();
  const isMac = useIsMac();
  const { open } = useSidebar();
  const fallback = resolveRoute(location.pathname);

  const matchedItem = useMemo(
    () => getMatchedNavItem(location.pathname, filteredItems),
    [location.pathname, filteredItems],
  );

  const category = matchedItem?.category ?? fallback.category;
  const title = matchedItem?.label ?? fallback.title;
  const categoryLabel = category
    ? t(`category.${category.toLowerCase()}`, category)
    : undefined;
  const categoryHubPath = category ? CATEGORY_HUB_PATHS[category as NavCategory] : undefined;
  const showCategoryCrumb = Boolean(
    categoryLabel && categoryLabel.toLowerCase() !== title.toLowerCase(),
  );

  const titleItems = useMemo(() => {
    if (!matchedItem) return [];

    if (matchedItem.parent) {
      const parent = filteredItems.find((item) => item.id === matchedItem.parent);
      const children = filteredItems.filter((item) => item.parent === matchedItem.parent);
      return parent ? [parent, ...children] : children;
    }

    const children = filteredItems.filter((item) => item.parent === matchedItem.id);
    return children.length > 0 ? [matchedItem, ...children] : [];
  }, [matchedItem, filteredItems]);
  const titleMainItems = useMemo(
    () => titleItems.filter((item) => !item.parent),
    [titleItems],
  );
  const titleSubItems = useMemo(
    () => titleItems.filter((item) => Boolean(item.parent)),
    [titleItems],
  );

  return (
    <header
      className={`relative sticky top-0 z-30 flex h-14 items-center gap-3 border-b px-4 ${
        isDesktop ? 'tauri-topbar' : 'bg-card'
      }`}
    >
      {isDesktop && <div data-tauri-drag-region className="absolute inset-x-0 top-0 z-0 h-3" />}
      <div className={`relative z-10 flex items-center gap-3 ${isDesktop && isMac && !open ? 'ml-14' : ''}`}>
        <SidebarTrigger />
        <Separator orientation="vertical" className="h-6" />
      </div>
      <div className="flex items-center gap-2 select-none">
        {showCategoryCrumb && categoryLabel && (
          <>
            {categoryHubPath ? (
              <button
                onClick={() => navigate(categoryHubPath)}
                className="inline-flex items-center rounded px-1 py-0.5 text-sm text-muted-foreground hover:bg-accent hover:text-foreground"
              >
                {categoryLabel}
              </button>
            ) : (
              <span className="text-sm text-muted-foreground">{categoryLabel}</span>
            )}
            <span className="text-muted-foreground">/</span>
          </>
        )}
        {titleItems.length > 1 ? (
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <button className="inline-flex items-center gap-1 rounded px-1 py-0.5 text-sm font-semibold hover:bg-accent">
                <span>{title}</span>
                <ChevronDown className="h-3.5 w-3.5" />
              </button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start">
              {titleMainItems.length > 0 && (
                <>
                  <DropdownMenuLabel>{t('ui.main', 'Main')}</DropdownMenuLabel>
                  {titleMainItems.map((item) => (
                    <DropdownMenuItem
                      key={item.id}
                      onSelect={() => navigate(item.path)}
                      className="gap-2"
                    >
                      <item.icon className="h-4 w-4" />
                      <span>{item.label}</span>
                      {isPathActive(location.pathname, item.path) && <Check className="ml-auto h-4 w-4" />}
                    </DropdownMenuItem>
                  ))}
                </>
              )}
              {titleMainItems.length > 0 && titleSubItems.length > 0 && <DropdownMenuSeparator />}
              {titleSubItems.length > 0 && (
                <>
                  <DropdownMenuLabel>{t('ui.subitems', 'Subitems')}</DropdownMenuLabel>
                  {titleSubItems.map((item) => (
                    <DropdownMenuItem
                      key={item.id}
                      onSelect={() => navigate(item.path)}
                      className="gap-2"
                    >
                      <item.icon className="h-4 w-4" />
                      <span>{item.label}</span>
                      {isPathActive(location.pathname, item.path) && <Check className="ml-auto h-4 w-4" />}
                    </DropdownMenuItem>
                  ))}
                </>
              )}
            </DropdownMenuContent>
          </DropdownMenu>
        ) : (
          <h1 className="text-sm font-semibold">{title}</h1>
        )}
      </div>
      <div className="flex-1" />
      <Popover>
        <PopoverTrigger asChild>
          <button
            className="relative z-10 flex items-center gap-1.5 rounded-md border border-amber-500/30 bg-amber-500/10 px-2.5 py-1.5 text-sm font-medium text-amber-700 hover:bg-amber-500/20 transition-colors dark:text-amber-400 dark:border-amber-400/30 dark:bg-amber-400/10 dark:hover:bg-amber-400/20"
          >
            <MessageCircle className="h-4 w-4" />
            <span className="hidden sm:inline">{t('ui.feedback_title', 'Feedback')}</span>
          </button>
        </PopoverTrigger>
        <PopoverContent side="bottom" align="end" className="w-72 p-4 space-y-2">
          <p className="font-semibold text-sm">
            {t('ui.feedback_title', 'Feedback & Support')}
          </p>
          <p className="text-sm text-muted-foreground leading-relaxed">
            {t('ui.feedback_description', 'Got ideas, found a bug, or need help? Drop by our Discord — it\'s the quickest way to reach us.')}
          </p>
          <div className="flex flex-col gap-2 pt-1">
            <a
              href={DISCORD_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-sm font-medium text-[#5865F2] hover:underline"
            >
              <svg className="h-4 w-4 shrink-0" viewBox="0 0 24 24" fill="currentColor"><path d="M20.317 4.37a19.791 19.791 0 0 0-4.885-1.515.074.074 0 0 0-.079.037c-.21.375-.444.864-.608 1.25a18.27 18.27 0 0 0-5.487 0 12.64 12.64 0 0 0-.617-1.25.077.077 0 0 0-.079-.037A19.736 19.736 0 0 0 3.677 4.37a.07.07 0 0 0-.032.027C.533 9.046-.32 13.58.099 18.057a.082.082 0 0 0 .031.057 19.9 19.9 0 0 0 5.993 3.03.078.078 0 0 0 .084-.028c.462-.63.874-1.295 1.226-1.994a.076.076 0 0 0-.041-.106 13.107 13.107 0 0 1-1.872-.892.077.077 0 0 1-.008-.128 10.2 10.2 0 0 0 .372-.292.074.074 0 0 1 .077-.01c3.928 1.793 8.18 1.793 12.062 0a.074.074 0 0 1 .078.01c.12.098.246.198.373.292a.077.077 0 0 1-.006.127 12.299 12.299 0 0 1-1.873.892.077.077 0 0 0-.041.107c.36.698.772 1.362 1.225 1.993a.076.076 0 0 0 .084.028 19.839 19.839 0 0 0 6.002-3.03.077.077 0 0 0 .032-.054c.5-5.177-.838-9.674-3.549-13.66a.061.061 0 0 0-.031-.03z" /></svg>
              {t('ui.feedback_discord', 'Join us on Discord')}
            </a>
            <a
              href={GITHUB_ISSUES_URL}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-sm font-medium text-foreground/80 hover:underline"
            >
              <svg className="h-4 w-4 shrink-0" viewBox="0 0 24 24" fill="currentColor"><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.42.36.81 1.096.81 2.22 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" /></svg>
              {t('ui.feedback_github', 'Or open an issue on GitHub')}
            </a>
          </div>
        </PopoverContent>
      </Popover>
      <select
        value={language}
        onChange={(e) => setLanguage(e.target.value as SupportedLanguage)}
        className="relative z-10 h-8 rounded-md border bg-background px-2 text-xs text-muted-foreground"
      >
        {SUPPORTED_LANGUAGES.map((l) => (
          <option key={l.code} value={l.code}>
            {l.code.toUpperCase()}
          </option>
        ))}
      </select>
      <button
        onClick={() => {
          const isMacPlatform = navigator.userAgent.includes('Mac');
          window.dispatchEvent(new KeyboardEvent('keydown', {
            key: 'k',
            metaKey: isMacPlatform,
            ctrlKey: !isMacPlatform,
          }));
        }}
        className="relative z-10 flex items-center gap-2 rounded-md border px-3 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground"
      >
        <Search className="h-4 w-4" />
        <span className="hidden sm:inline">{t('ui.search_placeholder', 'Search...')}</span>
        <kbd className="hidden rounded bg-muted px-1.5 py-0.5 text-xs font-mono sm:inline">
          {navigator.userAgent.includes('Mac') ? '\u2318' : 'Ctrl'}K
        </kbd>
      </button>
      {isDesktop && !isMac && <div className="relative z-10"><WinControls /></div>}
    </header>
  );
}
