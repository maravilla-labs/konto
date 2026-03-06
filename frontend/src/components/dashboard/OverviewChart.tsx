import { useState } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { useDashboardOverview } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { useI18n } from '@/i18n';
import { formatCurrency } from '@/lib/locale';
import { formatMonthLabel } from '@/lib/i18n-locale';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
} from 'recharts';

export function OverviewChart() {
  const { language, t } = useI18n();
  const { data: settings } = useSettings();
  const numberFormat = settings?.number_format ?? 'ch';
  const currency = settings?.default_currency_id ? 'CHF' : 'CHF';

  const currentYear = new Date().getFullYear();
  const [selectedYear, setSelectedYear] = useState<number>(currentYear);

  const { data, isLoading } = useDashboardOverview(selectedYear);

  const chartData = (data?.months ?? []).map((m) => ({
    month: formatMonthLabel(m.month, language),
    cumulative_income: parseFloat(m.cumulative_income),
    cumulative_expenses: parseFloat(m.cumulative_expenses),
  }));

  const totalIncome = data?.total_income ?? '0';
  const totalExpenses = data?.total_expenses ?? '0';
  const difference = data?.difference ?? '0';
  const availableYears = data?.available_years ?? [currentYear];

  return (
    <Card>
      <CardHeader className="pb-2">
        <div className="flex items-center justify-between">
          <CardTitle className="text-base">
            {t('dashboard.overview', 'Overview')}
          </CardTitle>
          <Select
            value={String(selectedYear)}
            onValueChange={(v) => setSelectedYear(Number(v))}
          >
            <SelectTrigger className="w-[120px] h-8 text-sm">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              {availableYears.map((y) => (
                <SelectItem key={y} value={String(y)}>
                  {y}
                </SelectItem>
              ))}
            </SelectContent>
          </Select>
        </div>
        {/* Summary totals */}
        {!isLoading && (
          <div className="flex gap-6 pt-2 text-sm">
            <div>
              <span className="text-muted-foreground">
                {t('dashboard.overview_income', 'Income')}
              </span>
              <p className="font-mono font-semibold text-emerald-600">
                {formatCurrency(totalIncome, currency, numberFormat)}
              </p>
            </div>
            <div>
              <span className="text-muted-foreground">
                {t('dashboard.overview_expenses', 'Expenses')}
              </span>
              <p className="font-mono font-semibold text-red-500">
                {formatCurrency(totalExpenses, currency, numberFormat)}
              </p>
            </div>
            <div>
              <span className="text-muted-foreground">
                {t('dashboard.overview_difference', 'Difference')}
              </span>
              <p className="font-mono font-semibold">
                {formatCurrency(difference, currency, numberFormat)}
              </p>
            </div>
          </div>
        )}
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <Skeleton className="h-[280px] w-full" />
        ) : (
          <ResponsiveContainer width="100%" height={280}>
            <AreaChart data={chartData}>
              <defs>
                <linearGradient id="fillIncome" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#10b981" stopOpacity={0.15} />
                  <stop offset="95%" stopColor="#10b981" stopOpacity={0} />
                </linearGradient>
                <linearGradient id="fillExpenses" x1="0" y1="0" x2="0" y2="1">
                  <stop offset="5%" stopColor="#ef4444" stopOpacity={0.15} />
                  <stop offset="95%" stopColor="#ef4444" stopOpacity={0} />
                </linearGradient>
              </defs>
              <XAxis dataKey="month" tick={{ fontSize: 11 }} />
              <YAxis tick={{ fontSize: 11 }} />
              <Tooltip
                formatter={(value: unknown) => {
                  const numeric = typeof value === 'number' ? value : Number(value ?? 0);
                  return formatCurrency(numeric, currency, numberFormat);
                }}
              />
              <Area
                type="monotone"
                dataKey="cumulative_income"
                name={t('dashboard.overview_income', 'Income')}
                stroke="#10b981"
                fill="url(#fillIncome)"
                fillOpacity={1}
              />
              <Area
                type="monotone"
                dataKey="cumulative_expenses"
                name={t('dashboard.overview_expenses', 'Expenses')}
                stroke="#ef4444"
                fill="url(#fillExpenses)"
                fillOpacity={1}
              />
            </AreaChart>
          </ResponsiveContainer>
        )}
      </CardContent>
    </Card>
  );
}
