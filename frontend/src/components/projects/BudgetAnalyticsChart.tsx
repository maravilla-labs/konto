import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Skeleton } from '@/components/ui/skeleton';
import { useI18n } from '@/i18n';
import {
  AreaChart,
  Area,
  XAxis,
  YAxis,
  Tooltip,
  ResponsiveContainer,
  Legend,
} from 'recharts';

interface TimelineEntry {
  week_start: string;
  cumulative_budget: string;
  cumulative_actual: string;
}

interface BudgetAnalyticsChartProps {
  timeline?: TimelineEntry[];
  isLoading?: boolean;
}

export function BudgetAnalyticsChart({ timeline, isLoading }: BudgetAnalyticsChartProps) {
  const { t } = useI18n();

  if (isLoading) {
    return (
      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('project_budget.budget_vs_actual', 'Budget vs Actual')}</CardTitle>
        </CardHeader>
        <CardContent>
          <Skeleton className="h-64 w-full" />
        </CardContent>
      </Card>
    );
  }

  if (!timeline || timeline.length === 0) return null;

  // API returns cumulative values already
  const chartData = timeline.map((entry) => ({
    week: entry.week_start,
    budget: Number(entry.cumulative_budget) || 0,
    actual: Number(entry.cumulative_actual) || 0,
  }));

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">{t('project_budget.budget_vs_actual', 'Budget vs Actual')}</CardTitle>
        <p className="text-xs text-muted-foreground">{t('project_budget.cumulative_hours', 'Cumulative Hours')}</p>
      </CardHeader>
      <CardContent>
        <ResponsiveContainer width="100%" height={280}>
          <AreaChart data={chartData}>
            <XAxis
              dataKey="week"
              tick={{ fontSize: 11 }}
              tickLine={false}
              axisLine={false}
            />
            <YAxis
              tick={{ fontSize: 11 }}
              tickLine={false}
              axisLine={false}
              width={40}
            />
            <Tooltip
              contentStyle={{ fontSize: 12, borderRadius: 8, border: '1px solid hsl(var(--border))' }}
              formatter={(value: unknown, name?: string) => [
                `${value}h`,
                name === 'budget'
                  ? t('projects.budget', 'Budget')
                  : t('project_budget.actual', 'Actual'),
              ]}
            />
            <Legend
              formatter={(value: string) =>
                value === 'budget'
                  ? t('projects.budget', 'Budget')
                  : t('project_budget.actual', 'Actual')
              }
            />
            <Area
              type="monotone"
              dataKey="budget"
              stroke="hsl(220, 70%, 55%)"
              fill="hsl(220, 70%, 55%)"
              fillOpacity={0.15}
              strokeWidth={2}
            />
            <Area
              type="monotone"
              dataKey="actual"
              stroke="hsl(142, 70%, 45%)"
              fill="hsl(142, 70%, 45%)"
              fillOpacity={0.15}
              strokeWidth={2}
            />
          </AreaChart>
        </ResponsiveContainer>
      </CardContent>
    </Card>
  );
}
