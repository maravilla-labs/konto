import { NavLink } from 'react-router-dom';
import {
  Building2,
  BookOpen,
  FileText,
  CreditCard,
  Shield,
  UserCog,
  ChevronRight,
  type LucideIcon,
} from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

interface SettingsLink {
  label: string;
  path: string;
}

interface SettingsSection {
  title: string;
  icon: LucideIcon;
  links: SettingsLink[];
}

const sections: SettingsSection[] = [
  {
    title: 'General',
    icon: Building2,
    links: [
      { label: 'Company Settings', path: '/settings/company' },
      { label: 'Fiscal Years', path: '/settings/fiscal-years' },
      { label: 'Currencies', path: '/settings/currencies' },
      { label: 'Exchange Rates', path: '/settings/exchange-rates' },
      { label: 'Employees', path: '/settings/employees' },
    ],
  },
  {
    title: 'Accounting',
    icon: BookOpen,
    links: [
      { label: 'Default Accounts', path: '/settings/default-accounts' },
      { label: 'VAT Rates', path: '/settings/vat-rates' },
      { label: 'Expense Categories', path: '/settings/expense-categories' },
      { label: 'Activity Types', path: '/settings/activity-types' },
      { label: 'Rate Functions', path: '/settings/rate-functions' },
      { label: 'Fixed Assets', path: '/settings/fixed-assets' },
    ],
  },
  {
    title: 'Documents & Email',
    icon: FileText,
    links: [
      { label: 'Templates', path: '/settings/templates' },
      { label: 'Letterhead', path: '/settings/letterhead' },
      { label: 'Email Settings', path: '/settings/email' },
      { label: 'Email Templates', path: '/settings/email-templates' },
    ],
  },
  {
    title: 'Billing',
    icon: CreditCard,
    links: [
      { label: 'Bank Accounts', path: '/settings/bank-accounts' },
      { label: 'Dunning', path: '/settings/dunning' },
      { label: 'Payroll Settings', path: '/settings/payroll-settings' },
      { label: 'Project Sub-Statuses', path: '/settings/project-sub-statuses' },
    ],
  },
  {
    title: 'Legal & Compliance',
    icon: Shield,
    links: [
      { label: 'Gesellschafter', path: '/settings/shareholders' },
      { label: 'Audit Log', path: '/settings/audit-log' },
    ],
  },
  {
    title: 'Administration',
    icon: UserCog,
    links: [
      { label: 'Users', path: '/settings/users' },
      { label: 'Data Import', path: '/import' },
    ],
  },
];

export function SettingsHubPage() {
  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-lg font-semibold">Settings</h2>
        <p className="text-sm text-muted-foreground">
          Manage your application configuration and preferences
        </p>
      </div>
      <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3">
        {sections.map((section) => (
          <Card key={section.title}>
            <CardHeader className="flex flex-row items-center gap-3 pb-3">
              <div className="flex h-9 w-9 shrink-0 items-center justify-center rounded-lg bg-muted">
                <section.icon className="h-5 w-5 text-muted-foreground" />
              </div>
              <CardTitle className="text-base">{section.title}</CardTitle>
            </CardHeader>
            <CardContent className="pt-0">
              <ul className="space-y-1">
                {section.links.map((link) => (
                  <li key={link.path}>
                    <NavLink
                      to={link.path}
                      className="flex items-center justify-between rounded-md px-2 py-1.5 text-sm text-muted-foreground hover:bg-accent hover:text-accent-foreground transition-colors"
                    >
                      <span>{link.label}</span>
                      <ChevronRight className="h-4 w-4 opacity-50" />
                    </NavLink>
                  </li>
                ))}
              </ul>
            </CardContent>
          </Card>
        ))}
      </div>
    </div>
  );
}
