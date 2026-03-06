import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import {
  Dialog, DialogContent, DialogFooter, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import { usePayrollRuns, useCreatePayrollRun, useDeletePayrollRun } from '@/hooks/useApi';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { Plus, Trash2 } from 'lucide-react';
import { StickyToolbar, type ToolbarAction } from '@/components/ui/sticky-toolbar';

const MONTHS = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const MONTH_NAMES_DE = ['Jan', 'Feb', 'Mär', 'Apr', 'Mai', 'Jun', 'Jul', 'Aug', 'Sep', 'Okt', 'Nov', 'Dez'];

function statusColor(status: string) {
  switch (status) {
    case 'draft': return 'secondary';
    case 'calculated': return 'outline';
    case 'approved': return 'default';
    case 'paid': return 'default';
    default: return 'secondary';
  }
}

function fmt(n: number) {
  return n.toLocaleString('de-CH', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

export function PayrollRunsPage() {
  const { t } = useI18n();
  const navigate = useNavigate();
  const { data: runs, isLoading } = usePayrollRuns();
  const createRun = useCreatePayrollRun();
  const deleteRun = useDeletePayrollRun();
  const [dialogOpen, setDialogOpen] = useState(false);
  const now = new Date();
  const [month, setMonth] = useState(now.getMonth() + 1);
  const [year, setYear] = useState(now.getFullYear());

  function handleCreate() {
    createRun.mutate({ month, year }, {
      onSuccess: (res) => {
        toast.success(t('payroll.created'));
        setDialogOpen(false);
        navigate(`/payroll/${res.data.id}`);
      },
      onError: (err: unknown) => {
        const msg = (err as { response?: { data?: { message?: string } } })?.response?.data?.message || t('payroll.create_failed');
        toast.error(msg);
      },
    });
  }

  function handleDelete(id: string, e: React.MouseEvent) {
    e.stopPropagation();
    deleteRun.mutate(id, {
      onSuccess: () => toast.success(t('payroll.deleted')),
      onError: () => toast.error(t('payroll.delete_failed')),
    });
  }

  if (isLoading) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-48" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  return (
    <div className="space-y-4">
      <StickyToolbar
        actions={[
          { icon: <Plus className="h-4 w-4" />, label: t('payroll.new_run'), onClick: () => setDialogOpen(true), primary: true },
        ] satisfies ToolbarAction[]}
      >
        <Badge variant="secondary">{(runs ?? []).length} {t('payroll.runs')}</Badge>
      </StickyToolbar>

      <Dialog open={dialogOpen} onOpenChange={setDialogOpen}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{t('payroll.new_run')}</DialogTitle>
          </DialogHeader>
          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <Label>{t('payroll.month')}</Label>
              <select
                className="flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-sm"
                value={month}
                onChange={(e) => setMonth(Number(e.target.value))}
              >
                {MONTHS.map((m) => (
                  <option key={m} value={m}>{MONTH_NAMES_DE[m - 1]}</option>
                ))}
              </select>
            </div>
            <div>
              <Label>{t('payroll.year')}</Label>
              <Input
                type="number"
                value={year}
                onChange={(e) => setYear(Number(e.target.value))}
              />
            </div>
          </div>
          <DialogFooter>
            <Button variant="outline" onClick={() => setDialogOpen(false)}>{t('common.cancel')}</Button>
            <Button onClick={handleCreate} disabled={createRun.isPending}>
              {createRun.isPending ? t('common.saving') : t('common.create')}
            </Button>
          </DialogFooter>
        </DialogContent>
      </Dialog>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('payroll.runs')}</CardTitle>
        </CardHeader>
        <CardContent>
          {(!runs || runs.length === 0) ? (
            <p className="text-sm text-muted-foreground py-8 text-center">{t('payroll.no_runs')}</p>
          ) : (
            <Table>
              <TableHeader>
                <TableRow>
                  <TableHead>{t('payroll.period')}</TableHead>
                  <TableHead>{t('payroll.status')}</TableHead>
                  <TableHead className="text-right">{t('payroll.total_gross')}</TableHead>
                  <TableHead className="text-right">{t('payroll.total_net')}</TableHead>
                  <TableHead className="text-right">{t('payroll.total_employer_cost')}</TableHead>
                  <TableHead className="w-10" />
                </TableRow>
              </TableHeader>
              <TableBody>
                {runs.map((run) => (
                  <TableRow
                    key={run.id}
                    className="cursor-pointer"
                    onClick={() => navigate(`/payroll/${run.id}`)}
                  >
                    <TableCell className="font-medium">
                      {MONTH_NAMES_DE[run.period_month - 1]} {run.period_year}
                    </TableCell>
                    <TableCell>
                      <Badge variant={statusColor(run.status)}>{t(`payroll.status_${run.status}`)}</Badge>
                    </TableCell>
                    <TableCell className="text-right">{fmt(run.total_gross)}</TableCell>
                    <TableCell className="text-right">{fmt(run.total_net)}</TableCell>
                    <TableCell className="text-right">{fmt(run.total_employer_cost)}</TableCell>
                    <TableCell>
                      {(run.status === 'draft' || run.status === 'calculated') && (
                        <Button
                          variant="ghost"
                          size="icon"
                          onClick={(e) => handleDelete(run.id, e)}
                        >
                          <Trash2 className="h-4 w-4 text-destructive" />
                        </Button>
                      )}
                    </TableCell>
                  </TableRow>
                ))}
              </TableBody>
            </Table>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
