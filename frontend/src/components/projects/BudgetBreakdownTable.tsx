import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { Progress } from '@/components/ui/progress';
import { useI18n } from '@/i18n';

interface BreakdownRow {
  name: string;
  budget_hours: number | null;
  actual_hours: number;
  rate: number | null;
  amount: number;
}

interface BudgetBreakdownTableProps {
  memberBreakdown?: BreakdownRow[];
  activityBreakdown?: BreakdownRow[];
}

function BreakdownTable({ title, rows, t }: { title: string; rows: BreakdownRow[]; t: (key: string, fallback: string) => string }) {
  if (rows.length === 0) return null;

  return (
    <Card>
      <CardHeader className="pb-2">
        <CardTitle className="text-base">{title}</CardTitle>
      </CardHeader>
      <CardContent className="p-0">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>{t('common.name', 'Name')}</TableHead>
              <TableHead className="text-right">{t('project_budget.budget_hours', 'Budget')}</TableHead>
              <TableHead className="text-right">{t('project_budget.actual_hours', 'Actual')}</TableHead>
              <TableHead className="w-32">{t('project_budget.progress', 'Progress')}</TableHead>
              <TableHead className="text-right">{t('project_budget.rate', 'Rate')}</TableHead>
              <TableHead className="text-right">{t('project_budget.amount', 'Amount')}</TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows.map((row) => {
              const pct = row.budget_hours && row.budget_hours > 0
                ? Math.min(100, (row.actual_hours / row.budget_hours) * 100)
                : 0;
              return (
                <TableRow key={row.name}>
                  <TableCell className="font-medium">{row.name}</TableCell>
                  <TableCell className="text-right font-mono">
                    {row.budget_hours != null ? `${row.budget_hours}h` : '—'}
                  </TableCell>
                  <TableCell className="text-right font-mono">{row.actual_hours.toFixed(1)}h</TableCell>
                  <TableCell>
                    {row.budget_hours != null && row.budget_hours > 0 ? (
                      <div className="flex items-center gap-2">
                        <Progress value={pct} className="h-2 flex-1" />
                        <span className="text-xs text-muted-foreground w-10 text-right">{pct.toFixed(0)}%</span>
                      </div>
                    ) : (
                      <span className="text-xs text-muted-foreground">—</span>
                    )}
                  </TableCell>
                  <TableCell className="text-right font-mono">
                    {row.rate != null ? `${Number(row.rate).toFixed(2)}` : '—'}
                  </TableCell>
                  <TableCell className="text-right font-mono">
                    {Number(row.amount).toFixed(2)}
                  </TableCell>
                </TableRow>
              );
            })}
          </TableBody>
        </Table>
      </CardContent>
    </Card>
  );
}

export function BudgetBreakdownTable({ memberBreakdown, activityBreakdown }: BudgetBreakdownTableProps) {
  const { t } = useI18n();

  const members = memberBreakdown ?? [];
  const activities = activityBreakdown ?? [];

  if (members.length === 0 && activities.length === 0) return null;

  return (
    <div className="space-y-4">
      {members.length > 0 && (
        <BreakdownTable
          title={t('project_budget.per_member', 'Per Member')}
          rows={members}
          t={t}
        />
      )}
      {activities.length > 0 && (
        <BreakdownTable
          title={t('project_budget.per_activity', 'Per Activity')}
          rows={activities}
          t={t}
        />
      )}
    </div>
  );
}
