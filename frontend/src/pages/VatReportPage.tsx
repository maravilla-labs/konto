import { useState, useMemo } from 'react';
import { useVatReport, useCreateVatPayment } from '@/hooks/useApi';
import { useBankAccounts } from '@/hooks/useSettingsApi';
import { Card, CardContent } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Skeleton } from '@/components/ui/skeleton';
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from '@/components/ui/table';
import {
  Dialog, DialogContent, DialogHeader, DialogTitle,
} from '@/components/ui/dialog';
import {
  Select, SelectContent, SelectItem, SelectTrigger, SelectValue,
} from '@/components/ui/select';
import { StickyToolbar, type ToolbarAction, type ToolbarOverflowItem } from '@/components/ui/sticky-toolbar';
import { formatAmount } from '@/lib/format';
import { downloadCsv } from '@/lib/export';
import { reportsApi } from '@/api/reports';
import { Download, CreditCard, FileCode2, RefreshCw } from 'lucide-react';
import { useI18n } from '@/i18n';
import { toast } from 'sonner';
import type { VatReportEntry, ExportVatXmlRequest } from '@/types/report';

// --- Period helpers ---

type PeriodMode = 'semester' | 'quarter';

interface PeriodOption {
  label: string;
  value: string;
  from: string;
  to: string;
}

function buildPeriodOptions(year: number, mode: PeriodMode, t: (k: string) => string): PeriodOption[] {
  if (mode === 'semester') {
    return [
      { label: t('vat.period_h1'), value: 'H1', from: `${year}-01-01`, to: `${year}-06-30` },
      { label: t('vat.period_h2'), value: 'H2', from: `${year}-07-01`, to: `${year}-12-31` },
    ];
  }
  return [
    { label: t('vat.period_q1'), value: 'Q1', from: `${year}-01-01`, to: `${year}-03-31` },
    { label: t('vat.period_q2'), value: 'Q2', from: `${year}-04-01`, to: `${year}-06-30` },
    { label: t('vat.period_q3'), value: 'Q3', from: `${year}-07-01`, to: `${year}-09-30` },
    { label: t('vat.period_q4'), value: 'Q4', from: `${year}-10-01`, to: `${year}-12-31` },
  ];
}

function currentPeriodValue(mode: PeriodMode): string {
  const m = new Date().getMonth();
  if (mode === 'semester') return m < 6 ? 'H1' : 'H2';
  if (m < 3) return 'Q1';
  if (m < 6) return 'Q2';
  if (m < 9) return 'Q3';
  return 'Q4';
}

// --- VatTable component ---

function VatTable({ entries, totalLabel, totalTaxable, totalVat }: {
  entries: VatReportEntry[];
  totalLabel: string;
  totalTaxable: string;
  totalVat: string;
}) {
  const { t } = useI18n();
  if (entries.length === 0) {
    return (
      <p className="py-4 text-center text-sm text-muted-foreground">&mdash;</p>
    );
  }
  return (
    <Table>
      <TableHeader>
        <TableRow>
          <TableHead>{t('vat.code')}</TableHead>
          <TableHead>{t('vat.name')}</TableHead>
          <TableHead className="text-right">{t('vat.rate')} %</TableHead>
          <TableHead className="text-right">{t('vat.taxable_amount')}</TableHead>
          <TableHead className="text-right">{t('vat.vat_amount')}</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        {entries.map((row) => (
          <TableRow key={row.vat_code}>
            <TableCell className="font-mono font-medium">{row.vat_code}</TableCell>
            <TableCell>{row.vat_name}</TableCell>
            <TableCell className="text-right font-mono text-sm">
              {formatAmount(row.rate)}
            </TableCell>
            <TableCell className="text-right font-mono text-sm">
              {formatAmount(row.taxable_amount)}
            </TableCell>
            <TableCell className="text-right font-mono text-sm">
              {formatAmount(row.vat_amount)}
            </TableCell>
          </TableRow>
        ))}
        <TableRow className="bg-muted/50 font-bold">
          <TableCell colSpan={3}>{totalLabel}</TableCell>
          <TableCell className="text-right font-mono text-sm">
            {formatAmount(totalTaxable)}
          </TableCell>
          <TableCell className="text-right font-mono text-sm">
            {formatAmount(totalVat)}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
  );
}

// --- Main page ---

