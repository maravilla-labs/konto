import { useParams, useNavigate } from 'react-router-dom';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table, TableBody, TableCell, TableHead, TableHeader, TableRow,
} from '@/components/ui/table';
import {
  usePayrollRun, usePayoutEntries, useGeneratePayouts,
  useExportPain001, useMarkPayoutPaid, useMarkAllPayoutsPaid,
} from '@/hooks/useApi';
import { toast } from 'sonner';
import { useI18n } from '@/i18n';
import { ArrowLeft, CheckCircle, RefreshCw, FileOutput } from 'lucide-react';

const MONTH_NAMES_DE = ['Jan', 'Feb', 'Mar', 'Apr', 'Mai', 'Jun', 'Jul', 'Aug', 'Sep', 'Okt', 'Nov', 'Dez'];

function fmt(n: number) {
  return n.toLocaleString('de-CH', { minimumFractionDigits: 2, maximumFractionDigits: 2 });
}

function statusBadge(status: string) {
  switch (status) {
    case 'pending': return <Badge variant="secondary">Pending</Badge>;
    case 'exported': return <Badge variant="outline">Exported</Badge>;
    case 'paid': return <Badge variant="default">Paid</Badge>;
    default: return <Badge variant="secondary">{status}</Badge>;
  }
}

export function PayoutSchedulePage() {
  const { t } = useI18n();
  const { id } = useParams<{ id: string }>();
  const navigate = useNavigate();
  const { data: runData, isLoading: runLoading } = usePayrollRun(id!);
  const { data: entries, isLoading: entriesLoading } = usePayoutEntries(id!);
  const generatePayouts = useGeneratePayouts();
  const exportPain001 = useExportPain001();
  const markPaid = useMarkPayoutPaid();
  const markAllPaid = useMarkAllPayoutsPaid();

  if (runLoading || !runData) {
    return (
      <div className="space-y-4">
        <Skeleton className="h-8 w-64" />
        <Skeleton className="h-48 w-full" />
      </div>
    );
  }

  const { run } = runData;
  const period = `${MONTH_NAMES_DE[run.period_month - 1]} ${run.period_year}`;
  const totalAmount = entries?.reduce((sum, e) => sum + e.amount, 0) ?? 0;
  const pendingCount = entries?.filter((e) => e.status !== 'paid').length ?? 0;

  function handleGenerate() {
    generatePayouts.mutate(id!, {
      onSuccess: () => toast.success(t('payout.generated')),
      onError: () => toast.error(t('payout.generate_failed')),
    });
  }

  function handleExportPain001() {
    exportPain001.mutate(id!, {
      onSuccess: (response) => {
        const blob = new Blob([response.data], { type: 'application/xml' });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `pain001_payroll_${id}.xml`;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
        document.body.removeChild(a);
        toast.success(t('payout.exported'));
      },
      onError: () => toast.error(t('payout.export_failed')),
    });
  }

  function handleMarkPaid(entryId: string) {
    markPaid.mutate(entryId, {
      onSuccess: () => toast.success(t('payout.marked_paid')),
      onError: () => toast.error(t('payout.mark_paid_failed')),
    });
  }

  function handleMarkAllPaid() {
    markAllPaid.mutate(id!, {
      onSuccess: () => toast.success(t('payout.all_marked_paid')),
      onError: () => toast.error(t('payout.mark_all_paid_failed')),
    });
  }

  return (
    <div className="space-y-4">
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="icon" onClick={() => navigate(`/payroll/${id}`)}>
          <ArrowLeft className="h-4 w-4" />
        </Button>
        <div className="flex-1">
          <h2 className="text-lg font-semibold">{t('payout.title')} — {period}</h2>
          <p className="text-sm text-muted-foreground">
            {t('payout.subtitle')}
          </p>
        </div>
        <div className="flex gap-2">
          {(run.status === 'approved' || run.status === 'paid') && (
            <Button size="sm" variant="outline" onClick={handleGenerate} disabled={generatePayouts.isPending}>
              <RefreshCw className="mr-1 h-4 w-4" /> {t('payout.generate')}
            </Button>
          )}
          {entries && entries.length > 0 && (
            <>
              <Button size="sm" variant="outline" onClick={handleExportPain001} disabled={exportPain001.isPending}>
                <FileOutput className="mr-1 h-4 w-4" /> {t('payout.export_pain001')}
              </Button>
              {pendingCount > 0 && (
                <Button size="sm" onClick={handleMarkAllPaid} disabled={markAllPaid.isPending}>
                  <CheckCircle className="mr-1 h-4 w-4" /> {t('payout.mark_all_paid')}
                </Button>
              )}
            </>
          )}
        </div>
      </div>

      <div className="grid gap-4 sm:grid-cols-3">
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payout.total_payout')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{fmt(totalAmount)}</p></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payout.entries_count')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{entries?.length ?? 0}</p></CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2"><CardTitle className="text-sm">{t('payout.pending_count')}</CardTitle></CardHeader>
          <CardContent><p className="text-2xl font-bold">{pendingCount}</p></CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle className="text-base">{t('payout.entries')}</CardTitle>
        </CardHeader>
        <CardContent>
          {entriesLoading ? (
            <Skeleton className="h-48 w-full" />
          ) : !entries || entries.length === 0 ? (
            <p className="text-sm text-muted-foreground py-8 text-center">{t('payout.no_entries')}</p>
          ) : (
            <div className="overflow-x-auto">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>{t('payout.recipient')}</TableHead>
                    <TableHead>{t('payout.iban')}</TableHead>
                    <TableHead className="text-right">{t('payout.amount')}</TableHead>
                    <TableHead>{t('payout.reference')}</TableHead>
                    <TableHead>{t('common.status')}</TableHead>
                    <TableHead>{t('common.actions')}</TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {entries.map((entry) => (
                    <TableRow key={entry.id}>
                      <TableCell className="font-medium">{entry.recipient_name}</TableCell>
                      <TableCell className="font-mono text-sm">{entry.iban}</TableCell>
                      <TableCell className="text-right font-bold">{fmt(entry.amount)}</TableCell>
                      <TableCell className="text-sm">{entry.payment_reference}</TableCell>
                      <TableCell>{statusBadge(entry.status)}</TableCell>
                      <TableCell>
                        {entry.status !== 'paid' && (
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={() => handleMarkPaid(entry.id)}
                            disabled={markPaid.isPending}
                          >
                            <CheckCircle className="mr-1 h-4 w-4" /> {t('payout.mark_paid_btn')}
                          </Button>
                        )}
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}
