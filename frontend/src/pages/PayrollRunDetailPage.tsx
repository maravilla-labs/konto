import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  usePayrollRun, useCalculatePayrollRun, useApprovePayrollRun,
  useMarkPayrollRunPaid, useDeletePayrollRun,
} from '@/hooks/useApi';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { payrollRunsApi } from '@/api/payroll-runs';
import { ArrowLeft, Calculator, CheckCircle, Banknote, Trash2, FileOutput, Download, FileDown } from 'lucide-react';

const MONTH_NAMES_DE = ['Jan', 'Feb', 'Mar', 'Apr', 'Mai', 'Jun', 'Jul', 'Aug', 'Sep', 'Okt', 'Nov', 'Dez'];

function fmt(n: number) {
  return n.toLocaleString('de-CH', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

function statusColor(status: string) {
  switch (status) {
    case 'draft': return 'secondary';
    case 'calculated': return 'outline';
    case 'approved': return 'default';
    case 'paid': return 'default';
    default: return 'secondary';
  }
}

export function PayrollRunDetailPage() {
  const { t } = useI18n();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data, isLoading } = usePayrollRun(id!);
  const calculate = useCalculatePayrollRun();
  const approve = useApprovePayrollRun();
  const markPaid = useMarkPayrollRunPaid();
  const deleteRun = useDeletePayrollRun();

  if (isLoading || !data) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-48 w-full" />
        <Skeleton className="h-64 w-full" />
      </div>
    );
  }

  const { run, lines } = data;
  const period = `${MONTH_NAMES_DE[run.period_month - 1]} ${run.period_year}`;

  function handleCalculate() {
    calculate.mutate(id!, {
      onSuccess: () => toast.success(t('payroll.calculated')),
      onError: () => toast.error(t('payroll.calculate_failed')),
    });
  }

  function handleApprove() {
    approve.mutate(id!, {
      onSuccess: () => toast.success(t('payroll.approved')),
      onError: () => toast.error(t('payroll.approve_failed')),
    });
  }

  function handleMarkPaid() {
    markPaid.mutate(id!, {
      onSuccess: () => toast.success(t('payroll.marked_paid')),
      onError: () => toast.error(t('payroll.mark_paid_failed')),
    });
  }

  function handleDelete() {
    deleteRun.mutate(id!, {
      onSuccess: () => {
        toast.success(t('payroll.deleted'));
        navigate('/payroll');
      },
      onError: () => toast.error(t('payroll.delete_failed')),
    });
  }

  function downloadBlob(data: Blob, filename: string) {
    const url = URL.createObjectURL(data);
    const a = document.createElement('a');
    a.href = url;
    a.download = filename;
    a.click();
    URL.revokeObjectURL(url);
  }

  async function handleDownloadPayslip(employeeId: string) {
    try {
      const res = await payrollRunsApi.downloadPayslip(id!, employeeId);
      downloadBlob(new Blob([res.data]), `payslip-${period}-${employeeId}.pdf`);
    } catch {
      toast.error(t('payroll.download_failed'));
    }
  }

  async function handleDownloadAllPayslips() {
    try {
      const res = await payrollRunsApi.downloadPayslips(id!);
      downloadBlob(new Blob([res.data]), `payslips-${period}.zip`);
    } catch {
      toast.error(t('payroll.download_failed'));
    }
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate('/payroll')}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1">
          <h2 className="text-lg font-semibold">{t('payroll.run_detail')} — {period}</h2>
          <p className="text-sm text-muted-foreground">
            <Badge variant={statusColor(run.status)}>{t(`payroll.status_${run.status}`)}</Badge>
          </p>
        </div>
        <div className="flex gap-2">
          {run.status === 'draft' && (
            <Button size="sm" onClick={handleCalculate} disabled={calculate.isPending}>
              <Calculator className="mr-1 h-4 w-4" /> {t('payroll.calculate')}
            </Button>
          )}
          {run.status === 'calculated' && (
            <Button size="sm" onClick={handleApprove} disabled={approve.isPending}>
              <CheckCircle className="mr-1 h-4 w-4" /> {t('payroll.approve')}
            </Button>
          )}
          {(run.status === 'approved' || run.status === 'paid') && (
            <Button size="sm" variant="outline" onClick={() => navigate(`/payroll/${id}/payouts`)}>
              <FileOutput className="mr-1 h-4 w-4" /> {t('payout.title')}
            </Button>
          )}
          {run.status === 'approved' && (
            <Button size="sm" onClick={handleMarkPaid} disabled={markPaid.isPending}>
              <Banknote className="mr-1 h-4 w-4" /> {t('payroll.mark_paid')}
            </Button>
          )}
          {(run.status === 'draft' || run.status === 'calculated') && (
            <Button size="sm" variant="destructive" onClick={handleDelete} disabled={deleteRun.isPending}>
              <Trash2 className="mr-1 h-4 w-4" /> {t('common.delete')}
            </Button>
          )}
        </div>
      </div>

      <div className="grid gap-4 sm:grid-cols-3">
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payroll.total_gross')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{fmt(run.total_gross)}</p></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payroll.total_net')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{fmt(run.total_net)}</p></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payroll.total_employer_cost')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{fmt(run.total_employer_cost)}</p></CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-base">{t('payroll.employee_lines')}</CardTitle>
          {lines.length > 0 && run.status !== 'draft' && (
            <Button size="sm" variant="outline" onClick={handleDownloadAllPayslips}>
              <FileDown className="mr-1 h-4 w-4" /> {t('payroll.download_all_payslips')}
            </Button>
          )}
        </CardHeader>
        <CardContent>
          {lines.length === 0 ? (
            <p className="text-sm text-muted-foreground py-8 text-center">{t('payroll.no_lines')}</p>
          ) : (
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('payroll.employee')}</TableHead>
                    <TableHead className="text-right">{t('payroll.gross')}</TableHead>
                    <TableHead className="text-right">{t('payroll.ahv')}</TableHead>
                    <TableHead className="text-right">{t('payroll.alv')}</TableHead>
                    <TableHead className="text-right">{t('payroll.bvg')}</TableHead>
                    <TableHead className="text-right">{t('payroll.nbu')}</TableHead>
                    <TableHead className="text-right">{t('payroll.ktg')}</TableHead>
                    <TableHead className="text-right">{t('payroll.net')}</TableHead>
                    <TableHead className="text-right">{t('payroll.payout')}</TableHead>
                    {run.status !== 'draft' && <TableHead className="w-10" />}
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {lines.map((line) => (
                    <TableRow key={line.id}>
                      <TableCell className="font-medium">{line.employee_name || line.employee_id}</TableCell>
                      <TableCell className="text-right">{fmt(line.gross_salary)}</TableCell>
                      <TableCell className="text-right">{fmt(line.ahv_employee)}</TableCell>
                      <TableCell className="text-right">{fmt(line.alv_employee)}</TableCell>
                      <TableCell className="text-right">{fmt(line.bvg_employee)}</TableCell>
                      <TableCell className="text-right">{fmt(line.nbu_employee)}</TableCell>
                      <TableCell className="text-right">{fmt(line.ktg_employee)}</TableCell>
                      <TableCell className="text-right font-medium">{fmt(line.net_salary)}</TableCell>
                      <TableCell className="text-right font-bold">{fmt(line.payout_amount)}</TableCell>
                      {run.status !== 'draft' && (
                        <TableCell>
                          <Button variant="ghost" size="icon" onClick={() => handleDownloadPayslip(line.employee_id)}>
                            <Download className="h-4 w-4" />
                          </Button>
                        </TableCell>
                      )}
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          )}
        </CardContent>
      </Card>

      {lines.length > 0 && (
        <Card>
          <CardHeader>
            <CardTitle className="text-base">{t('payroll.employer_contributions')}</CardTitle>
          </CardHeader>
          <CardContent>
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('payroll.employee')}</TableHead>
                    <TableHead className="text-right">{t('payroll.ahv_er')}</TableHead>
                    <TableHead className="text-right">{t('payroll.alv_er')}</TableHead>
                    <TableHead className="text-right">{t('payroll.bvg_er')}</TableHead>
                    <TableHead className="text-right">{t('payroll.bu')}</TableHead>
                    <TableHead className="text-right">{t('payroll.ktg_er')}</TableHead>
                    <TableHead className="text-right">{t('payroll.fak')}</TableHead>
                    <TableHead className="text-right font-bold">{t('payroll.total_er')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {lines.map((line) => (
                    <TableRow key={line.id}>
                      <TableCell className="font-medium">{line.employee_name || line.employee_id}</TableCell>
                      <TableCell className="text-right">{fmt(line.ahv_employer)}</TableCell>
                      <TableCell className="text-right">{fmt(line.alv_employer)}</TableCell>
                      <TableCell className="text-right">{fmt(line.bvg_employer)}</TableCell>
                      <TableCell className="text-right">{fmt(line.bu_employer)}</TableCell>
                      <TableCell className="text-right">{fmt(line.ktg_employer)}</TableCell>
                      <TableCell className="text-right">{fmt(line.fak_employer)}</TableCell>
                      <TableCell className="text-right font-bold">{fmt(line.total_employer_cost)}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          </CardContent>
        </Card>
      )}
    </div>
  );
}
