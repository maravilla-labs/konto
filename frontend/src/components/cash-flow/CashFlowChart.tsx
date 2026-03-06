import {
  ComposedChart,
  Bar,
  Line,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  CartesianGrid,
  Legend,
} from 'recharts';
import { useI18n } from '@/i18n';
import { useSettings } from '@/hooks/useSettingsApi';
import { formatNumber } from '@/lib/locale';
import { formatMonthLabel } from '@/lib/i18n-locale';
import type { MonthlyCashFlow } from '@/types/dashboard-charts';

interface CashFlowChartProps {
  months: MonthlyCashFlow[];
}

export function CashFlowChart({ months }: CashFlowChartProps) {
  const { language, t } = useI18n();
  const { data: settings } = useSettings();
  const numberFormat = settings?.number_format ?? 'ch';

  const chartData = months.map((m) => ({
    month: formatMonthLabel(m.month, language),
    inflows: parseFloat(m.inflows),
    outflows: Math.abs(parseFloat(m.outflows)),
    cumulative: parseFloat(m.cumulative_balance),
  }));

  return (
    <ResponsiveContainer width="100%" height={320}>
      <ComposedChart data={chartData}>
        <CartesianGrid strokeDasharray="3 3" opacity={0.3} />
        <XAxis dataKey="month" tick={{ fontSize: 11 }} />
        <YAxis yAxisId="left" tick={{ fontSize: 11 }} />
        <YAxis yAxisId="right" orientation="right" tick={{ fontSize: 11 }} />
        <Tooltip
          formatter={(value: number | string | undefined, name: string | undefined) => {
            const num = typeof value === 'number' ? value : Number(value ?? 0);
            const label =
              name === 'inflows'
                ? t('cash_flow.inflows', 'Inflows')
                : name === 'outflows'
                  ? t('cash_flow.outflows', 'Outflows')
                  : t('cash_flow.ending_balance', 'Balance');
            return [formatNumber(num, numberFormat), label];
          }}
        />
        <Legend
          formatter={(value: string) =>
            value === 'inflows'
              ? t('cash_flow.inflows', 'Inflows')
              : value === 'outflows'
                ? t('cash_flow.outflows', 'Outflows')
                : t('cash_flow.ending_balance', 'Balance')
          }
        />
        <Bar
          yAxisId="left"
          dataKey="inflows"
          fill="#22c55e"
          radius={[3, 3, 0, 0]}
          barSize={20}
        />
        <Bar
          yAxisId="left"
          dataKey="outflows"
          fill="#ef4444"
          radius={[3, 3, 0, 0]}
          barSize={20}
        />
        <Line
          yAxisId="right"
          type="monotone"
          dataKey="cumulative"
          stroke="#3b82f6"
          strokeWidth={2}
          dot={{ r: 3 }}
        />
      </ComposedChart>
    </ResponsiveContainer>
  );
}
