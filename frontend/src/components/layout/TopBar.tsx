import { useMemo } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Check, ChevronDown, Search } from 'lucide-react';
import { SidebarTrigger, useSidebar } from '@/components/ui/sidebar';
import { Separator } from '@/components/ui/separator';
import { isTauri } from '@/lib/platform';
import { WinControls, useIsMac } from './WindowControls';
import { useNavigation } from '@/hooks/useNavigation';
import type { NavCategory, NavItem } from '@/lib/navigation';
import { SUPPORTED_LANGUAGES, type SupportedLanguage } from '@/lib/language';
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
