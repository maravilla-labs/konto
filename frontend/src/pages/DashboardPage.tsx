import { Link } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Button } from '@/components/ui/button';
import { useAuth } from '@/hooks/useAuth';
import { useDashboardStats } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatCurrency, formatDate } from '@/lib/locale';
import { RevenueChart } from '@/components/dashboard/RevenueChart';
import { ExpensesChart } from '@/components/dashboard/ExpensesChart';
import { InvoiceAgingChart } from '@/components/dashboard/InvoiceAgingChart';
import { TopOutstanding } from '@/components/dashboard/TopOutstanding';
import { OverviewChart } from '@/components/dashboard/OverviewChart';
import { useI18n } from '@/i18n';
import {
  BookOpen, Users, FileText, FolderKanban,
  TrendingUp, TrendingDown, Wallet, ReceiptText, Plus, Clock,
} from 'lucide-react';

export function DashboardPage() {
  const { user } = useAuth();
  const { data: settings } = useSettings();
  const { data, isLoading } = useDashboardStats();
  const { t } = useI18n();

  const numberFormat = settings?.number_format ?? 'ch';
  const dateFormat = settings?.date_format ?? 'dd.MM.yyyy';

  const stats = [
    { label: t('dashboard.revenue_mtd', 'Revenue (MTD)'), value: data?.revenue_mtd, icon: TrendingUp, format: true },
    { label: t('dashboard.expenses_mtd', 'Expenses (MTD)'), value: data?.expenses_mtd, icon: TrendingDown, format: true },
    { label: t('dashboard.cash_balance', 'Cash Balance'), value: data?.cash_balance, icon: Wallet, format: true },
    { label: t('dashboard.outstanding', 'Outstanding'), value: data?.total_outstanding, icon: ReceiptText, format: true },
    { label: t('dashboard.total_accounts', 'Total Accounts'), value: data?.account_count, icon: BookOpen, format: false },
    { label: t('dashboard.active_contacts', 'Active Contacts'), value: data?.active_contacts, icon: Users, format: false },
    { label: t('dashboard.journal_entries', 'Journal Entries'), value: data?.journal_entry_count, icon: FileText, format: false },
    { label: t('dashboard.active_projects', 'Active Projects'), value: data?.active_projects, icon: FolderKanban, format: false },
  ];

  return (
    <div className="space-y-6">
      <div>
        <h2 className="text-2xl font-bold tracking-tight">
          {t('dashboard.welcome_back', 'Welcome back')}, {user?.full_name?.split(' ')[0] ?? 'User'}
        </h2>
        <p className="text-muted-foreground">{t('dashboard.business_overview', 'Business overview')}</p>
      </div>

      {/* KPI Cards */}
      <div className="grid gap-4 grid-cols-2 lg:grid-cols-4">
        {stats.map((stat) => (
          <Card key={stat.label}>
            <CardHeader className="flex flex-row items-center justify-between pb-2">
              <CardTitle className="text-sm font-medium text-muted-foreground">
                {stat.label}
              </CardTitle>
              <stat.icon className="h-4 w-4 text-muted-foreground" />
            </CardHeader>
            <CardContent>
              {isLoading ? (
                <Skeleton className="h-8 w-24" />
              ) : (
                <div className="text-2xl font-bold font-mono">
                  {stat.format
                    ? formatCurrency(stat.value ?? '0', 'CHF', numberFormat)
                    : (stat.value ?? '—')}
                </div>
              )}
            </CardContent>
          </Card>
        ))}
      </div>

      {/* Overview Chart (full width) */}
      <OverviewChart />

      {/* Charts Row */}
      <div className="grid gap-4 lg:grid-cols-2">
        <RevenueChart />
        <ExpensesChart />
      </div>

      {/* Invoice Aging & Top Outstanding */}
      <div className="grid gap-4 lg:grid-cols-2">
        <InvoiceAgingChart />
        <TopOutstanding />
      </div>

      {/* Quick Actions */}
      <div className="grid gap-4 lg:grid-cols-2">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-base">{t('dashboard.quick_actions', 'Quick Actions')}</CardTitle>
          </CardHeader>
          <CardContent className="flex flex-wrap gap-2">
            <Button asChild variant="outline" size="sm">
              <Link to="/invoices/new">
                <Plus className="mr-1 h-4 w-4" /> {t('dashboard.new_invoice', 'New Invoice')}
              </Link>
            </Button>
            <Button asChild variant="outline" size="sm">
              <Link to="/time-entries">
                <Clock className="mr-1 h-4 w-4" /> {t('dashboard.time_entry', 'Time Entry')}
              </Link>
            </Button>
            <Button asChild variant="outline" size="sm">
              <Link to="/reports">
                <TrendingUp className="mr-1 h-4 w-4" /> {t('dashboard.reports', 'Reports')}
              </Link>
            </Button>
          </CardContent>
        </Card>

        {/* Recent Journal Entries */}
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-base">
              {t('dashboard.recent_journal_entries', 'Recent Journal Entries')}
            </CardTitle>
          </CardHeader>
          <CardContent>
            {isLoading ? (
              <div className="space-y-2">
                {Array.from({ length: 3 }).map((_, i) => (
                  <Skeleton key={i} className="h-8 w-full" />
                ))}
              </div>
            ) : data?.recent_entries && data.recent_entries.length > 0 ? (
              <div className="space-y-2">
                {data.recent_entries.slice(0, 5).map((entry) => (
                  <div key={entry.id} className="flex items-center justify-between rounded-md border px-3 py-2">
                    <div className="min-w-0 flex-1">
                      <p className="truncate text-sm font-medium">{entry.description}</p>
                      <p className="text-xs text-muted-foreground">
                        <span className="font-mono">{formatDate(entry.date, dateFormat)}</span>
                        {entry.reference && <span className="ml-2">{entry.reference}</span>}
                      </p>
                    </div>
                    <Badge variant={entry.status === 'posted' ? 'default' : 'secondary'} className="ml-2">
                      {t(`status.${entry.status}`, entry.status)}
                    </Badge>
                  </div>
                ))}
              </div>
            ) : (
              <p className="text-sm text-muted-foreground">
                {t('dashboard.no_entries_yet', 'No entries yet.')}
              </p>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
