import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { useMonthlyExpenses } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { useI18n } from '@/i18n';
import { formatNumber } from '@/lib/locale';
import { formatMonthLabel } from '@/lib/i18n-locale';
import { BarChart, Bar, XAxis, YAxis, Tooltip, ResponsiveContainer } from 'recharts';

export function ExpensesChart() {
  const { language, t } = useI18n();
  const { data: settings } = useSettings();
  const { data, isLoading } = useMonthlyExpenses(12);
  const numberFormat = settings?.number_format ?? 'ch';

  const chartData = (data ?? []).map((d) => ({
    month: formatMonthLabel(d.month, language),
    amount: parseFloat(d.amount),
  }));

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">
          {t('dashboard.expenses_12m', 'Expenses (12 Months)')}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <Skeleton className="h-[250px] w-full" />
        ) : (
          <ResponsiveContainer width="100%" height={250}>
            <BarChart data={chartData}>
              <XAxis dataKey="month" tick={{ fontSize: 11 }} />
              <YAxis tick={{ fontSize: 11 }} />
              <Tooltip
                formatter={(value: number | string | undefined) => {
                  const numeric = typeof value === 'number' ? value : Number(value ?? 0);
                  return formatNumber(numeric, numberFormat);
                }}
              />
              <Bar dataKey="amount" fill="#f97316" radius={[3, 3, 0, 0]} />
            </BarChart>
          </ResponsiveContainer>
        )}
      </CardContent>
    </Card>
  );
}