export function VatReportPage() {
  const { t } = useI18n();

  // Period state
  const [year, setYear] = useState(new Date().getFullYear());
  const [manualMode, setManualMode] = useState(false);

  // Determine period mode from data (flat_rate → semester, effective → quarter)
  // Default to quarter until data is loaded
  const [periodModeOverride, setPeriodModeOverride] = useState<PeriodMode | null>(null);
  const periodMode: PeriodMode = periodModeOverride ?? 'quarter';

  const [periodValue, setPeriodValue] = useState(currentPeriodValue('quarter'));

  const periodOptions = useMemo(
    () => buildPeriodOptions(year, periodMode, t),
    [year, periodMode, t],
  );

  const selectedPeriod = periodOptions.find((p) => p.value === periodValue) ?? periodOptions[0];

  const [manualFrom, setManualFrom] = useState(selectedPeriod?.from ?? '');
  const [manualTo, setManualTo] = useState(selectedPeriod?.to ?? '');

  const from = manualMode ? manualFrom : (selectedPeriod?.from ?? '');
  const to = manualMode ? manualTo : (selectedPeriod?.to ?? '');

  const { data, isLoading, refetch } = useVatReport({ from_date: from, to_date: to });
  const createPayment = useCreateVatPayment();
  const { data: bankAccounts } = useBankAccounts();

  // Update period mode when data arrives
  if (data && periodModeOverride === null) {
    const mode = data.vat_method === 'flat_rate' ? 'semester' : 'quarter';
    setPeriodModeOverride(mode);
    setPeriodValue(currentPeriodValue(mode));
  }

  const dataIsFlatRate = data?.vat_method === 'flat_rate';

  // Payment dialog
  const [paymentOpen, setPaymentOpen] = useState(false);
  const [paymentDate, setPaymentDate] = useState(new Date().toISOString().split('T')[0]);
  const [bankAccountId, setBankAccountId] = useState('');

  // XML export dialog
  const [xmlOpen, setXmlOpen] = useState(false);
  const [xmlSubmissionType, setXmlSubmissionType] = useState('1');
  const [xmlTotalConsideration, setXmlTotalConsideration] = useState('');
  const [xmlSuppliesForeign, setXmlSuppliesForeign] = useState('0');
  const [xmlSuppliesAbroad, setXmlSuppliesAbroad] = useState('0');
  const [xmlTransferNotification, setXmlTransferNotification] = useState('0');
  const [xmlSuppliesExempt, setXmlSuppliesExempt] = useState('0');
  const [xmlReductionConsideration, setXmlReductionConsideration] = useState('0');
  const [xmlVariousDeduction, setXmlVariousDeduction] = useState('0');
  const [xmlSubsidies, setXmlSubsidies] = useState('0');
  const [xmlDonations, setXmlDonations] = useState('0');
  const [xmlExporting, setXmlExporting] = useState(false);

  function openXmlDialog() {
    // Pre-fill total consideration from report data
    if (data) {
      if (dataIsFlatRate && data.gross_revenue) {
        setXmlTotalConsideration(data.gross_revenue);
      } else {
        // effective: total_output_taxable + total_output_vat
        const taxable = parseFloat(data.total_output_taxable) || 0;
        const vat = parseFloat(data.total_output_vat) || 0;
        setXmlTotalConsideration((taxable + vat).toFixed(2));
      }
    }
    setXmlOpen(true);
  }

  // Compute preview values
  const totalDed = [xmlSuppliesForeign, xmlSuppliesAbroad, xmlTransferNotification, xmlSuppliesExempt, xmlReductionConsideration, xmlVariousDeduction]
    .reduce((s, v) => s + (parseFloat(v) || 0), 0);
  const taxableTurnover = (parseFloat(xmlTotalConsideration) || 0) - totalDed;

  async function handleXmlExport() {
    setXmlExporting(true);
    try {
      const req: ExportVatXmlRequest = {
        from_date: from,
        to_date: to,
        type_of_submission: parseInt(xmlSubmissionType),
        form_of_reporting: 1,
        total_consideration: xmlTotalConsideration,
        supplies_to_foreign: xmlSuppliesForeign,
        supplies_abroad: xmlSuppliesAbroad,
        transfer_notification: xmlTransferNotification,
        supplies_exempt: xmlSuppliesExempt,
        reduction_of_consideration: xmlReductionConsideration,
        various_deduction: xmlVariousDeduction,
        subsidies: xmlSubsidies !== '0' ? xmlSubsidies : undefined,
        donations: xmlDonations !== '0' ? xmlDonations : undefined,
      };
      await reportsApi.exportVatXml(req);
      toast.success(t('vat.download_xml'));
      setXmlOpen(false);
    } catch (err: unknown) {
      const msg = err instanceof Error ? err.message : 'Export failed';
      toast.error(msg);
    } finally {
      setXmlExporting(false);
    }
  }

  function handleCreatePayment() {
    if (!bankAccountId) {
      toast.error(t('vat.select_bank_account'));
      return;
    }
    createPayment.mutate(
      { from_date: from, to_date: to, payment_date: paymentDate, bank_account_id: bankAccountId },
      {
        onSuccess: (res) => {
          toast.success(t('vat.payment_created'));
          setPaymentOpen(false);
          refetch();
          window.location.href = `/journal/${res.journal_entry_id}`;
        },
        onError: (err: unknown) => {
          const msg = err instanceof Error ? err.message : 'Failed';
          toast.error(msg);
        },
      },
    );
  }

  const toolbarActions = useMemo<ToolbarAction[]>(() => [
    {
      icon: <RefreshCw className="h-4 w-4" />,
      label: t('vat.generate'),
      onClick: () => refetch(),
      primary: true,
      disabled: isLoading,
      loading: isLoading,
    },
  ], [isLoading, t]);

  const toolbarOverflow = useMemo<ToolbarOverflowItem[]>(() => {
    const items: ToolbarOverflowItem[] = [
      {
        icon: <Download className="h-4 w-4" />,
        label: t('vat.export_csv'),
        onClick: () => downloadCsv('/reports/vat', { from_date: from, to_date: to }),
      },
    ];
    if (data) {
      items.push({
        icon: <FileCode2 className="h-4 w-4" />,
        label: t('vat.export_xml'),
        onClick: openXmlDialog,
      });
    }
    if (data && dataIsFlatRate) {
      items.push({
        icon: <CreditCard className="h-4 w-4" />,
        label: t('vat.book_payment'),
        onClick: () => setPaymentOpen(true),
        separator: true,
      });
    }
    return items;
  }, [from, to, data, dataIsFlatRate, t]);

  return (
    <div className="space-y-4">
      {/* Period Picker */}
      <StickyToolbar actions={toolbarActions} overflow={toolbarOverflow}>
        {manualMode ? (
          <>
            <Input type="date" value={manualFrom} onChange={(e) => setManualFrom(e.target.value)} className="h-8 w-36 text-sm" />
            <span className="text-xs text-muted-foreground">&ndash;</span>
            <Input type="date" value={manualTo} onChange={(e) => setManualTo(e.target.value)} className="h-8 w-36 text-sm" />
          </>
        ) : (
          <>
            <Select value={String(year)} onValueChange={(v) => setYear(parseInt(v))}>
              <SelectTrigger className="h-8 w-24 text-sm"><SelectValue /></SelectTrigger>
              <SelectContent>
                {[year - 2, year - 1, year, year + 1].map((y) => (
                  <SelectItem key={y} value={String(y)}>{y}</SelectItem>
                ))}
              </SelectContent>
            </Select>
            <Select value={periodValue} onValueChange={setPeriodValue}>
              <SelectTrigger className="h-8 w-40 text-sm"><SelectValue /></SelectTrigger>
              <SelectContent>
                {periodOptions.map((p) => (
                  <SelectItem key={p.value} value={p.value}>{p.label}</SelectItem>
                ))}
              </SelectContent>
            </Select>
          </>
        )}
        <Button
          variant="ghost"
          size="sm"
          onClick={() => setManualMode(!manualMode)}
          className="h-8 text-xs"
        >
          {manualMode ? t('vat.period_period') : t('vat.period_manual')}
        </Button>
      </StickyToolbar>

      {isLoading ? (
        <Card>
          <CardContent className="space-y-2 p-4">
            {Array.from({ length: 4 }).map((_, i) => (
              <Skeleton key={i} className="h-10 w-full" />
            ))}
          </CardContent>
        </Card>
      ) : data ? (
        <>
          {/* I. Output VAT */}
          <Card>
            <CardContent className="p-0">
              <div className="border-b px-4 py-3">
                <h3 className="font-semibold">I. {t('vat.output_vat')}</h3>
              </div>
              <VatTable
                entries={data.output_entries}
                totalLabel={t('vat.total_output')}
                totalTaxable={data.total_output_taxable}
                totalVat={data.total_output_vat}
              />
            </CardContent>
          </Card>

          {/* II. Input VAT (only for effective method) */}
          {!dataIsFlatRate && (
            <Card>
              <CardContent className="p-0">
                <div className="border-b px-4 py-3">
                  <h3 className="font-semibold">II. {t('vat.input_vat')}</h3>
                </div>
                <VatTable
                  entries={data.input_entries}
                  totalLabel={t('vat.total_input')}
                  totalTaxable={data.total_input_taxable}
                  totalVat={data.total_input_vat}
                />
              </CardContent>
            </Card>
          )}

          {/* Summary Card */}
          {dataIsFlatRate && data.flat_rate_percentage ? (
            <Card className="border-2 border-primary/20">
              <CardContent className="p-4 space-y-3">
                <h3 className="font-bold text-lg">{t('vat.sss_summary')}</h3>
                <div className="grid grid-cols-2 gap-2 text-sm">
                  <span className="text-muted-foreground">{t('vat.gross_revenue')}</span>
                  <span className="text-right font-mono">{formatAmount(data.gross_revenue ?? '0')}</span>

                  <span className="text-muted-foreground">{t('vat.flat_rate')} ({formatAmount(data.flat_rate_percentage)}%)</span>
                  <span className="text-right font-mono">{formatAmount(data.flat_rate_vat_owed ?? '0')}</span>

                  <span className="text-muted-foreground">{t('vat.collected_vat')}</span>
                  <span className="text-right font-mono">{formatAmount(data.collected_vat ?? '0')}</span>

                  <div className="col-span-2 border-t my-1" />

                  <span className="text-muted-foreground">{t('vat.saldo_ertrag')}</span>
                  <span className="text-right font-mono text-green-600">{formatAmount(data.saldo_ertrag ?? '0')}</span>

                  <span className="font-bold">{t('vat.payment_to_estv')}</span>
                  <span className="text-right font-mono font-bold text-primary">{formatAmount(data.net_vat_owed)}</span>
                </div>
              </CardContent>
            </Card>
          ) : (
            <Card className="border-2 border-primary/20">
              <CardContent className="flex items-center justify-between p-4">
                <span className="text-lg font-bold">{t('vat.net_owed')}</span>
                <span className="text-lg font-bold font-mono">
                  {formatAmount(data.total_output_vat)} &minus; {formatAmount(data.total_input_vat)} ={' '}
                  <span className="text-primary">{formatAmount(data.net_vat_owed)}</span>
                </span>
              </CardContent>
            </Card>
          )}
        </>
      ) : (
        <Card>
          <CardContent>
            <p className="py-8 text-center text-sm text-muted-foreground">
              {t('vat.generate')}
            </p>
          </CardContent>
        </Card>
      )}

      {/* Payment Dialog */}
      <Dialog open={paymentOpen} onOpenChange={setPaymentOpen}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>{t('vat.book_payment')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            <p className="text-sm text-muted-foreground">
              {t('vat.payment_description')}
            </p>
            {data && (
              <div className="rounded-md border p-3 space-y-1 text-sm">
                <div className="flex justify-between">
                  <span>{t('vat.collected_vat')}</span>
                  <span className="font-mono">{formatAmount(data.collected_vat ?? '0')}</span>
                </div>
                <div className="flex justify-between">
                  <span>{t('vat.saldo_ertrag')} (&rarr; 3809)</span>
                  <span className="font-mono text-green-600">{formatAmount(data.saldo_ertrag ?? '0')}</span>
                </div>
                <div className="flex justify-between font-bold border-t pt-1">
                  <span>{t('vat.payment_to_estv')}</span>
                  <span className="font-mono">{formatAmount(data.net_vat_owed)}</span>
                </div>
              </div>
            )}
            <div>
              <Label>{t('vat.payment_date')}</Label>
              <Input type="date" value={paymentDate} onChange={(e) => setPaymentDate(e.target.value)} />
            </div>
            <div>
              <Label>{t('vat.bank_account')}</Label>
              <Select value={bankAccountId} onValueChange={setBankAccountId}>
                <SelectTrigger><SelectValue placeholder={t('vat.select_bank_account')} /></SelectTrigger>
                <SelectContent>
                  {(bankAccounts ?? []).map((ba) => (
                    <SelectItem key={ba.id} value={ba.id}>{ba.name} ({ba.iban})</SelectItem>
                  ))}
                </SelectContent>
              </Select>
            </div>
            <Button
              onClick={handleCreatePayment}
              className="w-full"
              disabled={createPayment.isPending || !bankAccountId}
            >
              {createPayment.isPending ? t('common.saving') : t('vat.create_payment_entry')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>

      {/* eCH-0217 XML Export Dialog */}
      <Dialog open={xmlOpen} onOpenChange={setXmlOpen}>
        <DialogContent className="max-w-lg max-h-[85vh] overflow-y-auto">
          <DialogHeader>
            <DialogTitle>{t('vat.xml_dialog_title')}</DialogTitle>
          </DialogHeader>
          <div className="space-y-4">
            {/* Period (read-only) */}
            <div className="rounded-md border p-3 text-sm">
              <span className="text-muted-foreground">{t('vat.period_period')}: </span>
              <span className="font-medium">{from} &mdash; {to}</span>
            </div>

            {/* Submission type */}
            <div>
              <Label>{t('vat.submission_type')}</Label>
              <Select value={xmlSubmissionType} onValueChange={setXmlSubmissionType}>
                <SelectTrigger><SelectValue /></SelectTrigger>
                <SelectContent>
                  <SelectItem value="1">{t('vat.submission_initial')}</SelectItem>
                  <SelectItem value="2">{t('vat.submission_correction')}</SelectItem>
                  <SelectItem value="3">{t('vat.submission_annual')}</SelectItem>
                </SelectContent>
              </Select>
            </div>

            {/* Turnover Computation */}
            <div className="space-y-2">
              <h4 className="font-semibold text-sm">{t('vat.turnover_computation')}</h4>
              <div>
                <Label className="text-xs">{t('vat.total_consideration')}</Label>
                <Input type="number" step="0.01" value={xmlTotalConsideration} onChange={(e) => setXmlTotalConsideration(e.target.value)} />
              </div>
              <div className="grid grid-cols-2 gap-2">
                <div>
                  <Label className="text-xs">{t('vat.supplies_foreign')}</Label>
                  <Input type="number" step="0.01" value={xmlSuppliesForeign} onChange={(e) => setXmlSuppliesForeign(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.supplies_abroad')}</Label>
                  <Input type="number" step="0.01" value={xmlSuppliesAbroad} onChange={(e) => setXmlSuppliesAbroad(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.transfer_notification')}</Label>
                  <Input type="number" step="0.01" value={xmlTransferNotification} onChange={(e) => setXmlTransferNotification(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.supplies_exempt')}</Label>
                  <Input type="number" step="0.01" value={xmlSuppliesExempt} onChange={(e) => setXmlSuppliesExempt(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.reduction_consideration')}</Label>
                  <Input type="number" step="0.01" value={xmlReductionConsideration} onChange={(e) => setXmlReductionConsideration(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.various_deduction')}</Label>
                  <Input type="number" step="0.01" value={xmlVariousDeduction} onChange={(e) => setXmlVariousDeduction(e.target.value)} />
                </div>
              </div>
            </div>

            {/* Other Flows */}
            <div className="space-y-2">
              <h4 className="font-semibold text-sm">{t('vat.other_flows')}</h4>
              <div className="grid grid-cols-2 gap-2">
                <div>
                  <Label className="text-xs">{t('vat.subsidies')}</Label>
                  <Input type="number" step="0.01" value={xmlSubsidies} onChange={(e) => setXmlSubsidies(e.target.value)} />
                </div>
                <div>
                  <Label className="text-xs">{t('vat.donations')}</Label>
                  <Input type="number" step="0.01" value={xmlDonations} onChange={(e) => setXmlDonations(e.target.value)} />
                </div>
              </div>
            </div>

            {/* Preview Summary */}
            <div className="rounded-md border p-3 space-y-1 text-sm">
              <h4 className="font-semibold">{t('vat.preview_summary')}</h4>
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t('vat.total_deductions')}</span>
                <span className="font-mono">{totalDed.toFixed(2)}</span>
              </div>
              <div className="flex justify-between">
                <span className="text-muted-foreground">{t('vat.taxable_turnover')}</span>
                <span className="font-mono">{taxableTurnover.toFixed(2)}</span>
              </div>
              {data && (
                <div className="flex justify-between font-bold border-t pt-1">
                  <span>{t('vat.net_owed')}</span>
                  <span className="font-mono text-primary">{formatAmount(data.net_vat_owed)}</span>
                </div>
              )}
            </div>

            <Button
              onClick={handleXmlExport}
              className="w-full"
              disabled={xmlExporting || !xmlTotalConsideration}
            >
              <FileCode2 className="mr-1 h-4 w-4" />
              {xmlExporting ? t('common.saving') : t('vat.download_xml')}
            </Button>
          </div>
        </DialogContent>
      </Dialog>
    </div>
  );
}
