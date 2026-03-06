import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { useInvoiceAging } from '@/hooks/useApi';
import { useSettings } from '@/hooks/useSettingsApi';
import { useI18n } from '@/i18n';
import { PieChart, Pie, Cell, Tooltip, ResponsiveContainer, Legend } from 'recharts';
import { formatCurrency } from '@/lib/locale';

const STATUS_COLORS: Record<string, string> = {
  draft: '#94a3b8',
  sent: '#3b82f6',
  overdue: '#ef4444',
  paid: '#22c55e',
  cancelled: '#64748b',
};

export function InvoiceAgingChart() {
  const { t } = useI18n();
  const { data: settings } = useSettings();
  const { data, isLoading } = useInvoiceAging();
  const numberFormat = settings?.number_format ?? 'ch';

  const chartData = (data ?? [])
    .filter((b) => b.count > 0)
    .map((b) => ({
      name: t(`status.${b.status}`, b.status),
      value: b.count,
      status: b.status,
      total: b.total,
      color: STATUS_COLORS[b.status] ?? '#94a3b8',
    }));

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">
          {t('dashboard.invoice_status', 'Invoice Status')}
        </CardTitle>
      </CardHeader>
      <CardContent>
        {isLoading ? (
          <Skeleton className="h-[250px] w-full" />
        ) : chartData.length === 0 ? (
          <p className="flex h-[250px] items-center justify-center text-sm text-muted-foreground">
            {t('dashboard.no_invoices_yet', 'No invoices yet')}
          </p>
        ) : (
          <ResponsiveContainer width="100%" height={250}>
            <PieChart>
              <Pie
                data={chartData}
                cx="50%"
                cy="50%"
                innerRadius={50}
                outerRadius={90}
                paddingAngle={2}
                dataKey="value"
              >
                {chartData.map((entry, i) => (
                  <Cell key={i} fill={entry.color} />
                ))}
              </Pie>
              <Tooltip
                formatter={(
                  value: number | string | undefined,
                  _name: string | undefined,
                  props: { payload?: { total?: string } },
                ) => {
                  const count = typeof value === 'number' ? value : Number(value ?? 0);
                  const total = props.payload?.total ?? '0';
                  return [
                    `${count} - ${formatCurrency(total, 'CHF', numberFormat)}`,
                    '',
                  ];
                }}
              />
              <Legend />
            </PieChart>
          </ResponsiveContainer>
        )}
      </CardContent>
    </Card>
  );
}
